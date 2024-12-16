use std::sync::mpsc::channel;

use crate::dsp::Volume;
use dsp::dsp::{
    channel_0, channel_1, UIEnum, UIEnum_Address, UIEnum_Passive_Shortname, UIEnum_Shortname,
    UIEnum_Structured,
};
use faust_types::{FaustDsp, ParamIndex};
pub mod dsp;

fn main() {
    println!("Hello, world!");
    let mut dsp = Volume::new();
    dsp.init(44_100);
    dsp.set_param_enum(UIEnum::channel_0(channel_0::Volume), 1.0_f64); // alternative 1
    channel_0::Volume.set(&mut dsp, 1.0f64); //alternative 2

    dsp.set_param_enum(UIEnum::channel_1(channel_1::Volume), 10.0_f64);
    UIEnum_Address::volumecontrol_channel_0_volume.set(&mut dsp, 10.0f64); //alternative 3

    let ib = [[1.0f64], [10.0f64]];
    let mut ob = [[0f64], [0f64]];
    dsp.compute(1, &ib, &mut ob);
    println!(
        "{}",
        UIEnum_Shortname::channel_0_level.get(&mut dsp) //alternative 4
    );
    println!(
        "{}",
        UIEnum_Passive_Shortname::channel_1_level.get(&mut dsp) //alternative 5
    );
    println!(
        "{}",
        UIEnum_Structured::default().channel_1.level.get(&mut dsp) //alternative 5 + 6
    );
}
