//! Provides first order IIR filters.
//!
//! A `LowPass` or `HighPass` filter will provide a 3dB attenuation at the
//! cutoff frequency, and roughly 6dB per octave rolloff in the attenuation
//! region.
//!
//! A `LowShelf` or `HighShelf` filter will provide a shelf starting at the
//! cutoff frequency, and will provide the specified gain in the shelf region.

#![experimental]
#![allow(non_snake_case)]


use std::f32::consts::PI;
use std::num::Float;
use std::vec::Vec;

use core::components::{InputArray, OutputArray};
use core::types::{SAMPLE_RATE, Device, Sample, Time};
use core::util::decibel_to_ratio;

pub use self::FilterMode::{LowPass, HighPass, LowShelf, HighShelf};


/// Specifies the mode for a first order `Filter`.
///
/// `LowPass` and `HighPass` filters specify the cutoff frequency in Hz.
///
/// `LowShelf` and `HighShelf` filters specify the cutoff frequency in Hz, and 
/// the gain for the shelf region in decibels.
#[derive(Clone, Copy, Show)]
pub enum FilterMode {
    LowPass(f32),       // cutoff
    HighPass(f32),
    LowShelf(f32, f32), // cutoff, gain
    HighShelf(f32, f32)
}

/// A filter that uses a first order all pass filter to perform the specified
/// mode. Each of the channels will be filtered independently.
pub struct Filter {
    /// Input audio channels
    pub inputs: InputArray<Sample>,
    /// Output audio channels
    pub outputs: OutputArray<Sample>,

    num_channels: usize, 
    x_last: Vec<Sample>,
    y1_last: Vec<Sample>,
    mode: FilterMode,
    alpha: f32,
    H0: f32
}

impl Filter {
    /// Creates a new first order filter with the provided mode.
    pub fn new(mode: FilterMode, num_channels: usize) -> Filter {
        // Populate the last vectors
        let mut x_last = Vec::<f32>::with_capacity(num_channels);
        let mut y1_last = Vec::<f32>::with_capacity(num_channels);
        for _ in range(0, num_channels) {
            x_last.push(0.0);
            y1_last.push(0.0);
        }

        // Compute the parameter values. H0 is ignored for Pass filters
        let (alpha, H0) = compute_parameters(mode.clone());

        Filter {
            inputs: InputArray::new(num_channels),
            outputs: OutputArray::new(num_channels),
            num_channels: num_channels,
            x_last: x_last,
            y1_last: y1_last,
            mode: mode,
            alpha: alpha,
            H0: H0
        }
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

impl Device for Filter {
    fn tick(&mut self, t: Time) {
        for i in range(0, self.num_channels) {
            let x = self.inputs.get(i, t).unwrap_or(0.0);

            // Run the all pass filter, and feedback the result
            let y1 = self.alpha*x + self.x_last[i] - self.alpha*self.y1_last[i];
            let y = match self.mode {
                LowPass(_) => (x+y1)/2.0,
                HighPass(_) => (x-y1)/2.0,
                LowShelf(_,_) => self.H0*(x+y1)/2.0 + x,
                HighShelf(_,_) => self.H0*(x-y1)/2.0 + x
            };

            // Store our results
            self.x_last[i] = x;
            self.y1_last[i] = y1;
            self.outputs.push(i, y);
        }
    }
}
