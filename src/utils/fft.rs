//! An efficient Fast Fourier transform.
//!
//! A `Transformer` must first be created to perform FFTs. The transformer
//! stores precomputed information to speed up the transform, and can only be
//! used to perform FFTs of the chosen size.
//!
//! # Example
//!
//! The following demonstrates how to use the `Transformer` to perform an 8-point FFT.
//!
//! ```
//! # extern crate num;
//! # extern crate oxcable;
//! # fn main() {
//! use num::complex::Complex32;
//! use num::traits::Zero;
//! use oxcable::utils::fft;
//!
//! // Initialize the FFT
//! let transformer = fft::Transformer::new(8);
//!
//! // Generate an input signal
//! let mut input = Vec::with_capacity(8);
//! for i in 0..8 {
//!     input.push(Complex32::new(i as f32, 0.0));
//! }
//!
//! // Generate an empty output buffer
//! let mut output = vec![Complex32::zero(); 8];
//!
//! // Perform the FFT
//! transformer.fft(&input, &mut output);
//! # }
//! ```
//!
//! After this example, `output` will contain the FFT of `input`.

use std::f32::consts::PI;
use std::vec::Vec;

use num::complex::Complex32;
use num::traits::Zero;


/// A container for precomputed values to perform FFTs of a fixed size.
pub struct Transformer {
    size: usize,
    bit_reverses: Vec<usize>,
    twiddles: Vec<Complex32>,
}

impl Transformer {
    /// Initialize a transformer to perform FFTs of the provided size, rounded
    /// up to the nearest power of two.
    pub fn new(size: usize) -> Transformer {
        // Only operate in powers of two
        let bufsize = size.next_power_of_two();

        // Populate the bit reverses
        // We only use the lower log2(size) bits to express the index
        let mut bit_reverses = Vec::with_capacity(bufsize);
        for i in (0 .. size) {
            let br = bit_reverse(i as u32, int_log(bufsize as u32));
            bit_reverses.push(br as usize);
        }

        // Populate the twiddle factors w_n^i
        // w_n = exp(-j*2*pi*n/N)
        let mut twiddles = Vec::with_capacity(bufsize);
        let exponent = Complex32::new(0.0, -2.0*PI/(bufsize as f32));
        for i in (0 .. bufsize) {
            let c1 = exponent.scale(i as f32);
            let c = Complex32::new(c1.im.cos(), c1.im.sin()).scale(c1.re.exp());
            twiddles.push(c);
        }

        Transformer { size: bufsize, bit_reverses: bit_reverses,
            twiddles: twiddles }
    }

    /// Return the size FFT this Transformer performs.
    pub fn get_size(&self) -> usize {
        self.size
    }

    /// Perform an FFT on `input`, and place the result in `output`.
    ///
    /// The input is zero padded if less than `size` samples are provided, and
    /// truncated if more than `size` samples are provided.
    pub fn fft(&self, input: &[Complex32], output: &mut[Complex32]) {
        self.transform(input, output, false);
    }

    /// Perform an inverse FFT on `input`, and place the result in `output`.
    ///
    /// The input is zero padded if less than `size` samples are provided, and
    /// truncated if more than `size` samples are provided.
    pub fn ifft(&self, input: &[Complex32], output: &mut[Complex32]) {
        self.transform(input, output, true);
    }

    /// Perform the actual transform on `input`, placing the result in `output`.
    ///
    /// This function performs both forward and backwards transforms, since
    /// there are only minor algorithmic differences in the beginning and end
    /// of transformation.
    ///
    /// The input is zero padded if less than `size` samples are provided, and
    /// truncated if more than `size` samples are provided.
    fn transform(&self, input: &[Complex32], output: &mut[Complex32],
                 inverse: bool) {
        // Copy the input into bit reverse order, zero padding if necessary,
        // conjugating if we are inverse transforming
        for i in (0 .. input.len()) {
            output[self.bit_reverses[i]] =
                if inverse {
                    input[i].conj()
                } else {
                    input[i]
                }
        }
        for i in self.bit_reverses[input.len() .. self.size].iter() {
            output[*i] = Complex32::zero();
        }

        // Iteratively perform FFT, starting at 2 points
        let mut n = 2;
        while n <= self.size {
            // For each of the small FFTs
            for set in (0 .. self.size/n) {
                // For each pair of n
                for i in (0 .. n/2) {
                    let ilo = n*set + i;
                    let ihi = ilo + n/2;

                    // Grab out the lower and upper n
                    let lower = output[ilo];
                    let upper = output[ihi] * self.twiddles[self.size/n * i];

                    // Assign them back using a butterfly
                    output[ilo] = lower + upper;
                    output[ihi] = lower - upper;
                }
            }

            // Double the number of points per FFT
            n *= 2;
        }

        // If we are inverse transforming, conjugate and normalize the output
        if inverse {
            for i in (0 .. self.size) {
                output[i] = output[i].conj().scale(1.0/(self.size as f32));
            }
        }
    }
}


/// Return the bit reverse of `n`, for the lower `bits` bits.
///
/// For small examples, the bit reverse of 0b00011010 is 0b01011000, but the bit
/// reverse of just the lower 5 bits is 0b00001011.
fn bit_reverse(n: u32, bits: u32) -> u32 {
    let mut i = n;
    i = (i >> 16) | (i << 16);
    i = ((i & 0xFF00FF00) >> 8) | ((i & 0x00FF00FF) << 8);
    i = ((i & 0xF0F0F0F0) >> 4) | ((i & 0x0F0F0F0F) << 4);
    i = ((i & 0xCCCCCCCC) >> 2) | ((i & 0x33333333) << 2);
    i = ((i & 0xAAAAAAAA) >> 1) | ((i & 0x55555555) << 1);
    i >> ((32 - bits) as u32)
}


/// Return the log base 2 of n, rounded up.
fn int_log(n: u32) -> u32 {
    let mut i = n-1; // correct for exact powers of 2
    let mut res = 0;
    while i > 0 {
        i = i >> 1;
        res += 1;
    }
    res
}


// Unit tests...
#[cfg(test)]
mod test {
    use std::vec::Vec;

    use num::complex::Complex32;
    use num::traits::{One, Zero};

    use super::{Transformer, int_log, bit_reverse};

    // For floating point comparison
    static EPSILON: f32 = 1e-6;

    /// Tests int_log with many values
    #[test]
    fn test_int_log() {
        assert_eq!(int_log(1), 0);
        assert_eq!(int_log(2), 1);
        assert_eq!(int_log(3), 2);
        assert_eq!(int_log(4), 2);
        assert_eq!(int_log(7), 3);
        assert_eq!(int_log(8), 3);
        assert_eq!(int_log(31), 5);
        assert_eq!(int_log(32), 5);
    }

    /// Tests bit_reverse.
    #[test]
    fn test_bit_reverse() {
        assert_eq!(bit_reverse(0x00000000, 32), 0x00000000);
        assert_eq!(bit_reverse(0xFFFFFFFF, 32), 0xFFFFFFFF);
        assert_eq!(bit_reverse(0x00000001, 32), 0x80000000);
        assert_eq!(bit_reverse(0x11111111, 32), 0x88888888);
        assert_eq!(bit_reverse(0x234f9e01, 32), 0x8079f2c4); //random
        assert_eq!(bit_reverse(0x00000001, 4), 0x00000008);
        assert_eq!(bit_reverse(0x0000000F, 4), 0x0000000F);
    }

    /// Tests the FFT of an impulse function.
    ///
    /// Analytically, an impulse function has a constant fourier transform.
    #[test]
    fn test_fft_impulse() {
        let mut impulse = Vec::with_capacity(8);
        let mut out = Vec::with_capacity(8);
        for i in (0 .. 8) {
            impulse.push(if i == 0 { Complex32::one() } else { Complex32::zero() });
            out.push(Complex32::zero());
        }

        let t = Transformer::new(8);
        t.fft(&impulse, &mut out);

        for c in out.iter() {
            assert!(c.eq(&Complex32::one()))
        }
    }

    /// Tests the IFFT of an impulse function.
    ///
    /// Analytically, a constant frequency domain results in an impulse
    /// function.
    #[test]
    fn test_ifft_impulse() {
        let mut impulse = Vec::with_capacity(8);
        let mut out = Vec::with_capacity(8);
        for _i in (0 .. 8) {
            impulse.push(Complex32::one());
            out.push(Complex32::zero());
        }

        let t = Transformer::new(8);
        t.ifft(&impulse, &mut out);

        assert!(out[0].eq(&Complex32::one()));
        for c in out[1 .. 8].iter() {
            assert!(c.eq(&Complex32::zero()));
        }
    }

    /// Tests that the identify property, i.e. IFFT(FTT(f)) == f
    #[test]
    fn test_fft_identity() {
        let mut input = Vec::with_capacity(8);
        let mut fft = Vec::with_capacity(8);
        let mut out = Vec::with_capacity(8);
        for i in (0 .. 8) {
            input.push(Complex32::new((i+1) as f32, 0.0));
            fft.push(Complex32::zero());
            out.push(Complex32::zero());
        }

        let t = Transformer::new(8);
        t.fft(&input, &mut fft);
        t.ifft(&fft, &mut out);

        for i in (0 .. 7) {
            println!("{}",out[i].re - ((i+1) as f32));
            assert!(out[i].re - ((i+1) as f32) < EPSILON);
            assert!(out[i].im < EPSILON);
        }
    }

    /// Tests that the Transformer properly handles input buffers that are too
    /// short by zero padding them.
    #[test]
    fn test_fft_zero_pad() {
        let mut input = Vec::with_capacity(7);
        let mut fft = Vec::with_capacity(8);
        let mut out = Vec::with_capacity(8);
        for i in (0 .. 8) {
            if i < 7 {
                input.push(Complex32::new((i+1) as f32, 0.0));
            }
            fft.push(Complex32::zero());
            out.push(Complex32::zero());
        }

        let t = Transformer::new(8);
        t.fft(&input, &mut fft);
        t.ifft(&fft, &mut out);

        for i in (0 .. 7) {
            println!("{}",out[i].re - ((i+1) as f32));
            assert!(out[i].re - ((i+1) as f32) < EPSILON);
            assert!(out[i].im < EPSILON);
        }
    }
}
