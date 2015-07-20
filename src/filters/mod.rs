//! Linear IIR filters.
//!
//! These filters can be used to amplify or attenuate different frequencies in
//! the provided signal. A second order filter will provide smaller transition
//! bands, and therefore sharper cutoffs.

pub mod first_order;
pub mod second_order;
