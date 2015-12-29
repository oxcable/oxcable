//! A collection of window functions.
//!
//! All functions take a size `n` and return vectors with `n` elements.

use std::f32::consts::PI;

use types::Sample;


/// Returns a bartlett (triangular) window of size `n`.
pub fn bartlett(n: usize) -> Vec<Sample> {
    let mut window = Vec::with_capacity(n);
    for i in 0..n {
        let z = 2.0*i as f32/(n as f32-1.0);
        if z <= 1.0 {
            window.push(z);
        } else {
            window.push(2.0-z);
        }
    }
    window
}


/// Returns a generalized cosine window of size `n`, with the provided
/// coefficients.
///
/// Hanning, hamming, and blackmann windows are all generalized cosine windows.
pub fn generalized_cosine_window(alphas: &[Sample], n: usize) -> Vec<Sample> {
    let mut window = Vec::with_capacity(n);
    let f0 = 2.0*PI / ((n-1) as f32);
    for i in 0..n {
        let mut wi = 0.0;
        for (k, ak) in alphas.iter().enumerate() {
            wi += ak * (f0*k as f32*i as f32).cos();
        }
        window.push(wi);
    }
    window
}

/// Returns a hanning window of size `n`.
pub fn hanning(n: usize) -> Vec<Sample> {
    generalized_cosine_window(&[0.5, -0.5], n)
}

/// Returns a hamming window of size `n`.
pub fn hamming(n: usize) -> Vec<Sample> {
    generalized_cosine_window(&[0.54, -0.46], n)
}

/// Returns a blackman window of size `n`.
pub fn blackman(n: usize) -> Vec<Sample> {
    generalized_cosine_window(&[0.42, -0.5, 0.08], n)
}


#[cfg(test)]
mod tests {
    use utils::helpers::flt_eq;
    use types::Sample;
    static EPSILON: f32 = 1e-6;

    fn check_window(actual: &[Sample], expected: &[Sample]) {
        for (a, e) in actual.iter().zip(expected) {
            println!("{}, {}", a, e);
            assert!(flt_eq(*a, *e, EPSILON));
        }
    }

    #[test]
    fn test_bartlett() {
        use super::bartlett;
        check_window(&bartlett(8),
                     &[0.0, 0.28571429, 0.57142857, 0.85714286, 0.85714286,
                       0.57142857, 0.28571429, 0.0])
    }

    #[test]
    fn test_hanning() {
        use super::hanning;
        check_window(&hanning(8),
                     &[0.0, 0.1882551, 0.61126047, 0.95048443, 0.95048443,
                       0.61126047, 0.1882551, 0.0]);
    }

    #[test]
    fn test_hamming() {
        use super::hamming;
        check_window(&hamming(8),
                     &[0.08, 0.25319469, 0.64235963, 0.95444568, 0.95444568,
                       0.64235963, 0.25319469, 0.08]);
    }

    #[test]
    fn test_blackman() {
        use super::blackman;
        check_window(&blackman(8),
                     &[-1.38777878e-17, 9.04534244e-02, 4.59182958e-01,
                       9.20363618e-01, 9.20363618e-01, 4.59182958e-01,
                       9.04534244e-02, -1.38777878e-17]);
    }
}
