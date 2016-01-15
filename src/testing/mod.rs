//! A collection of utilities for testing.


/// Compares floating point numbers for equality.
///
/// The maximum difference is specified by `epsilon`; that is, `f1` and `f2` are
/// equal if they differ by at most epsilon.
pub fn flt_eq_eps(f1: f32, f2: f32, epsilon: f32) -> bool {
    (f1 - f2).abs() < epsilon
}

/// Compares floating point numbers for equality with a default epsilon.
pub fn flt_eq(f1: f32, f2: f32) -> bool {
    flt_eq_eps(f1, f2, 1e-6)
}
