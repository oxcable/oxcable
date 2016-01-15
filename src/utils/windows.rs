//! A collection of window functions.
//!
//! All functions take a size `n` and return vectors with `n` elements.
//!
//! Window calculation can be costly depending on the size and type of window,
//! so it is recommended to precompute windows whenever possible.

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


/// Numerically estimates a zeroth order modified bessel function.
///
/// The mathematical function is an infinite summation, but the terms quickly go
/// to zero, so we instead use just the first k terms.
fn i0(x: Sample, k: usize) -> Sample {
    let mut ifact = 1.0;
    let mut y = 0.0;
    for i in 0..k {
        // Compute i factorial iteratively
        if i > 1 { ifact *= i as f32; }
        y += (x/2.0).powf(2.0*i as f32) / (ifact*ifact);
    }
    y
}

/// Returns a kaiser window of size `n`, with the provided beta.
pub fn kaiser(beta: Sample, n: usize) -> Vec<Sample> {
    let mut window = Vec::with_capacity(n);
    let i0_beta = i0(beta, 20);
    for i in 0..n {
        let tmp = 2.0*i as f32/(n-1) as f32 - 1.0;
        let x = beta * (1.0 - tmp*tmp).sqrt();
        window.push(i0(x, 20)/i0_beta);
    }
    window
}


#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use testing::flt_eq;
    use types::Sample;

    fn check_window(actual: &[Sample], expected: &[Sample]) {
        for (a, e) in actual.iter().zip(expected) {
            assert!(flt_eq(*a, *e));
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

    #[test]
    fn test_kaiser() {
        use super::kaiser;
        check_window(&kaiser(2.0*PI, 8),
                     &[0.01147993, 0.18336612, 0.57527808, 0.94267182,
                       0.94267182, 0.57527808, 0.18336612, 0.01147993]);
    }
}
