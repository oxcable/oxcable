use types::{AudioDevice, Sample, Time};
use utils::helpers::decibel_to_ratio;


/// A gain filter.
pub struct Gain {
    num_channels: usize,
    gain: f32,
}

impl Gain {
    /// Returns a new gain filter.
    ///
    /// `gain` is in decibels.
    pub fn new(gain: f32, num_channels: usize) -> Gain {
        Gain {
            num_channels: num_channels,
            gain: decibel_to_ratio(gain)
        }
    }
}

impl AudioDevice for Gain {
    fn num_inputs(&self) -> usize {
        self.num_channels
    }

    fn num_outputs(&self) -> usize {
        self.num_channels
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        for (i,s) in inputs.iter().enumerate() {
            outputs[i] = self.gain*s;
        }
    }
}
