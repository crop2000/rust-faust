mod dsp;
use crate::dsp::Volume;
use core::slice;
use dsp::dsp::FAUST_INPUTS;
use dsp::dsp::FAUST_OUTPUTS;
use faust_types::ParamIndex;
use jack::AudioIn;
use jack::*;
use rand::thread_rng;
use rand::Rng;
use std::array::TryFromSliceError;
use std::io;

fn main() -> Result<(), TryFromSliceError> {
    let mut dsp = Volume::new();
    dsp.init(44_100);
    dsp.set_param(ParamIndex(0), 10.0_f32);

    //example summary
    let iv = vec![0.0f32; 4];
    let i = iv.as_slice();
    let inputs = [i, i];
    #[allow(unused_variables)]
    let too_few_inputs = [i];
    let mut ov1 = vec![0.0f32; 4];
    let mut ov2 = vec![0.0f32; 4];
    let mut outputs = [ov1.as_mut_slice(), ov2.as_mut_slice()];
    // dsp.compute(4, &too_few_inputs, &mut outputs)?; //ok
    // dsp.compute_arrays(4, &too_few_inputs, &mut outputs); //fails to compile

    dsp.compute_arrays(4, &inputs, &mut outputs); //no to check for result

    run_dsp_as_jack_client(dsp);
    Ok(())
}

pub fn run_dsp_as_jack_client(mut dsp: Volume) {
    // Create JACK client
    // input from 0 1 and output to 2 3
    let input_indexes = [0, 1];
    let output_indexes = [2, 3];
    // i use this example because i thought about the benefits of [&[]] vs using a slice for io.
    // the benefit is that references to abritary arrays can be used.
    let (client, in_ports, mut out_ports) =
        create_jack_client("ArrayTest", FAUST_INPUTS, FAUST_OUTPUTS + 2);

    // Init DSP with a given sample rate
    dsp.init(client.sample_rate() as i32);

    // Init input and output buffers
    let buffer_size = client.buffer_size() as usize;
    let mut all_buffers: Vec<Vec<f32>> = vec![vec![0_f32; buffer_size]; 10];

    // Create JACK process closure that runs for each buffer
    let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        let mut rng = thread_rng();
        let len = ps.n_frames();
        assert!(len as usize <= buffer_size);

        let volume: f32 = rng.gen_range(-70..0) as f32;
        eprintln!("level:  {} dB", dsp.get_param(ParamIndex(0)).unwrap());
        eprintln!("mul:  {} dB", dsp.get_param(ParamIndex(1)).unwrap());
        dsp.set_param(ParamIndex(1), volume);

        // the following buffer gymnastics is not what the example is about
        for index_port in 0..FAUST_INPUTS {
            let port = in_ports[index_port].as_slice(ps);
            all_buffers[index_port][0..len as usize].copy_from_slice(port);
        }

        let mut outputs: [&mut [f32]; FAUST_OUTPUTS] = output_indexes.map(|i| unsafe {
            slice::from_raw_parts_mut(all_buffers[i].as_mut_ptr(), buffer_size)
        });

        let inputs: [&[f32]; FAUST_INPUTS] = input_indexes
            .iter()
            .map(|i| all_buffers[*i].as_slice())
            .collect::<Vec<&[f32]>>()
            .try_into()
            .unwrap();

        #[allow(unused_variables)]
        let not_enough_inputs: [&[f32]; 1] = [4]
            .iter()
            .map(|i| all_buffers[*i].as_slice())
            .collect::<Vec<&[f32]>>()
            .try_into()
            .unwrap();

        // this is what it is about inputs and outputs need to have the correct length otherwise the program doens't compile
        // dsp.compute(len as i32, &not_enough_inputs, &mut outputs);
        dsp.compute(len as i32, &inputs, &mut outputs);

        // Copy audio output for all ports from faust to the jack output
        for index_port in 0..FAUST_OUTPUTS + 2 {
            let port = out_ports[index_port].as_mut_slice(ps);
            port.copy_from_slice(&all_buffers[index_port][0..len as usize]);
        }

        jack::Control::Continue
    };
    // Init JACK process handler.
    let process = jack::ClosureProcessHandler::new(process_callback);

    // Activate the client, which starts the processing.
    let active_client = jack::AsyncClient::new(client, (), process).unwrap();

    // Wait for user input to quit
    println!("Press enter/return to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).ok();
    active_client.deactivate().unwrap();
}

fn create_jack_client(
    name: &str,
    num_inputs: usize,
    num_outputs: usize,
) -> (jack::Client, Vec<Port<AudioIn>>, Vec<Port<AudioOut>>) {
    let (client, _status) = jack::Client::new(name, jack::ClientOptions::NO_START_SERVER).unwrap();
    let mut in_ports: Vec<Port<AudioIn>> = Vec::new();
    let mut out_ports: Vec<Port<AudioOut>> = Vec::new();

    for i in 0..num_inputs {
        let port = client
            .register_port(&format!("in{}", i), jack::AudioIn)
            .unwrap();
        in_ports.push(port);
    }
    for i in 0..num_outputs {
        let port = client
            .register_port(&format!("out{}", i), jack::AudioOut)
            .unwrap();
        out_ports.push(port);
    }
    (client, in_ports, out_ports)
}
