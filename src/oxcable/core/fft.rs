use std::f64::consts::PI;
use std::num::next_power_of_two;
use std::vec::Vec;

use core::complex::Complex;


/// Performs Discrete Fourier Transforms of a fixed size.
///
/// 
pub struct Transformer {
    size: uint,
    bit_reverses: Vec<uint>,
    twiddles: Vec<Complex>,    
}

impl Transformer {
    pub fn new(size: uint) -> Transformer {
        // Populate the bit reverses
        // Uses bit shift because we only use the lower log2(size) bits to
        // express the index, and these get shoved into the upper bits of the
        // reverse
        let bufsize = next_power_of_two(size);
        let mut bit_reverses = Vec::with_capacity(bufsize);
        for i in range(0, size) {
            bit_reverses.push((bit_reverse(i as u32) as uint) >> 
                              (32-int_log(bufsize as u32) as uint));
        }

        // Populate the twiddle factors w_n^i
        // w_n = exp(-j*2*pi*n/N)
        let mut twiddles = Vec::with_capacity(bufsize);
        let exponent = Complex::new(0.0, -2.0*PI/(bufsize as f64));
        for i in range(0, bufsize) {
            twiddles.push(exponent.scale(i as f64).exp());
        }

        Transformer { size: bufsize, bit_reverses: bit_reverses, 
            twiddles: twiddles }
    }

    pub fn get_size(&self) -> uint {
        self.size
    }

    pub fn fft(&self, input: &Vec<Complex>, output: &mut Vec<Complex>) -> 
        Result<(),()> {
        self.transform(input, output, false)
    }

    pub fn ifft(&self, input: &Vec<Complex>, output: &mut Vec<Complex>) ->
        Result<(),()> {
        self.transform(input, output, true)
    }

    fn transform(&self, input: &Vec<Complex>, output: &mut Vec<Complex>,
                     inverse: bool) -> Result<(),()> {
        // Verify the provided vector is big enough for the result
        if output.len() < self.size {
            return Err(())
        }

        // Copy the input into bit reverse order, zero padding if necessary,
        // conjugating if we are inverse transforming
        for i in range(0, input.len()) {
            output[self.bit_reverses[i]] = 
                if inverse { 
                    input[i].conj()
                } else { 
                    input[i]
                }
        }
        for i in self.bit_reverses.slice(input.len(), self.size).iter() {
            output[*i] = Complex::from_real(0.0);
        }

        // Iteratively perform FFT, starting at 2 points
        let mut n = 2;
        while n <= self.size {
            // For each of the small FFTs
            for set in range(0, self.size/n) {
                // For each pair of n
                for i in range(0, n/2) {
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
            for i in range(0, self.size) {
                output[i] = output[i].conj().scale(1.0/(self.size as f64));
            }
        }

        Ok(())
    }
}


fn bit_reverse(n: u32) -> u32 {
    let mut i = n;
    i = (i >> 16) | (i << 16);
    i = ((i & 0xFF00FF00) >> 8) | ((i & 0x00FF00FF) << 8);
    i = ((i & 0xF0F0F0F0) >> 4) | ((i & 0x0F0F0F0F) << 4);
    i = ((i & 0xCCCCCCCC) >> 2) | ((i & 0x33333333) << 2);
    i = ((i & 0xAAAAAAAA) >> 1) | ((i & 0x55555555) << 1);
    i
}

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

    use core::complex::Complex;
    use super::{Transformer, int_log, bit_reverse};

    #[test]
    /// Tests int_log a few edge cases
    fn test_int_log() {
        assert!(int_log(1) == 0);
        assert!(int_log(2) == 1);
        assert!(int_log(3) == 2);
        assert!(int_log(4) == 2);
        assert!(int_log(7) == 3);
        assert!(int_log(8) == 3);
        assert!(int_log(31) == 5);
        assert!(int_log(32) == 5);
    }

    #[test]
    /// Tests bit_reverse.
    fn test_bit_reverse() {
        assert!(bit_reverse(0x00000000) == 0x00000000);
        assert!(bit_reverse(0xFFFFFFFF) == 0xFFFFFFFF);
        assert!(bit_reverse(0x00000001) == 0x80000000);
        assert!(bit_reverse(0x11111111) == 0x88888888);
        assert!(bit_reverse(0x234f9e01) == 0x8079f2c4); //random
    }

    #[test]
    /// Tests the FFT of an impulse function.
    ///
    /// Analytically, an impulse function has a constant fourier transform.
    fn test_fft_impulse() {
        let zero = Complex::zero();
        let one = Complex::from_real(1.0);

        let mut impulse = Vec::with_capacity(8);
        let mut out = Vec::with_capacity(8);
        for i in range(0u, 8) {
            impulse.push(if i == 0 { one } else { zero });
            out.push(Complex::zero());
        }

        let t = Transformer::new(8);
        assert!(t.fft(&impulse, &mut out).is_ok());

        for c in out.iter() {
            assert!(c.eq(&one))
        }
    }

    #[test]
    /// Tests the IFFT of an impulse function.
    ///
    /// Analytically, a constant frequency domain results in an impulse
    /// function.
    fn test_ifft_impulse() {
        let zero = Complex::zero();
        let one = Complex::from_real(1.0);

        let mut impulse = Vec::with_capacity(8);
        let mut out = Vec::with_capacity(8);
        for _i in range(0u, 8) {
            impulse.push(one);
            out.push(zero);
        }

        let t = Transformer::new(8);
        assert!(t.ifft(&impulse, &mut out).is_ok());

        assert!(out[0].eq(&one));
        for c in out.slice(1,8).iter() {
            assert!(c.eq(&zero));
        }
    }

    #[test]
    /// Tests that the identify property, i.e. IFFT(FTT(f)) == f
    fn test_fft_identity() {
        let zero = Complex::zero();
        let epsilon = 1e-8;

        let mut input = Vec::with_capacity(8);
        let mut fft = Vec::with_capacity(8);
        let mut out = Vec::with_capacity(8);
        for i in range(0u, 8) {
            input.push(Complex::from_real((i+1) as f64));
            fft.push(zero);
            out.push(zero);
        }

        let t = Transformer::new(8);
        assert!(t.fft(&input, &mut fft).is_ok());
        assert!(t.ifft(&fft, &mut out).is_ok());

        for i in range(0u,7) {
            println!("{}",out[i].real() - ((i+1) as f64));
            assert!(out[i].real() - ((i+1) as f64) < epsilon);
            assert!(out[i].imag() < epsilon);
        }
    }

    #[test]
    /// Tests that the Transformer properly handles input buffers that are too
    /// short by zero padding them.
    fn test_fft_zero_pad() {
        let zero = Complex::zero();
        let epsilon = 1e-8;

        let mut input = Vec::with_capacity(7);
        let mut fft = Vec::with_capacity(8);
        let mut out = Vec::with_capacity(8);
        for i in range(0u, 8) {
            if i < 7 {
                input.push(Complex::from_real((i+1) as f64));
            } 
            fft.push(zero);
            out.push(zero);
        }

        let t = Transformer::new(8);
        assert!(t.fft(&input, &mut fft).is_ok());
        assert!(t.ifft(&fft, &mut out).is_ok());

        for i in range(0u,7) {
            println!("{}",out[i].real() - ((i+1) as f64));
            assert!(out[i].real() - ((i+1) as f64) < epsilon);
            assert!(out[i].imag() < epsilon);
        }
    }

    #[test]
    /// Tests that the transformer fails when the output buffer is too short.
    fn test_fft_output_buffer_too_small() {
        let zero = Complex::zero();

        let mut input = Vec::with_capacity(8);
        let mut out = Vec::with_capacity(7);
        for i in range(0u, 8) {
            input.push(zero);
            if i < 7 {
                out.push(zero);
            } 
        }

        let t = Transformer::new(8);
        assert!(t.fft(&input, &mut out).is_err());
    }
}
