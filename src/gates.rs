use std::{f64::consts::SQRT_2};

use crate::{complex::Complex, qubit::{Qubit, QubitState}, qubit_matrix::QubitMatrix};

pub const C_ZERO: Complex = Complex { re: 0.0, im: 0.0 };
pub const C_ONE: Complex = Complex { re: 1.0, im: 0.0 };
pub const C_N_ONE: Complex = Complex { re: -1.0, im: 0.0 };
pub const C_I: Complex = Complex { re: 0.0, im: 1.0 };
pub const C_NI: Complex = Complex { re: 0.0, im: -1.0 };

// computational basis states
pub const KET_0: QubitState = QubitState(
    Complex { re: 1.0, im: 0.0 },
    Complex { re: 0.0, im: 0.0 }
);
pub const KET_1: QubitState = QubitState(
    Complex { re: 0.0, im: 0.0 },
    Complex { re: 1.0, im: 0.0 }
);

// quantum gate matrices
pub const C_IDENTITY: QubitMatrix = QubitMatrix(
    C_ONE,  C_ZERO,
    C_ZERO, C_ONE
);
pub const X_MAT: QubitMatrix = QubitMatrix(
    C_ZERO, C_ONE,
    C_ONE, C_ZERO
);
pub const Y_MAT: QubitMatrix = QubitMatrix(
    C_ZERO, C_NI,
    C_I, C_ZERO
);
pub const Z_MAT: QubitMatrix = QubitMatrix(
    C_ONE, C_ZERO,
    C_ZERO, C_N_ONE
);
pub const H_MAT: QubitMatrix = QubitMatrix(
    Complex { re: 1.0 / SQRT_2, im: 0.0 }, Complex { re:  1.0 / SQRT_2, im: 0.0 },
    Complex { re: 1.0 / SQRT_2, im: 0.0 }, Complex { re: -1.0 / SQRT_2, im: 0.0 }
);

// public functions, called the same way as quantum gates

/*
    The X gate is one of the standard Pauli gates, loosely analogous to
    a classical "NOT" gate - it can change the state of a qubit from |0>
    to |1>, and vice versa. In a more complete sense, if you have a
    superposition a|0> + b|1>, applying the X gate will change that to
    b|0> + a|1>.
*/
#[allow(non_snake_case)]
pub fn X(q: &mut Qubit) {
    let _ = q.apply_matrix(X_MAT);
}

#[allow(non_snake_case)]
pub fn Y(q: &mut Qubit) {
    let _ = q.apply_matrix(Y_MAT);
}

#[allow(non_snake_case)]
pub fn Z(q: &mut Qubit) {
    let _ = q.apply_matrix(Z_MAT);
}

#[allow(non_snake_case)]
pub fn H(q: &mut Qubit) {
    let _ = q.apply_matrix(H_MAT);
}

#[cfg(test)]
pub mod tests {

    use crate::qubit::{QubitState};

    use super::*;
    
    #[test]
    fn test_apply_x() {
        let mut q = Qubit::new();

        X(&mut q);

        assert_eq!(q.observe_state_debug(), QubitState(Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)));
    }

    #[test]
    fn test_apply_h() {
        let mut q = Qubit::new();

        let hadamard_plus = (1.0 / SQRT_2 * 1_000_000.0_f64).round() / 1_000_000.0;

        let hadamard_state = QubitState(Complex::new(hadamard_plus, 0.0), Complex::new(hadamard_plus, 0.0));

        H(&mut q);
        assert_eq!(q.observe_state_debug(), hadamard_state);
        H(&mut q);
        assert_eq!(q.observe_state_debug(), KET_0);
    }
}