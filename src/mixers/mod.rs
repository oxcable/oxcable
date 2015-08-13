//! Tools for combining several audio signals.

mod adder;
mod gain;
mod multiplexer;
mod multiplier;

#[doc(inline)]
pub use self::adder::Adder;
#[doc(inline)]
pub use self::gain::Gain;
#[doc(inline)]
pub use self::multiplier::Multiplier;
#[doc(inline)]
pub use self::multiplexer::Multiplexer;
