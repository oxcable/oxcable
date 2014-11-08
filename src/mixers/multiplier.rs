//! Device for multiplying multiple channels into one

#![experimental]

use core::{AudioDevice, Time};
use core::channel::{InputChannelArray, OutputChannelArray};


/// Multiplies all its inputs into a single output
pub struct Multiplier {
    /// The input array, receiving many signals
    pub inputs: InputChannelArray,
    /// The output array, with a single channel
    pub output: OutputChannelArray,

    num_inputs: uint, 
}

impl Multiplier {
    /// Returns a new multiplier with `num_inputs` input channels
    pub fn new(num_inputs: uint) -> Multiplier {
        Multiplier {
            inputs: InputChannelArray::new(num_inputs),
            output: OutputChannelArray::new(1),
            num_inputs: num_inputs
        }
    }
}

impl AudioDevice for Multiplier {
    fn tick(&mut self, t: Time) {
        let mut s = 1.0;
        for i in range(0, self.num_inputs) {
            s *= self.inputs.get_sample(i, t).unwrap_or(0.0);
        }
        self.output.push_sample(0, s);
    }
}
