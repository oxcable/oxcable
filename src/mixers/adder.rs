//! `Device` for adding multiple channels into one.

use types::{AudioDevice, DeviceIOType, Sample, Time};


/// An adder.
///
/// The adder sums all its inputs into a single output.
pub struct Adder;

impl Adder {
    /// Returns a new adder with `num_inputs` input channels.
    pub fn new() -> Adder {
        Adder
    }
}

impl AudioDevice for Adder {
    fn num_inputs(&self) -> DeviceIOType {
        DeviceIOType::Any
    }

    fn num_outputs(&self) -> DeviceIOType {
        DeviceIOType::Exactly(1)
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        let mut sum = 0.0;
        for s in inputs.iter() {
            sum += *s;
        }
        outputs[0] = sum;
    }
}
