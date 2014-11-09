//! Provides a noise gate filter

#![experimental]

use std::vec::Vec;

use core::{AudioDevice, Time};
use core::channel::{InputChannelArray, OutputChannelArray};
use core::util::decibel_to_ratio;
use dynamics::level_detector::LevelDetector;


/// A noise gate
pub struct NoiseGate {
    pub inputs: InputChannelArray,
    pub outputs: OutputChannelArray,

    level_detectors: Vec<LevelDetector>,
    active: bool,
    num_channels: uint, 
    on_threshold: f32,
    off_threshold: f32,
    gain: f32
}

impl NoiseGate {
    /// Creates a new compressor.
    ///
    /// The noise gate will pass audio once it hits the on_threshold (in
    /// decibels), and continue passing until the signal level drops below the
    /// off_threshold (also in decibels).
    /// 
    /// Additionally, the specified gain (in decibels) will be applied to the
    /// signal after compression.
    pub fn new(on_threshold: f32, off_threshold: f32, gain: f32, 
               num_channels: uint) -> NoiseGate {
        // Create our level detectors
        let mut levels = Vec::with_capacity(num_channels);
        for _ in range(0, num_channels) {
            levels.push(LevelDetector::default());
        }

        NoiseGate {
            inputs: InputChannelArray::new(num_channels),
            outputs: OutputChannelArray::new(num_channels),
            level_detectors: levels,
            active: false,
            num_channels: num_channels,
            on_threshold: decibel_to_ratio(on_threshold),
            off_threshold: decibel_to_ratio(off_threshold),
            gain: decibel_to_ratio(gain)
        }
    }
}

impl AudioDevice for NoiseGate {
    fn tick(&mut self, t: Time) {
        for i in range(0, self.num_channels) {
            let mut s = self.inputs.get_sample(i, t).unwrap_or(0.0);

            // Look for a state change
            let level = self.level_detectors[i].compute_next_level(s);
            if self.active && level < self.off_threshold {
                self.active = false;
            } else if !self.active && level > self.on_threshold {
                self.active = true;
            }

            // Gate the signal
            if !self.active {
                s = 0.0;
            }

            self.outputs.push_sample(i, self.gain*s);
        }
    }
}
