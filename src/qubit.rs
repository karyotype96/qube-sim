use std::{fmt::{Display}};

use crate::{complex::Complex, gates::{KET_0, KET_1}, qubit_matrix::QubitMatrix};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QubitState(pub Complex, pub Complex);

pub struct Qubit {
    state: QubitState,
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

    pub fn apply_matrix(&mut self, m: QubitMatrix) -> Result<(), String> {
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
        NOTE: This is used for actually measuring the state of a qubit in the 
        computational basis. It will lock the qubit into one of the basis vectors,
        |0> or |1>.
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