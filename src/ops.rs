use crate::{complex::Complex, qmatrix::QMatrix, qubit::Qubit};

pub const C_ZERO: Complex = Complex { re: 0.0, im: 0.0 };
pub const C_ONE: Complex = Complex { re: 1.0, im: 0.0 };
pub const C_I: Complex = Complex { re: 0.0, im: 1.0 };
pub const IDENTITY: QMatrix = QMatrix(
    C_ONE,  C_ZERO,
    C_ZERO, C_ONE
);

// public functions, called the same way as quantum gates
pub fn X(q: &mut Qubit) {
    let x_mat = QMatrix::new(C_ZERO, C_ONE, C_ONE, C_ZERO);

    let _ = q.apply_matrix(x_mat);
}

pub fn Y(q: &mut Qubit) {
    let y_mat = QMatrix::new(C_ZERO, -C_I, C_I, C_ZERO);

    let _ = q.apply_matrix(y_mat);
}

pub fn Z(q: &mut Qubit) {
    let z_mat = QMatrix::new(C_ONE, C_ZERO, C_ZERO, -C_ONE);

    let _ = q.apply_matrix(z_mat);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    fn test_apply_x() {
        let mut q = Qubit::new();

        X(&mut q);

        assert_eq!(q.observe_state_debug(), (Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)));
    }
}