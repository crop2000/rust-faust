#![allow(clippy::all)]
#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]
use faust_macro::ComputeDsp;
use faust_types::*;
#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[derive(ComputeDsp)]
#[repr(C)]
pub struct Saw {
    iVec1: [i32; 2],
    fSampleRate: i32,
    fConst0: F32,
    fConst1: F32,
    fConst2: F32,
    fHslider0: F32,
    fRec2: [F32; 2],
    fConst3: F32,
    fRec1: [F32; 2],
    fHslider1: F32,
    fRec3: [F32; 2],
}
pub type FaustFloat = F32;

// basically suggestion how one should deal with buffers
pub struct SawSIG0 {
    iVec0: [i32; 2],
    iRec0: [i32; 2],
}
impl SawSIG0 {
    fn get_num_inputsSawSIG0(&self) -> i32 {
        return 0;
    }
    fn get_num_outputsSawSIG0(&self) -> i32 {
        return 1;
    }
    pub fn new(sample_rate: i32) -> Self {
        Self {
            iVec0: from_fn(|l1| 0),
            iRec0: from_fn(|l1| 0),
        }
    }
    pub fn fillSawSIG0(&mut self) -> [F32; 65536] {
        from_fn(|i1| {
            self.iVec0[0] = 1;
            self.iRec0[0] = (i32::wrapping_add(self.iVec0[1], self.iRec0[1])) % 65536;
            let r = F32::sin(9.58738e-05 * (self.iRec0[0]) as F32);
            self.iVec0[1] = self.iVec0[0];
            self.iRec0[1] = self.iRec0[0];
            r
        })
    }
}

static mut ftbl0SawSIG0: LazyLock<[F32; 65536]> = LazyLock::new(|| {
    let mut i = SawSIG0::new(-1);
    i.fillSawSIG0()
});

mod ffi {
    use std::os::raw::c_float;
    #[cfg_attr(not(target_os = "windows"), link(name = "m"))]
    unsafe extern "C" {
        pub fn remainderf(from: c_float, to: c_float) -> c_float;
        pub fn rintf(val: c_float) -> c_float;
    }
}
fn remainder_f32(from: f32, to: f32) -> f32 {
    unsafe { ffi::remainderf(from, to) }
}
fn rint_f32(val: f32) -> f32 {
    unsafe { ffi::rintf(val) }
}
pub const FAUST_INPUTS: usize = 0;
pub const FAUST_OUTPUTS: usize = 1;
pub const FAUST_ACTIVES: usize = 2;
pub const FAUST_PASSIVES: usize = 0;
impl Saw {
    pub fn new() -> Saw {
        Saw {
            iVec1: [0; 2],
            fSampleRate: 0,
            fConst0: 0.0,
            fConst1: 0.0,
            fConst2: 0.0,
            fHslider0: 0.0,
            fRec2: [0.0; 2],
            fConst3: 0.0,
            fRec1: [0.0; 2],
            fHslider1: 0.0,
            fRec3: [0.0; 2],
        }
    }
    pub fn metadata(&self, m: &mut dyn Meta) {
        m.declare("author", r"Grame");
        m.declare("basics.lib/name", r"Faust Basic Element Library");
        m.declare("basics.lib/version", r"1.21.0");
        m.declare(
            "compile_options",
            r"-lang rust -cm -ct 1 -cn Saw -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0",
        );
        m.declare("copyright", r"(c)GRAME 2009");
        m.declare("filename", r"saw.dsp");
        m.declare("license", r"BSD");
        m.declare("maths.lib/author", r"GRAME");
        m.declare("maths.lib/copyright", r"GRAME");
        m.declare("maths.lib/license", r"LGPL with exception");
        m.declare("maths.lib/name", r"Faust Math Library");
        m.declare("maths.lib/version", r"2.8.1");
        m.declare("name", r"saw");
        m.declare("oscillators.lib/name", r"Faust Oscillator Library");
        m.declare("oscillators.lib/version", r"1.6.0");
        m.declare("platform.lib/name", r"Generic Platform Library");
        m.declare("platform.lib/version", r"1.3.0");
        m.declare("signals.lib/name", r"Faust Signal Routing Library");
        m.declare("signals.lib/version", r"1.6.0");
        m.declare("version", r"1.0");
    }
    pub fn get_sample_rate(&self) -> i32 {
        self.fSampleRate as i32
    }
    pub fn class_init(sample_rate: i32) {}
    pub fn instance_reset_params(&mut self) {
        self.fHslider0 = 1e+03;
        self.fHslider1 = 0.0;
    }
    pub fn instance_clear(&mut self) {
        for l2 in 0..2 {
            self.iVec1[l2 as usize] = 0;
        }
        for l3 in 0..2 {
            self.fRec2[l3 as usize] = 0.0;
        }
        for l4 in 0..2 {
            self.fRec1[l4 as usize] = 0.0;
        }
        for l5 in 0..2 {
            self.fRec3[l5 as usize] = 0.0;
        }
    }
    pub fn instance_constants(&mut self, sample_rate: i32) {
        self.fSampleRate = sample_rate;
        self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
        self.fConst1 = 44.1 / self.fConst0;
        self.fConst2 = 1.0 - self.fConst1;
        self.fConst3 = 1.0 / self.fConst0;
    }
    pub fn instance_init(&mut self, sample_rate: i32) {
        self.instance_constants(sample_rate);
        self.instance_reset_params();
        self.instance_clear();
    }
    pub fn init(&mut self, sample_rate: i32) {
        Saw::class_init(sample_rate);
        self.instance_init(sample_rate);
    }
    pub fn build_user_interface(&self, ui_interface: &mut dyn UI<FaustFloat>) {
        Self::build_user_interface_static(ui_interface);
    }
    pub fn build_user_interface_static(ui_interface: &mut dyn UI<FaustFloat>) {
        ui_interface.open_vertical_box("Oscillator");
        ui_interface.declare(Some(ParamIndex(0)), "unit", "Hz");
        ui_interface.add_horizontal_slider("freq", ParamIndex(0), 1e+03, 2e+01, 2e+03, 1.0);
        ui_interface.declare(Some(ParamIndex(1)), "unit", "dB");
        ui_interface.add_horizontal_slider("volume", ParamIndex(1), 0.0, -5e+01, 0.0, 0.1);
        ui_interface.close_box();
    }
    pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
        match param.0 {
            0 => Some(self.fHslider0),
            1 => Some(self.fHslider1),
            _ => None,
        }
    }
    pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
        match param.0 {
            0 => self.fHslider0 = value,
            1 => self.fHslider1 = value,
            _ => {}
        }
    }
    pub fn compute(
        &mut self,
        count: usize,
        inputs: &[impl AsRef<[FaustFloat]>],
        outputs: &mut [impl AsMut<[FaustFloat]>],
    ) {
        let [outputs0, ..] = outputs.as_mut() else {
            panic!("wrong number of output buffers");
        };
        let outputs0 = outputs0.as_mut()[..count].iter_mut();
        let mut fSlow0: F32 = self.fConst1 * self.fHslider0;
        let mut fSlow1: F32 = self.fConst1 * F32::powf(1e+01, 0.05 * self.fHslider1);
        let zipped_iterators = outputs0;
        for output0 in zipped_iterators {
            self.iVec1[0] = 1;
            self.fRec2[0] = fSlow0 + self.fConst2 * self.fRec2[1];
            let mut fTemp0: F32 = (if i32::wrapping_sub(1, self.iVec1[1]) != 0 {
                0.0
            } else {
                self.fRec1[1] + self.fConst3 * self.fRec2[0]
            });
            self.fRec1[0] = fTemp0 - F32::floor(fTemp0);
            self.fRec3[0] = fSlow1 + self.fConst2 * self.fRec3[1];
            *output0 = self.fRec3[0]
                * unsafe {
                    ftbl0SawSIG0[(std::cmp::max(
                        0,
                        std::cmp::min((65536.0 * self.fRec1[0]) as i32, 65535),
                    )) as usize]
                }
                + *output0;
            self.iVec1[1] = self.iVec1[0];
            self.fRec2[1] = self.fRec2[0];
            self.fRec1[1] = self.fRec1[0];
            self.fRec3[1] = self.fRec3[0];
        }
    }
}
impl FaustDsp for Saw {
    type T = FaustFloat;
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::new()
    }
    fn metadata(&self, m: &mut dyn Meta) {
        self.metadata(m)
    }
    fn get_sample_rate(&self) -> i32 {
        self.get_sample_rate()
    }
    fn get_num_inputs(&self) -> i32 {
        FAUST_INPUTS as i32
    }
    fn get_num_outputs(&self) -> i32 {
        FAUST_OUTPUTS as i32
    }
    fn class_init(sample_rate: i32)
    where
        Self: Sized,
    {
        Self::class_init(sample_rate);
    }
    fn instance_reset_params(&mut self) {
        self.instance_reset_params()
    }
    fn instance_clear(&mut self) {
        self.instance_clear()
    }
    fn instance_constants(&mut self, sample_rate: i32) {
        self.instance_constants(sample_rate)
    }
    fn instance_init(&mut self, sample_rate: i32) {
        self.instance_init(sample_rate)
    }
    fn init(&mut self, sample_rate: i32) {
        self.init(sample_rate)
    }
    fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
        self.build_user_interface(ui_interface)
    }
    fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>)
    where
        Self: Sized,
    {
        Self::build_user_interface_static(ui_interface);
    }
    fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
        self.get_param(param)
    }
    fn set_param(&mut self, param: ParamIndex, value: Self::T) {
        self.set_param(param, value)
    }
    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
        self.compute(count as usize, inputs, outputs)
    }
}
use std::array::from_fn;
use std::convert::TryInto;
use std::sync::LazyLock;
use strum::{
    Display, EnumCount, EnumDiscriminants, EnumIter, IntoStaticStr, VariantArray, VariantNames,
};
impl FaustFloatDsp for Saw {
    type F = FaustFloat;
}
impl UIEnumsDsp for Saw {
    type DA = UIActive;
    type EA = UIActiveValue;
    type DP = UIPassive;
    type EP = UIPassiveValue;
}
impl SetDsp for Saw {
    type E = UIActiveValue;
    fn set(&mut self, value: impl TryInto<Self::E>) -> bool {
        if let Ok(value) = value.try_into() {
            UISelfSet::set(&value, self);
            true
        } else {
            false
        }
    }
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
#[strum_discriminants(name(UIActive))]
pub enum UIActiveValue {
    Freq(FaustFloat),
    Volume(FaustFloat),
}
impl UISelfSet for UIActiveValue {
    type D = Saw;
    fn set(&self, dsp: &mut Saw) {
        match self {
            UIActiveValue::Freq(value) => dsp.fHslider0 = *value,
            UIActiveValue::Volume(value) => dsp.fHslider1 = *value,
        }
    }
    fn get(&self) -> <Self::D as FaustFloatDsp>::F {
        match self {
            UIActiveValue::Freq(value) => *value,
            UIActiveValue::Volume(value) => *value,
        }
    }
}
impl UISet for UIActive {
    type D = Saw;
    fn set(&self, dsp: &mut Saw, value: <Self::D as FaustFloatDsp>::F) {
        match self {
            UIActive::Freq => dsp.fHslider0 = value,
            UIActive::Volume => dsp.fHslider1 = value,
        }
    }
}
impl UIRange for UIActive {
    fn min(&self) -> f32 {
        match self {
            UIActive::Freq => 20f32,
            UIActive::Volume => -50f32,
        }
    }
    fn max(&self) -> f32 {
        match self {
            UIActive::Freq => 2000f32,
            UIActive::Volume => 0f32,
        }
    }
}
impl UIActive {
    pub fn value(&self, value: FaustFloat) -> UIActiveValue {
        match self {
            UIActive::Freq => UIActiveValue::Freq(value),
            UIActive::Volume => UIActiveValue::Volume(value),
        }
    }
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
#[strum_discriminants(name(UIPassive))]
pub enum UIPassiveValue {}
impl UIGet for UIPassive {
    type D = Saw;
    fn get_value(&self, dsp: &Self::D) -> <Self::D as FaustFloatDsp>::F {
        panic!("cannot be called")
    }
    fn get_enum(&self, dsp: &Saw) -> <Self::D as UIEnumsDsp>::EP {
        panic!("cannot be called")
    }
}
impl UIPassive {
    pub fn value(&self, value: FaustFloat) -> UIPassiveValue {
        panic!("cannot be called")
    }
}
impl UIRange for UIPassive {
    fn min(&self) -> f32 {
        panic!("cannot be called")
    }
    fn max(&self) -> f32 {
        panic!("cannot be called")
    }
}
#[derive(Debug)]
pub struct DspUiOscillator {
    pub freq: UIActive,
    pub volume: UIActive,
}
impl DspUiOscillator {
    const fn static_ui() -> Self {
        Self {
            freq: UIActive::Freq,
            volume: UIActive::Volume,
        }
    }
}
pub static DSP_UI: DspUiOscillator = DspUiOscillator::static_ui();
pub mod meta {
    pub const AUTHOR: &'static str = "Grame";
    pub const COMPILE_OPTIONS: &'static str =
        "-lang rust -cm -ct 1 -cn Saw -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0";
    pub const COPYRIGHT: &'static str = "(c)GRAME 2009";
    pub const FILENAME: &'static str = "saw.dsp";
    pub const LICENSE: &'static str = "BSD";
    pub const NAME: &'static str = "saw";
    pub const VERSION: &'static str = "1.0";
    pub mod libs {
        pub mod basics {
            pub const NAME: &'static str = "Faust Basic Element Library";
            pub const VERSION: &'static str = "1.21.0";
        }
        pub mod maths {
            pub const AUTHOR: &'static str = "GRAME";
            pub const COPYRIGHT: &'static str = "GRAME";
            pub const LICENSE: &'static str = "LGPL with exception";
            pub const NAME: &'static str = "Faust Math Library";
            pub const VERSION: &'static str = "2.8.1";
        }
        pub mod oscillators {
            pub const NAME: &'static str = "Faust Oscillator Library";
            pub const VERSION: &'static str = "1.6.0";
        }
        pub mod platform {
            pub const NAME: &'static str = "Generic Platform Library";
            pub const VERSION: &'static str = "1.3.0";
        }
        pub mod signals {
            pub const NAME: &'static str = "Faust Signal Routing Library";
            pub const VERSION: &'static str = "1.6.0";
        }
    }
}
