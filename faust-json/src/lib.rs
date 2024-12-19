use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FaustJson {
    pub name: String,
    pub filename: String,
    pub version: String,
    pub compile_options: String,
    #[serde(default)] //allow empty list
    pub library_list: Vec<String>,
    #[serde(default)] //allow empty list
    pub include_pathnames: Vec<String>,
    pub size: u32,
    pub inputs: usize,
    pub outputs: usize,
    pub author: Option<String>,
    pub license: Option<String>,
    pub copyright: Option<String>,
    pub classname: Option<String>,
    #[serde(default)] //allow empty list
    pub meta: Vec<Meta>,
    #[serde(default)] //allow empty list
    pub ui: Vec<LayoutItem>,
}

#[derive(Debug, PartialEq)]
pub struct Meta {
    pub key: String,
    pub value: String,
}

impl<'de> Deserialize<'de> for Meta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map: std::collections::HashMap<String, Option<String>> =
            Deserialize::deserialize(deserializer).unwrap();
        let Some((key, Some(value))): Option<(&String, &Option<String>)> = map.iter().next() else {
            panic!("bla")
        };

        Ok(Meta {
            key: key.to_owned(),
            value: value.to_owned(),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum LayoutItem {
    TGroup {
        label: String,
        items: Vec<LayoutItem>,
        #[serde(default)]
        meta: Vec<Meta>,
    },
    VGroup {
        label: String,
        items: Vec<LayoutItem>,
        #[serde(default)]
        meta: Vec<Meta>,
    },
    HGroup {
        label: String,
        items: Vec<LayoutItem>,
        #[serde(default)]
        meta: Vec<Meta>,
    },
    VSlider {
        label: String,
        shortname: String,
        address: String,
        varname: String,
        init: f32,
        min: f32,
        max: f32,
        step: f32,
        #[serde(default)]
        meta: Vec<Meta>,
    },
    HSlider {
        label: String,
        shortname: String,
        address: String,
        varname: String,
        init: f32,
        min: f32,
        max: f32,
        step: f32,
        #[serde(default)]
        meta: Vec<Meta>,
    },
    NEntry {
        label: String,
        shortname: String,
        address: String,
        varname: String,
        init: Option<f32>,
        min: f32,
        max: f32,
        step: f32,
        #[serde(default)]
        meta: Vec<Meta>,
    },
    Button {
        label: String,
        shortname: String,
        address: String,
        varname: String,
        init: Option<f32>,
        #[serde(default)]
        meta: Vec<Meta>,
    },
    CheckBox {
        label: String,
        shortname: String,
        address: String,
        varname: String,
        init: Option<f32>,
        #[serde(default)]
        meta: Vec<Meta>,
    },
    VBarGraph {
        label: String,
        shortname: String,
        address: String,
        varname: String,
        min: f32,
        max: f32,
        #[serde(default)]
        meta: Vec<Meta>,
    },
    HBarGraph {
        label: String,
        shortname: String,
        address: String,
        varname: String,
        min: f32,
        max: f32,
        #[serde(default)]
        meta: Vec<Meta>,
    },

    Soundfile {
        label: String,
        url: String,
        address: String,
        varname: String,
        #[serde(default)]
        meta: Vec<Meta>,
    },
}

pub struct ParamInfo {
    is_active: bool,
    shortname: String,
    varname: String,
}

pub trait GetParmInfo {
    fn get_param_info(&self) -> Vec<ParamInfo>;
}

impl GetParmInfo for FaustJson {
    fn get_param_info(&self) -> Vec<ParamInfo> {
        self.ui
            .iter()
            .flat_map(|items| items.get_param_info())
            .collect()
    }
}

impl GetParmInfo for LayoutItem {
    fn get_param_info(&self) -> Vec<ParamInfo> {
        match self {
            LayoutItem::TGroup { items, .. } => items
                .iter()
                .flat_map(|items| items.get_param_info())
                .collect(),
            LayoutItem::VGroup { items, .. } => items
                .iter()
                .flat_map(|items| items.get_param_info())
                .collect(),
            LayoutItem::HGroup { items, .. } => items
                .iter()
                .flat_map(|items| items.get_param_info())
                .collect(),
            LayoutItem::VSlider {
                shortname, varname, ..
            } => vec![ParamInfo {
                is_active: true,
                shortname: shortname.to_string(),
                varname: varname.to_string(),
            }],
            LayoutItem::HSlider {
                shortname, varname, ..
            } => vec![ParamInfo {
                is_active: true,
                shortname: shortname.to_string(),
                varname: varname.to_string(),
            }],
            LayoutItem::NEntry {
                shortname, varname, ..
            } => vec![ParamInfo {
                is_active: true,
                shortname: shortname.to_string(),
                varname: varname.to_string(),
            }],
            LayoutItem::Button {
                shortname, varname, ..
            } => vec![ParamInfo {
                is_active: true,
                shortname: shortname.to_string(),
                varname: varname.to_string(),
            }],
            LayoutItem::CheckBox {
                shortname, varname, ..
            } => vec![ParamInfo {
                is_active: true,
                shortname: shortname.to_string(),
                varname: varname.to_string(),
            }],
            LayoutItem::VBarGraph {
                shortname, varname, ..
            } => vec![ParamInfo {
                is_active: false,
                shortname: shortname.to_string(),
                varname: varname.to_string(),
            }],
            LayoutItem::HBarGraph {
                shortname, varname, ..
            } => vec![ParamInfo {
                is_active: false,
                shortname: shortname.to_string(),
                varname: varname.to_string(),
            }],
            LayoutItem::Soundfile {
                address, varname, ..
            } => {
                vec![ParamInfo {
                    is_active: false,
                    shortname: address.to_string(),
                    varname: varname.to_string(),
                }]
            }
        }
    }
}

fn create_qualified_enum(infos: &[&ParamInfo], quality: &str) -> TokenStream {
    let enumname = format_ident!("UI{}Shortname", quality);
    let i: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| format_ident!("{}", param_info.shortname).to_token_stream())
        .collect();
    quote! { pub enum #enumname {
            #(#i ),*
        }
    }
}

fn create_active_impl(infos: &[&ParamInfo], dsp_name: &str) -> TokenStream {
    let enumname = format_ident!("UIActiveShortname");
    let dsp_name = format_ident!("{dsp_name}");
    let matches: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| {
            let shortname = format_ident!("{}", param_info.shortname);
            let varname = format_ident!("{}", param_info.varname);
            quote! { #enumname::#shortname => dsp.#varname = value}
        })
        .collect();
    quote! {
        impl #enumname {
            pub fn set(&self, dsp: &mut #dsp_name, value: FaustFloat) {
                match self {
                    #(#matches ),*
                }
            }
        }
    }
}

pub fn create_passive_impl(infos: &[&ParamInfo], dsp_name: &str) -> TokenStream {
    let enumname = format_ident!("UIPassiveShortname");
    let dsp_name = format_ident!("{dsp_name}");
    let matches: Vec<TokenStream> = infos
        .iter()
        .map(|param_info| {
            let shortname = format_ident!("{}", param_info.shortname);
            let varname = format_ident!("{}", param_info.varname);
            quote! { #enumname::#shortname => dsp.#varname}
        })
        .collect();
    quote! {
        impl #enumname {
            pub fn get(&self, dsp: & #dsp_name) -> FaustFloat {
                match self {
                #(#matches ),*
                }
            }
        }
    }
}

pub fn create_enums(v: Vec<ParamInfo>, dsp_name: &str) -> TokenStream {
    let active: Vec<&ParamInfo> = v.iter().filter(|i| i.is_active).collect();
    let passive: Vec<&ParamInfo> = v.iter().filter(|i| !i.is_active).collect();
    let (active_enum, active_impl) = if active.is_empty() {
        (TokenStream::new(), TokenStream::new())
    } else {
        (
            create_qualified_enum(&active, "Active"),
            create_active_impl(&active, dsp_name),
        )
    };
    let (passive_enum, passive_impl) = if active.is_empty() {
        (TokenStream::new(), TokenStream::new())
    } else {
        (
            create_qualified_enum(&passive, "Passive"),
            create_passive_impl(&passive, dsp_name),
        )
    };
    quote::quote! {
        #active_enum
        #active_impl
        #passive_enum
        #passive_impl
    }
}
