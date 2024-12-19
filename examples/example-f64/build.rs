use std::{
    fs::{self, File},
    io::BufReader,
};

use faust_build::FaustBuilder;
use faust_json::{self, Faust, GetParmInfo};
use quote::{format_ident, quote};
use syn::parse_str;

fn main() {
    println!("cargo:rerun-if-changed=dsp");
    let b = FaustBuilder::new("dsp/volume.dsp", "src/dsp.rs")
        .set_faust_path("/home/olaf/projects/rust/faust4rust/faust-test/faust/build/bin/faust")
        .faust_arg("-I")
        .faust_arg("/home/olaf/projects/rust/faust4rust/faust-test/faust/libraries/")
        .set_use_double(true);
    b.build_xml();
    b.build_json();

    let file = File::open("dsp/volume.dsp.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let f: Faust = serde_json::from_reader(reader).unwrap_or_else(|err| {
        panic!("{}", err);
    });

    let dsp_code: String = b.build_to_stdout();
    let dsp_code = parse_str::<proc_macro2::TokenStream>(&dsp_code)
        .expect("Failed to parse string into tokens");

    let module_name = format_ident!("{}", b.module_name.as_str());
    let struct_name = format_ident!("{}", b.get_struct_name());
    let parameter_info_enum = faust_json::create_enums(f.get_param_info(), &b.get_struct_name());

    let template = quote! {
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

            #parameter_info_enum
        }

        pub use #module_name::#struct_name;
        pub use #module_name::UIActiveShortname;
        pub use #module_name::UIPassiveShortname;

    };

    println!("{}", &template.to_string());
    let parsed: syn::File = syn::parse_file(&template.to_string())
        .unwrap_or_else(|err| panic!("syn failed with: {}", err.into_compile_error()));
    let pp = prettyplease::unparse(&parsed);
    fs::write("src/dsp.rs", pp).expect("failed to write to destination path");
}
