//! Provides a noise gate filter.

#![unstable]

use std::vec::Vec;

use components::{InputArray, OutputArray};
use types::{Device, Sample, Time};
use utils::helpers::decibel_to_ratio;
use dynamics::level_detector::LevelDetector;


/// A noise gate
pub struct NoiseGate {
    /// Input audio channels
    #[stable]
    pub inputs: InputArray<Sample>,
    /// Output audio channels
    #[stable]
    pub outputs: OutputArray<Sample>,

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
    #[stable]
    pub fn new(on_threshold: f32, off_threshold: f32, gain: f32,
               num_channels: usize) -> NoiseGate {
        // Create our level detectors
        let mut levels = Vec::with_capacity(num_channels);
        for _ in (0 .. num_channels) {
            levels.push(LevelDetector::default());
        }

        NoiseGate {
            inputs: InputArray::new(num_channels),
            outputs: OutputArray::new(num_channels),
            level_detectors: levels,
            active: false,
            num_channels: num_channels,
            on_threshold: decibel_to_ratio(on_threshold),
            off_threshold: decibel_to_ratio(off_threshold),
            gain: decibel_to_ratio(gain)
        }
    }
}

impl Device for NoiseGate {
    fn tick(&mut self, t: Time) {
        for i in (0 .. self.num_channels) {
            let mut s = self.inputs.get(i, t).unwrap_or(0.0);

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

            self.outputs.push(i, self.gain*s);
        }
    }
}
