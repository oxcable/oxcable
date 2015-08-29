//! A first order IIR filter.
//!
//! A `LowPass` or `HighPass` filter will provide a 3dB attenuation at the
//! cutoff frequency, and a 6dB per octave rolloff in the attenuation region.
//!
//! A `LowShelf` or `HighShelf` filter will provide a shelf starting at the
//! cutoff frequency, and will provide the specified gain in the shelf region.

#![allow(non_snake_case)]

use std::f32::consts::PI;
use num::traits::Float;

use types::{SAMPLE_RATE, AudioDevice, MessageReceiver, Sample, Time};
use utils::helpers::decibel_to_ratio;


/// Defines the messages that the Filter supports.
#[derive(Clone, Copy, Debug)]
pub enum Message {
    /// Sets the filter's mode of operation.
    SetMode(FilterMode),
}
pub use self::Message::*;


/// Specifies the mode for a first order `Filter`.
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
    HighShelf(f32, f32)
}
pub use self::FilterMode::*;


/// A single pole filter.
pub struct Filter {
    num_channels: usize,
    x_last: Vec<Sample>,
    y1_last: Vec<Sample>,
    mode: FilterMode,
    alpha: f32,
    H0: f32
}

impl Filter {
    /// Creates a new first order filter with the provided mode. Each channel is
    /// filtered independently.
    pub fn new(mode: FilterMode, num_channels: usize) -> Self {
        // Compute the parameter values. H0 is ignored for Pass filters
        let (alpha, H0) = compute_parameters(mode);

        Filter {
            num_channels: num_channels,
            x_last: vec![0.0; num_channels],
            y1_last: vec![0.0; num_channels],
            mode: mode,
            alpha: alpha,
            H0: H0
        }
    }
}

impl MessageReceiver for Filter {
    type Msg = Message;
    fn handle_message(&mut self, msg: Message) {
        let SetMode(mode) = msg;
        let (alpha, H0) = compute_parameters(mode);
        self.mode = mode;
        self.alpha = alpha;
        self.H0 = H0;
    }
}

/// Computes the (alpha, H0) parameters for our filter
fn compute_parameters(mode: FilterMode) -> (f32, f32) {
    let cutoff = match mode {
        LowPass(cutoff) => cutoff,
        HighPass(cutoff) => cutoff,
        LowShelf(cutoff, _) => cutoff,
        HighShelf(cutoff, _) => cutoff
    };
    let K = (PI * cutoff / (SAMPLE_RATE as f32)).tan();

    match mode {
        LowPass(_) | HighPass(_) => {
            ((K-1.0) / (K+1.0), 0.0)
        },
        LowShelf(_, gain) => {
            let V0 = decibel_to_ratio(gain/2.0); // amplitude dB
            let H0 = V0 - 1.0;
            let alpha = if gain < 0.0 {
                (K-V0) / (K+V0)
            } else {
                (K-1.0) / (K+1.0)
            };
            (alpha, H0)
        },
        HighShelf(_, gain) => {
            let V0 = decibel_to_ratio(gain/2.0); // amplitude dB
            let H0 = V0 - 1.0;
            let alpha = if gain > 0.0 {
                (V0*K-1.0) / (K+1.0)
            } else {
                (K-1.0) / (K+1.0)
            };
            (alpha, H0)
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
            let y1 = self.alpha*x + self.x_last[i] - self.alpha*self.y1_last[i];
            let y = match self.mode {
                LowPass(_) => (x+y1)/2.0,
                HighPass(_) => (x-y1)/2.0,
                LowShelf(_,_) => self.H0*(x+y1)/2.0 + x,
                HighShelf(_,_) => self.H0*(x-y1)/2.0 + x
            };

            // Store our results
            self.x_last[i] = *x;
            self.y1_last[i] = y1;
            outputs[i] = y;
        }
    }
}
