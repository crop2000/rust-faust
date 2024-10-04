use faust_types::*;

pub mod <<moduleName>> {
    #![allow(clippy::all)]
    #![allow(unused_parens)]
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(dead_code)]
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(non_upper_case_globals)]
    
    <<includeIntrinsic>>
    <<includeclass>>
}

impl FaustDsp for <<moduleName>>::<<structName>> {
    type T = T;

    fn new() -> Self {
        Self::new()
    }

    fn metadata(&self, m: &mut dyn Meta) {
        Self::metadata(self, m)
    }

    fn get_sample_rate(&self) -> i32 {
        Self::get_sample_rate(self)
    }

    fn get_num_inputs(&self) -> i32 {
        Self::get_num_inputs(self)
    }

    fn get_num_outputs(&self) -> i32 {
        Self::get_num_outputs(self)
    }

    fn class_init(sample_rate: i32) {
        Self::class_init(sample_rate)
    }

    fn instance_reset_params(&mut self) {
        Self::instance_reset_params(self)
    }

    fn instance_clear(&mut self) {
        Self::instance_clear(self)
    }

    fn instance_constants(&mut self, sample_rate: i32) {
        Self::instance_constants(self, sample_rate)
    }

    fn instance_init(&mut self, sample_rate: i32) {
        Self::init(self, sample_rate)
    }

    fn init(&mut self, sample_rate: i32) {
        Self::init(self, sample_rate)
    }

    fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
        Self::build_user_interface(self, ui_interface)
    }

    fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
        Self::build_user_interface_static(ui_interface)
    }

    fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
        Self::get_param(self, param)
    }

    fn set_param(&mut self, param: ParamIndex, value: Self::T) {
        Self::set_param(self, param, value)
    }

    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
        Self::compute(self, count, inputs, outputs)
    }
}

pub use <<moduleName>>::<<structName>>;
