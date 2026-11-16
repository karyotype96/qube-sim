use std::fmt::Write;

use crate::{complex::Complex, error_handling::QubeError, qubit_matrix::QubitMatrix, qubit_vector::{QubitVector}};

#[derive(Copy, Clone, Default, Debug)]
pub enum QubitState {
    #[default]
    Unobserved,
    Ket0,
    Ket1
}

#[derive(Copy, Clone, Debug)]
pub struct Qubit(QubitState);

pub struct QuantumCircuit {
    qubits: Vec<Qubit>,
    amplitudes: QubitVector
}

impl QuantumCircuit {
    pub fn new(qubit_count: usize) -> Self {
        Self {
            qubits: vec![Qubit(QubitState::default()); qubit_count],
            amplitudes: QubitVector::new(2_usize.pow(qubit_count as u32))
        }
    }

    pub fn qubit_count(&self) -> usize {
        self.qubits.len()
    }

    /*
        The Pauli X gate is roughly equivalent to the NOT gate in a classical computer.
        Let's say we have a qubit in the state of a|0> + b|1>: the X gate would change
        that to b|0> + a|1>. 
    */
    #[allow(non_snake_case)]
    pub fn X(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let x = QubitMatrix::x_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(x);

        Ok(())
    }

    /*
        For the Pauli Y gate, imagine we have a qubit represented by a Bloch sphere. A
        Y gate would rotate the state of the qubit around the Y axis by pi radians, 
        effectively combining a bit flip with a phase change.
    */
    #[allow(non_snake_case)]
    pub fn Y(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let y = QubitMatrix::y_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(y);

        Ok(())
    }

    #[allow(non_snake_case)]
    pub fn Z(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() })
        }

        let z = QubitMatrix::z_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(z);

        Ok(())
    }

    #[allow(non_snake_case)]
    pub fn H(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() })
        }

        let h = QubitMatrix::h_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(h);

        Ok(())
    }

    #[allow(non_snake_case)]
    pub fn CNOT(&mut self, control: usize, target: usize) -> Result<(), QubeError> {
        let qubit_count = self.qubit_count();
        if control == target {
            return Err(QubeError::ControlEqualsTarget { index: control });
        }
        if control > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: control, max: self.qubit_count() });
        }
        if target > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: target, max: self.qubit_count() })
        }

        let actual_ctl = qubit_count.saturating_sub(control);
        let actual_tgt = qubit_count.saturating_sub(target);

        let tmp_amps = self.amplitudes.vector.clone();

        for i in 0..tmp_amps.len() {
            if i & (1 << actual_ctl) > 0 {
                let other_index = i ^ (1 << actual_tgt);
                self.amplitudes.vector[i] = tmp_amps[other_index];
            }
        }

        Ok(())
    }

    #[allow(non_snake_case)]
    pub fn CCNOT(&mut self, control1: usize, control2: usize, target: usize) -> Result<(), QubeError> {
        if control1 == control2 {
            return Err(QubeError::Control1EqualsControl2 { index: control1 });
        }
        if control1 == target {
            return Err(QubeError::ControlEqualsTarget { index: control1 });
        }
        if control2 == target {
            return Err(QubeError::ControlEqualsTarget { index: control2 });
        }

        let qubit_count = self.qubit_count();
        if control1 > qubit_count {
            return Err(QubeError::QubitOutOfRange { index: control1, max: qubit_count });
        }
        if control2 > qubit_count {
            return Err(QubeError::QubitOutOfRange { index: control2, max: qubit_count });
        }
        if target > qubit_count {
            return Err(QubeError::QubitOutOfRange { index: target, max: qubit_count });
        }

        let actual_ctl1 = qubit_count.saturating_sub(control1);
        let actual_ctl2 = qubit_count.saturating_sub(control2);
        let actual_tgt = qubit_count.saturating_sub(target);

        let tmp_amps = self.amplitudes.vector.clone();

        for i in 0..tmp_amps.len() {
            if i & (1 << actual_ctl1) > 0 && i & (1 << actual_ctl2) > 0 {
                let other_index = i ^ (1 << actual_tgt);
                self.amplitudes.vector[i] = tmp_amps[other_index];
            }
        }

        Ok(())
    }

    /*
        WARNING: This is not the same as observing the states in the computational
        basis, which would change the state to either |0> or |1>. For this reason,
        a function like this will have zero actual implementation in a real quantum
        computer.
    */
    pub fn observe_debug(&self) -> String {
        let mut res = String::new();

        let qubit_count = self.qubit_count();

        let mut printed_values: Vec<Complex> = vec![Complex::new(0.0, 0.0); self.amplitudes.len()];

        for i in 0..self.amplitudes.len() {
            let bit_rev_i = i.reverse_bits() >> (usize::BITS - qubit_count as u32);
            printed_values[bit_rev_i] = self.amplitudes.vector[i];
        }

        for i in 0..printed_values.len() {
            write!(&mut res, "({})|{:0qubit_count$b}>", self.amplitudes.vector[i], i).unwrap();
            if i < self.amplitudes.len()-1 {
                res.push_str(" + ");
            }
        }

        res
    }

    /*
        This is for observing the states in the computational basis, and should be done
        after all gates have been applied to the qubits in the circuit.
    */
    #[allow(non_snake_case)]
    pub fn M(&mut self) -> Vec<u8> {
        let mut weights: Vec<u64> = self.amplitudes.vector
            .iter()
            .map(|x| ((*x * *x).modulus() * 1_000_000.0) as u64)
            .collect();

        for i in 1..weights.len() {
            weights[i] += weights[i-1];
        }

        let prob_value = rand::random_range(0..1_000_000);

        let mut option: usize = weights.len();
        for (i, &v) in weights.iter().enumerate().rev() {
            if prob_value < v {
                option = i;
            } else {
                break;
            }
        }

        let mut measured: Vec<u8> = Vec::new();
        let count = self.qubit_count();

        for i in 0..count {
            measured.push(if option & (1 << i) > 0 { 1 } else { 0 });
            self.qubits[i].0 = if measured[i] == 1 { QubitState::Ket1 } else { QubitState::Ket0 }
        }

        measured.reverse();

        measured
    }
}