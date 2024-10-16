pub mod <<moduleName>> {
    #![allow(clippy::all)]
    #![allow(unused_parens)]
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(dead_code)]
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(non_upper_case_globals)]
    use faust_types::*;

    <<includeIntrinsic>>
    <<includeclass>>

    impl HasMeta for <<structName>> {
        fn metadata(&self, m: &mut dyn Meta) {
            self.metadata(m)
        }
    }

    impl HasParam for <<structName>> {
        type T = FaustFloat;
        fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
            self.build_user_interface(ui_interface)
        }
    }

    impl HasCompute for <<structName>> {
        type T = FaustFloat;

        fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
            self.get_param(param)
        }

        fn set_param(&mut self, param: ParamIndex, value: Self::T) {
            self.set_param(param, value)
        }

        fn compute(&mut self, count: usize, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
            self.compute(count, inputs, outputs)
        }

        fn get_num_inputs(&self) -> i32 {
            FAUST_INPUTS as i32
        }

        fn get_num_outputs(&self) -> i32 {
            FAUST_OUTPUTS as i32
        }

        fn get_sample_rate(&self) -> i32 {
            self.get_sample_rate()
        }

        fn init(&mut self, sample_rate: i32) {
            self.init(sample_rate)
        }
    }

}

pub use <<moduleName>>::<<structName>>;
