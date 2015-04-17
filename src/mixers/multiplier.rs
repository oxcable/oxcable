//! `Device` for multiplying multiple channels into one.

use components::{InputArray, OutputElement};
use types::{Device, Sample, Time};


/// A multiplier.
///
/// Multiplies all its inputs into a single output.
pub struct Multiplier {
    /// Input audio channels
    pub inputs: InputArray<Sample>,
    /// A single audio output channel.
    pub output: OutputElement<Sample>,

    num_inputs: usize,
}

impl Multiplier {
    /// Returns a new multiplier with `num_inputs` input channels
    pub fn new(num_inputs: usize) -> Multiplier {
        Multiplier {
            inputs: InputArray::new(num_inputs),
            output: OutputElement::new(),
            num_inputs: num_inputs
        }
    }
}

impl Device for Multiplier {
    fn tick(&mut self, t: Time) {
        let mut s = 1.0;
        for i in (0 .. self.num_inputs) {
            s *= self.inputs.get(i, t).unwrap_or(0.0);
        }
        self.output.push(s);
    }
}
