#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    // clippy::cargo
)]
#![allow(clippy::missing_panics_doc)]

use crate::{
    builder::{get_declared_value, FaustBuilder},
    code_option::CodeOption,
};
use heck::SnakeCase;
use std::{env, iter::FromIterator, path::PathBuf, str::FromStr};
use syn::{parse::Parse, Error, Expr, ExprArray, ExprPath, LitStr, Token};

fn get_flags_token(ts: proc_macro2::TokenStream) -> Vec<String> {
    get_declared_value("flags", ts).map_or_else(std::vec::Vec::new, |s| {
        s.split_whitespace()
            .map(std::borrow::ToOwned::to_owned)
            .collect()
    })
}

pub struct FileMacroArgs {
    pub dsp_path: LitStr,
    pub flags: Vec<CodeOption>,
}

impl FileMacroArgs {
    fn parse_enums(input_expr: ExprArray) -> syn::Result<Vec<CodeOption>> {
        let elems = input_expr.elems;
        elems
            .iter()
            .map(|expr| {
                let Expr::Path(ExprPath { path, .. }) = expr else {
                    return Result::Err(Error::new_spanned(
                        expr,
                        "Can not parse Array Element as Enum Variant",
                    ));
                };

                let Some(name) = path.get_ident().map(std::string::ToString::to_string) else {
                    return Result::Err(Error::new_spanned(
                        path,
                        "Can not parse Array Element as CodeGenerationOption Enum Variant",
                    ));
                };
                let Ok(fa) = CodeOption::from_str(&name) else {
                    return Result::Err(Error::new_spanned(
                        path,
                        format!("Can not parse Array Element as CodeGenerationOption Enum Variant {name}"),
                    ));
                };
                Ok(fa)
            })
            .collect()
    }
}

impl Parse for FileMacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let dsp_path = input.parse()?;
        if input.is_empty() {
            Ok(Self {
                dsp_path,
                flags: Vec::new(),
            })
        } else {
            let _comma: Token![,] = input.parse()?;
            let flags = Self::parse_enums(input.parse()?)?;
            Ok(Self { dsp_path, flags })
        }
    }
}

#[cfg(feature = "faust-ui")]
#[must_use]
pub fn build_faust_file_from_macro(args: FileMacroArgs) -> proc_macro2::TokenStream {
    use crate::code_option::CodeOptionMap;

    let source_file =
        env::var("CARGO_MANIFEST_DIR").expect("environment variable CARGO_MANIFEST_DIR is not set");
    let folder: PathBuf = source_file.into();
    let flags = CodeOptionMap::from_iter(args.flags);
    let relative_dsp_path: PathBuf = args.dsp_path.value().into();
    let dsp_path = folder.join(&relative_dsp_path);
    assert!(
        dsp_path.exists(),
        "dsp file does not exist at: {:?}",
        dsp_path
    );

    let builder = FaustBuilder::default_for_include_macro(dsp_path, flags);
    builder.build()
}

#[cfg(feature = "faust-ui")]
#[must_use]
pub fn build_dsp_code_from_macro(input: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let faust_code = format!("{input}").replace(';', ";\n");

    let flags = get_flags_token(input.clone());
    let flags = CodeOption::arg_map_from_str_iter(flags.iter());

    let builder = FaustBuilder::default_for_dsp_macro(&faust_code, flags);

    builder.write_debug_dsp_file(&builder.get_struct_name().to_snake_case());
    let dsp_code = builder.build();
    builder.write_debug_json_file(&builder.get_struct_name().to_snake_case());
    dsp_code
}

#[must_use]
pub fn derive_faust_dsp(ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    quote::quote! {
    impl FaustDsp for #ident {
        type T = FaustFloat;
        fn new() -> Self
        where
            Self: Sized,
        {
            Self::new()
        }
        fn metadata(&self, m: &mut dyn Meta) {
            self.metadata(m)
        }
        fn get_sample_rate(&self) -> i32 {
            self.get_sample_rate()
        }
        fn get_num_inputs(&self) -> i32 {
            FAUST_INPUTS as i32
        }
        fn get_num_outputs(&self) -> i32 {
            FAUST_OUTPUTS as i32
        }
        fn class_init(sample_rate: i32)
        where
            Self: Sized,
        {
            Self::class_init(sample_rate);
        }
        fn instance_reset_params(&mut self) {
            self.instance_reset_params()
        }
        fn instance_clear(&mut self) {
            self.instance_clear()
        }
        fn instance_constants(&mut self, sample_rate: i32) {
            self.instance_constants(sample_rate)
        }
        fn instance_init(&mut self, sample_rate: i32) {
            self.instance_init(sample_rate)
        }
        fn init(&mut self, sample_rate: i32) {
            self.init(sample_rate)
        }
        fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
            self.build_user_interface(ui_interface)
        }
        fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>)
        where
            Self: Sized,
        {
            Self::build_user_interface_static(ui_interface);
        }
        fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
            self.get_param(param)
        }
        fn set_param(&mut self, param: ParamIndex, value: Self::T) {
            self.set_param(param, value)
        }
        fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
            self.compute(count as usize, inputs, outputs)
        }
    }
        }
}

#[must_use]
pub fn derive_compute_dsp(ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    quote::quote! {
        impl ComputeDsp for #ident {
            type F = FaustFloat;
            fn compute(&mut self, count: i32, inputs: &[&[Self::F]], outputs: &mut [&mut [Self::F]]) {
                self.compute(count as usize, inputs, outputs)
            }
            fn compute_vec(&mut self, count: i32, inputs: &[Vec<Self::F>], outputs: &mut [Vec<Self::F>]) {
                self.compute(count as usize, inputs, outputs)
            }
        }

        impl<'a> #ident{
            pub fn as_compute_dsp(&'a mut self) -> &'a mut dyn ComputeDsp<
            F = <#ident as ComputeDsp>::F,
        > {
                self as &'a mut dyn ComputeDsp<
                        F = <#ident as ComputeDsp>::F,
                    >
            }
        }
    }
}

#[must_use]
pub fn derive_faustfloat_dsp(ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    quote::quote! {
        impl FaustFloatDsp for #ident {
            type F = FaustFloat;
        }
    }
}

#[must_use]
pub fn derive_init_dsp(ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    quote::quote! {
        impl InitDsp for #ident {
            fn instance_init(&mut self, sample_rate: i32) {
                self.instance_init(sample_rate)
            }
        }
    }
}

#[must_use]
pub fn derive_inplace_dsp(ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    quote::quote! {
        impl InPlaceDsp for #ident where Self: FaustFloatDsp<F = FaustFloat>{
            fn compute(&mut self, count: i32, ios: &mut [&mut [Self::F]]) {
                self.compute(count as usize, ios)
            }
            fn compute_vec(&mut self, count: i32, ios: &mut [Vec<Self::F>]) {
                self.compute(count as usize, ios)
            }
        }
        impl<'a> #ident {
            pub fn as_inplace_dsp(
                &'a mut self,
            ) -> &'a mut dyn InPlaceDsp<F = <#ident as faust_types::FaustFloatDsp>::F> {
                self
            }
        }
    }
}

#[must_use]
pub fn derive_external_control_dsp(ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    quote::quote! {
        impl ExternalControlDsp for #ident where Self: FaustFloatDsp<F = FaustFloat>{
            type S = UIActive;
            type V = UIActiveValue;
            fn control(&mut self) {
                self.control()
            }
            fn update_controls(&mut self,control: &[&Self::F]) {
                //this would be different if faust wouldn't keep a copy of the controls in the struct
                <Self::S as strum::VariantArray>::VARIANTS
                .iter()
                .zip(control.iter())
                .for_each(|(k, v)| UISet::set(k, self, **v));
                self.control()
            }
            fn update_control_values(&mut self,controls: &[&Self::V]) {
                //this would be different if faust wouldn't keep a copy of the controls in the struct
                controls.iter().for_each(|v|
                    UISelfSet::<#ident>::set(*v, self)
                );
                self.control()
            }
        }
    }
}
