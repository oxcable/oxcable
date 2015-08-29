//! A second order IIR filter.
//!
//! A `LowPass` or `HighPass` filter will provide a 3dB attenuation at the
//! cutoff frequency, and a 12dB per octave rolloff in the attenuation region.
//!
//! A `LowShelf` or `HighShelf` filter will provide a shelf starting at the
//! cutoff frequency, and will provide the specified gain in the shelf region.
//!
//! A `Peak` filter will provide the specified gain around a center frequency,
//! with the width of the peak determined by the Q. A higher Q means a narrower
//! peak.


use std::f32::consts::PI;
use num::traits::Float;

use types::{SAMPLE_RATE, AudioDevice, MessageReceiver, Sample, Time};
use utils::helpers::decibel_to_ratio;


/// Defines the messages that the Filter supports
#[derive(Clone, Copy, Debug)]
pub enum Message {
    SetMode(FilterMode),
}
pub use self::Message::*;


/// Specifies the mode for a second order `Filter`.
///
/// Cutoffs are provided in Hz, gains are provided in decibels.
#[derive(Clone, Copy, Debug)]
pub enum FilterMode {
    /// LowPass(cutoff)
    LowPass(f32),
    /// HighPass(cutoff)
    HighPass(f32),
    /// LowShelf(cutoff, gain)
    LowShelf(f32, f32),
    /// HighShelf(cutoff, gain)
    HighShelf(f32, f32),
    /// Peak(cutoff, gain, Q)
    Peak(f32, f32, f32),
}
pub use self::FilterMode::*;


/// A two pole filter.
pub struct Filter {
    num_channels: usize,
    x_last1: Vec<Sample>, x_last2: Vec<Sample>, // two time step delay elements
    y_last1: Vec<Sample>, y_last2: Vec<Sample>,
    b0: f32, b1: f32, b2: f32, a1: f32, a2: f32
}

impl Filter {
    /// Creates a new second order filter with the provided mode. Each channel
    /// is filtered independently.
    pub fn new(mode: FilterMode, num_channels: usize) -> Self {
        // Compute the parameter values
        let (b0, b1, b2, a1, a2) = compute_parameters(mode);

        Filter {
            num_channels: num_channels,
            x_last1: vec![0.0; num_channels],
            x_last2: vec![0.0; num_channels],
            y_last1: vec![0.0; num_channels],
            y_last2: vec![0.0; num_channels],
            b0: b0, b1: b1, b2: b2, a1: a1, a2: a2
        }
    }
}

impl MessageReceiver for Filter {
    type Msg = Message;
    fn handle_message(&mut self, msg: Message) {
        let SetMode(mode) = msg;
        let (b0, b1, b2, a1, a2) = compute_parameters(mode);
        self.b0 = b0;
        self.b1 = b1;
        self.b2 = b2;
        self.a1 = a1;
        self.a2 = a2;
    }
}

/// Computes the parameters for our filter
#[allow(non_snake_case)]
fn compute_parameters(mode: FilterMode) -> (f32, f32, f32, f32, f32) {
    let cutoff = match mode {
        LowPass(cutoff) => cutoff,
        HighPass(cutoff) => cutoff,
        LowShelf(cutoff, _) => cutoff,
        HighShelf(cutoff, _) => cutoff,
        Peak(center, _, _) => center
    };
    let K = (PI * cutoff / (SAMPLE_RATE as f32)).tan();

    match mode {
        LowPass(_)  => {
            let b0 = K*K/(1.0 + 2.0.sqrt()*K + K*K);
            let b1 = 2.0*K*K/(1.0 + 2.0.sqrt()*K + K*K);
            let b2 = K*K/(1.0 + 2.0.sqrt()*K + K*K);
            let a1 = 2.0*(K*K - 1.0) / (1.0 + 2.0.sqrt()*K + K*K);
            let a2 = (1.0 - 2.0.sqrt()*K + K*K) / (1.0 + 2.0.sqrt()*K + K*K);
            (b0, b1, b2, a1, a2)
        }
        HighPass(_) => {
            let b0 = 1.0/(1.0 + 2.0.sqrt()*K + K*K);
            let b1 = -2.0/(1.0 + 2.0.sqrt()*K + K*K);
            let b2 = 1.0/(1.0 + 2.0.sqrt()*K + K*K);
            let a1 = 2.0*(K*K - 1.0) / (1.0 + 2.0.sqrt()*K + K*K);
            let a2 = (1.0 - 2.0.sqrt()*K + K*K) / (1.0 + 2.0.sqrt()*K + K*K);
            (b0, b1, b2, a1, a2)
        },
        LowShelf(_, gain) => {
            if gain < 0.0 { // cut
                let V0 = 1.0 / decibel_to_ratio(gain/2.0); // amplitude dB
                let b0 = (1.0 + 2.0.sqrt()*K + K*K) /
                    (1.0 + (2.0*V0).sqrt()*K + V0*K*K);
                let b1 = 2.0*(K*K - 1.0) / (1.0 + (2.0*V0).sqrt()*K + V0*K*K);
                let b2 = (1.0 - 2.0.sqrt()*K + K*K) /
                    (1.0 + (2.0*V0).sqrt()*K + V0*K*K);
                let a1 = 2.0*(V0*K*K - 1.0) / (1.0 + (2.0*V0).sqrt()*K + V0*K*K);
                let a2 = (1.0 - (2.0*V0).sqrt()*K + V0*K*K) /
                    (1.0 + (2.0*V0).sqrt()*K + V0*K*K);
                (b0, b1, b2, a1, a2)
            } else { // boost
                let V0 = decibel_to_ratio(gain/2.0); // amplitude dB
                let b0 = (1.0 + (2.0*V0).sqrt()*K + V0*K*K) /
                    (1.0 + 2.0.sqrt()*K + K*K);
                let b1 = 2.0*(V0*K*K - 1.0) / (1.0 + 2.0.sqrt()*K + K*K);
                let b2 = (1.0 - (2.0*V0).sqrt()*K + V0*K*K) /
                    (1.0 + 2.0.sqrt()*K + K*K);
                let a1 = 2.0*(K*K - 1.0) / (1.0 + 2.0.sqrt()*K + K*K);
                let a2 = (1.0 - 2.0.sqrt()*K + K*K) / (1.0 + 2.0.sqrt()*K +
                                                       K*K);
                (b0, b1, b2, a1, a2)
            }
        },
        HighShelf(_, gain) => {
            if gain < 0.0 { // cut
                let V0 = 1.0 / decibel_to_ratio(gain/2.0); // amplitude dB
                let b0 = (1.0 + 2.0.sqrt()*K + K*K) /
                    (V0 + (2.0*V0).sqrt()*K + K*K);
                let b1 = 2.0*(K*K - 1.0) / (V0 + (2.0*V0).sqrt()*K + K*K);
                let b2 = (1.0 - 2.0.sqrt()*K + K*K) /
                    (V0 + (2.0*V0).sqrt()*K + K*K);
                let a1 = 2.0*(K*K/V0 - 1.0) / (1.0 + (2.0/V0).sqrt()*K + K*K/V0);
                let a2 = (1.0 - (2.0/V0).sqrt()*K + K*K/V0) /
                    (1.0 + (2.0/V0).sqrt()*K + K*K/V0);
                (b0, b1, b2, a1, a2)
            } else { // boost
                let V0 = decibel_to_ratio(gain/2.0); // amplitude dB
                let b0 = (V0 + (2.0*V0).sqrt()*K + K*K) /
                    (1.0 + 2.0.sqrt()*K + K*K);
                let b1 = 2.0*(K*K - V0) / (1.0 + 2.0.sqrt()*K + K*K);
                let b2 = (V0 - (2.0*V0).sqrt()*K + K*K) /
                    (1.0 + 2.0.sqrt()*K + K*K);
                let a1 = 2.0*(K*K - 1.0) / (1.0 + 2.0.sqrt()*K + K*K);
                let a2 = (1.0 - 2.0.sqrt()*K + K*K) /
                    (1.0 + 2.0.sqrt()*K + K*K);
                (b0, b1, b2, a1, a2)
            }
        },
        Peak(_, gain, Q) => {
            if gain < 0.0 { // cut
                let V0 = 1.0 / decibel_to_ratio(gain/2.0); // amplitude dB
                let b0 = (1.0 + K/Q + K*K) / (1.0 + V0*K/Q + K*K);
                let b1 = 2.0*(K*K - 1.0) / (1.0 + V0*K/Q + K*K);
                let b2 = (1.0 - K/Q + K*K) / (1.0 + V0*K/Q + K*K);
                let a1 = 2.0*(K*K - 1.0) / (1.0 + V0*K/Q + K*K);
                let a2 = (1.0 - V0*K/Q + K*K) / (1.0 + V0*K/Q + K*K);
                (b0, b1, b2, a1, a2)
            } else { // boost
                let V0 = decibel_to_ratio(gain/2.0); // amplitude dB
                let b0 = (1.0 + V0*K/Q + K*K) / (1.0 + K/Q + K*K);
                let b1 = 2.0*(K*K - 1.0) / (1.0 + K/Q + K*K);
                let b2 = (1.0 - V0*K/Q + K*K) / (1.0 + K/Q + K*K);
                let a1 = 2.0*(K*K - 1.0) / (1.0 + K/Q + K*K);
                let a2 = (1.0 - K/Q + K*K) / (1.0 + K/Q + K*K);
                (b0, b1, b2, a1, a2)
            }
        }
    }
}

impl AudioDevice for Filter {
    fn num_inputs(&self) -> usize {
        self.num_channels
    }

    fn num_outputs(&self) -> usize {
        self.num_channels
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        for (i,x) in inputs.iter().enumerate() {
            // Run the all pass filter, and feedback the result
            let y = self.b0*x + self.b1*self.x_last1[i]
                + self.b2*self.x_last2[i] - self.a1*self.y_last1[i]
                - self.a2*self.y_last2[i];

            // Store our results
            self.x_last2[i] = self.x_last1[i];
            self.y_last2[i] = self.y_last1[i];
            self.x_last1[i] = *x;
            self.y_last1[i] = y;
            outputs[i] = y;
        }
    }
}
