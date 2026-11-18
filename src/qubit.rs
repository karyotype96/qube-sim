use std::fmt::Write;

use crate::{complex::{Complex, QFloat}, error_handling::QubeError, qubit_matrix::QubitMatrix, qubit_vector::QubitVector};

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

    // global phase matrix, included for completeness in case it's
    // REALLY needed for some reason
    #[allow(non_snake_case)]
    pub fn Gphase(&mut self, qubit_index: usize, theta: QFloat) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let ph = QubitMatrix::global_phase_matrix(self.qubit_count(), qubit_index, theta);

        let _ = self.amplitudes.apply_matrix(ph);

        Ok(())
    }

    #[allow(non_snake_case)]
    pub fn X(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let x = QubitMatrix::x_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(x);

        Ok(())
    }

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
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let h = QubitMatrix::h_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(h);

        Ok(())
    }

    // S gate and its aliases
    #[allow(non_snake_case)]
    pub fn S(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() })
        }

        let s = QubitMatrix::s_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(s);

        Ok(())
    }
    #[allow(non_snake_case)]
    pub fn SqrtZ(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        self.S(qubit_index)
    }

    // Sqrt(X) gate and its aliases
    #[allow(non_snake_case)]
    pub fn SqrtX(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let sqrtx = QubitMatrix::sqrtx_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(sqrtx);

        Ok(())
    }
    #[allow(non_snake_case)]
    pub fn SX(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        self.SqrtX(qubit_index)
    }

    // Rotational matrices
    #[allow(non_snake_case)]
    pub fn Rx(&mut self, qubit_index: usize, theta: QFloat) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let rx = QubitMatrix::rx_matrix(self.qubit_count(), qubit_index, theta);

        let _ = self.amplitudes.apply_matrix(rx);
        
        Ok(())
    }

    #[allow(non_snake_case)]
    pub fn Ry(&mut self, qubit_index: usize, theta: QFloat) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let ry = QubitMatrix::ry_matrix(self.qubit_count(), qubit_index, theta);

        let _ = self.amplitudes.apply_matrix(ry);
        
        Ok(())
    }

    #[allow(non_snake_case)]
    pub fn Rz(&mut self, qubit_index: usize, theta: QFloat) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let rz = QubitMatrix::rz_matrix(self.qubit_count(), qubit_index, theta);

        let _ = self.amplitudes.apply_matrix(rz);
        
        Ok(())
    }

    // controlled NOT
    #[allow(non_snake_case)]
    pub fn CNOT(&mut self, control: usize, target: usize) -> Result<(), QubeError> {
        let qubit_count = self.qubit_count();
        if control == target {
            return Err(QubeError::ControlEqualsTarget { index: control });
        }
        if control > qubit_count {
            return Err(QubeError::QubitOutOfRange { index: control, max: qubit_count });
        }
        if target > qubit_count {
            return Err(QubeError::QubitOutOfRange { index: target, max: qubit_count })
        }

        // let actual_ctl = qubit_count.saturating_sub(control);
        // let actual_tgt = qubit_count.saturating_sub(target);
        let actual_ctl = control.saturating_sub(1);
        let actual_tgt = target.saturating_sub(1);

        let tmp_amps = self.amplitudes.vector.clone();

        for i in 0..tmp_amps.len() {
            if i & (1 << actual_ctl) > 0 {
                let other_index = i ^ (1 << actual_tgt);
                self.amplitudes.vector[i] = tmp_amps[other_index];
            }
        }

        Ok(())
    }

    // anti-controlled NOT
    #[allow(non_snake_case)]
    pub fn ACNOT(&mut self, control: usize, target: usize) -> Result<(), QubeError> {
        let qubit_count = self.qubit_count();
        if control == target {
            return Err(QubeError::ControlEqualsTarget { index: control });
        }
        if control > qubit_count {
            return Err(QubeError::QubitOutOfRange { index: control, max: qubit_count });
        }
        if target > qubit_count {
            return Err(QubeError::QubitOutOfRange { index: target, max: qubit_count });
        }

        let actual_ctl = control.saturating_sub(1);
        let actual_tgt = target.saturating_sub(1);

        let tmp_amps = self.amplitudes.vector.clone();

        for i in 0..tmp_amps.len() {
            if i & (1 << actual_ctl) == 0 {
                let other_index = i ^ (1 << actual_tgt);
                self.amplitudes.vector[i] = tmp_amps[other_index];
            }
        }

        Ok(())
    }

    // controlled Z gate
    #[allow(non_snake_case)]
    pub fn CZ(&mut self, control: usize, target: usize) -> Result<(), QubeError> {
        let qubit_count = self.qubit_count();
        if control == target {
            return Err(QubeError::ControlEqualsTarget { index: control });
        }
        if control > qubit_count {
            return Err(QubeError::QubitOutOfRange { index: control, max: qubit_count });
        }
        if target > qubit_count {
            return Err(QubeError::QubitOutOfRange { index: target, max: qubit_count })
        }

        let actual_ctl = control.saturating_sub(1);
        let actual_tgt = control.saturating_sub(1);

        for i in 0..self.amplitudes.len() {
            if i & (1 << actual_ctl) > 0 && i & (1 << actual_tgt) > 0 {
                self.amplitudes.vector[i] = -self.amplitudes.vector[i];
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

        let actual_ctl1 = control1.saturating_sub(1);
        let actual_ctl2 = control2.saturating_sub(1);
        let actual_tgt = target.saturating_sub(1);

        let tmp_amps = self.amplitudes.vector.clone();

        for i in 0..tmp_amps.len() {
            if i & (1 << actual_ctl1) > 0 && i & (1 << actual_ctl2) > 0 {
                let other_index = i ^ (1 << actual_tgt);
                self.amplitudes.vector[i] = tmp_amps[other_index];
            }
        }

        Ok(())
    }

    /* Custom one-qubit gate */
    #[allow(non_snake_case)]
    pub fn Custom1(&mut self, qubit_index: usize, matrix: QubitMatrix) -> Result<(), QubeError> {
        let size = matrix.get_size();
        
        if size != 2 {
            return Err(QubeError::OneQubitCustomGateNot2x2);
        }

        if !matrix.is_unitary() {
            return Err(QubeError::OneQubitCustomGateNotUnitary);
        }

        let mut start_matrix = if qubit_index == 0 { matrix.copy() } else { QubitMatrix::identity_matrix(size) };

        for i in 0..self.qubit_count() {
            let m = if qubit_index == i { matrix.copy() } else { QubitMatrix::identity_matrix(size) };

            start_matrix = QubitMatrix::tensor_product(start_matrix, m);
        }

        let _ = self.amplitudes.apply_matrix(start_matrix);

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
            /* 
                the bits needs to be reversed for each printed value on the screen because
                the usual intuition in a quantum circuit is that in a ket, |abc>, a is qubit
                0, b is qubit 1, etc., whereas the intuition in a classical computer is the
                opposite
            */
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