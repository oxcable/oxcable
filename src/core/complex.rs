//! Provides a complex number type and corresponding arithmetic.

#![unstable="operator overloads in rust are not yet stable"]

use std::fmt;
use std::num::Float;
use std::ops::{Neg, Add, Sub, Mul};


/// A complex number, stored in rectangular form to 64-bit precision.
///
/// This type has operator overloads for addition, subtraction, multiplication
/// and negation.
#[derive(Clone, Copy)]
pub struct Complex {
    r: f32,
    i: f32,
}

impl Complex {
    /// Returns a complex number with real part `r` and imaginary part `i`.
    pub fn new(r: f32, i: f32) -> Complex {
        Complex { r: r, i: i }
    }

    /// Returns a complex number with real part `r` and zero imaginary part.
    pub fn from_real(r: f32) -> Complex {
        Complex { r: r, i: 0f32 }
    }

    /// Returns a complex number with zero real and imaginary parts.
    pub fn zero() -> Complex {
        Complex { r: 0.0, i: 0.0 }
    }

    /// Returns the real part.
    #[inline]
    pub fn real(&self) -> f32 {
        self.r
    }

    /// Returns the imaginary part.
    #[inline]
    pub fn imag(&self) -> f32 {
        self.i
    }

    /// Returns the complex conjugate.
    #[inline]
    pub fn conj(&self) -> Complex {
        Complex { r: self.r, i: -self.i }
    }

    /// Returns the absolute value.
    #[inline]
    pub fn abs(&self) -> f32 {
        (self.r*self.r + self.i*self.i).sqrt()
    }

    /// Multiplies the complex number by the scalar `s`.
    #[inline]
    pub fn scale(&self, s: f32) -> Complex {
        Complex { r: self.r*s, i: self.i*s }
    }

    /// For a complex number `c`, returns `e^c`.
    #[inline]
    pub fn exp(&self) -> Complex {
        Complex { r: self.i.cos(), i: self.i.sin() }.scale(self.r.exp())
    }

    /// Returns true if the two complex numbers are equal.
    #[inline]
    pub fn eq(self, rhs: Complex) -> bool {
        self.r == rhs.r && self.i == rhs.i
    }
}

impl fmt::Display for Complex {
    /// Prints the complex number in rectangular form.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.i == 0.0 {
            write!(f, "{}", self.r)
        } else if self.r == 0.0 {
            write!(f, "{}j", self.i)
        } else if self.i >= 0.0 {
            write!(f, "{}+{}j", self.r, self.i)
        } else { // self.i < 0.0
            write!(f, "{}{}j", self.r, self.i)
        }
    }
}

impl Neg for Complex {
    type Output = Complex;
    #[inline]
    fn neg(self) -> Complex {
        Complex { r: -self.r, i: -self.i }
    }
}

impl Add for Complex {
    type Output = Complex;
    #[inline]
    fn add(self, rhs: Complex) -> Complex {
        Complex {r: self.r + rhs.r, i: self.i + rhs.i}
    }
}

impl Sub for Complex {
    type Output = Complex;
    #[inline]
    fn sub(self, rhs: Complex) -> Complex {
        Complex {r: self.r - rhs.r, i: self.i - rhs.i}
    }
}

impl Mul for Complex {
    type Output = Complex;
    #[inline]
    fn mul(self, rhs: Complex) -> Complex {
        Complex {r: self.r*rhs.r - self.i*rhs.i, 
                 i: self.r*rhs.i + self.i*rhs.r}
    }
}
