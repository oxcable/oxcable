//! `Device` for multiplying multiple channels into one.

use types::{AudioDevice, Sample, Time};


/// A multiplier.
///
/// Multiplies all its inputs into a single output.
pub struct Multiplier{
    num_channels: usize
}

impl Multiplier {
    /// Returns a new multiplier with `num_inputs` input channels
    pub fn new(num_channels: usize) -> Multiplier {
        Multiplier { num_channels: num_channels }
    }
}

impl AudioDevice for Multiplier {
    fn num_inputs(&self) -> usize {
        self.num_channels
    }

    fn num_outputs(&self) -> usize {
        1
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        let mut prod = 1.0;
        for s in inputs.iter() {
            prod *= *s;
        }
        outputs[0] = prod;
    }
}
