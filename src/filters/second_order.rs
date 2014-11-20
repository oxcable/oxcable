//! Provides second order IIR filters.
//!
//! A `LowPass` or `HighPass` filter will provide a 3dB attenuation at the
//! cutoff frequency, and roughly 12dB per octave rolloff in the attenuation
//! region.
//!
//! A `LowShelf` or `HighShelf` filter will provide a shelf starting at the
//! cutoff frequency, and will provide the specified gain in the shelf region.
//!
//! A `Peak` filter will provide the specified gain around a center frequency,
//! with the width of the peak determined by the Q (higher Q means a narrower
//! peak).

#![experimental]
#![allow(non_snake_case)]


use std::f32::consts::PI;
use std::num::{Float, FloatMath};
use std::vec::Vec;

use core::{SAMPLE_RATE, AudioDevice, Sample, Time};
use core::components::{InputArray, OutputArray};
use core::util::decibel_to_ratio;

use self::FilterMode::{LowPass, HighPass, LowShelf, HighShelf, Peak};


/// Specifies the mode for a first order `Filter`
///
/// `LowPass` and `HighPass` filters specify the cutoff frequency in Hz
///
/// `LowShelf` and `HighShelf` filters specify the cutoff frequency in Hz, and 
/// the gain for the shelf region in decibels
///
/// `Peak` filters specify the center frequency in Hz, the gain for the peak in
/// decibels, and the filter Q
pub enum FilterMode {
    LowPass(f32),        // cutoff
    HighPass(f32),
    LowShelf(f32, f32),  // cutoff, gain
    HighShelf(f32, f32),
    Peak(f32, f32, f32), // center frequency, gain, Q
}

/// A filter that uses a second order all pass filter to perform the specified
/// mode. Each of the channels will be filtered independently.
pub struct Filter {
    pub inputs: InputArray,
    pub outputs: OutputArray,

    num_channels: uint, 
    
    x_last1: Vec<Sample>, x_last2: Vec<Sample>, // two time step delay elements
    y_last1: Vec<Sample>, y_last2: Vec<Sample>,
    b0: f32, b1: f32, b2: f32, a1: f32, a2: f32
}

impl Filter {
    /// Creates a new second order filter with the provided mode.
    pub fn new(mode: FilterMode, num_channels: uint) -> Filter {
        // Populate the delay elements
        let mut x_last1 = Vec::<f32>::with_capacity(num_channels);
        let mut x_last2 = Vec::<f32>::with_capacity(num_channels);
        let mut y_last1 = Vec::<f32>::with_capacity(num_channels);
        let mut y_last2 = Vec::<f32>::with_capacity(num_channels);
        for _ in range(0, num_channels) {
            x_last1.push(0.0);
            x_last2.push(0.0);
            y_last1.push(0.0);
            y_last2.push(0.0);
        }

        // Compute the parameter values
        let (b0, b1, b2, a1, a2) = compute_parameters(mode);

        Filter {
            inputs: InputArray::new(num_channels),
            outputs: OutputArray::new(num_channels),
            num_channels: num_channels,
            x_last1: x_last1, x_last2: x_last2,
            y_last1: y_last1, y_last2: y_last2,
            b0: b0, b1: b1, b2: b2, a1: a1, a2: a2
        }
    }
}

/// Computes the parameters for our filter
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
    fn tick(&mut self, t: Time) {
        for i in range(0, self.num_channels) {
            let x = self.inputs.get(i, t).unwrap_or(0.0);

            // Run the all pass filter, and feedback the result
            let y = self.b0*x + self.b1*self.x_last1[i]
                + self.b2*self.x_last2[i] - self.a1*self.y_last1[i]
                - self.a2*self.y_last2[i];

            // Store our results
            self.x_last2[i] = self.x_last1[i];
            self.y_last2[i] = self.y_last1[i];
            self.x_last1[i] = x;
            self.y_last1[i] = y;
            self.outputs.push(i, y);
        }
    }
}
