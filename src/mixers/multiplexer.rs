//! `Device` for selecting one of several channels.

#![unstable]

use core::components::{InputArray, OutputElement};
use core::types::{Device, Sample, Time};


/// A multiplexer.
///
/// Mirrors one of its inputs into a single output while ignoring the rest.
pub struct Multiplexer {
    /// Input audio channels
    #[stable]
    pub inputs: InputArray<Sample>,
    /// A single output audio channel
    #[stable]
    pub output: OutputElement<Sample>,

    num_inputs: usize, 
    selected: usize,
}

impl Multiplexer {
    /// Returns a new multiplexer with `num_inputs` input channels.
    #[stable]
    pub fn new(num_inputs: usize) -> Multiplexer {
        Multiplexer {
            inputs: InputArray::new(num_inputs),
            output: OutputElement::new(),
            num_inputs: num_inputs,
            selected: 0
        }
    }

    /// Mirror channel `i` to the output.
    ///
    /// Returns Err if the channel is out of range.
    pub fn select_input(&mut self, i: usize) -> Result<(),()> {
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
        self.output.push(s);
    }
}
