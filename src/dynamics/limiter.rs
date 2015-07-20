use types::{AudioDevice, Sample, Time};
use utils::helpers::decibel_to_ratio;
use dynamics::level_detector::LevelDetector;


/// A limiter.
///
/// Limiters provide a hard limit on the maximum amplitude of a signal. When the
/// signal amplitude exceeds the threshold, it is attenuated down to the
/// threshold.
pub struct Limiter {
    level_detectors: Vec<LevelDetector>,
    num_channels: usize,
    threshold: f32,
    gain: f32
}

impl Limiter {
    /// Creates a new limiter.
    ///
    /// The `threshold` specifies the decibel level to limit the signal to.
    ///
    /// The specified `gain` (in decibels) will be applied to the signal after
    /// compression.
    pub fn new(threshold: f32, gain: f32, num_channels: usize) -> Limiter {
        Limiter {
            level_detectors: vec![LevelDetector::default(); num_channels],
            num_channels: num_channels,
            threshold: decibel_to_ratio(threshold),
            gain: decibel_to_ratio(gain)
        }
    }
}

impl AudioDevice for Limiter {
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
            let limit = if level > self.threshold {
                self.threshold / level
            } else {
                1.0
            };

            outputs[i] = self.gain*limit*s;
        }
    }
}
