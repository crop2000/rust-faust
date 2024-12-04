use faust_build::FaustBuilder;

fn main() {
    println!("cargo:rerun-if-changed=dsp");
    let b = FaustBuilder::new("dsp/volume.dsp", "src/dsp.rs")
        .set_faust_path("/home/olaf/projects/rust/faust4rust/faust-test/faust/build/bin/faust")
        .faust_arg("-I")
        .faust_arg("/home/olaf/projects/rust/faust4rust/faust-test/faust/libraries/")
        .set_use_double(true);
    b.build();
    b.build_xml();
}
