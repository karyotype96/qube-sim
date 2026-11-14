use std::{f64::consts::SQRT_2};

use crate::{complex::Complex, qubit::{Qubit}, qubit_matrix::QubitMatrix};

pub const C_ZERO: Complex = Complex { re: 0.0, im: 0.0 };
pub const C_ONE: Complex = Complex { re: 1.0, im: 0.0 };
pub const C_N_ONE: Complex = Complex { re: -1.0, im: 0.0 };
pub const C_I: Complex = Complex { re: 0.0, im: 1.0 };
pub const C_NI: Complex = Complex { re: 0.0, im: -1.0 };