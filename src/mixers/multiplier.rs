//! Device for multiplying multiple channels into one

#![experimental]

use core::components::{InputArray, OutputArray};
use core::types::{Device, Sample, Time};


/// Multiplies all its inputs into a single output
pub struct Multiplier {
    /// The input array, receiving many signals
    pub inputs: InputArray<Sample>,
    /// The output array, with a single channel
    pub output: OutputArray<Sample>,

    num_inputs: uint, 
}

impl Multiplier {
    /// Returns a new multiplier with `num_inputs` input channels
    pub fn new(num_inputs: uint) -> Multiplier {
        Multiplier {
            inputs: InputArray::new(num_inputs),
            output: OutputArray::new(1),
            num_inputs: num_inputs
        }
    }
}

impl Device for Multiplier {
    fn tick(&mut self, t: Time) {
        let mut s = 1.0;
        for i in range(0, self.num_inputs) {
            s *= self.inputs.get(i, t).unwrap_or(0.0);
        }
        self.output.push(0, s);
    }
}
