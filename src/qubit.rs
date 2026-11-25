use std::{fmt::Write, ops::Index};

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
    /// Creates a new quantum circuit with ```qubit_count``` qubits.
    pub fn new(qubit_count: usize) -> Self {
        Self {
            qubits: vec![Qubit(QubitState::default()); qubit_count],
            amplitudes: QubitVector::new(2_usize.pow(qubit_count as u32))
        }
    }

    /// Returns the number of qubits the circuit is operating on.
    pub fn qubit_count(&self) -> usize {
        self.qubits.len()
    }

    /// Applies a global phase change by a factor of ```theta``` on the qubit
    /// at index ```qubit_index```. This is mostly included for completeness,
    /// since global phase has no real effect on the outcome of measuring the
    /// qubits.
    #[allow(non_snake_case)]
    pub fn Gphase(&mut self, qubit_index: usize, theta: QFloat) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let ph = QubitMatrix::global_phase_matrix(self.qubit_count(), qubit_index, theta);

        let _ = self.amplitudes.apply_matrix(ph);

        Ok(())
    }

    /// Applies a Pauli X gate (somewhat analogous to a classical NOT gate) to
    /// the qubit at index ```qubit_index```.
    #[allow(non_snake_case)]
    pub fn X(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let x = QubitMatrix::x_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(x);

        Ok(())
    }

    /// Applies a Pauli Y gate (which rotates a qubit's state around the y-axis by
    /// pi radians) to the qubit at index ```qubit_index```.
    #[allow(non_snake_case)]
    pub fn Y(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let y = QubitMatrix::y_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(y);

        Ok(())
    }

    /// Applies a Pauli Z gate (which is essentially a phase flip) to the qubit
    /// at index ```qubit_index```.
    #[allow(non_snake_case)]
    pub fn Z(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() })
        }

        let z = QubitMatrix::z_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(z);

        Ok(())
    }

    /// Applies a Hadamard gate, which changes |0> to 1 / sqrt(2) and |1> to 
    /// -1 / sqrt(2), to the qubit at index ```qubit_index```. 
    #[allow(non_snake_case)]
    pub fn H(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let h = QubitMatrix::h_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(h);

        Ok(())
    }

    /// Applies the S gate, which adds a pi/2 phase shift to |1>, to the qubit
    /// at index ```qubit_index```. This gate is equivalent to the Pauli Z
    /// gate when it is applied twice.
    #[allow(non_snake_case)]
    pub fn S(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() })
        }

        let s = QubitMatrix::s_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(s);

        Ok(())
    }

    /// An alias for ```S```.
    #[allow(non_snake_case)]
    pub fn SqrtZ(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        self.S(qubit_index)
    }

    /// Applies the Sqrt(X) gate to the qubit at index ```qubit_index```. This gate
    /// is equivalent to the Pauli X gate when it is applied twice.
    #[allow(non_snake_case)]
    pub fn SqrtX(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let sqrtx = QubitMatrix::sqrtx_matrix(self.qubit_count(), qubit_index);

        let _ = self.amplitudes.apply_matrix(sqrtx);

        Ok(())
    }

    /// An alias for ```SqrtX```.
    #[allow(non_snake_case)]
    pub fn SX(&mut self, qubit_index: usize) -> Result<(), QubeError> {
        self.SqrtX(qubit_index)
    }

    /// Rotates the state of a qubit, as visualized by a Bloch sphere, at index
    /// ```qubit_index``` by ```theta``` radians about the x-axis.
    #[allow(non_snake_case)]
    pub fn Rx(&mut self, qubit_index: usize, theta: QFloat) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let rx = QubitMatrix::rx_matrix(self.qubit_count(), qubit_index, theta);

        let _ = self.amplitudes.apply_matrix(rx);
        
        Ok(())
    }

    /// Rotates the state of a qubit, as visualized by a Bloch sphere, at index
    /// ```qubit_index``` by ```theta``` radians about the y-axis.
    #[allow(non_snake_case)]
    pub fn Ry(&mut self, qubit_index: usize, theta: QFloat) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let ry = QubitMatrix::ry_matrix(self.qubit_count(), qubit_index, theta);

        let _ = self.amplitudes.apply_matrix(ry);
        
        Ok(())
    }

    /// Rotates the state of a qubit, as visualized by a Bloch sphere, at index
    /// ```qubit_index``` by ```theta``` radians about the z-axis.
    #[allow(non_snake_case)]
    pub fn Rz(&mut self, qubit_index: usize, theta: QFloat) -> Result<(), QubeError> {
        if qubit_index > self.qubit_count() {
            return Err(QubeError::QubitOutOfRange { index: qubit_index, max: self.qubit_count() });
        }

        let rz = QubitMatrix::rz_matrix(self.qubit_count(), qubit_index, theta);

        let _ = self.amplitudes.apply_matrix(rz);
        
        Ok(())
    }

    /// A controlled-NOT gate, also called CNOT, applies an X gate to the qubit
    /// at index ```target``` based on the state of the qubit at index ```control```.
    /// If the control qubit equals 1 in any permutation, the corresponding qubit
    /// is "flipped" (i.e. has its amplitude swapped with the permutation that has
    /// the target qubit set to the opposite value).
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

    /// An anticontrolled-NOT gate, or ACNOT, works similarly to the CNOT
    /// gate above, except it flips the state of the ```target``` qubit if
    /// the ```control``` qubit equals 0 instead of 1.
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

    /// Applies a Z gate to the qubit at index ```target``` if the qubit at index
    /// ```control``` is set to 1.
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

    /// A controlled-controlled-NOT gate, also called a CCNOT or Toffoli gate,
    /// is similar to CNOT, except it flips the qubit at index ```target``` based
    /// on two qubits, ```control1``` and ```control2```, instead.
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

    /// A multi-controlled-NOT gate works the same as the CNOT and Toffoli gates,
    /// except there is no limit on the number of qubits that can be used as the control.
    #[allow(non_snake_case)]
    pub fn MCNOT(&mut self, controls: &Vec<usize>, target: usize) -> Result<(), QubeError> {
        if controls.contains(&target) {
            return Err(QubeError::ControlsContainTarget { target });
        }

        let qubit_count = self.qubit_count();
        for i in 0..controls.len() {
            if i > qubit_count {
                return Err(QubeError::QubitOutOfRange { index: i, max: qubit_count });
            }
        }
        if target > qubit_count {
            return Err(QubeError::QubitOutOfRange { index: target, max: qubit_count });
        }

        let mask = controls.iter()
            .fold(0, |acc, &x| {
                acc | (1 << (x.saturating_sub(1)))
            }
        );

        let tmp_amps = self.amplitudes.vector.clone();

        for i in 0..tmp_amps.len() {
            if i & (mask) == mask {
                let other_index = i ^ (1 << (target.saturating_sub(1)));
                self.amplitudes.vector[i] = tmp_amps[other_index];
            }
        }

        Ok(())
    }

    /// Swaps the state of the qubit at index ```qubit1``` with the state of the
    /// qubit at ```qubit2```.
    #[allow(non_snake_case)]
    pub fn SWAP(&mut self, qubit1: usize, qubit2: usize) -> Result<(), QubeError> {
        if qubit1 == qubit2 {
            return Err(QubeError::SwapSameQubit);
        }

        let qubit_count = self.qubit_count();
        if qubit1 > qubit_count {
            return Err(QubeError::QubitOutOfRange { index: qubit1, max: qubit_count });
        }
        if qubit2 > qubit_count {
            return Err(QubeError::QubitOutOfRange { index: qubit2, max: qubit_count });
        }

        let actual_qubit1 = qubit1.saturating_sub(1);
        let actual_qubit2 = qubit2.saturating_sub(1);

        let tmp_amps = self.amplitudes.vector.clone();

        for i in 0..tmp_amps.len() {
            if (i & (1 << actual_qubit1) > 0) ^ (i & (1 << actual_qubit2) > 0) {
                let other_index = (i ^ (1 << actual_qubit1)) ^ (1 << actual_qubit2);
                self.amplitudes.vector[i] = tmp_amps[other_index];
            }
        }

        Ok(())
    }

    /// Allows a custom gate, represented as a 2x2 matrix, to be applied to a qubit
    /// in the circuit. This comes with a set of limitations:
    /// - The size of the matrix must be 2x2.
    /// - The matrix must be unitary.
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

            start_matrix = QubitMatrix::kronecker_product(start_matrix, m);
        }

        let _ = self.amplitudes.apply_matrix(start_matrix);

        Ok(())
    }

    /// Displays the state of all qubits in the quantum circuit without measuring them.
    /// This should only be used for debugging, because a real quantum circuit's state
    /// will collapse to a basis state when it is observed.
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
    /// Observes the state of the quantum circuit. This will cause its state to collapse
    /// to a computational basis state, and returns a 2-tuple consisting of:
    /// - The number of qubits
    /// - The basis state measured, returned as 16-bit number
    #[allow(non_snake_case)]
    pub fn M(&mut self) -> (usize, u16) {
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

        let mut measured: u16 = 0;
        let count = self.qubit_count();

        for i in 0..count {
            measured += (option & (1 << i)) as u16;
            self.qubits[i].0 = if measured & (1 << i) > 0 { QubitState::Ket1 } else { QubitState::Ket0 };
        }

        measured = measured.reverse_bits() >> (u16::BITS - count as u32);

        (self.qubit_count(), measured)        
    }

    /// Observes the state of a single qubit. This will cause the state
    /// of the qubit to collapse to a computational basis state and leave
    /// the other qubits alone. This will also effectively remove the
    /// qubit from the circuit.
    #[allow(non_snake_case)]
    pub fn Msingle(&mut self, qubit_index: usize) -> bool {
        let actual_index = qubit_index.saturating_sub(1);

        // The probability that the qubit will measure as 1.
        let prob1 = self.amplitudes.vector
            .iter()
            .enumerate()
            .fold(0.0, |acc, (i, &x)| {
                if i & (1 << actual_index) > 0 { 
                    acc + (x * x).modulus() 
                } else {
                    acc
                } 
            }
        );

        let r = rand::random_range(0..1_000_000) as usize;
        let threshold = (prob1 * 1_000_000.0).round() as usize;

        // Measure the value of this one qubit
        let qubit_state: usize = if r >= threshold { 0 } else { 1 };

        self.qubits.remove(actual_index);

        // Remove any amplitudes from the main amplitude vector if the corresponding state's
        // qubit does not equal the measured state
        let mut new_vector = QubitVector::new(2_usize.pow(self.qubit_count() as u32));
        let mut vector_index = 0;
        for (i, c) in self.amplitudes.vector.iter().enumerate() {
            if i & (1 << actual_index) == qubit_state {
                new_vector.vector[vector_index] = *c;
                vector_index += 1;
            }
        }

        new_vector.normalize();

        self.amplitudes = new_vector;

        qubit_state == 1
    }
}