//! Device for selecting one of several channels

#![experimental]

use core::components::{InputArray, OutputArray};
use core::types::{Device, Sample, Time};


/// Mirrors one of its inputs into a single output
pub struct Multiplexer {
    /// The input array, receiving many signals
    pub inputs: InputArray<Sample>,
    /// The output array, with a single channel
    pub output: OutputArray<Sample>,

    num_inputs: uint, 
    selected: uint,
}

impl Multiplexer {
    /// Returns a new multiplexer with `num_inputs` input channels
    pub fn new(num_inputs: uint) -> Multiplexer {
        Multiplexer {
            inputs: InputArray::new(num_inputs),
            output: OutputArray::new(1),
            num_inputs: num_inputs,
            selected: 0
        }
    }

    /// Mirror channel `i` to the output.
    ///
    /// Returns Err if the channel is out of range
    pub fn select_input(&mut self, i: uint) -> Result<(),()> {
        if i < self.num_inputs {
            self.selected = i;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl Device for Multiplexer {
    fn tick(&mut self, t: Time) {
        let s = self.inputs.get(self.selected, t).unwrap_or(0.0);
        self.output.push(0, s);
    }
}
