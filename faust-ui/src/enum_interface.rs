use faust_json::{FaustJson, LayoutItem};
use heck::CamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::vec;
use syn::Ident;

const UIENUMPREFIX: &str = "UI";
const UIENUMVALUE: &str = "Value";
const UIENUMACTIVE: &str = "Active";
const UIENUMPASSIVE: &str = "Passive";

#[must_use]
fn enum_active_value_ident() -> Ident {
    format_ident!("{UIENUMPREFIX}{UIENUMACTIVE}{UIENUMVALUE}")
}

#[must_use]
fn enum_passive_value_ident() -> Ident {
    format_ident!("{UIENUMPREFIX}{UIENUMPASSIVE}{UIENUMVALUE}")
}

#[must_use]
pub(crate) fn enum_active_discriminants_ident() -> Ident {
    format_ident!("{UIENUMPREFIX}{UIENUMACTIVE}")
}

#[must_use]
pub(crate) fn enum_passive_discriminants_ident() -> Ident {
    format_ident!("{UIENUMPREFIX}{UIENUMPASSIVE}")
}

struct ParamInfo {
    is_active: bool,
    shortname: Ident,
    varname: Ident,
    min: f32,
    max: f32,
}

impl ParamInfo {
    fn active(shortname: &str, varname: &str, min: f32, max: f32) -> Vec<Self> {
        vec![Self {
            is_active: true,
            shortname: format_ident!("{shortname}"),
            varname: format_ident!("{varname}"),
            min,
            max,
        }]
    }
    fn passive(shortname: &str, varname: &str, min: f32, max: f32) -> Vec<Self> {
        vec![Self {
            is_active: false,
            shortname: format_ident!("{shortname}"),
            varname: format_ident!("{varname}"),
            min,
            max,
        }]
    }
}
trait GetParmInfo {
    fn get_param_info(&self) -> Vec<ParamInfo>;
}

impl GetParmInfo for FaustJson {
    fn get_param_info(&self) -> Vec<ParamInfo> {
        self.ui
            .iter()
            .flat_map(GetParmInfo::get_param_info)
            .collect()
    }
}

impl GetParmInfo for LayoutItem {
    fn get_param_info(&self) -> Vec<ParamInfo> {
        match self {
            Self::HGroup { items, .. }
            | Self::VGroup { items, .. }
            | Self::TGroup { items, .. } => {
                items.iter().flat_map(GetParmInfo::get_param_info).collect()
            }
            Self::Button {
                shortname, varname, ..
            }
            | Self::CheckBox {
                shortname, varname, ..
            } => ParamInfo::active(&shortname.to_camel_case(), varname, 0.0, 1.0),
            Self::VSlider {
                shortname,
                varname,
                min,
                max,
                ..
            }
            | Self::HSlider {
                shortname,
                varname,
                min,
                max,
                ..
            }
            | Self::NEntry {
                shortname,
                varname,
                min,
                max,
                ..
            } => ParamInfo::active(&shortname.to_camel_case(), varname, *min, *max),
            Self::VBarGraph {
                shortname,
                varname,
                min,
                max,
                ..
            }
            | Self::HBarGraph {
                shortname,
                varname,
                min,
                max,
                ..
            } => ParamInfo::passive(&shortname.to_camel_case(), varname, *min, *max),
            Self::Soundfile {
                address, varname, ..
            } => ParamInfo::active(address, varname, 0.0, 1.0),
        }
    }
}

fn create_qualified_enum(infos: &[&ParamInfo], is_active: bool) -> TokenStream {
    let enum_name = if is_active {
        enum_active_value_ident()
    } else {
        enum_passive_value_ident()
    };
    let discriminants_name = if is_active {
        enum_active_discriminants_ident()
    } else {
        enum_passive_discriminants_ident()
    };
    let i: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| format_ident!("{}", param_info.shortname).to_token_stream())
        .collect();
    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Display, EnumIter, EnumCount,EnumDiscriminants,VariantNames)]
        #[strum_discriminants(derive(Display,EnumIter, EnumCount,IntoStaticStr,VariantArray,VariantNames,Hash))]
        #[strum_discriminants(name(#discriminants_name))]
        pub enum #enum_name {
            #(#i(FaustFloat)),*
        }
    }
}

fn create_empty_active_impl(
    dsp_name: &Ident,
    enum_name: &Ident,
    enum_name_discriminant: &Ident,
) -> TokenStream {
    quote! {
        impl UISelfSet<#dsp_name> for #enum_name {
            type F = FaustFloat;
            fn set(&self, dsp: &mut #dsp_name) {
                panic!("cannot be called")
            }
            fn get(&self) -> FaustFloat {
                panic!("cannot be called")
            }
        }
        impl UISet<#dsp_name,FaustFloat> for #enum_name_discriminant {
            fn set(&self, dsp: &mut #dsp_name, value: FaustFloat) {
                panic!("cannot be called")
            }
        }
        impl UIRange for #enum_name_discriminant {
            fn min(&self) -> f32{
                panic!("cannot be called")
            }
            fn max(&self) -> f32{
                panic!("cannot be called")
            }
        }
        impl #enum_name_discriminant {
            pub fn value(&self, value: FaustFloat) -> #enum_name {
                panic!("cannot be called")
            }
        }
    }
}

fn create_full_active_impl(
    infos: &[&ParamInfo],
    dsp_name: &Ident,
    enum_name: &Ident,
    enum_name_discriminant: &Ident,
) -> TokenStream {
    let matches_set: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| {
            let shortname = format_ident!("{}", param_info.shortname);
            let varname = format_ident!("{}", param_info.varname);
            quote! { #enum_name::#shortname(value) => dsp.#varname = *value}
        })
        .collect();
    let matches_get: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| {
            let shortname = format_ident!("{}", param_info.shortname);
            quote! { #enum_name::#shortname(value) => *value}
        })
        .collect();

    let matches_discriminant: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| {
            let shortname = format_ident!("{}", param_info.shortname);
            let varname = format_ident!("{}", param_info.varname);
            quote! { #enum_name_discriminant::#shortname => dsp.#varname = value}
        })
        .collect();

    let matches_value: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| {
            let shortname = format_ident!("{}", param_info.shortname);
            quote! { #enum_name_discriminant::#shortname => #enum_name::#shortname(value)}
        })
        .collect();

    let matches_min: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| {
            let shortname = format_ident!("{}", param_info.shortname);
            let value = proc_macro2::Literal::f32_suffixed(param_info.min);
            quote! { #enum_name_discriminant::#shortname => #value}
        })
        .collect();

    let matches_max: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| {
            let shortname = format_ident!("{}", param_info.shortname);
            let value = proc_macro2::Literal::f32_suffixed(param_info.max);
            quote! { #enum_name_discriminant::#shortname => #value}
        })
        .collect();

    quote! {
        impl UISelfSet<#dsp_name> for #enum_name {
            type F = FaustFloat;
            fn set(&self, dsp: &mut #dsp_name) {
                match self {
                    #(#matches_set ),*
                }
            }
            fn get(&self) -> FaustFloat {
                match self {
                    #(#matches_get ),*
                }
            }
        }
        impl UISet<#dsp_name,FaustFloat> for #enum_name_discriminant {
            fn set(&self, dsp: &mut #dsp_name, value: FaustFloat) {
                match self {
                    #(#matches_discriminant ),*
                }
            }
        }
        impl UIRange for #enum_name_discriminant {
            fn min(&self) -> f32{
                match self {
                    #(#matches_min ),*
                }
            }
            fn max(&self) -> f32{
                match self {
                    #(#matches_max ),*
                }
            }
        }
        impl #enum_name_discriminant {
            pub fn value(&self, value: FaustFloat) -> #enum_name {
                match self {
                    #(#matches_value ),*
                }
            }
        }
    }
}

fn create_active_impl(infos: &[&ParamInfo], dsp_name: &Ident) -> TokenStream {
    let enum_name = enum_active_value_ident();
    let enum_name_discriminant = enum_active_discriminants_ident();

    if infos.is_empty() {
        create_empty_active_impl(dsp_name, &enum_name, &enum_name_discriminant)
    } else {
        create_full_active_impl(infos, dsp_name, &enum_name, &enum_name_discriminant)
    }
}

fn create_empty_passive_impl(
    dsp_name: &Ident,
    enum_name: &Ident,
    enum_name_discriminant: &Ident,
) -> TokenStream {
    quote! {
        impl UIGet<#dsp_name> for #enum_name_discriminant {
            type E = #enum_name;
            type F = FaustFloat;
            fn get_value(&self, dsp: & #dsp_name) -> Self::F {
                panic!("cannot be called")
            }
            fn get_enum(&self, dsp: & #dsp_name) -> Self::E {
                panic!("cannot be called")
            }
        }
        impl #enum_name_discriminant {
            pub fn value(&self, value: FaustFloat) -> #enum_name {
                panic!("cannot be called")
            }
        }
        impl UIRange for #enum_name_discriminant {
            fn min(&self) -> f32{
                panic!("cannot be called")
            }
            fn max(&self) -> f32{
                panic!("cannot be called")
            }
        }
    }
}

fn create_full_passive_impl(
    infos: &[&ParamInfo],
    dsp_name: &Ident,
    enum_name: &Ident,
    enum_name_discriminant: &Ident,
) -> TokenStream {
    let matches_dsp_value: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| {
            let shortname = format_ident!("{}", param_info.shortname);
            let varname = format_ident!("{}", param_info.varname);
            quote! { #enum_name_discriminant::#shortname => dsp.#varname}
        })
        .collect();

    let matches_enum: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| {
            let shortname = format_ident!("{}", param_info.shortname);
            let varname = format_ident!("{}", param_info.varname);
            quote! { #enum_name_discriminant::#shortname => #enum_name::#shortname(dsp.#varname)}
        })
        .collect();

    let matches_value: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| {
            let shortname = format_ident!("{}", param_info.shortname);
            quote! { #enum_name_discriminant::#shortname => #enum_name::#shortname(value)}
        })
        .collect();

    let matches_min: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| {
            let shortname = format_ident!("{}", param_info.shortname);
            let value = proc_macro2::Literal::f32_suffixed(param_info.min);
            quote! { #enum_name_discriminant::#shortname => #value}
        })
        .collect();

    let matches_max: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| {
            let shortname = format_ident!("{}", param_info.shortname);
            let value = proc_macro2::Literal::f32_suffixed(param_info.max);
            quote! { #enum_name_discriminant::#shortname => #value}
        })
        .collect();

    quote! {
        impl UIGet<#dsp_name> for #enum_name_discriminant {
            type E = #enum_name;
            type F = FaustFloat;
            fn get_value(&self, dsp: & #dsp_name) -> Self::F {
                match self {
                #(#matches_dsp_value ),*
                }
            }
            fn get_enum(&self, dsp: & #dsp_name) -> Self::E {
                match self {
                #(#matches_enum ),*
                }
            }
        }
        impl #enum_name_discriminant {
            pub fn value(&self, value: FaustFloat) -> #enum_name {
                match self {
                    #(#matches_value ),*
                }
            }
        }
        impl UIRange for #enum_name_discriminant {
            fn min(&self) -> f32{
                match self {
                    #(#matches_min ),*
                }
            }
            fn max(&self) -> f32{
                match self {
                    #(#matches_max ),*
                }
            }
        }
    }
}

fn create_passive_impl(infos: &[&ParamInfo], dsp_name: &Ident) -> TokenStream {
    let enum_name: Ident = enum_passive_value_ident();

    let enum_name_discriminant = enum_passive_discriminants_ident();

    if infos.is_empty() {
        create_empty_passive_impl(dsp_name, &enum_name, &enum_name_discriminant)
    } else {
        create_full_passive_impl(infos, dsp_name, &enum_name, &enum_name_discriminant)
    }
}

fn create_from_paraminfo(v: &[ParamInfo], dsp_name: &Ident) -> TokenStream {
    let active: Vec<&ParamInfo> = v.iter().filter(|i| i.is_active).collect();
    let passive: Vec<&ParamInfo> = v.iter().filter(|i| !i.is_active).collect();
    // because of strum bug #433 we cannot create empty enums
    quote::quote! {
        use strum::{Display,EnumIter,EnumCount,EnumDiscriminants,IntoStaticStr,VariantArray,VariantNames};

        #active_enum
        #active_impl
        #passive_enum
        #passive_impl
    }
}

#[must_use]
pub fn create(dsp_json: &FaustJson, dsp_name: &Ident) -> TokenStream {
    let param_info = dsp_json.get_param_info();
    create_from_paraminfo(&param_info, dsp_name)
}
