use faust_build::architecture::Architecture;
use faust_build::builder::FaustBuilder;
use faust_build::code_option::CodeOption;
use proc_macro2::TokenStream;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=dsp");
    let dsp_folder = Path::new("dsp");
    let rs_folder = PathBuf::from("src");
    for file in fs::read_dir(dsp_folder).unwrap() {
        let file = file.unwrap();
        let path = file.path();
        dbg!(&path);
        if path.extension().unwrap_or_default() == "dsp" {
            let file = path.file_stem().unwrap();
            let mut rs = rs_folder.clone();
            rs.push(file);
            let rs = rs.with_extension("rs");
            let mut b = FaustBuilder::default_for_file_with_ui(path, rs);
            b.set_architecture(Architecture::Function(&ui));
            b.set_code_option(CodeOption::ComputeMix);
            // b.build();
        }
    }
}

#[must_use]
fn ui(builder: &FaustBuilder, dsp_code: &TokenStream) -> TokenStream {
    let c: String = dsp_code.to_string();
    let s = c.find(r"# [repr (C)]").expect(&c);
    let (a, b) = c.split_at(s);
    let c = [a, "#[derive(ComputeDsp)]", b].concat();
    let dsp_code: TokenStream = c.parse().unwrap();
    let struct_name = builder.get_struct_name();
    let json_path = builder.get_json_path();
    match std::fs::exists(&json_path) {
        Ok(b) => {
            assert!(b, "json file not found at path: {:?}", json_path);
        }
        Err(err) => core::panic!("json file not found at path: {:?}", err),
    }
    let ui_code = FaustBuilder::generate_ui_from_json(&json_path, struct_name);
    quote::quote! {

        #![allow(clippy::all)]
        #![allow(unused_parens)]
        #![allow(non_snake_case)]
        #![allow(non_camel_case_types)]
        #![allow(dead_code)]
        #![allow(unused_variables)]
        #![allow(unused_mut)]
        #![allow(non_upper_case_globals)]
        use faust_macro::{ComputeDsp};
        use faust_types::*;
        // #[derive(ComputeDsp)] // works after next faust release
        #dsp_code
        #ui_code
    }
}
