use crate::builder::FaustBuilder;
use proc_macro2::TokenStream;
use quote::quote;
use std::path::{Path, PathBuf};
use syn::parse_str;

pub enum Architecture {
    None,
    Function(&'static dyn Fn(&FaustBuilder, &TokenStream) -> TokenStream),
    Object(Box<dyn ObjectInterface>),
    File(PathBuf),
}

impl Architecture {
    #[cfg(feature = "faust-ui")]
    #[must_use]
    pub fn ui() -> Self {
        Self::Function(&ui)
    }
    #[must_use]
    pub fn get_file_path(&self) -> Option<&Path> {
        match self {
            Self::File(arch_file) => Some(arch_file),
            _ => None,
        }
    }

    pub(crate) fn apply(&self, builder: &FaustBuilder, dsp_code: &str) -> TokenStream {
        match self {
            Self::None => {
                //or would it be better to do really no architecture?
                let ts =
                    parse_str::<TokenStream>(dsp_code).expect("Failed to parse string into tokens");
                default(builder, &ts)
            }
            Self::Function(architecture_function) => {
                let ts =
                    parse_str::<TokenStream>(dsp_code).expect("Failed to parse string into tokens");
                architecture_function(builder, &ts)
            }
            Self::Object(architecture_interface) => {
                let ts =
                    parse_str::<TokenStream>(dsp_code).expect("Failed to parse string into tokens");
                architecture_interface.apply(builder, &ts)
            }
            Self::File(_path_buf) => {
                let struct_name: &str = builder.get_struct_name();
                let dsp_code = dsp_code
                    .replace("<<moduleName>>", "dsp")
                    .replace("<<structName>>", struct_name);
                parse_str::<TokenStream>(&dsp_code).expect("Failed to parse string into tokens")
            }
        }
    }
}

impl Default for Architecture {
    fn default() -> Self {
        Self::Function(&default)
    }
}
pub trait ObjectInterface {
    fn apply(&self, builder: &FaustBuilder, dsp_code: &TokenStream) -> TokenStream;
}

#[must_use]
pub fn default(_builder: &FaustBuilder, dsp_code: &TokenStream) -> TokenStream {
    quote! {
            #![allow(clippy::all)]
            #![allow(unused_parens)]
            #![allow(non_snake_case)]
            #![allow(non_camel_case_types)]
            #![allow(dead_code)]
            #![allow(unused_variables)]
            #![allow(unused_mut)]
            #![allow(non_upper_case_globals)]
            use faust_types::*;
            #dsp_code
    }
}

#[cfg(feature = "faust-ui")]
#[must_use]
fn ui(builder: &FaustBuilder, dsp_code: &TokenStream) -> TokenStream {
    let module_name = builder.get_module_name();
    let struct_name = builder.get_struct_name();
    let json_path = builder.get_json_path();
    match std::fs::exists(&json_path) {
        Ok(b) => {
            assert!(b, "json file not found at path: {:?}", json_path);
        }
        Err(err) => core::panic!("json file not found at path: {:?}", err),
    }
    let (ui_code, ui_reexport) =
        FaustBuilder::get_ui_from_json(&json_path, module_name, struct_name);

    let module_name = quote::format_ident!("{}", module_name);
    let struct_name = quote::format_ident!("{}", struct_name);
    quote! {
        mod #module_name {
            #![allow(clippy::all)]
            #![allow(unused_parens)]
            #![allow(non_snake_case)]
            #![allow(non_camel_case_types)]
            #![allow(dead_code)]
            #![allow(unused_variables)]
            #![allow(unused_mut)]
            #![allow(non_upper_case_globals)]
            use faust_types::*;
            #dsp_code
            #ui_code
        }

        pub use #module_name::#struct_name;
        #ui_reexport
    }
}

// Architecture Object needs a usecase first
// pub struct ArchitectureUI {}

// impl ArchitectureInterface for ArchitectureUI {
//     fn wrap(&self, builder: &FaustBuilder, dsp_code: TokenStream) -> TokenStream {
//         let module_name = builder.get_module_name();
//         let struct_name = builder.get_struct_name();
//         let json_path = builder.get_json_path();
//         match fs::exists(&json_path) {
//             Ok(b) => {
//                 assert!(b, "json file not found at path: {:?}", json_path);
//             }
//             Err(err) => panic!("json file not found at path: {:?}", err),
//         }
//         let (ui_code, ui_reexport) =
//             FaustBuilder::get_ui_from_json(&json_path, module_name, struct_name);

//         let module_name = format_ident!("{}", module_name);
//         let struct_name = format_ident!("{}", struct_name);
//         quote! {
//             mod #module_name {
//                 #![allow(clippy::all)]
//                 #![allow(unused_parens)]
//                 #![allow(non_snake_case)]
//                 #![allow(non_camel_case_types)]
//                 #![allow(dead_code)]
//                 #![allow(unused_variables)]
//                 #![allow(unused_mut)]
//                 #![allow(non_upper_case_globals)]
//                 use faust_types::*;
//                 #dsp_code
//                 #ui_code
//             }

//             pub use #module_name::#struct_name;
//             #ui_reexport
//         }
//     }
// }
