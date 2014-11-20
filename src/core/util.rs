//! A collection of small utility functions

#![unstable]

use std::num::Float;


/// Compares floating point numbers for equality
///
/// The maximum difference is specified by `epsilon`; that is, `f1` and `f2` are
/// equal if they differ by at most epsilon
pub fn flt_eq(f1: f32, f2: f32, epsilon: f32) -> bool {
    (f1 - f2).abs() < epsilon
}

/// Converts a decibel multiplier to a power ratio
pub fn decibel_to_ratio(db: f32) -> f32 {
    10.0.powf(db/10.0)
}

/// Converts a power ratio to decibels
pub fn ratio_to_decibel(ratio: f32) -> f32 {
    10.0*ratio.log10()
}


#[cfg(test)]
mod tests {
    use super::flt_eq;
    static EPSILON: f32 = 1e-6;

    /// Tests some basic decibel vaulues
    #[test]
    fn test_decibel_to_ratio() {
        use super::decibel_to_ratio;
        assert!(flt_eq(decibel_to_ratio(-10.0), 0.1, EPSILON));
        assert!(flt_eq(decibel_to_ratio(-3.0), 0.501187234, EPSILON));
        assert!(flt_eq(decibel_to_ratio(0.0), 1.0, EPSILON));
        assert!(flt_eq(decibel_to_ratio(3.0), 1.995262315, EPSILON));
        assert!(flt_eq(decibel_to_ratio(10.0), 10.0, EPSILON));
    }

    /// Tests some basic ratio vaulues
    #[test]
    fn test_ratio_to_decibel() {
        use super::ratio_to_decibel;
        assert!(flt_eq(ratio_to_decibel(0.1), -10.0, EPSILON));
        assert!(flt_eq(ratio_to_decibel(0.501187234), -3.0, EPSILON));
        assert!(flt_eq(ratio_to_decibel(1.0), 0.0, EPSILON));
        assert!(flt_eq(ratio_to_decibel(1.995262315), 3.0, EPSILON));
        assert!(flt_eq(ratio_to_decibel(10.0), 10.0, EPSILON));
    }
}
