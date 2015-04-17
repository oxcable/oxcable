//! `Device` for changing channel levels.

use components::{InputArray, OutputArray};
use types::{Device, Sample, Time};
use utils::helpers::decibel_to_ratio;


/// A gain filter.
///
/// Scales each input by the gain, and puts it in the corresponding output.  For
/// example, for a 3dB stereo gain, channel 0 in inputs will be multiplied by
/// 2 then placed in channel 0 of the outputs; channel 1 in the inputs will be
/// scaled by 2 then placed in channel 1 of the outputs.
pub struct Gain {
    /// Input audio channels.
    pub inputs: InputArray<Sample>,
    /// Output audio channels.
    pub outputs: OutputArray<Sample>,

    num_channels: usize,
    gain: f32,
}

impl Gain {
    /// Returns a new gain filter.
    ///
    /// `gain` should be in decibels.
    pub fn new(gain: f32, num_channels: usize) -> Gain {
        Gain {
            inputs: InputArray::new(num_channels),
            outputs: OutputArray::new(num_channels),
            num_channels: num_channels,
            gain: decibel_to_ratio(gain)
        }
    }
}

impl Device for Gain {
    fn tick(&mut self, t: Time) {
        for i in (0 .. self.num_channels) {
            let s = self.inputs.get(i, t).unwrap_or(0.0);
            self.outputs.push(i, s*self.gain);
        }
    }
}
