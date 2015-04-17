//! Provides a compression filter.

use std::vec::Vec;

use components::{InputArray, OutputArray};
use types::{Device, Sample, Time};
use utils::helpers::decibel_to_ratio;
use dynamics::level_detector::LevelDetector;


/// A compression filter.
pub struct Compressor {
    /// Input audio channels
    pub inputs: InputArray<Sample>,
    /// Output audio channels
    pub outputs: OutputArray<Sample>,

    level_detectors: Vec<LevelDetector>,
    num_channels: usize,
    threshold: f32,
    compression_ratio: f32,
    gain: f32
}

impl Compressor {
    /// Creates a new compressor.
    ///
    /// * `threshold` specifies the decibel level at which compression begins.
    /// * `ratio` specifies the ratio of attenuation above the threshold. For
    ///   example, a compression ratio of 0 provides NO attenuation;
    ///   a compression ratio of 1 forces the output down to the threshold.
    /// * The specified `gain` (in decibels) will be applied to the
    ///   signal after compression.
    pub fn new(threshold: f32, compression_ratio: f32, gain: f32,
               num_channels: usize) -> Compressor {
        // Create our level detectors
        let mut levels = Vec::with_capacity(num_channels);
        for _ in (0 .. num_channels) {
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

impl Device for Compressor {
    fn tick(&mut self, t: Time) {
        for i in (0 .. self.num_channels) {
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
