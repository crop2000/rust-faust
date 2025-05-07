// #![warn(
//     clippy::all,
//     // clippy::restriction,
//     clippy::pedantic,
//     clippy::nursery,
//     // clippy::cargo
//     unused_crate_dependencies,
//     clippy::unwrap_used
// )]
// #![allow(clippy::missing_panics_doc)]
// #![allow(clippy::missing_const_for_fn)]

use faust_types::{UIRange, UISelfSet};
use faust_ui::utils::ui_active_sparse_update_tripple_buffer;
use rand::thread_rng;
use rand::{
    random,
    seq::{IteratorRandom, SliceRandom},
};
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::{
    io,
    thread::{self, sleep},
    time::Duration,
};
use strum::{
    Display, EnumCount, EnumDiscriminants, EnumIter, IntoStaticStr, VariantArray, VariantNames,
};

use crate::saw::Saw;
use crate::tri::Tri;

mod saw;
mod tri;

pub enum AllDsps {
    Saw(Saw),
    Tri(Tri),
}

#[derive(
    Debug, Clone, Copy, PartialEq, Display, EnumIter, EnumCount, EnumDiscriminants, VariantNames,
)]
#[strum_discriminants(derive(
    Display,
    EnumIter,
    EnumCount,
    IntoStaticStr,
    VariantArray,
    VariantNames,
    Hash
))]
#[strum_discriminants(name(AllUIActive))]
pub enum AllUIActiveValue {
    Freq(f32),
    Volume(f32),
}

impl TryFrom<AllUIActiveValue> for saw::UIActiveValue {
    type Error = Box<dyn Error>;
    fn try_from(value: AllUIActiveValue) -> Result<Self, Self::Error> {
        match value {
            AllUIActiveValue::Freq(v) => Ok(Self::Freq(v)),
            AllUIActiveValue::Volume(v) => Ok(Self::Volume(v)),
        }
    }
}

impl TryFrom<AllUIActiveValue> for tri::UIActiveValue {
    type Error = Box<dyn Error>;

    fn try_from(value: AllUIActiveValue) -> Result<Self, Self::Error> {
        match value {
            AllUIActiveValue::Freq(v) => Ok(Self::Freq(v)),
            AllUIActiveValue::Volume(v) => Ok(Self::Volume(v)),
        }
    }
}

impl UIRange for AllUIActive {
    fn min(&self) -> f32 {
        match self {
            Self::Freq => 20f32,
            Self::Volume => -50f32,
        }
    }
    fn max(&self) -> f32 {
        match self {
            Self::Freq => 2000f32,
            Self::Volume => 0f32,
        }
    }
}

impl AllUIActive {
    pub fn value(&self, value: f32) -> AllUIActiveValue {
        match self {
            Self::Freq => AllUIActiveValue::Freq(value),
            Self::Volume => AllUIActiveValue::Volume(value),
        }
    }
}

impl AllDsps {
    fn try_set(&mut self, msg: &AllUIActiveValue) -> bool {
        match self {
            Self::Saw(saw) => {
                if let Ok(active) = TryInto::<saw::UIActiveValue>::try_into(*msg) {
                    active.set(saw);
                    true
                } else {
                    false
                }
            }
            Self::Tri(tri) => match TryInto::<tri::UIActiveValue>::try_into(*msg) {
                Ok(active) => {
                    active.set(tri);
                    true
                }
                Err(_) => false,
            },
        }
    }
    fn compute(&mut self, len: usize, inputs: &Vec<&[f32]>, outputs: &mut Vec<&mut [f32]>) {
        match self {
            Self::Saw(saw) => saw.compute(len, inputs, outputs),
            Self::Tri(tri) => tri.compute(len, inputs, outputs),
        }
    }
}

fn main() {
    const NUM: usize = 2; //number of oscilators

    println!("Author: {}", saw::meta::AUTHOR);
    println!("DSP Name: {}", saw::meta::NAME);

    // Get number of inputs and ouputs
    let num_inputs = saw::FAUST_INPUTS;
    let num_outputs = saw::FAUST_OUTPUTS;

    eprintln!("inputs: {num_inputs}");
    eprintln!("outputs: {num_outputs}");
    eprintln!(
        "active params: {:?}",
        <AllUIActive as strum::VariantNames>::VARIANTS
    );
    // eprintln!(
    //     "passive params: {:?}",
    //     <AllUIPassive as strum::VariantNames>::VARIANTS
    // );
    eprintln!("UI: {:#?}", saw::DSP_UI);

    // wait-free buffers for control io

    let (mut send, mut recv) = ui_active_sparse_update_tripple_buffer();
    // Spawn a thread to do state changes.
    // This could be a GUI thread or API server.
    thread::spawn(move || {
        loop {
            let n = rand::random::<u8>() % 5;
            println!("create {n} random updates");
            let mut guard = send.input_buffer_publisher();
            for _ in 0..(n) {
                //randomly choose parameter to be updated
                let v = <AllUIActive as strum::VariantArray>::VARIANTS
                    .choose(&mut thread_rng())
                    .unwrap();

                //generate random parameter value
                let v = v.value(v.map(rand::random::<f32>()));

                //print what will be send
                eprintln!("[active]: {v:?}");

                //insert it into the "message" to be send
                guard.insert(v);
            }
            let r = guard.iter().size_hint().0;

            drop(guard); // send

            println!("send {r} different updates");

            sleep(Duration::from_millis(500));
        }
    });

    // Create JACK client
    let (client, in_ports, mut out_ports) =
        jack_utils::create_jack_client("jacktest", num_inputs, num_outputs);

    // Init DSP with a given sample rate
    let sample_rate = client.sample_rate();

    // no need
    saw::Saw::class_init(sample_rate as i32);

    let mut dsps = (0..NUM)
        .map(|_| -> _ {
            if random::<bool>() {
                println!("saw");
                let mut dsp = saw::Saw::new();
                dsp.instance_init(sample_rate as i32);
                AllDsps::Saw(dsp)
            } else {
                println!("tri");
                let mut dsp = tri::Tri::new();
                dsp.instance_init(sample_rate as i32);
                AllDsps::Tri(dsp)
            }
        })
        .collect::<Vec<_>>();

    sleep(Duration::from_secs(3));

    // Create JACK process closure that runs for each buffer
    let process_callback = move |c: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        let len = ps.n_frames() as usize;
        assert!(
            (c.sample_rate() == sample_rate),
            "unequal sr: {} {}",
            c.sample_rate(),
            sample_rate
        );

        let inputs = in_ports.iter().map(|p| p.as_slice(ps)).collect::<Vec<_>>();
        let mut outputs = out_ports
            .iter_mut()
            .map(|p| p.as_mut_slice(ps))
            .collect::<Vec<_>>();
        for channel in outputs.iter_mut() {
            for sample in channel.iter_mut() {
                *sample = 0.0f32;
            }
        }

        {
            // randomly choose a dsp to update

            let i = (0..NUM).choose(&mut thread_rng()).unwrap();
            let dsp = &mut dsps[i];

            //
            if let Some(msgs) = recv.try_read() {
                println!("                        dsp: {i} updated");

                for msg in msgs {
                    dsp.try_set(msg);
                }
            }
        }

        // run dsp computation
        for dsp in &mut dsps {
            dsp.compute(len, &inputs, &mut outputs);
        }

        // send all passive controls
        // todo

        jack::Control::Continue
    };
    // Init JACK process handler.
    let process = jack::contrib::ClosureProcessHandler::new(process_callback);

    // Activate the client, which starts the processing.
    let active_client = jack::AsyncClient::new(client, (), process).unwrap();

    // Wait for user input to quit
    println!("Press enter/return to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).ok();
    active_client.deactivate().unwrap();
}
