//! Defines types and constants to be used globally in oxcable


/// The global sample rate, in Hz
pub static SAMPLE_RATE: uint = 44100;

/// The datatype of a single sample
pub type Sample = f32;
/// The datatype of a single sample time
pub type Time   = u64;


/// An interface for a synchronous processing device
#[experimental]
pub trait Device {
    /// Process a single frame worth of data
    ///
    /// This function should be called once per time step, starting at `t=1`.
    fn tick(&mut self, t: Time);
}
