use faust_build::FaustBuilder;

fn main() {
    println!("cargo:rerun-if-changed=dsp");
    // FaustBuilder::new("dsp/volume.dsp", "src/dsp.rs")
    //     .faust_arg("-uim".to_string())
    //     .build();
}
