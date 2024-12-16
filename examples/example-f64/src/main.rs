use std::sync::mpsc::channel;

use crate::dsp::Volume;
use dsp::dsp::{channel_0, channel_1, UIEnum};
use faust_types::{FaustDsp, ParamIndex};
pub mod dsp;

fn main() {
    println!("Hello, world!");
    let mut dsp = Volume::new();
    dsp.init(44_100);
    dsp.set_param_enum(UIEnum::channel_0(channel_0::Volume), 1.0_f64);
    channel_0::Volume.set(&mut dsp, 1.0f64); //alternative

    dsp.set_param_enum(UIEnum::channel_1(channel_1::Volume), 10.0_f64);

    let ib = [[1.0f64], [10.0f64]];
    let mut ob = [[0f64], [0f64]];
    dsp.compute(1, &ib, &mut ob);
    println!(
        "{}",
        dsp.get_param_enum(UIEnum::channel_0(channel_0::Level))
    );
    println!(
        "{}",
        dsp.get_param_enum(UIEnum::channel_1(channel_1::Level))
    );
}
