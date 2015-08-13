//! Audio devices to perform dynamics processing.
//!
//! Dynamics processors use an estimate of the signal envelope to modify the
//! dynamics of the final signal. The `LevelDetector` can be used to track the
//! peak amplitude of a signal over time, and is uesd by the other three
//! filters.

mod compressor;
mod level_detector;
mod limiter;
mod noise_gate;

#[doc(inline)]
pub use self::compressor::Compressor;
#[doc(inline)]
pub use self::level_detector::LevelDetector;
#[doc(inline)]
pub use self::limiter::Limiter;
#[doc(inline)]
pub use self::noise_gate::NoiseGate;
