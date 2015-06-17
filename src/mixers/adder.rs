//! `Device` for adding multiple channels into one.

use types::{AudioDevice, Sample, Time};


/// An adder.
///
/// The adder sums all its inputs into a single output.
pub struct Adder {
    num_channels: usize
}

impl Adder {
    /// Returns a new adder with `num_inputs` input channels.
    pub fn new(num_channels: usize) -> Adder {
        Adder { num_channels: num_channels }
    }
}

impl AudioDevice for Adder {
    fn num_inputs(&self) -> usize {
        self.num_channels
    }

    fn num_outputs(&self) -> usize {
        1
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        let mut sum = 0.0;
        for s in inputs.iter() {
            sum += *s;
        }
        outputs[0] = sum;
    }
}
