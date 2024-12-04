use faust_build::FaustBuilder;
use proc_macro::{TokenStream, TokenTree};
use std::{
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

fn strip_quotes(name: TokenTree) -> String {
    name.to_string()
        .strip_prefix('\"')
        .expect("prefix is not \"")
        .strip_suffix('\"')
        .expect("postfix is not \"")
        .to_string()
}

fn get_name_token(ts: TokenStream) -> String {
    // find the token that declares the name in the dsp file
    let mut ii = ts.into_iter();
    while let Some(n) = ii.next() {
        if n.to_string() == "declare" {
            if let Some(n) = ii.next() {
                if n.to_string() == "name" {
                    if let Some(name) = ii.next() {
                        if let Some(semicolon) = ii.next() {
                            if semicolon.to_string() == ";" {
                                return strip_quotes(name);
                            }
                        }
                    }
                }
            }
        }
    }
    panic! {"name declaration is not found.\n Expect 'declare name NAMESTRING;' in faust code."};
}

fn get_flags_token(ts: TokenStream) -> Vec<String> {
    // find the token that declares the flags in the dsp file
    let mut ii = ts.into_iter();
    while let Some(n) = ii.next() {
        if n.to_string() == "declare" {
            if let Some(n) = ii.next() {
                if n.to_string() == "flags" {
                    if let Some(flags) = ii.next() {
                        if let Some(semicolon) = ii.next() {
                            if semicolon.to_string() == ";" {
                                return strip_quotes(flags)
                                    .split_whitespace()
                                    .map(|s| s.to_owned())
                                    .collect();
                            }
                        }
                    }
                }
            }
        }
    }
    vec![]
}

fn write_dsp_file(path: &PathBuf, faust_code: String) {
    let dsp = File::create(path)
        .unwrap_or_else(|_| panic!("failed creating dsp file {}", path.to_str().unwrap()));
    let mut f = BufWriter::new(dsp);
    f.write_all(faust_code.as_bytes())
        .expect("Unable to write to dsp file");
}

fn faust_build(faust_code: String, name: String, flags: Vec<String>) -> TokenStream {
    // define paths for .dsp and .rs files that help debugging
    let dsp_path = Path::new(".")
        .join("target")
        .join(&name)
        .with_extension("dsp");

    let rs_path = Path::new(".")
        .join("target")
        .join(&name)
        .with_extension("rs");

    write_dsp_file(&dsp_path, faust_code);

    let dsp_path_str = dsp_path.to_str().expect("dsp file path contains non-UTF-8");
    let rs_path_str = rs_path.to_str().expect("rs file path contains non-UTF-8");

    let b = FaustBuilder::new(dsp_path_str, rs_path_str)
        .set_faust_path("/home/olaf/projects/rust/faust4rust/faust-test/faust/build/bin/faust")
        .set_struct_name(&name)
        .faust_arg("-I")
        .faust_arg("/home/olaf/projects/rust/faust4rust/faust-test/faust/libraries/")
        .set_module_name(&("dsp_".to_owned() + &name));
    let b = flags.iter().fold(b, |b, flag| b.faust_arg(&flag));

    b.build();

    let stdout = read_to_string(rs_path).expect("rs file reading failed");
    stdout.parse().expect("rs file parsing failed")
}

#[proc_macro]
pub fn faust(input: TokenStream) -> TokenStream {
    let faust_code = format!("{}", input).replace(';', ";\n");
    let name = get_name_token(input.clone());
    let flags = get_flags_token(input);
    faust_build(faust_code, name, flags)
}
