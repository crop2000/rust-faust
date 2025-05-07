use crate::saw::FaustFloat;
use faust_types::{ComputeDsp, FaustFloatDsp, UIRange, UISelfSetAny};
use faust_ui::utils::ui_active_sparse_update_tripple_buffer;
use rand::{
    random,
    seq::{IteratorRandom, SliceRandom},
};
use std::{
    any::Any,
    io,
    thread::{self, sleep},
    time::Duration,
};
use strum::VariantArray as _;

mod saw;
mod tri;

const NUM: usize = 2;

impl From<tri::UIActiveValue> for saw::UIActiveValue {
    fn from(value: tri::UIActiveValue) -> Self {
        match value {
            tri::UIActiveValue::Freq(v) => Self::Freq(v),
            tri::UIActiveValue::Volume(v) => Self::Volume(v),
        }
    }
}

impl From<saw::UIActiveValue> for tri::UIActiveValue {
    fn from(value: saw::UIActiveValue) -> Self {
        match value {
            saw::UIActiveValue::Freq(v) => Self::Freq(v),
            saw::UIActiveValue::Volume(v) => Self::Volume(v),
        }
    }
}

fn main() {
    println!("Author: {}", saw::meta::AUTHOR);
    println!("DSP Name: {}", saw::meta::NAME);

    // Get number of inputs and ouputs
    let num_inputs = saw::FAUST_INPUTS;
    let num_outputs = saw::FAUST_OUTPUTS;

    eprintln!("inputs: {num_inputs}");
    eprintln!("outputs: {num_outputs}");
    eprintln!("active params: {:?}", saw::UIActive::VARIANTS);
    eprintln!("passive params: {:?}", saw::UIPassive::VARIANTS);
    eprintln!("UI: {:#?}", saw::DSP_UI);

    // wait-free buffers for control io

    let (mut send, mut recv) = ui_active_sparse_update_tripple_buffer();
    // Spawn a thread to do state changes.
    // This could be a GUI thread or API server.
    thread::spawn(move || loop {
        let n = rand::random::<u8>() % 5;
        println!("create {n} random updates");
        let mut guard = send.input_buffer_publisher();
        for _ in 0..(n) {
            //randomly choose parameter to be updated
            let v = saw::UIActive::VARIANTS
                .choose(&mut rand::thread_rng())
                .unwrap();

            //generate random parameter value
            let v = v.value(v.map(rand::random::<f32>()));

            //print what will be send
            eprintln!("[active]: {:?}", v);

            //insert it into the "message" to be send
            guard.insert(v);
        }
        let r = guard.iter().size_hint().0;

        drop(guard); // send

        println!("send {r} different updates");

        sleep(Duration::from_millis(500));
    });

    // Create JACK client
    let (client, in_ports, mut out_ports) =
        jack_utils::create_jack_client("jacktest", num_inputs, num_outputs);

    // Init DSP with a given sample rate
    let sample_rate = client.sample_rate();

    // no need
    saw::Saw::class_init(sample_rate as i32);
    tri::Tri::class_init(sample_rate as i32);

    let mut dsps = (0..NUM)
        .map(|_| -> _ {
            if random::<bool>() {
                println!("saw");
                let mut dsp = Box::new(saw::Saw::new());
                dsp.instance_init(sample_rate as i32);
                dsp as Box<dyn ComputeDsp<F = <saw::Saw as FaustFloatDsp>::F>>
            } else {
                println!("tri");
                let mut dsp = Box::new(tri::Tri::new());
                dsp.instance_init(sample_rate as i32);
                dsp as Box<dyn ComputeDsp<F = <tri::Tri as FaustFloatDsp>::F>>
            }
        })
        .collect::<Vec<_>>();

    sleep(Duration::from_secs(3));

    // Create JACK process closure that runs for each buffer
    let process_callback = move |c: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        let len = ps.n_frames() as usize;
        if c.sample_rate() != sample_rate {
            panic!("unequal sr: {} {}", c.sample_rate(), sample_rate)
        }

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

            let i = (0..NUM).choose(&mut rand::thread_rng()).unwrap();
            let dsp = &mut dsps[i];

            //
            if let Some(msgs) = recv.try_read() {
                println!("                        dsp: {i} updated");
                let dsp = &mut **dsp;
                for msg in msgs {
                    //if a function returns true or-clause aborts
                    let _ = Into::<saw::UIActiveValue>::into(*msg).set(&mut *dsp as &mut dyn Any)
                        || Into::<tri::UIActiveValue>::into(*msg).set(&mut *dsp as &mut dyn Any)
                        || should_never_panic(dsp);
                }
            }
        }
        for dsp in dsps.iter_mut() {
            dsp.compute(len, &inputs, &mut outputs);
        }

        // run dsp computation

        // send all passive controls
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

fn should_never_panic(dsp: &dyn ComputeDsp<F = f32>) -> bool {
    panic!("unkown dsp type id: {:?}", (*dsp).type_id())
}
