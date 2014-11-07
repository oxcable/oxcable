//! A complex number type and corresponding arithmetic.
//!
//! This module provides operator overloads for addition, subtraction,
//! multiplication and negation.

#![unstable="operator overloads in rust are not yet stable"]

use std::fmt;


#[unstable="operator overloads in rust are not yet stable"]
/// A complex number, stored to in rectangular form to 64-bit precision.
pub struct Complex {
    r: f64,
    i: f64,
}

#[stable]
impl Complex {
    #[stable]
    /// Returns a complex number with real part `r` and imaginary part `i`.
    pub fn new(r: f64, i: f64) -> Complex {
        Complex { r: r, i: i }
    }

    #[stable]
    /// Returns a complex number with real part `r` and zero imaginary part.
    pub fn from_real(r: f64) -> Complex {
        Complex { r: r, i: 0f64 }
    }

    #[stable]
    /// Returns a complex number with zero real and imaginary parts.
    pub fn zero() -> Complex {
        Complex { r: 0.0, i: 0.0 }
    }

    #[stable]
    /// Returns the real part.
    pub fn real(&self) -> f64 {
        self.r
    }

    #[stable]
    /// Returns the imaginary part.
    pub fn imag(&self) -> f64 {
        self.i
    }

    #[stable]
    /// Returns the complex conjugate.
    pub fn conj(&self) -> Complex {
        Complex { r: self.r, i: -self.i }
    }

    #[stable]
    /// Returns the absolute value.
    pub fn abs(&self) -> f64 {
        (self.r*self.r + self.i*self.i).sqrt()
    }

    #[stable]
    /// Multiplies the complex number by the scalar `s`.
    pub fn scale(&self, s: f64) -> Complex {
        Complex { r: self.r*s, i: self.i*s }
    }

    #[stable]
    /// For a complex number `c`, returns `e^c`.
    pub fn exp(&self) -> Complex {
        Complex { r: self.i.cos(), i: self.i.sin() }.scale(self.r.exp())
    }

    #[stable]
    /// Returns true if the two complex numbers are equal.
    pub fn eq(&self, other: &Complex) -> bool {
        self.r == other.r && self.i == other.i
    }
}

impl fmt::Show for Complex {
    /// Prints the complex number in rectangular form.
    #[stable]
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

impl Neg<Complex> for Complex {
    fn neg(&self) -> Complex {
        Complex { r: -self.r, i: -self.i }
    }
}

impl Add<Complex, Complex> for Complex {
    fn add(&self, other: &Complex) -> Complex {
        Complex {r: self.r + other.r, i: self.i + other.i}
    }
}

impl Sub<Complex, Complex> for Complex {
    fn sub(&self, other: &Complex) -> Complex {
        Complex {r: self.r - other.r, i: self.i - other.i}
    }
}

impl Mul<Complex, Complex> for Complex {
    fn mul(&self, other: &Complex) -> Complex {
        Complex {r: self.r*other.r - self.i*other.i, 
                 i: self.r*other.i + self.i*other.r}
    }
}
