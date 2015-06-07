//! `Device` for multiplying multiple channels into one.

use types::{AudioDevice, DeviceIOType, Sample, Time};


/// A multiplier.
///
/// Multiplies all its inputs into a single output.
pub struct Multiplier;

impl Multiplier {
    /// Returns a new multiplier with `num_inputs` input channels
    pub fn new() -> Multiplier {
        Multiplier
    }
}

impl AudioDevice for Multiplier {
    fn num_inputs(&self) -> DeviceIOType {
        DeviceIOType::Any
    }

    fn num_outputs(&self) -> DeviceIOType {
        DeviceIOType::Exactly(1)
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        let mut prod = 1.0;
        for s in inputs.iter() {
            prod *= *s;
        }
        outputs[0] = prod;
    }
}
