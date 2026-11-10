use std::{fmt, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

// to make it clear this type must always be used for complex numbers
pub type QFloat = f64;

// Yes, I REALIZE that there is already a complex-num crate, but I refuse
// to use it because I want to learn as much as I can about the mathematics
// behind quantum computing.
#[derive(Copy, Clone)]
pub struct Complex {
    pub re: QFloat,
    pub im: QFloat,
}

/*
    Adding two complex numbers is simple:
    - add the real components and return them as the real component of a new complex number
    - do the same as above for the imaginary components
*/
impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}
impl Add<QFloat> for Complex {
    type Output = Self;

    fn add(self, rhs: QFloat) -> Self::Output {
        Self { re: self.re + rhs, im: self.im }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}
impl AddAssign<QFloat> for Complex {
    fn add_assign(&mut self, rhs: QFloat) {
        self.re += rhs;
    }
}

// Subtraction works much the same way as adding two complex numbers
impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { re: self.re - rhs.re, im: self.im - rhs.im }
    }
}
impl Sub<QFloat> for Complex {
    type Output = Self;

    fn sub(self, rhs: QFloat) -> Self::Output {
        Self { re: self.re - rhs, im: self.im }
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}
impl SubAssign<QFloat> for Complex {
    fn sub_assign(&mut self, rhs: QFloat) {
        self.re -= rhs;
    }
}

// Simply make both components negative
impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { re: -self.re, im: -self.im }
    }
}

/* 
    Multiplication is trickier since it has to follow the "FOIL" rule. 
    Here's how that works, in-depth, if we have two complex numbers:

    (a + bi) * (c + di)

    - ...then they are multiplied like so:

    = ac + adi + bci + (bi * di)
    = ac + (ad + bc)i + bdi^2
    = ac + (ad + bc)i - bd
    = (ac - bd) + (ad + bc)i
*/
impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.re;
        let b = self.im;
        let c = rhs.re;
        let d = rhs.im;

        Self { 
            re: (((a*c - b*d) * 1_000_000.0).round()) / 1_000_000.0, 
            im: (((a*d + b*c) * 1_000_000.0).round()) / 1_000_000.0
        }
    }
}
impl Mul<QFloat> for Complex {
    type Output = Self;

    fn mul(self, rhs: QFloat) -> Self::Output {
        Self { re: self.re * rhs, im: self.im * rhs }
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        let a = self.re;
        let b = self.im;
        let c = rhs.re;
        let d = rhs.im;

        self.re = (((a*c - b*d) * 1_000_000.0).round()) / 1_000_000.0;
        self.im = (((a*d + b*c) * 1_000_000.0).round()) / 1_000_000.0;
    }
}
impl MulAssign<QFloat> for Complex {
    fn mul_assign(&mut self, rhs: QFloat) {
        self.re *= rhs;
        self.im *= rhs;
    }
}

/*
    Dividing two complex numbers: let's say we have this:

    (a + bi) / (c + di)

    ... This division would be handled by multiplying both sides of the fraction
        by the denominator's complex conjugate

    (a + bi) * (c - di)
    -------------------
    (c + di) * (c - di)

    ... We already know how to multiply two complex numbers, and doing so would
        simplify the fraction to this:

    (ac + bd) + (bc - ad)i
    ---------------------
    c^2 + cdi - cdi -d^2*i^2

    ... Removing redundant or simplifiable parts of the denominator:

    (ac + bd) + (bc - ad)i
    ---------------------
          c^2 + d^2
    
    ... From here, we have everything we need.
*/
impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let a = self.re;
        let b = self.im;
        let c = rhs.re;
        let d = rhs.im;

        let denom = c*c + d*d;

        Self { re: (a*c + b*d) / denom, im: (b*c - a*d) / denom }
    }
}
impl Div<QFloat> for Complex {
    type Output = Self;

    fn div(self, rhs: QFloat) -> Self::Output {
        Self { re: self.re / rhs, im: self.im / rhs }
    }
}

impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        let a = self.re;
        let b = self.im;
        let c = rhs.re;
        let d = rhs.im;

        let denom = c*c + d*d;

        self.re = (a*c + b*d) / denom;
        self.im = (b*c - a*d) / denom;
    }
}
impl DivAssign<QFloat> for Complex {
    fn div_assign(&mut self, rhs: QFloat) {
        self.re /= rhs;
        self.im /= rhs;
    }
}

// Make it possible to convert a single float into a complex number
impl From<QFloat> for Complex {
    fn from(value: QFloat) -> Self {
        Self { re: value, im: 0.0 }
    }
}

// Make it printable
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.im < 0.0 {
            "-"
        } else {
            " + "
        };

        write!(f, "{} {} {}i", self.re, sign, self.im.abs())
    }
}

// Make it debug printable
impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Real: {}, Imag: {}", self.re, self.im)
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

// Now that standard operations are squared away...
impl Complex {
    pub fn new(re: QFloat, im: QFloat) -> Self {
        Self { re, im }
    }

    // Imagine that the complex number is represented by a vector.
    // The modulus of that complex number would be the same as the
    // magnitude of the vector, that is:

    // modulus(a + bi) = sqrt(a^2 + b^2)
    pub fn modulus(&self) -> QFloat {
        (self.re*self.re + self.im*self.im).sqrt()
    }

    // Normalize the complex number, setting its modulus to 1
    pub fn norm(&self) -> Self {
        Self { re: self.re / self.modulus(), im: self.im / self.modulus() }
    }

    // Invert the imaginary component of the number
    pub fn conjugate(&self) -> Self {
        Self { re: self.re, im: -self.im }
    }
}

// Tests go here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let c1 = Complex::new(5.0, 1.0);
        let c2 = Complex::new(3.0, 4.0);

        assert_eq!((c1 + c2), Complex::new(8.0, 5.0));
    }

    #[test]
    fn test_sub() {
        let c1 = Complex::new(5.0, 3.0);
        let c2 = Complex::new(2.0, 6.0);

        assert_eq!((c1 - c2), Complex::new(3.0, -3.0));
    }

    #[test]
    fn test_mul() {
        let c1 = Complex::new(3.0, 5.0);
        let c2 = Complex::new(2.0, 4.0);

        assert_eq!((c1 * c2), Complex::new(-14.0, 22.0));
    }

    #[test]
    fn test_div() {
        let c1 = Complex::new(3.0, 5.0);
        let c2 = Complex::new(2.0, 4.0);

        assert_eq!((c1 / c2), Complex::new(1.3, -0.1));
    }

    #[test]
    fn test_neg() {
        let c = Complex::new(3.0, 2.0);

        assert_eq!(-c, Complex::new(-3.0, -2.0));
    }

    #[test]
    fn test_modulus() {
        let c = Complex::new(3.0, 4.0);

        assert_eq!(c.modulus(), 5.0);
    }

    #[test]
    fn test_norm() {
        let c = Complex::new(3.0, 4.0);

        assert_eq!(c.norm(), Complex::new(3.0 / 5.0, 4.0 / 5.0));
    }

    #[test]
    fn test_conjugate() {
        let c = Complex::new(3.0, 5.0);

        assert_eq!(c.conjugate(), Complex::new(3.0, -5.0))
    }
}