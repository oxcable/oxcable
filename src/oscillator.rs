//! Provides an oscillator that generates periodic waveforms.

#![unstable="naming scheme for PI unstable"]

extern crate rand;

use std::f32::consts::PI;
use std::num::Float;

use core::components::OutputElement;
use core::types::{SAMPLE_RATE, Device, Sample, Time};

pub use self::AntialiasType::{Aliased, PolyBlep};
pub use self::Waveform::{Sine, Saw, Square, Tri, WhiteNoise, PulseTrain};

/// Defines the messages that the Oscillator supports
#[unstable="needs more modes"]
#[derive(Clone, Copy, Debug)]
pub enum OscillatorMessage { 
    /// Sets the frequency in Hz
    SetFreq(f32)
}

/// Antialiasing method for certain waveforms.
///
/// Aliased waveforms will use naive methods that produce aliasing.
/// PolyBLEP (Polynomial Bandlimited Step) uses offsets to round off sharp edges
/// and reduce aliasing.
#[stable]
#[derive(Clone, Copy, Debug)]
pub enum AntialiasType {
    /// Naive, aliasing waveforms.
    Aliased, 
    /// Antialiasing using PolyBLEP.
    PolyBlep
}

/// Oscillator waveforms.
///
/// `Saw`, `Square`, and `Tri` provides either aliased waveforms, or antialiased
/// waveforms using PolyBLEP. Aliased waveformsare useful for control signals,
/// but not for raw audio signals. For audible signals, instead used the
/// corresponding `PolyBlep` waveforms.
#[stable]
#[derive(Clone, Copy, Debug)]
pub enum Waveform {
    Sine, 
    Saw(AntialiasType), 
    Square(AntialiasType), 
    Tri(AntialiasType), 
    WhiteNoise, 
    PulseTrain
}

/// An oscillator that generates a periodic waveform.
pub struct Oscillator {
    /// A single output audio channel
    #[stable]
    pub output: OutputElement<Sample>,

    waveform: Waveform,
    phase: f32,
    phase_delta: f32,
    last_sample: Sample,
}

impl Oscillator {
    /// Returns an oscillator with the specified waveform at the specified
    /// frequency, in Hz.
    #[stable]
    pub fn new(waveform: Waveform, freq: f32) -> Oscillator {
        Oscillator { 
            output: OutputElement::new(),
            waveform: waveform, 
            phase: 0.0, 
            phase_delta: freq*2.0*PI/(SAMPLE_RATE as f32),
            last_sample: 0.0
        }
    }

    /// Applies the message to the oscillator
    pub fn handle_message(&mut self, msg: OscillatorMessage) {
        match msg {
            OscillatorMessage::SetFreq(freq) => {
                self.phase_delta = freq*2.0*PI/(SAMPLE_RATE as f32);
            }
        }
    }
}

impl Device for Oscillator {
    fn tick(&mut self, _t: Time) {
        // Tick the phase
        self.phase += self.phase_delta;
        if self.phase >= 2.0*PI {
            self.phase -= 2.0*PI;
        }

        // Compute the next sample
        let s: Sample = match self.waveform.clone() {
            Sine => self.phase.sin(),
            Saw(aa) => {
                let mut out = self.phase/PI -1.0;
                match aa {
                    PolyBlep => {
                        out -= poly_belp_offset(self.phase/(2.0*PI), 
                                                self.phase_delta/(2.0*PI));
                    },
                    _ => ()
                }
                out
            },
            Square(aa) => {
                let mut out = if self.phase < PI { 1.0 } else { -1.0 };
                match aa {
                    PolyBlep => {
                        // two discontinuities, at rising and falling edges
                        out += poly_belp_offset(self.phase/(2.0*PI), 
                                                self.phase_delta/(2.0*PI));
                        out -= poly_belp_offset(fmod(self.phase/(2.0*PI)+0.5, 
                                                     1.0),
                                                self.phase_delta/(2.0*PI));
                    },
                    _ => ()
                }
                out
            },
            Tri(aa) => {
                // Compute a square wave signal
                let mut out = if self.phase < PI { 1.0 } else { -1.0 };
                match aa {
                    PolyBlep => {
                        // two discontinuities, at rising and falling edges
                        out += poly_belp_offset(self.phase/(2.0*PI), 
                                                self.phase_delta/(2.0*PI));
                        out -= poly_belp_offset(fmod(self.phase/(2.0*PI)+0.5, 
                                                     1.0),
                                                self.phase_delta/(2.0*PI));
                    },
                    _ => ()
                }

                // Perform leaky integration
                self.phase_delta*out + (1.0-self.phase_delta)*self.last_sample
            },
            WhiteNoise => 2.0*rand::random::<f32>() - 1.0,
            PulseTrain => {
                // If we wrapped around...
                if self.phase < self.phase_delta {
                    1.0
                } else {
                    0.0
                }
            }
        };

        // Push the sample out
        self.output.push(s);
    }
}


/// Floating point modulus
fn fmod(n: f32, base: f32) -> f32 {
    assert!(base > 0.0);
    let mut out = n;
    while out < 0.0 {
        out += base;
    }
    while out >= base {
        out -= base;
    }
    out
}


/// Computes an offset for PolyBLEP antialiasing
///
/// `t` should be the current waveform phase, normalized
/// `dt` should be the change in phase for one sample time, normalized
fn poly_belp_offset(t: f32, dt: f32) -> f32 {
    if t < dt { // t ~= 0
        let t = dt;
        -t*t + 2.0*t - 1.0
    } else if t > 1.0-dt { // t ~= 1
        let t = (t-1.0) / dt;
        t*t + 2.0*t + 1.0
    } else {
        0.0
    }
}


// A couple of basic unit tests...
#[cfg(test)]
mod test {
    use core::util::flt_eq;
    static EPSILON: f32 = 1e-6;

    /// Tests fmod with many values
    #[test]
    fn test_fmod() {
        use super::fmod;

        // negatives...
        assert!(flt_eq(fmod(-1.5, 1.0), 0.5, EPSILON));
        assert!(flt_eq(fmod(-1.0, 1.0), 0.0, EPSILON));
        assert!(flt_eq(fmod(-0.5, 1.0), 0.5, EPSILON));

        // in range...
        assert!(flt_eq(fmod(0.0, 1.0), 0.0, EPSILON));
        assert!(flt_eq(fmod(0.5, 1.0), 0.5, EPSILON));
        assert!(flt_eq(fmod(0.9, 1.0), 0.9, EPSILON));

        // above...
        assert!(flt_eq(fmod(1.0, 1.0), 0.0, EPSILON));
        assert!(flt_eq(fmod(1.5, 1.0), 0.5, EPSILON));
        assert!(flt_eq(fmod(2.0, 1.0), 0.0, EPSILON));
        assert!(flt_eq(fmod(2.5, 1.0), 0.5, EPSILON));

        // different base...
        assert!(flt_eq(fmod(-0.5, 0.9), 0.4, EPSILON));
        assert!(flt_eq(fmod(0.0, 0.9), 0.0, EPSILON));
        assert!(flt_eq(fmod(0.5, 0.9), 0.5, EPSILON));
        assert!(flt_eq(fmod(0.9, 0.9), 0.0, EPSILON));
        assert!(flt_eq(fmod(1.0, 0.9), 0.1, EPSILON));
    }

    /// Tests square wave
    #[test]
    fn test_naive_square() {
        use super::{AntialiasType, Waveform, Oscillator};
        use core::types::Device;
        let mut osc = Oscillator::new(Waveform::Square(AntialiasType::Aliased),
                                      4410.0);

        osc.tick(0); assert!(osc.output.get(0).unwrap() == 1.0);
        osc.tick(1); assert!(osc.output.get(1).unwrap() == 1.0);
        osc.tick(2); assert!(osc.output.get(2).unwrap() == 1.0);
        osc.tick(3); assert!(osc.output.get(3).unwrap() == 1.0);
        osc.tick(4); assert!(osc.output.get(4).unwrap() == -1.0);
        osc.tick(5); assert!(osc.output.get(5).unwrap() == -1.0);
        osc.tick(6); assert!(osc.output.get(6).unwrap() == -1.0);
        osc.tick(7); assert!(osc.output.get(7).unwrap() == -1.0);
        osc.tick(8); assert!(osc.output.get(8).unwrap() == -1.0);
    }

    /// Tests saw wave
    #[test]
    fn test_naive_saw() {
        use super::{AntialiasType, Waveform, Oscillator};
        use core::types::Device;
        let mut osc = Oscillator::new(Waveform::Saw(AntialiasType::Aliased),
                                      4410.0);

        osc.tick(0); assert!(flt_eq(osc.output.get(0).unwrap(), -0.8, EPSILON));
        osc.tick(1); assert!(flt_eq(osc.output.get(1).unwrap(), -0.6, EPSILON));
        osc.tick(2); assert!(flt_eq(osc.output.get(2).unwrap(), -0.4, EPSILON));
        osc.tick(3); assert!(flt_eq(osc.output.get(3).unwrap(), -0.2, EPSILON));
        osc.tick(4); assert!(flt_eq(osc.output.get(4).unwrap(), 0.0, EPSILON));
        osc.tick(5); assert!(flt_eq(osc.output.get(5).unwrap(), 0.2, EPSILON));
        osc.tick(6); assert!(flt_eq(osc.output.get(6).unwrap(), 0.4, EPSILON));
        osc.tick(7); assert!(flt_eq(osc.output.get(7).unwrap(), 0.6, EPSILON));
        osc.tick(8); assert!(flt_eq(osc.output.get(8).unwrap(), 0.8, EPSILON));
    }
}
