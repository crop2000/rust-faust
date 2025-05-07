use faust_build::{architecture::Architecture, builder::FaustBuilder, code_option::CodeOption};
use proc_macro2::TokenStream;

fn main() {
    println!("cargo:rerun-if-changed=dsp");
    //example of setting up compilation
    // without any conveniens functions
    let mut b = FaustBuilder::default();
    b.set_dsp_path("dsp/volume.dsp");
    b.set_out_path("src/dsp.rs");
    b.set_faust_path("/home/olaf/projects/rust/faust4rust/faust-test/faust/build/bin/faust");
    b.set_import_dir("/home/olaf/projects/rust/faust4rust/faust-test/faust/libraries/");
    b.write_json_file();
    b.write_xml_file();
    b.set_code_option(CodeOption::StructName("Amplifer".to_owned()));
    b.set_code_option(CodeOption::Double);
    b.set_code_option(CodeOption::NoFaustDsp);
    b.set_code_option(CodeOption::ExternalControl);
    b.set_architecture(Architecture::Function(&ui));
    b.build();

    let mut b = FaustBuilder::default();
    b.set_dsp_path("dsp/volume.dsp");
    b.set_out_path("src/dsp2.rs");
    b.set_faust_path("/home/olaf/projects/rust/faust4rust/faust-test/faust/build/bin/faust");
    b.set_import_dir("/home/olaf/projects/rust/faust4rust/faust-test/faust/libraries/");
    b.write_json_file();
    b.write_xml_file();
    b.set_code_option(CodeOption::StructName("Amplifer".to_owned()));
    b.set_code_option(CodeOption::Double);
    b.set_code_option(CodeOption::NoFaustDsp);
    b.set_code_option(CodeOption::ExternalControl);
    b.set_architecture(Architecture::Function(&ui));
    b.build();
}

#[must_use]
fn ui(builder: &FaustBuilder, dsp_code: &TokenStream) -> TokenStream {
    let c: String = dsp_code.to_string();
    let s = c.find(r"# [repr (C)]").expect(&c);
    let (a, b) = c.split_at(s);
    let c = [a, "#[derive(ComputeDsp, ExternalControlDsp)]", b].concat();
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
        use faust_macro::{ComputeDsp,ExternalControlDsp};
        use faust_types::*;
        // #[derive(ComputeDsp)]
        #dsp_code
        #ui_code
    }
}
