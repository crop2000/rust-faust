use dsp::{UIActiveShortname, UIPassiveShortname, Volume};
use faust_types::ParamIndex;
pub mod dsp;

fn main() {
    println!("Hello, world!");
    let mut dsp = Volume::new();
    dsp.init(44_100);
    dsp.set_param(ParamIndex(1), 1.0_f64); // alternative 1
    UIActiveShortname::channel_0_volume.set(&mut dsp, 1.0f64); //alternative 2

    dsp.set_param(ParamIndex(3), 10.0_f64);
    UIActiveShortname::channel_0_volume.set(&mut dsp, 10.0f64); //alternative 3

    let ib = [[1.0f64], [10.0f64]];
    let mut ob = [[0f64], [0f64]];
    dsp.compute(1, &ib, &mut ob);
    println!(
        "{}",
        // UIEnum_Shortname::channel_0_level.get(&mut dsp) //alternative 4
        UIPassiveShortname::channel_0_level.get(&dsp) //alternative 5
    );
    println!(
        "{}",
        UIPassiveShortname::channel_1_level.get(&dsp) //alternative 5
                                                      // UIEnum_Structured::default().channel_1.level.get(&mut dsp) //alternative 5 + 6
    );
}
