use std::{fmt::{Display}, ops::MulAssign};

use crate::{complex::Complex, qmatrix::QMatrix};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QubitState(pub Complex, pub Complex);

const KET_0: QubitState = QubitState(
    Complex { re: 1.0, im: 0.0 },
    Complex { re: 0.0, im: 0.0 }
);
const KET_1: QubitState = QubitState(
    Complex { re: 0.0, im: 0.0 },
    Complex { re: 1.0, im: 0.0 }
);

pub struct Qubit {
    state: QubitState,
}

impl MulAssign<QMatrix> for Qubit {
    fn mul_assign(&mut self, rhs: QMatrix) {
        let _ = self.apply_matrix(rhs);
    }
}

impl Display for QubitState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})|0> + ({})|1>", self.0, self.1)
    }
}

impl Qubit {
    pub fn new() -> Self {
        Self {
            state: KET_0
        }
    }

    pub fn apply_matrix(&mut self, m: QMatrix) -> Result<(), String> {
        let state = self.state;

        if !m.is_unitary() {
            return Err(format!("apply_matrix: matrix is not unitary"));
        }

        self.state.0 = m.0 * state.0 + m.1 * state.1;
        self.state.1 = m.2 * state.0 + m.3 * state.1;

        Ok(())
    }

    /*
        NOTE: This should only be used for debugging and testing. In a real
        quantum computer, observing the state would cause the qubit's state
        to change to one of the basis vectors.
    */
    pub fn observe_state_debug(&self) -> QubitState {
        self.state
    }

    /*
        NOTE: This is used for actually measuring the state of a qubit. It
        will lock the qubit into one of the basis vectors, |0> or |1>
    */
    pub fn observe_state(&mut self) -> QubitState {
        let a = ((self.state.0 * self.state.0).modulus() * 1_000_000.0).round() as usize;

        let r = rand::random_range(0..=1_000_000);

        if r < a {
            self.state = KET_0;
        } else {
            self.state = KET_1;
        }

        self.state
    }
}