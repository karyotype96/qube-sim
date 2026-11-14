use crate::{complex::Complex};

#[derive(Copy, Clone, Default, Debug)]
pub enum QubitState {
    #[default]
    Unobserved,
    Ket0,
    Ket1
}

#[derive(Copy, Clone, Debug)]
pub struct Qubit(pub QubitState);

pub struct QuantumCircuit {
    qubits: Vec<Qubit>,
    amplitudes: Vec<Complex>
}

impl QuantumCircuit {
    pub fn new(qubit_count: usize) -> Self {
        let mut s = Self {
            qubits: vec![Qubit(QubitState::Unobserved); qubit_count],
            amplitudes: vec![Complex::new(0.0, 0.0)],
        };

        if qubit_count > 0 {
            s.amplitudes[0].re = 1.0;
        }

        s
    }
}