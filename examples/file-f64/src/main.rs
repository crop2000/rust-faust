use faust_types::UIGetAny;
use faust_types::UISelfSetAny;
use faust_types::UISetAny;
use std::any::Any;

pub mod dsp;
pub mod dsp2;

fn main() {
    println!("Author: {}", dsp::meta::AUTHOR);
    println!("DSP Name: {}", dsp::meta::NAME);

    // dummy buffer
    let ib = vec![vec![1.0f64], vec![1.0f64]];
    let mut ob = vec![vec![0f64], vec![0f64]];

    // a confusing example because dsp and dsp2 are different on the type level but they are not different in reality.
    let mut a1 = dsp::Amplifer::new();
    let mut a2 = dsp2::Amplifer::new(); // we rename Volume to Amplifier with CodeOption::StructName in build.rs
    a1.instance_init(44_100);
    a2.instance_init(44_100);

    let dsp1 = a1.as_compute_dsp();
    let dsp2 = a2.as_compute_dsp();

    dsp::Amplifer::class_init(44_100); // not necessary for this kind of dsp
    dsp2::Amplifer::class_init(44_100);
    let mut v = vec![dsp1, dsp2];

    // 4 ways to set a parameter
    // enum with value:
    dsp::UIActiveValue::Channel0Volume(10.0f64).set(v[0] as &mut dyn Any); // depends on rust 1.86, casting not necessary but causes false error reported by rust analyzer
                                                                           // static path to enum with value:
    dsp::DSP_UI
        .channel_1
        .volume
        .value(10.0f64)
        .set(v[0] as &mut dyn Any);
    // enum without value:
    dsp2::UIActive::Channel0Volume.set(v[1] as &mut dyn Any, 10.0f64);
    // static path to enum without value:
    dsp2::DSP_UI
        .channel_1
        .volume
        .set(v[1] as &mut dyn Any, 10.0f64);

    // run compute in a way that needs a trait object:
    for d in &mut v {
        d.compute_vec(1, ib.as_slice(), ob.as_mut_slice());
    }

    // two ways to access values returned from the dsp:
    println!(
        "1: channel0: {:?}",
        dsp::UIPassive::Channel0Level.get_enum(v[0] as &mut dyn Any)
    );
    println!(
        "1: channel1: {:?}",
        dsp::DSP_UI.channel_1.level.get_enum(v[0] as &mut dyn Any)
    );
    println!(
        "2: channel0: {}",
        dsp2::UIPassive::Channel0Level
            .get_value(v[1] as &mut dyn Any)
            .unwrap()
    );
    println!(
        "2: channel1: {}",
        dsp2::DSP_UI
            .channel_1
            .level
            .get_value(v[1] as &mut dyn Any)
            .unwrap()
    );
}
