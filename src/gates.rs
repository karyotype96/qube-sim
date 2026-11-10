use crate::{complex::{Complex}, qmatrix::QMatrix, qubit::Qubit};

pub const C_ZERO: Complex = Complex { re: 0.0, im: 0.0 };
pub const C_ONE: Complex = Complex { re: 1.0, im: 0.0 };
pub const C_I: Complex = Complex { re: 0.0, im: 1.0 };
pub const IDENTITY: QMatrix = QMatrix(
    C_ONE,  C_ZERO,
    C_ZERO, C_ONE
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
    let x_mat = QMatrix::new(C_ZERO, C_ONE, C_ONE, C_ZERO);

    let _ = q.apply_matrix(x_mat);
}

#[allow(non_snake_case)]
pub fn Y(q: &mut Qubit) {
    let y_mat = QMatrix::new(C_ZERO, -C_I, C_I, C_ZERO);

    let _ = q.apply_matrix(y_mat);
}

#[allow(non_snake_case)]
pub fn Z(q: &mut Qubit) {
    let z_mat = QMatrix::new(C_ONE, C_ZERO, C_ZERO, -C_ONE);

    let _ = q.apply_matrix(z_mat);
}

#[allow(non_snake_case)]
pub fn H(q: &mut Qubit) {
    let sqrt_2 = 2.0_f64.sqrt();

    let h_mat = QMatrix::new(
        C_ONE / sqrt_2, C_ONE / sqrt_2,
        C_ONE / sqrt_2, -C_ONE / sqrt_2
    );

    let _ = q.apply_matrix(h_mat);
}

#[cfg(test)]
pub mod tests {
    use crate::qubit::QubitState;

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

        H(&mut q);
        H(&mut q);
    }
}