//! Provides a compression filter.

use types::{AudioDevice, Sample, Time};
use utils::helpers::decibel_to_ratio;
use dynamics::level_detector::LevelDetector;


/// A compression filter.
pub struct Compressor {
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
            level_detectors: levels,
            num_channels: num_channels,
            threshold: decibel_to_ratio(threshold),
            compression_ratio: compression_ratio,
            gain: decibel_to_ratio(gain)
        }
    }
}

impl AudioDevice for Compressor {
    fn num_inputs(&self) -> usize {
        self.num_channels
    }

    fn num_outputs(&self) -> usize {
        self.num_channels
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        for (i,s) in inputs.iter().enumerate() {
            // Get the current signal level and use it to calculate the gain
            // correction
            let level = self.level_detectors[i].compute_next_level(*s);
            let compression = if level > self.threshold {
                self.compression_ratio*self.threshold/level +
                    1.0-self.compression_ratio
            } else {
                1.0
            };

            outputs[i] = self.gain*compression*s;
        }
    }
}
