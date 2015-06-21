//! Provides an oscillator that generates periodic waveforms.

use std::f32::consts::PI;
use num::traits::Float;
use rand::random;

use types::{SAMPLE_RATE, AudioDevice, Sample, Time};


/// Defines the messages that the Oscillator supports
#[derive(Clone, Copy, Debug)]
pub enum OscillatorMessage {
    /// Sets the frequency in Hz
    SetFreq(f32),
    /// Sets the waveform type
    SetWaveform(Waveform),
    /// Sets the LFO vibrato depth, in steps
    SetLFOIntensity(f32),
    /// Sets the pitch bend, in steps
    SetBend(f32),
}
pub use self::OscillatorMessage::*;


/// Antialiasing method for certain waveforms.
///
/// Aliased waveforms will use naive methods that produce aliasing.
/// PolyBLEP (Polynomial Bandlimited Step) uses offsets to round off sharp edges
/// and reduce aliasing.
#[derive(Clone, Copy, Debug)]
pub enum AntialiasType {
    /// Naive, aliasing waveforms.
    Aliased,
    /// Antialiasing using PolyBLEP.
    PolyBlep
}
pub use self::AntialiasType::*;


/// Oscillator waveforms.
///
/// `Saw`, `Square`, and `Tri` provides either aliased waveforms, or antialiased
/// waveforms using PolyBLEP. Aliased waveformsare useful for control signals,
/// but not for raw audio signals. For audible signals, instead used the
/// corresponding `PolyBlep` waveforms.
#[derive(Clone, Copy, Debug)]
pub enum Waveform {
    Sine,
    Saw(AntialiasType),
    Square(AntialiasType),
    Tri(AntialiasType),
    WhiteNoise,
    PulseTrain
}
pub use self::Waveform::*;


/// An oscillator that generates a periodic waveform.
pub struct Oscillator {
    waveform: Waveform,
    lfo_intensity: f32,
    bend: f32,
    phase: f32,
    phase_delta: f32,
    last_sample: Sample,
}

impl Oscillator {
    /// Returns an oscillator with the specified waveform at the specified
    /// frequency, in Hz.
    pub fn new(waveform: Waveform) -> Oscillator {
        Oscillator {
            waveform: waveform,
            lfo_intensity: 0.0,
            bend: 1.0,
            phase: 0.0,
            phase_delta: 0.0,
            last_sample: 0.0
        }
    }

    pub fn freq(mut self, freq: f32) -> Oscillator {
        self.handle_message(SetFreq(freq));
        self
    }

    pub fn lfo_intensity(mut self, lfo_intensity: f32) -> Oscillator {
        self.handle_message(SetLFOIntensity(lfo_intensity));
        self
    }

    /// Applies the message to the oscillator
    pub fn handle_message(&mut self, msg: OscillatorMessage) {
        match msg {
            SetFreq(freq) => {
                self.phase_delta = freq*2.0*PI/(SAMPLE_RATE as f32);
            },
            SetWaveform(waveform) => {
                self.waveform = waveform;
            },
            SetLFOIntensity(steps) => {
                self.lfo_intensity = steps/12.0;
            },
            SetBend(steps) => {
                self.bend = 2.0.powf(steps/12.0);
            },
        }
    }
}

impl AudioDevice for Oscillator {
    fn num_inputs(&self) -> usize {
        1
    }

    fn num_outputs(&self) -> usize {
        1
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        // Tick the phase
        let phase_delta = if inputs.len() > 0 {
            self.phase_delta*2.0.powf(inputs[0]*self.lfo_intensity)*self.bend
        } else {
            self.phase_delta*self.bend
        };
        self.phase += phase_delta;
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
                                                phase_delta/(2.0*PI));
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
                                                phase_delta/(2.0*PI));
                        out -= poly_belp_offset(fmod(self.phase/(2.0*PI)+0.5, 1.0),
                                                phase_delta/(2.0*PI));
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
                                                phase_delta/(2.0*PI));
                        out -= poly_belp_offset(fmod(self.phase/(2.0*PI)+0.5, 1.0),
                                                phase_delta/(2.0*PI));
                    },
                    _ => ()
                }

                // Perform leaky integration
                phase_delta*out + (1.0-phase_delta)*self.last_sample
            },
            WhiteNoise => 2.0*random::<f32>() - 1.0,
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
        self.last_sample = s;
        outputs[0] = s;
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
        let t = t / dt;
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
    use utils::helpers::flt_eq;
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
        use types::AudioDevice;
        let mut osc = Oscillator::new(Waveform::Square(AntialiasType::Aliased),
                                      4410.0);
        let input = vec![];
        let mut output = vec![0.0];

        osc.tick(0, &input, &mut output); assert!(output[0] == 1.0);
        osc.tick(1, &input, &mut output); assert!(output[0] == 1.0);
        osc.tick(2, &input, &mut output); assert!(output[0] == 1.0);
        osc.tick(3, &input, &mut output); assert!(output[0] == 1.0);
        osc.tick(4, &input, &mut output); assert!(output[0] == -1.0);
        osc.tick(5, &input, &mut output); assert!(output[0] == -1.0);
        osc.tick(6, &input, &mut output); assert!(output[0] == -1.0);
        osc.tick(7, &input, &mut output); assert!(output[0] == -1.0);
        osc.tick(8, &input, &mut output); assert!(output[0] == -1.0);
    }

    /// Tests saw wave
    #[test]
    fn test_naive_saw() {
        use super::{AntialiasType, Waveform, Oscillator};
        use types::AudioDevice;
        let mut osc = Oscillator::new(Waveform::Saw(AntialiasType::Aliased),
                                      4410.0);
        let input = vec![];
        let mut output = vec![0.0];

        osc.tick(0, &input, &mut output); assert!(flt_eq(output[0], -0.8, EPSILON));
        osc.tick(1, &input, &mut output); assert!(flt_eq(output[0], -0.6, EPSILON));
        osc.tick(2, &input, &mut output); assert!(flt_eq(output[0], -0.4, EPSILON));
        osc.tick(3, &input, &mut output); assert!(flt_eq(output[0], -0.2, EPSILON));
        osc.tick(4, &input, &mut output); assert!(flt_eq(output[0], 0.0, EPSILON));
        osc.tick(5, &input, &mut output); assert!(flt_eq(output[0], 0.2, EPSILON));
        osc.tick(6, &input, &mut output); assert!(flt_eq(output[0], 0.4, EPSILON));
        osc.tick(7, &input, &mut output); assert!(flt_eq(output[0], 0.6, EPSILON));
        osc.tick(8, &input, &mut output); assert!(flt_eq(output[0], 0.8, EPSILON));
    }
}
