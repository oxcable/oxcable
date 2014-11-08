//! Core definitions, components, and functions for creating audio devices.

#![experimental]

/// The global sample rate, in Hz
pub static SAMPLE_RATE: uint = 44100;
/// The datatype of a single sample
pub type Sample = f32;
/// The datatype of a single sample time
pub type Time   = u64;

pub mod channel;
pub mod complex;
pub mod fft;
pub mod util;

/// An interface for a synchronous audio device
#[experimental]
pub trait AudioDevice {
    /// Process a single sample worth of audio
    ///
    /// This function should be called once per time step, starting at `t=1`.
    fn tick(&mut self, t: Time);
}
