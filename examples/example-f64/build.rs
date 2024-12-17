use std::{
    fs::{self, File},
    io::BufReader,
};

use faust_build::FaustBuilder;
use faust_json::{self, Faust, LayoutItem};
use quote::quote;
use tempfile::NamedTempFile;

trait FaustParamEnumBuilder {
    fn get_isactive_shortname_varname(&self) -> Vec<(bool, String, String)>;
}

impl FaustParamEnumBuilder for Faust {
    fn get_isactive_shortname_varname(&self) -> Vec<(bool, String, String)> {
        self.ui
            .iter()
            .flat_map(|items| items.get_isactive_shortname_varname())
            .collect()
    }
}

impl FaustParamEnumBuilder for LayoutItem {
    fn get_isactive_shortname_varname(&self) -> Vec<(bool, String, String)> {
        match self {
            LayoutItem::TGroup { items, .. } => items
                .iter()
                .flat_map(|items| items.get_isactive_shortname_varname())
                .collect(),
            LayoutItem::VGroup { items, .. } => items
                .iter()
                .flat_map(|items| items.get_isactive_shortname_varname())
                .collect(),
            LayoutItem::HGroup { items, .. } => items
                .iter()
                .flat_map(|items| items.get_isactive_shortname_varname())
                .collect(),
            LayoutItem::VSlider {
                shortname, varname, ..
            } => vec![(true, shortname.clone(), varname.clone())],
            LayoutItem::HSlider {
                shortname, varname, ..
            } => vec![(true, shortname.clone(), varname.clone())],
            LayoutItem::NEntry {
                shortname, varname, ..
            } => vec![(true, shortname.clone(), varname.clone())],
            LayoutItem::Button {
                shortname, varname, ..
            } => vec![(true, shortname.clone(), varname.clone())],
            LayoutItem::CheckBox {
                shortname, varname, ..
            } => vec![(true, shortname.clone(), varname.clone())],
            LayoutItem::VBarGraph {
                shortname, varname, ..
            } => vec![(false, shortname.clone(), varname.clone())],
            LayoutItem::HBarGraph {
                shortname, varname, ..
            } => vec![(false, shortname.clone(), varname.clone())],
            LayoutItem::Soundfile {
                address, varname, ..
            } => {
                vec![(true, address.clone(), varname.clone())]
            }
        }
    }
}

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

    f.get_isactive_shortname_varname();
    let enum_string = create_enums(f.get_isactive_shortname_varname());

    let template_head = r###"
    mod <<moduleName>> {
        #![allow(clippy::all)]
        #![allow(unused_parens)]
        #![allow(non_snake_case)]
        #![allow(non_camel_case_types)]
        #![allow(dead_code)]
        #![allow(unused_variables)]
        #![allow(unused_mut)]
        #![allow(non_upper_case_globals)]
        
        use faust_types::*;
        <<includeIntrinsic>>
        <<includeclass>>


    "###;

    let template_tail = r###"


    }
    
    pub use <<moduleName>>::<<structName>>;
    pub use <<moduleName>>::UIActiveShortname;
    pub use <<moduleName>>::UIPassiveShortname;

    "###;

    let template_code = template_head.to_owned() + &enum_string + template_tail;

    let default_arch = NamedTempFile::new().expect("failed creating temporary file");
    fs::write(default_arch.path(), template_code).expect("failed writing temporary file");
    let template_path = default_arch.path().to_str().unwrap().to_owned();

    let b = b.set_arch_file(&template_path);

    b.build();
}

fn create_qualified_enum<'a>(
    v: impl Iterator<Item = &'a (bool, String, String)>,
    q: &str,
) -> String {
    let active_lines = v
        .map(|(_, shortname, _)| shortname.to_owned() + ",")
        .collect::<String>();
    let active_enum = "pub enum UI".to_owned() + q + "Shortname {" + &active_lines + "}\n";
    active_enum
}

fn create_active_impl<'a>(v: impl Iterator<Item = &'a (bool, String, String)>) -> String {
    let active_matches = v
        .map(|(b, shortname, varname)| {
            "UIActiveShortname::".to_owned() + shortname + "=> dsp." + varname + " = value,\n"
        })
        .collect::<String>();
    let active_impl = "impl UIActiveShortname {".to_owned()
        + "pub fn set(&self, dsp: &mut Volume, value: FaustFloat) {"
        + "match self {"
        + &active_matches
        + "}"
        + "}"
        + "}\n";
    active_impl
}

fn create_passive_impl<'a>(v: impl Iterator<Item = &'a (bool, String, String)>) -> String {
    let active_matches = v
        .map(|(b, shortname, varname)| {
            "UIPassiveShortname::".to_owned() + shortname + "=> dsp." + varname + ",\n"
        })
        .collect::<String>();
    let active_impl = "impl UIPassiveShortname {".to_owned()
        + "pub fn get(&self, dsp: &mut Volume) -> FaustFloat {"
        + "match self {"
        + &active_matches
        + "}"
        + "}"
        + "}\n";
    active_impl
}

fn create_enums(v: Vec<(bool, String, String)>) -> String {
    let active_iter = v.iter().filter(|(a, ..)| *a);
    let passive_iter = v.iter().filter(|(a, ..)| !*a);
    let active_enum = create_qualified_enum(active_iter.clone(), "Active");
    let passive_enum = create_qualified_enum(passive_iter.clone(), "Passive");
    let active_impl = create_active_impl(active_iter.clone());
    let passive_impl = create_passive_impl(passive_iter.clone());
    let all = active_enum + &active_impl + &passive_enum + &passive_impl;
    let t = syn::parse_file(&all).unwrap();
    let ps = prettyplease::unparse(&t);
    ps
}
