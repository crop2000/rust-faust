use faust_build::FaustBuilder;

fn main() {
    println!("cargo:rerun-if-changed=dsp");
    FaustBuilder::new("dsp/volume.dsp", "src/dsp.rs").build();
}
