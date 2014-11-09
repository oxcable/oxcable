//! Provides a limiter filter

#![experimental]

use std::vec::Vec;

use core::{AudioDevice, Time};
use core::channel::{InputChannelArray, OutputChannelArray};
use core::util::decibel_to_ratio;
use dynamics::level_detector::LevelDetector;


/// A limiter
pub struct Limiter {
    pub inputs: InputChannelArray,
    pub outputs: OutputChannelArray,

    level_detectors: Vec<LevelDetector>,
    num_channels: uint, 
    threshold: f32,
    gain: f32
}

impl Limiter {
    /// Creates a new limiter.
    ///
    /// The threshold specifies the decibel level to limit the signal at.
    /// 
    /// Additionally, the specified gain (in decibels) will be applied to the
    /// signal after compression.
    pub fn new(threshold: f32, gain: f32, num_channels: uint) -> Limiter {
        // Create our level detectors
        let mut levels = Vec::with_capacity(num_channels);
        for _ in range(0, num_channels) {
            levels.push(LevelDetector::default());
        }

        Limiter {
            inputs: InputChannelArray::new(num_channels),
            outputs: OutputChannelArray::new(num_channels),
            level_detectors: levels,
            num_channels: num_channels,
            threshold: decibel_to_ratio(threshold),
            gain: decibel_to_ratio(gain)
        }
    }
}

impl AudioDevice for Limiter {
    fn tick(&mut self, t: Time) {
        for i in range(0, self.num_channels) {
            let s = self.inputs.get_sample(i, t).unwrap_or(0.0);

            // Get the current signal level and use it to calculate the gain
            // correction
            let level = self.level_detectors[i].compute_next_level(s);
            let limit = if level > self.threshold {
                self.threshold / level
            } else {
                1.0
            };

            self.outputs.push_sample(i, self.gain*limit*s);
        }
    }
}
