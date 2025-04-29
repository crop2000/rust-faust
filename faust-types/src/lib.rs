use std::any::Any;
use std::convert::TryInto;
use std::hash::Hash;
use strum::IntoEnumIterator;

pub type F32 = f32;
pub type F64 = f64;

#[derive(Copy, Clone, Debug)]
pub struct ParamIndex(pub i32);

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct Soundfile<'a> {
    fBuffers: &'a &'a F32,
    fLength: &'a i32,
    fSR: &'a i32,
    fOffset: &'a i32,
    fChannels: i32,
}

pub trait FaustDsp {
    type T;

    fn new() -> Self
    where
        Self: Sized;
    fn metadata(&self, m: &mut dyn Meta);
    fn get_sample_rate(&self) -> i32;
    fn get_num_inputs(&self) -> i32;
    fn get_num_outputs(&self) -> i32;
    fn class_init(sample_rate: i32)
    where
        Self: Sized;
    fn instance_reset_params(&mut self);
    fn instance_clear(&mut self);
    fn instance_constants(&mut self, sample_rate: i32);
    fn instance_init(&mut self, sample_rate: i32);
    fn init(&mut self, sample_rate: i32);
    fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>);
    fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>)
    where
        Self: Sized;
    fn get_param(&self, param: ParamIndex) -> Option<Self::T>;
    fn set_param(&mut self, param: ParamIndex, value: Self::T);
    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]);
}

pub trait Meta {
    // -- metadata declarations
    fn declare(&mut self, key: &str, value: &str);
}

pub trait UI<T> {
    // -- widget's layouts
    fn open_tab_box(&mut self, label: &str);
    fn open_horizontal_box(&mut self, label: &str);
    fn open_vertical_box(&mut self, label: &str);
    fn close_box(&mut self);

    // -- active widgets
    fn add_button(&mut self, label: &str, param: ParamIndex);
    fn add_check_button(&mut self, label: &str, param: ParamIndex);
    fn add_vertical_slider(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: T,
        min: T,
        max: T,
        step: T,
    );
    fn add_horizontal_slider(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: T,
        min: T,
        max: T,
        step: T,
    );
    fn add_num_entry(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);

    // -- passive widgets
    fn add_horizontal_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);
    fn add_vertical_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);

    // -- metadata declarations
    fn declare(&mut self, param: Option<ParamIndex>, key: &str, value: &str);
}

// trait to provide access to parameters
// this trait is generated with the ui interace
// traits for generated code
pub trait UISet:
    Clone + Send + Sync + Eq + Hash + Into<&'static str> + IntoEnumIterator + 'static
{
    type D: FaustFloatDsp;
    fn set(&self, dsp: &mut Self::D, value: <Self::D as FaustFloatDsp>::F);
}

pub trait UISetAny:
    Clone + Send + Sync + Eq + Hash + Into<&'static str> + IntoEnumIterator + 'static
{
    type D: FaustFloatDsp;
    fn set(&self, dsp: &mut dyn Any, value: <Self::D as FaustFloatDsp>::F) -> bool;
}

impl<T: UISet> UISetAny for T {
    type D = T::D;
    fn set(&self, dsp: &mut dyn Any, value: <Self::D as FaustFloatDsp>::F) -> bool {
        if let Some(dsp) = dsp.downcast_mut::<Self::D>() {
            self.set(dsp, value);
            true
        } else {
            false
        }
    }
}

pub trait UISelfSet: Clone + Send + Sync + IntoEnumIterator + 'static {
    type D: FaustFloatDsp;
    fn set(&self, dsp: &mut Self::D);
    fn get(&self) -> <Self::D as FaustFloatDsp>::F;
}

pub trait UIGet:
    Clone + Send + Sync + Eq + Hash + Into<&'static str> + IntoEnumIterator + 'static
{
    type D: FaustFloatDsp + UIEnumsDsp;
    fn get_value(&self, dsp: &Self::D) -> <Self::D as FaustFloatDsp>::F;
    fn get_enum(&self, dsp: &Self::D) -> <Self::D as UIEnumsDsp>::EP;
}

// trait to describe value ranges
// this trait is generated with the ui interace
pub trait UIRange {
    fn min(&self) -> f32;
    fn max(&self) -> f32;
    fn map(&self, f01: f32) -> f32 {
        let min = self.min();
        let max = self.max();
        let range = max - min;
        (f01 * range) + min
    }
}

// traits to provide alternative interface to UISelfSet and UIGet
// the impl of these traits is provided here
pub trait UISelfSetAny: Clone + Send + Sync + IntoEnumIterator + 'static {
    type D: FaustFloatDsp;
    fn set(&self, dsp: &mut dyn Any) -> bool;
}

impl<T: UISelfSet> UISelfSetAny for T {
    type D = T::D;
    fn set(&self, dsp: &mut dyn Any) -> bool {
        if let Some(dsp) = dsp.downcast_mut::<Self::D>() {
            self.set(dsp);
            true
        } else {
            false
        }
    }
}

pub trait UIGetAny:
    Clone + Send + Sync + Eq + Hash + Into<&'static str> + IntoEnumIterator + 'static
{
    type D: FaustFloatDsp + UIEnumsDsp;
    fn get_value(&self, dsp: &dyn Any) -> Option<<Self::D as FaustFloatDsp>::F>;
    fn get_enum(&self, dsp: &dyn Any) -> Option<<Self::D as UIEnumsDsp>::EP>;
}

impl<T: UIGet> UIGetAny for T {
    type D = T::D;
    fn get_value(&self, dsp: &dyn Any) -> Option<<Self::D as FaustFloatDsp>::F> {
        dsp.downcast_ref::<Self::D>().map(|dsp| self.get_value(dsp))
    }
    fn get_enum(&self, dsp: &dyn Any) -> Option<<Self::D as UIEnumsDsp>::EP> {
        dsp.downcast_ref::<Self::D>().map(|dsp| self.get_enum(dsp))
    }
}

// traits that describe the relation between types
// these traits are generated with the ui interace
pub trait FaustFloatDsp: Any + Send + Sync + 'static {
    type F;
}

pub trait UIEnumsDsp: FaustFloatDsp + Any + Send + Sync + 'static {
    type DA: UISet<D = Self>;
    type EA: strum::IntoDiscriminant + UISelfSet;
    type DP: UIGet<D = Self>;
    type EP: strum::IntoDiscriminant;
}

// traits that provide a interface to a specific functionality of a dsp
// that match to the dsp struct depending on flags
// these traits can be implemented via derive macros
pub trait ComputeDsp: FaustFloatDsp + Any + Send + Sync + 'static {
    fn compute(&mut self, count: usize, inputs: &[&[Self::F]], outputs: &mut [&mut [Self::F]]);
    fn compute_vec(&mut self, count: usize, inputs: &[Vec<Self::F>], outputs: &mut [Vec<Self::F>]);
}

pub trait InitDsp: Any {
    fn instance_init(&mut self, sample_rate: i32);
}

pub trait InPlaceDsp: FaustFloatDsp + Any {
    fn compute(&mut self, count: usize, ios: &mut [&mut [Self::F]]);
    fn compute_vec(&mut self, count: usize, ios: &mut [Vec<Self::F>]);
}

pub trait ExternalControlDsp: FaustFloatDsp + Any + Sized {
    type S: UISet;
    type V: UISelfSet;
    fn control(&mut self);
    fn update_controls(&mut self, controls: &[&Self::F]);
    fn update_control_values(&mut self, controls: &[&Self::V]);
}

pub trait SetDsp {
    type E: UISelfSet<D = Self>;
    fn set(&mut self, value: impl TryInto<Self::E>) -> bool;
}
