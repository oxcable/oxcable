//! Provides a noise gate filter.

use types::{AudioDevice, DeviceIOType, Sample, Time};
use utils::helpers::decibel_to_ratio;
use dynamics::level_detector::LevelDetector;


/// A noise gate
pub struct NoiseGate {
    level_detectors: Vec<LevelDetector>,
    active: bool,
    num_channels: usize,
    on_threshold: f32,
    off_threshold: f32,
    gain: f32
}

impl NoiseGate {
    /// Creates a new compressor.
    ///
    /// The noise gate will pass audio once it hits the `on_threshold` (in
    /// decibels), and continue passing until the signal level drops below the
    /// `off_threshold` (also in decibels).
    ///
    /// The specified `gain` (in decibels) will be applied to the signal after
    /// compression.
    pub fn new(on_threshold: f32, off_threshold: f32, gain: f32,
               num_channels: usize) -> NoiseGate {
        // Create our level detectors
        let mut levels = Vec::with_capacity(num_channels);
        for _ in (0 .. num_channels) {
            levels.push(LevelDetector::default());
        }

        NoiseGate {
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
    fn num_inputs(&self) -> DeviceIOType {
        DeviceIOType::Exactly(self.num_channels)
    }

    fn num_outputs(&self) -> DeviceIOType {
        DeviceIOType::Exactly(self.num_channels)
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        for (i,s) in inputs.iter().enumerate() {
            // Look for a state change
            let level = self.level_detectors[i].compute_next_level(*s);
            if self.active && level < self.off_threshold {
                self.active = false;
            } else if !self.active && level > self.on_threshold {
                self.active = true;
            }

            // Gate the signal
            outputs[i] = if self.active { self.gain*s } else { 0.0 };
        }
    }
}
