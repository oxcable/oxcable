//! Provides envelope estimation for a signal

#![experimental]

use std::num::Float;

use core::{SAMPLE_RATE, Sample};


/// Provides envelope estimation for a signal.
///
/// The level detector uses 
pub struct LevelDetector {
    attack_alpha: f32,
    release_alpha: f32,
    last_power: f32
}

impl LevelDetector {
    pub fn new(attack_tau: f32, release_tau: f32) -> LevelDetector {
        LevelDetector {
            attack_alpha: time_constant_to_multiplier(attack_tau),
            release_alpha: time_constant_to_multiplier(release_tau),
            last_power: 0.0
        }
    }

    pub fn default() -> LevelDetector {
        LevelDetector::new(1.0, 100.0)
    }

    pub fn compute_next_level(&mut self, s: Sample) -> f32 {
        // Perform leaky integration on the signal power
        let pow = s*s;
        let alpha = if pow > self.last_power {
            self.attack_alpha
        } else { 
            self.release_alpha 
        };
        self.last_power = alpha*self.last_power + (1.0-alpha)*pow;

        // Convert from power to amplitude and return
        self.last_power.sqrt()
    }
}

fn time_constant_to_multiplier(tau: f32) -> f32 {
    (-1.0 / (tau/1000.0 * (SAMPLE_RATE as f32))).exp()
}
