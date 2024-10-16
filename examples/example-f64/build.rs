use faust_build::FaustBuilder;

fn main() {
    println!("cargo:rerun-if-changed=dsp");
    FaustBuilder::new("dsp/volume.dsp", "src/dsp.rs")
        .faust_arg("-rnt".to_string())
        .faust_arg("-json".to_string())
        .faust_arg("-xml".to_string())
        .set_use_double(true)
        .set_struct_name("VolumeControl".to_string())
        .build();
}
