use faust_build::faust_arg::{FaustArg, FaustArgs2Args};
use faust_json::{self, FaustJson, GetParmInfo};
use proc_macro::{TokenStream, TokenTree};
use quote::{format_ident, quote};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
};
use syn::parse_str;
use tempfile::NamedTempFile;

fn strip_quotes(name: TokenTree) -> String {
    name.to_string()
        .strip_prefix('\"')
        .expect("prefix is not \"")
        .strip_suffix('\"')
        .expect("postfix is not \"")
        .to_string()
}

fn get_declared_value(key: &str, ts: TokenStream) -> Option<String> {
    // find the token that declares a key in the dsp file
    let mut ii = ts.into_iter();
    while let Some(n) = ii.next() {
        if n.to_string() == "declare" {
            if let Some(n) = ii.next() {
                if n.to_string() == key {
                    if let Some(value) = ii.next() {
                        if let Some(semicolon) = ii.next() {
                            if semicolon.to_string() == ";" {
                                return Some(strip_quotes(value));
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn get_name_token(ts: TokenStream) -> String {
    get_declared_value("name", ts)
        .expect("name declaration is not found.\n Expect 'declare name NAMESTRING;' in faust code.")
}

fn get_flags_token(ts: TokenStream) -> Vec<String> {
    match get_declared_value("flags", ts) {
        None => vec![],
        Some(s) => s.split_whitespace().map(|s| s.to_owned()).collect(),
    }
}

fn write_temp_dsp_file(faust_code: String) -> NamedTempFile {
    let temp_dsp = NamedTempFile::new().expect("failed creating temp dsp file");
    let mut f = BufWriter::new(temp_dsp);
    f.write_all(faust_code.as_bytes())
        .expect("Unable to write to temp dsp file");
    f.into_inner().expect("temp dsp error on flush")
}

fn faust_command(name: &str, temp_dsp_path: &Path, flags: Vec<String>) -> Command {
    let mut faust =
        Command::new("/home/olaf/projects/rust/faust4rust/faust-test/faust/build/bin/faust");
    let mut args: Vec<FaustArg> = Vec::new();
    let faust_libs =
        PathBuf::from("/home/olaf/projects/rust/faust4rust/faust-test/faust/libraries/");
    args.push(FaustArg::ImportDir(&faust_libs));

    args.push(FaustArg::default_lang());
    args.push(FaustArg::default_timeout());
    args.push(FaustArg::StructName(name));

    for arg in flags.iter() {
        args.push(FaustArg::Arg(arg))
    }

    args.push(FaustArg::Json());
    args.push(FaustArg::DspFile(temp_dsp_path));

    faust.args(args.to_args());
    faust
}

fn fill_template(
    dsp_code: proc_macro2::TokenStream,
    name: &str,
    faust_json: FaustJson,
) -> TokenStream {
    let module_name = format_ident!("dsp_{}", name);
    let struct_name = format_ident!("{}", name);
    let parameter_info_enum = faust_json::create_enums(faust_json.get_param_info(), name);

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

    template.into()
}

fn faust_build(input: TokenStream) -> TokenStream {
    let faust_code = format!("{}", input).replace(';', ";\n");
    let name = get_name_token(input.clone());
    let flags = get_flags_token(input.clone());

    let temp_dsp = write_temp_dsp_file(faust_code);
    let temp_dsp_path = temp_dsp.path();
    let temp_json_path = PathBuf::from(
        temp_dsp_path
            .to_str()
            .expect("tmp file is not utf8")
            .to_owned()
            + ".json",
    );

    // define paths for .dsp and .json files that help debugging
    let debug_dsp = Path::new(".")
        .join("target")
        .join("DEBUG_".to_owned() + &name)
        .with_extension("dsp");

    let debug_json = Path::new(".")
        .join("target")
        .join("DEBUG_".to_owned() + &name)
        .with_extension("json");

    let debug_rs = Path::new(".")
        .join("target")
        .join("DEBUG_".to_owned() + &name)
        .with_extension("rs");

    if cfg!(debug_assertions) {
        fs::copy(temp_dsp_path, &debug_dsp).expect("temp dsp file cannot be copied to target");
    } else {
        let _ignore_error = fs::remove_file(&debug_dsp);
    }

    let mut faust = faust_command(&name, temp_dsp_path, flags);
    let output = faust.output().expect("Failed to execute faust");
    if !output.status.success() {
        panic!(
            "faust compilation failed: {}",
            String::from_utf8(output.stderr).unwrap()
        );
    }
    let dsp_code: String =
        String::from_utf8(output.stdout).expect("could not parse stdout from command");

    if cfg!(debug_assertions) {
        fs::copy(&temp_json_path, &debug_json).expect("temp json file cannot be copied to target");
    } else {
        let _ignore_error = fs::remove_file(&debug_json);
    }

    if cfg!(debug_assertions) {
        fs::write(debug_rs, &dsp_code).expect("failed to write debug rs file");
    } else {
        let _ignore_error = fs::remove_file(&debug_dsp);
    }

    let dsp_code = parse_str::<proc_macro2::TokenStream>(&dsp_code)
        .expect("Failed to parse string into tokens");

    let json_file = File::open(temp_json_path).expect("Failed to open json file");
    let json_reader = BufReader::new(json_file);
    let faust_json: FaustJson = serde_json::from_reader(json_reader).unwrap_or_else(|err| {
        panic!("json parsing error: {}", err);
    });

    fill_template(dsp_code, &name, faust_json)
}

#[proc_macro]
pub fn faust(input: TokenStream) -> TokenStream {
    faust_build(input)
}
