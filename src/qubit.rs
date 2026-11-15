use std::fmt::Write;

use crate::{qubit_matrix::QubitMatrix, qubit_vector::QubitVector};

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

    #[allow(non_snake_case)]
    pub fn X(&mut self, qubit_index: usize) {
        let x = QubitMatrix::x_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(x);
    } 

    #[allow(non_snake_case)]
    pub fn Y(&mut self, qubit_index: usize) {
        let y = QubitMatrix::y_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(y);
    }

    #[allow(non_snake_case)]
    pub fn Z(&mut self, qubit_index: usize) {
        let z = QubitMatrix::z_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(z);
    }

    #[allow(non_snake_case)]
    pub fn H(&mut self, qubit_index: usize) {
        let h = QubitMatrix::h_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(h);
    }

    #[allow(non_snake_case)]
    pub fn CNOT(&mut self, control: usize, target: usize) {
        let actual_ctl = control - 1;
        let actual_tgt = target - 1;

        let tmp_amps = self.amplitudes.vector.clone();

        for i in 0..tmp_amps.len() {
            println!("i = {i}; {i} & (1 << {actual_ctl}) = {}", i & (1 << actual_ctl));
            if i & (1 << actual_ctl) > 0 {
                let other_index = i ^ (1 << actual_tgt);
                self.amplitudes.vector[i] = tmp_amps[other_index];
            }
        }
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

        for i in 0..self.amplitudes.len() {
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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_circuit() {
        let mut qc = QuantumCircuit::new(2);
        println!("qc (init): {}", qc.observe_debug());

        qc.H(1);
        println!("qc (H): {}", qc.observe_debug());

        qc.CNOT(2, 1);
        println!("qc (CNOT): {}", qc.observe_debug());

        let measured_result = qc.M();
        println!("result: {:?}", measured_result);
    }
}