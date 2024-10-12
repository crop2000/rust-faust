use faust_build::{build_dsp, FaustBuilder};

fn main() {
    println!("cargo:rerun-if-changed=dsp");
    // build_dsp("dsp/volume.dsp");
    FaustBuilder::new("dsp/volume.dsp", "src/dsp.rs")
        .faust_arg("-uim".to_string())
        .set_struct_name("JVolume".to_owned())
        .set_module_name("dsp_volume".to_owned())
        .build();
}
