//! `Device` for changing channel levels.

use types::{AudioDevice, DeviceIOType, Sample, Time};
use utils::helpers::decibel_to_ratio;


/// A gain filter.
///
/// Scales each input by the gain, and puts it in the corresponding output.  For
/// example, for a 3dB stereo gain, channel 0 in inputs will be multiplied by
/// 2 then placed in channel 0 of the outputs; channel 1 in the inputs will be
/// scaled by 2 then placed in channel 1 of the outputs.
pub struct Gain {
    num_channels: usize,
    gain: f32,
}

impl Gain {
    /// Returns a new gain filter.
    ///
    /// `gain` should be in decibels.
    pub fn new(gain: f32, num_channels: usize) -> Gain {
        Gain {
            num_channels: num_channels,
            gain: decibel_to_ratio(gain)
        }
    }
}

impl AudioDevice for Gain {
    fn num_inputs(&self) -> DeviceIOType {
        DeviceIOType::Exactly(self.num_channels)
    }

    fn num_outputs(&self) -> DeviceIOType {
        DeviceIOType::Exactly(self.num_channels)
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        for (i,s) in inputs.iter().enumerate() {
            outputs[i] = self.gain*s;
        }
    }
}
