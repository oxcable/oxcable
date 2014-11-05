use std::fmt;

pub struct Complex {
    r: f64,
    i: f64,
}

impl Complex {
    pub fn new(r: f64, i: f64) -> Complex {
        Complex { r: r, i: i }
    }

    pub fn from_real(r: f64) -> Complex {
        Complex { r: r, i: 0f64 }
    }

    pub fn zero() -> Complex {
        Complex { r: 0.0, i: 0.0 }
    }

    pub fn real(&self) -> f64 {
        self.r
    }

    pub fn imag(&self) -> f64 {
        self.i
    }

    pub fn conj(&self) -> Complex {
        Complex { r: self.r, i: -self.i }
    }

    pub fn abs(&self) -> f64 {
        (self.r*self.r + self.i*self.i).sqrt()
    }

    pub fn scale(&self, s: f64) -> Complex {
        Complex { r: self.r*s, i: self.i*s }
    }

    pub fn exp(&self) -> Complex {
        Complex { r: self.i.cos(), i: self.i.sin() }.scale(self.r.exp())
    }

    pub fn eq(&self, other: &Complex) -> bool {
        self.r == other.r && self.i == other.i
    }
}

impl fmt::Show for Complex {
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
