/* ------------------------------------------------------------
author: "Franz Heinzmann"
license: "BSD"
name: "volumecontrol"
version: "1.0"
Code generated with Faust 2.75.10 (https://faust.grame.fr)
Compilation options: -a /tmp/.tmp2qqdnv -lang rust -ct 1 -cn Volume -es 1 -mcd 16 -mdd 1024 -mdy 33 -uim -double -ftz 0
------------------------------------------------------------ */
pub mod dsp {
    #![allow(clippy::all)]
    #![allow(unused_parens)]
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(dead_code)]
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(non_upper_case_globals)]
    use faust_types::*;

    use std::{
        iter::Zip,
        slice::{Iter, IterMut},
    };

    mod ffi {
        use std::os::raw::c_double;
        // Conditionally compile the link attribute only on non-Windows platforms
        #[cfg_attr(not(target_os = "windows"), link(name = "m"))]
        extern "C" {
            pub fn remainder(from: c_double, to: c_double) -> c_double;
            pub fn rint(val: c_double) -> c_double;
        }
    }
    pub fn remainder_f64(from: f64, to: f64) -> f64 {
        unsafe { ffi::remainder(from, to) }
    }
    pub fn rint_f64(val: f64) -> f64 {
        unsafe { ffi::rint(val) }
    }

    #[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
    #[repr(C)]
    pub struct Volume {
        fSampleRate: i32,
        fConst0: F64,
        fConst1: F64,
        fConst2: F64,
        fVslider0: F64,
        fRec0: [F64; 2],
        fConst3: F64,
        fRec1: [F64; 2],
        fVbargraph0: F64,
        fConst4: F64,
    }

    pub type FaustFloat = F64;

    // i think we should use tuples because this is how we treat this collection of io buffers
    // tuples are also optimized away more easily,
    // i think,
    // i decided for now against tuples because of i want to be able to use iterators

    // static part that doesn't really need to be in the faust codegen

    // optional idea for a fixed size compute function
    pub const FIXED_BUFFERLENGTH: usize = 64;
    pub type FixedBuffer = [FaustFloat; FIXED_BUFFERLENGTH];
    pub type FixedInputBuffers<'a> = [&'a FixedBuffer; FAUST_INPUTS];
    pub type FixedOutputBuffers<'a> = [&'a mut FixedBuffer; FAUST_OUTPUTS];

    // optional idea for a fixed size compute function
    pub type FixedFixedBuffer = [FaustFloat; FIXED_BUFFERLENGTH];
    pub type FixedFixedInputBuffers<'a> = [FixedBuffer; FAUST_INPUTS];
    pub type FixedFixedOutputBuffers<'a> = [FixedBuffer; FAUST_OUTPUTS];

    // types for current compute
    pub type InputSlice<'a> = &'a [&'a [FaustFloat]];
    pub type OutputSlice<'a> = &'a mut [&'a mut [FaustFloat]];

    pub type InputBuffers<'a> = [&'a [FaustFloat]; FAUST_INPUTS];
    pub type OutputBuffers<'a> = [&'a mut [FaustFloat]; FAUST_OUTPUTS];

    pub type InputIter<'a> = Iter<'a, FaustFloat>;
    pub type OutputIter<'a> = IterMut<'a, FaustFloat>;
    pub type InputIterArray<'a> = [Iter<'a, FaustFloat>; FAUST_INPUTS];
    pub type OutputIterArray<'a> = [IterMut<'a, FaustFloat>; FAUST_OUTPUTS];
    pub type IterTuple<'a, 'b> = (InputIterArray<'a>, OutputIterArray<'b>);

    //INPUT TRANSFORMATIONS
    //slice of slice to array of slice
    fn arr_in<'a>(inputs: InputSlice<'a>) -> InputBuffers<'a> {
        inputs[..FAUST_INPUTS]
            .try_into()
            .expect("too few input buffers")
    }
    //array of slice to array of slize cut to the right length
    fn block_in<'a, 'b>(count: usize, inputs: InputBuffers) -> InputBuffers {
        inputs.map(|i| &i[..count])
    }

    // array of slice to array of iters
    fn it_in<'b, 'a>(inputs: InputBuffers<'a>) -> InputIterArray<'a> {
        inputs.map(|i| i.iter())
    }
    fn it_fixed_size_in<'b, 'a>(inputs: FixedInputBuffers<'a>) -> InputIterArray<'a> {
        inputs.map(|i| i.iter())
    }

    //OUTPUT TRANSFORMATIONS
    //array of slice to array of slize cut to the right length
    fn block_out<'a, 'b>(count: usize, mut outputs: OutputBuffers) -> OutputBuffers {
        outputs.map(|i| &mut i[..count])
    }
    // array of slice to array of iters
    fn it_out<'b, 'a>(outputs: OutputBuffers<'a>) -> OutputIterArray<'a> {
        outputs.map(|i| i.iter_mut())
    }
    fn it_fixed_size_out<'b, 'a>(outputs: FixedOutputBuffers<'a>) -> OutputIterArray<'a> {
        outputs.map(|i| i.iter_mut())
    }

    // IO TRANSFORMATIONS THAT NEED MACROS
    // faust generated/macro part

    // this could be generated in a macro
    fn arr_out<'a>(outputs: OutputSlice<'a>) -> OutputBuffers<'a> {
        let [o0, o1] = outputs else {
            panic!("too few input buffers");
        };
        [o0, o1]
    }

    // this could be generated in a macro
    pub type ZippedIters<'a> =
        Zip<Zip<Zip<InputIter<'a>, InputIter<'a>>, OutputIter<'a>>, OutputIter<'a>>;

    // this could be generated in a macro
    fn zip<'a>(iter_tuple: IterTuple<'a, 'a>) -> ZippedIters<'a> {
        let (inputs, outputs) = iter_tuple;
        let [i0, i1] = inputs;
        let [o0, o1] = outputs;
        i0.zip(i1).zip(o0).zip(o1)
    }

    impl Volume {
        pub fn compute_zipped_iter(&mut self, zipped_iterators: ZippedIters) {
            //kr part
            let mut fSlow0: F64 = self.fConst1 * F64::powf(1e+01, 0.05 * self.fVslider0);
            //ar part
            for (((input0, input1), output0), output1) in zipped_iterators {
                self.fRec0[0] = fSlow0 + self.fConst2 * self.fRec0[1];
                let mut fTemp0: F64 = *input0;
                let mut fTemp1: F64 = *input1;
                self.fRec1[0] = F64::max(
                    self.fRec1[1] - self.fConst3,
                    F64::abs(0.5 * self.fRec0[0] * (fTemp0 + fTemp1)),
                );
                self.fVbargraph0 = 2e+01
                    * F64::log10(F64::max(
                        2.2250738585072014e-308,
                        F64::max(0.00031622776601683794, self.fRec1[0]),
                    ));
                *output0 = self.fConst4 + fTemp0 * self.fRec0[0];
                *output1 = fTemp1 * self.fRec0[0];
                self.fRec0[1] = self.fRec0[0];
                self.fRec1[1] = self.fRec1[0];
            }
        }

        pub fn compute_iter<'a>(
            &mut self,
            inputs: [InputIter; FAUST_INPUTS],
            outputs: [OutputIter; FAUST_OUTPUTS],
        ) {
            let z = zip((inputs, outputs));
            self.compute_zipped_iter(z);
        }

        // use arrays to ensure correct number of buffers
        pub fn compute_arrays<'a>(
            &mut self,
            count: usize,
            inputs: [&[FaustFloat]; FAUST_INPUTS],
            outputs: [&'a mut [FaustFloat]; FAUST_INPUTS],
        ) {
            let b_in = block_in(count, inputs);
            let b_out = block_out(count, outputs);
            let it_in = it_in(b_in);
            let it_out = it_out(b_out);
            let z = zip((it_in, it_out));
            self.compute_zipped_iter(z);
        }

        // use fixedsize arrays to use buffers more efficiently
        pub fn compute_fixed_arrays<'a>(
            &mut self,
            count: usize,
            inputs: FixedInputBuffers<'a>,
            outputs: FixedOutputBuffers<'a>,
        ) {
            let it_in = it_fixed_size_in(inputs);
            let it_out = it_fixed_size_out(outputs);
            let z = zip((it_in, it_out));
            self.compute_zipped_iter(z);
        }

        pub fn compute_slices<'a>(
            &mut self,
            count: usize,
            inputs: &[&[FaustFloat]],
            outputs: &'a mut [&'a mut [FaustFloat]],
        ) {
            let arr = (arr_in(inputs), arr_out(outputs));
            let block = (block_in(count, arr.0), block_out(count, arr.1));
            let itt = (it_in(block.0), it_out(block.1));
            let z = zip(itt);
            self.compute_zipped_iter(z);
        }

        // use fixedsize that are allocated in one block
        // i didn't put much work into this
        // but i was surprised that i had again problems to get the references
        pub fn compute_fixed_fixed_arrays<'a>(
            &mut self,
            count: usize,
            inputs: FixedFixedInputBuffers<'a>,
            mut outputs: FixedFixedOutputBuffers<'a>,
        ) {
            // let block = block_fixed_size(count, inputs, outputs);
            let (i0, inputs) = inputs.split_first().unwrap();
            let (i1, inputs) = inputs.split_first().unwrap();

            let (o0, outputs) = outputs.split_first_mut().unwrap();
            let (o1, outputs) = outputs.split_first_mut().unwrap();
            let z = zip(([i0.iter(), i1.iter()], [o0.iter_mut(), o1.iter_mut()]));
            self.compute_zipped_iter(z);
        }

        pub fn new() -> Volume {
            Volume {
                fSampleRate: 0,
                fConst0: 0.0,
                fConst1: 0.0,
                fConst2: 0.0,
                fVslider0: 0.0,
                fRec0: [0.0; 2],
                fConst3: 0.0,
                fRec1: [0.0; 2],
                fVbargraph0: 0.0,
                fConst4: 0.0,
            }
        }
        pub fn metadata(&self, m: &mut dyn Meta) {
            m.declare("author", r"Franz Heinzmann");
            m.declare("basics.lib/name", r"Faust Basic Element Library");
            m.declare(
                "basics.lib/tabulateNd",
                r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>",
            );
            m.declare("basics.lib/version", r"1.19.1");
            m.declare("compile_options", r"-a /tmp/.tmp2qqdnv -lang rust -ct 1 -cn Volume -es 1 -mcd 16 -mdd 1024 -mdy 33 -uim -double -ftz 0");
            m.declare("filename", r"volume.dsp");
            m.declare("license", r"BSD");
            m.declare("maths.lib/author", r"GRAME");
            m.declare("maths.lib/copyright", r"GRAME");
            m.declare("maths.lib/license", r"LGPL with exception");
            m.declare("maths.lib/name", r"Faust Math Library");
            m.declare("maths.lib/version", r"2.8.0");
            m.declare("name", r"volumecontrol");
            m.declare("options", r"[osc:on]");
            m.declare("platform.lib/name", r"Generic Platform Library");
            m.declare("platform.lib/version", r"1.3.0");
            m.declare("signals.lib/name", r"Faust Signal Routing Library");
            m.declare("signals.lib/version", r"1.6.0");
            m.declare("version", r"1.0");
        }

        pub fn get_sample_rate(&self) -> i32 {
            return self.fSampleRate;
        }
        pub fn get_num_inputs(&self) -> i32 {
            return 2;
        }
        pub fn get_num_outputs(&self) -> i32 {
            return 2;
        }

        pub fn class_init(sample_rate: i32) {}
        pub fn instance_reset_params(&mut self) {
            self.fVslider0 = 0.0;
        }
        pub fn instance_clear(&mut self) {
            for l0 in 0..2 {
                self.fRec0[l0 as usize] = 0.0;
            }
            for l1 in 0..2 {
                self.fRec1[l1 as usize] = 0.0;
            }
        }
        pub fn instance_constants(&mut self, sample_rate: i32) {
            self.fSampleRate = sample_rate;
            self.fConst0 = F64::min(1.92e+05, F64::max(1.0, (self.fSampleRate) as F64));
            self.fConst1 = 44.1 / self.fConst0;
            self.fConst2 = 1.0 - self.fConst1;
            self.fConst3 = 1.0 / self.fConst0;
            self.fConst4 = (0) as F64;
        }
        pub fn instance_init(&mut self, sample_rate: i32) {
            self.instance_constants(sample_rate);
            self.instance_reset_params();
            self.instance_clear();
        }
        pub fn init(&mut self, sample_rate: i32) {
            Volume::class_init(sample_rate);
            self.instance_init(sample_rate);
        }

        pub fn build_user_interface(&self, ui_interface: &mut dyn UI<FaustFloat>) {
            Self::build_user_interface_static(ui_interface);
        }

        pub fn build_user_interface_static(ui_interface: &mut dyn UI<FaustFloat>) {
            ui_interface.open_vertical_box("volumecontrol");
            ui_interface.declare(Some(ParamIndex(0)), "2", "");
            ui_interface.declare(Some(ParamIndex(0)), "style", "dB");
            ui_interface.declare(Some(ParamIndex(0)), "unit", "dB");
            ui_interface.add_vertical_bargraph("level", ParamIndex(0), -6e+01, 5.0);
            ui_interface.add_vertical_slider("volume", ParamIndex(1), 0.0, -7e+01, 4.0, 0.1);
            ui_interface.close_box();
        }

        pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
            match param.0 {
                0 => Some(self.fVbargraph0),
                1 => Some(self.fVslider0),
                _ => None,
            }
        }

        pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
            match param.0 {
                0 => self.fVbargraph0 = value,
                1 => self.fVslider0 = value,
                _ => {}
            }
        }
    }

    pub const FAUST_INPUTS: usize = 2;
    pub const FAUST_OUTPUTS: usize = 2;
    pub const FAUST_ACTIVES: usize = 1;
    pub const FAUST_PASSIVES: usize = 1;

    impl HasMeta for Volume {
        fn metadata(&self, m: &mut dyn Meta) {
            self.metadata(m)
        }
    }

    impl HasParam for Volume {
        type T = FaustFloat;
        fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
            self.build_user_interface(ui_interface)
        }
    }

    impl HasCompute for Volume {
        type T = FaustFloat;

        fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
            self.get_param(param)
        }

        fn set_param(&mut self, param: ParamIndex, value: Self::T) {
            self.set_param(param, value)
        }

        fn compute<'a>(
            &mut self,
            count: usize,
            inputs: &[&[Self::T]],
            outputs: &'a mut [&'a mut [Self::T]],
        ) {
            self.compute_slices(count, inputs, outputs)
        }

        fn get_sample_rate(&self) -> i32 {
            self.get_sample_rate()
        }

        fn get_num_inputs(&self) -> i32 {
            self.get_num_inputs()
        }

        fn get_num_outputs(&self) -> i32 {
            self.get_num_outputs()
        }

        fn init(&mut self, sample_rate: i32) {
            self.init(sample_rate)
        }
    }
}

pub use dsp::Volume;
