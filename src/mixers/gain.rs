//! Device for changing channel levels

#![experimental]

use core::{AudioDevice, Time};
use core::components::{InputArray, OutputArray};
use core::util::decibel_to_ratio;


/// Scales each input by the gain, and puts it in the corresponding output.
///
/// For example, for a 3dB stereo gain, channel 0 in inputs will be multiplied
/// by 2 then placed in channel 0 of the outputs; channel 1 in the inputs will
/// be scaled by 2 then placed in channel 1 of the outputs.
pub struct Gain {
    pub inputs: InputArray,
    pub outputs: OutputArray,

    num_channels: uint, 
    gain: f32,
}

impl Gain {
    /// Returns a new gain filter.
    ///
    /// `gain` should be in decibels
    pub fn new(gain: f32, num_channels: uint) -> Gain {
        Gain {
            inputs: InputArray::new(num_channels),
            outputs: OutputArray::new(num_channels),
            num_channels: num_channels,
            gain: decibel_to_ratio(gain)
        }
    }
}

impl AudioDevice for Gain {
    fn tick(&mut self, t: Time) {
        for i in range(0, self.num_channels) {
            let s = self.inputs.get(i, t).unwrap_or(0.0);
            self.outputs.push(i, s*self.gain);
        }
    }
}
