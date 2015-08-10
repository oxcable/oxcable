use types::{AudioDevice, Sample, Time};
use utils::helpers::decibel_to_ratio;
use dynamics::level_detector::LevelDetector;


/// A compression filter.
///
/// Compression filters provide a soft limit on the maximum amplitude of
/// a signal. When a signal exceeds the threshold of the filter, it is partially
/// attenuated to bring it closer to the threshold.
///
/// The intensity of the attenuation is determined by the compression ratio.
/// A ratio of zero provides no attenuation. A ratio of 1 will hard limit the
/// signal to the threshold. Values in between will provide partial attenuation.
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
    /// The `threshold` specifies the decibel level at which compression begins.
    ///
    /// The `compression_ratio` specifies the ratio of attenuation above the
    /// threshold.
    ///
    /// The `gain` (in decibels) will be applied to the signal after
    /// compression.
    pub fn new(threshold: f32, compression_ratio: f32, gain: f32,
               num_channels: usize) -> Self {
        Compressor {
            level_detectors: vec![LevelDetector::default(); num_channels],
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
