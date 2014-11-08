//! Device for adding multiple channels into one

#![experimental]

use core::{AudioDevice, Time};
use core::channel::{InputChannelArray, OutputChannelArray};


/// Sums all its inputs into a single output
pub struct Adder {
    /// The input array, receiving many signals
    pub inputs: InputChannelArray,
    /// The output array, with a single channel
    pub output: OutputChannelArray,

    num_inputs: uint, 
}

impl Adder {
    /// Returns a new adder with `num_inputs` input channels
    pub fn new(num_inputs: uint) -> Adder {
        Adder {
            inputs: InputChannelArray::new(num_inputs),
            output: OutputChannelArray::new(1),
            num_inputs: num_inputs
        }
    }
}

impl AudioDevice for Adder {
    fn tick(&mut self, t: Time) {
        let mut s = 0.0;
        for i in range(0, self.num_inputs) {
            s += self.inputs.get_sample(i, t).unwrap_or(0.0);
        }
        self.output.push_sample(0, s);
    }
}
