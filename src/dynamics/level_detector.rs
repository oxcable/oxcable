use num::traits::Float;

use types::{SAMPLE_RATE, Sample};


/// Performs envelope estimation for a signal.
///
/// The level detector performs leaky integration of the input signal to
/// estimate the envelope. Therefore, for a signal `x(t)`, the level
/// `l(t) = alpha*l(t-1) + (1-alpha)*x(t)`.
///
/// When creating a new level detector, a tau is specified, rather than an
/// alpha. Tau is the time constant, which corresponds to how quickly the
/// envelope estimate reaches steady state.
///
/// The integration uses two different values of tau, depending on whether the
/// signal is greater or less than the current envelope. This means the attack
/// value can be set very low to respond quickly to bursts in signal power,
/// while the release value can be high in order to coast through periodic
/// troughs in the signal.
#[derive(Clone, Copy, Debug)]
pub struct LevelDetector {
    attack_alpha: f32,
    release_alpha: f32,
    last_power: f32
}

impl LevelDetector {
    /// Returns a level detector with default `tau` values tuned for reasonable
    /// performance.
    pub fn default() -> Self {
        LevelDetector::new(1.0, 100.0)
    }

    /// Returns a new level detector.
    ///
    /// * `attack_tau` specifies the time constant when the signal is growing,
    ///   in milliseconds.
    /// * `release_tau` specifies the time constant when the signal id decaying,
    ///   in milliseconds.
    pub fn new(attack_tau: f32, release_tau: f32) -> Self {
        LevelDetector {
            attack_alpha: time_constant_to_multiplier(attack_tau),
            release_alpha: time_constant_to_multiplier(release_tau),
            last_power: 0.0
        }
    }

    /// Given the next input sample `s`, computes the current estimate of the
    /// envelope value.
    pub fn compute_next_level(&mut self, s: Sample) -> f32 {
        // Perform leaky integration on the signal power, rather than amplitude
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

/// Converts a time constant in milliseconds to a leak rate.
fn time_constant_to_multiplier(tau: f32) -> f32 {
    (-1.0 / (tau/1000.0 * (SAMPLE_RATE as f32))).exp()
}
