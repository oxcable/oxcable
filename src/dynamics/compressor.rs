//! Provides a compression filter

#![experimental]

use std::vec::Vec;

use core::{AudioDevice, Time};
use core::components::{InputArray, OutputArray};
use core::util::decibel_to_ratio;
use dynamics::level_detector::LevelDetector;


/// A compressor
pub struct Compressor {
    pub inputs: InputArray,
    pub outputs: OutputArray,

    level_detectors: Vec<LevelDetector>,
    num_channels: uint, 
    threshold: f32,
    compression_ratio: f32,
    gain: f32
}

impl Compressor {
    /// Creates a new compressor.
    ///
    /// The threshold specifies the decibel level at which compression begins.
    /// 
    /// Additionally, the specified gain (in decibels) will be applied to the
    /// signal after compression.
    pub fn new(threshold: f32, compression_ratio: f32, gain: f32, 
               num_channels: uint) -> Compressor {
        // Create our level detectors
        let mut levels = Vec::with_capacity(num_channels);
        for _ in range(0, num_channels) {
            levels.push(LevelDetector::default());
        }

        Compressor {
            inputs: InputArray::new(num_channels),
            outputs: OutputArray::new(num_channels),
            level_detectors: levels,
            num_channels: num_channels,
            threshold: decibel_to_ratio(threshold),
            compression_ratio: compression_ratio,
            gain: decibel_to_ratio(gain)
        }
    }
}

impl AudioDevice for Compressor {
    fn tick(&mut self, t: Time) {
        for i in range(0, self.num_channels) {
            let s = self.inputs.get(i, t).unwrap_or(0.0);

            // Get the current signal level and use it to calculate the gain
            // correction
            let level = self.level_detectors[i].compute_next_level(s);
            let compression = if level > self.threshold {
                self.compression_ratio*self.threshold/level + 
                    1.0-self.compression_ratio
            } else {
                1.0
            };

            self.outputs.push(i, self.gain*compression*s);
        }
    }
}
