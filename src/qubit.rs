use std::{fmt::{Display}, ops::MulAssign};

use rand_distr::Uniform;

use crate::{complex::Complex, qmatrix::QMatrix};

pub struct Qubit {
    state: (Complex, Complex),
}

impl MulAssign<QMatrix> for Qubit {
    fn mul_assign(&mut self, rhs: QMatrix) {
        self.apply_matrix(rhs);
    }
}

impl Display for Qubit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})|0> + ({})|1>", self.state.0, self.state.1)
    }
}

impl Qubit {
    pub fn new() -> Self {
        Self {
            state: (Complex::new(1.0, 0.0), Complex::new(0.0, 0.0))
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
    pub fn observe_state_debug(&self) -> (Complex, Complex) {
        self.state
    }

    /*
        NOTE: This is used for actually measuring the state of a qubit. It
        will lock the qubit into one of the basis vectors, |0> or |1>
    */
    pub fn observe_state(&mut self) -> (Complex, Complex) {
        let (a, b) = (self.state.0 * self.state.0, self.state.1 * self.state.1);

        todo!()
    }
}