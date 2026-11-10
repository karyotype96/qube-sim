use std::{fmt::{Debug, Display}, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use crate::{complex::{Complex, QFloat}, gates::{IDENTITY}};

#[derive(Copy, Clone)]
pub struct QMatrix(pub Complex, pub Complex, pub Complex, pub Complex);

// simple arithmetic functions
impl Add for QMatrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, self.3 + rhs.3)
    }
}
impl AddAssign for QMatrix {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self.3 += rhs.3;
    }
}

impl Sub for QMatrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2, self.3 - rhs.3)
    }
}
impl SubAssign for QMatrix {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
        self.3 -= rhs.3;
    }
}

impl Mul for QMatrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (a, b, c, d) = (self.0, self.1, self.2, self.3);
        let (e, f, g, h) = (rhs.0, rhs.1, rhs.2, rhs.3);

        Self(a*e + b*g, a*f + b*h, c*e + d*g, c*f + d*h)
    }
}
impl Mul<QFloat> for QMatrix {
    type Output = Self;

    fn mul(self, rhs: QFloat) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl MulAssign for QMatrix {
    fn mul_assign(&mut self, rhs: Self) {
        let (a, b, c, d) = (self.0, self.1, self.2, self.3);
        let (e, f, g, h) = (rhs.0, rhs.1, rhs.2, rhs.3);

        self.0 = a*e + b*g;
        self.1 = a*f + b*h;
        self.2 = c*e + d*g;
        self.3 = c*f + d*h;
    }
}
impl MulAssign<QFloat> for QMatrix {
    fn mul_assign(&mut self, rhs: QFloat) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
        self.3 *= rhs;
    }
}

impl Div<QFloat> for QMatrix {
    type Output = Self;
    
    fn div(self, rhs: QFloat) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}
impl DivAssign<QFloat> for QMatrix {
    fn div_assign(&mut self, rhs: QFloat) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
        self.3 /= rhs;
    }
}

impl PartialEq for QMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0 || self.1 != other.1 || self.2 != other.2 || self.3 != other.3
    }
}

// Printing functionality

impl Display for QMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},\t{}]\n[{},\t{}]", self.0, self.1, self.2, self.3)
    }
}

impl Debug for QMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}

impl QMatrix {
    pub fn new(a1: Complex, a2: Complex, b1: Complex, b2: Complex) -> Self {
        Self(a1, a2, b1, b2)
    }

    /* 
        If we have a matrix:
    
        [a, b]
        [c, d]

        ... the transpose would be simply...

        [a, c]
        [b, d]
    */
    pub fn transpose(&self) -> Self {
        Self(self.0, self.2, self.1, self.3)
    }

    pub fn conjugate(&self) -> Self {
        Self(self.0.conjugate(), self.1.conjugate(), self.2.conjugate(), self.3.conjugate())
    }

    pub fn adjoint(&self) -> Self {
        self.transpose().conjugate()
    }

    pub fn is_unitary(&self) -> bool {
        *self * self.adjoint() == IDENTITY
    }
}

// Tests go here
#[cfg(test)]
pub mod tests {
    use crate::gates::{C_I, C_ONE, C_ZERO};

    use super::*;

    #[test]
    fn test_qmatrix_add() {
        let matrix1 = QMatrix::new(1.0.into(), 2.0.into(), 3.0.into(), 4.0.into());
        let matrix2 = QMatrix::new(4.0.into(), 3.0.into(), 2.0.into(), 1.0.into());

        assert_eq!(matrix1 + matrix2, QMatrix::new(5.0.into(), 5.0.into(), 5.0.into(), 5.0.into()))
    }

    #[test]
    fn test_qmatrix_sub() {
        let matrix1 = QMatrix::new(2.0.into(), 3.0.into(), 4.0.into(), 5.0.into());
        let matrix2 = QMatrix::new(1.0.into(), 1.0.into(), 2.0.into(), 2.0.into());

        assert_eq!(matrix1 - matrix2, QMatrix::new(1.0.into(), 2.0.into(), 2.0.into(), 3.0.into()))
    }

    #[test]
    fn test_qmatrix_transpose() {
        let matrix = QMatrix::new(1.0.into(), 2.0.into(), 3.0.into(), 4.0.into());

        assert_eq!(matrix.transpose(), QMatrix::new(1.0.into(), 3.0.into(), 2.0.into(), 4.0.into()))
    }

    #[test]
    fn test_qmatrix_conjugate() {
        let matrix = QMatrix::new(
            Complex::new(3.0, 5.0), 
            Complex::new(2.0, -1.0),
            Complex::new(-1.0, 5.0),
            Complex::new(-2.0, -4.0)
        );

        let conjugate = QMatrix::new(
            Complex::new(3.0, -5.0),
            Complex::new(2.0, 1.0),
            Complex::new(-1.0, -5.0),
            Complex::new(-2.0, 4.0)
        );

        assert_eq!(matrix.conjugate(), conjugate);
    }

    #[test]
    fn test_qmatrix_unitary() {
        let y = QMatrix::new(
            C_ZERO, -C_I,
            C_I, C_ZERO
        );

        let z = QMatrix::new(
            C_ONE, C_ZERO,
            C_ZERO, -C_ONE
        );

        assert!(y.is_unitary());
        assert!(z.is_unitary());
    }
}