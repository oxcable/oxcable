//! Audio devices to perform dynamics processing.
//!
//! Dynamics processors use an estimate of the signal envelope to modify the
//! dynamics of the final signal. The `LevelDetector` can be used to track the
//! peak amplitude of a signal over time, and is uesd by the other three
//! filters.

pub mod compressor;
pub mod level_detector;
pub mod limiter;
pub mod noise_gate;

pub use self::compressor::Compressor;
pub use self::level_detector::LevelDetector;
pub use self::limiter::Limiter;
pub use self::noise_gate::NoiseGate;
