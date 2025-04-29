#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    // clippy::cargo
    unused_crate_dependencies,
    clippy::unwrap_used

)]
#![allow(clippy::missing_panics_doc)]

use faust_build::macro_lib::{
    build_dsp_code_from_macro, build_faust_file_from_macro, derive_faust_dsp, FileMacroArgs,
};
use syn::DeriveInput;

#[proc_macro]
pub fn dsp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    build_dsp_code_from_macro(&input.into()).into()
}

#[proc_macro]
pub fn include(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(input as FileMacroArgs);
    build_faust_file_from_macro(args).into()
}

#[proc_macro_derive(FaustDsp)]
pub fn faust_dsp(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, .. } = syn::parse_macro_input!(item);
    derive_faust_dsp(&ident).into()
}

#[proc_macro_derive(ComputeDsp)]
pub fn compute_dsp(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, .. } = syn::parse_macro_input!(item);
    faust_build::macro_lib::derive_compute_dsp(&ident).into()
}

#[proc_macro_derive(InitDsp)]
pub fn init_dsp(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, .. } = syn::parse_macro_input!(item);
    faust_build::macro_lib::derive_init_dsp(&ident).into()
}

#[proc_macro_derive(InPlaceDsp)]
pub fn inplace_dsp(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, .. } = syn::parse_macro_input!(item);
    faust_build::macro_lib::derive_inplace_dsp(&ident).into()
}

#[proc_macro_derive(ExternalControlDsp)]
pub fn external_control_dsp(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, .. } = syn::parse_macro_input!(item);
    faust_build::macro_lib::derive_external_control_dsp(&ident).into()
}
