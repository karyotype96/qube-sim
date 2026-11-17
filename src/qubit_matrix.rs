use std::{f64::consts::SQRT_2, fmt::{Debug, Write}, ops::Mul};

use crate::{complex::{Complex, QFloat}, qubit};

pub struct QubitMatrix {
    pub matrix: Vec<Vec<Complex>>
}

impl Mul for QubitMatrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let size = self.get_size();
        
        if size != rhs.get_size() {
            panic!("error: attempted to multiply two matrices of different sizes");
        }

        let mut result = QubitMatrix::new(self.get_size());

        for row in 0..self.get_size() {
            for col in 0..self.get_size() {
                for i in 0..size {
                    result.matrix[row][col] += self.matrix[row][i] * rhs.matrix[i][col];
                }
            }
        }

        result
    }
}

impl Debug for QubitMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for row in 0..self.get_size() {
            res.push_str("[");
            for col in 0..self.get_size() {
                write!(&mut res, "{}", self.matrix[row][col]).unwrap();
                if col < self.get_size()-1 {
                    res.push_str(", ");
                }
            }
            res.push_str("]\n");
        }

        write!(f, "{}", res)
    }
}

impl PartialEq for QubitMatrix {
    fn eq(&self, other: &Self) -> bool {
        if self.get_size() != other.get_size() {
            return false;
        }

        for row in 0..self.matrix.len() {
            for col in 0..self.matrix[row].len() {
                if self.matrix[row][col] != other.matrix[row][col] { return false; }
            }
        }

        true
    }

    fn ne(&self, other: &Self) -> bool {
        if self.get_size() != other.get_size() {
            return true;
        }

        for row in 0..self.matrix.len() {
            for col in 0..self.matrix[row].len() {
                if self.matrix[row][col] != other.matrix[row][col] { return true; }
            }
        }

        false
    }
}

impl QubitMatrix {
    // note: all qubit matrices are NxN in size
    pub fn new(size: usize) -> Self {
        let mut matrix: Vec<Vec<Complex>> = Vec::new();

        for i in 0..size {
            matrix.push(Vec::new());

            for _ in 0..size {
                matrix[i].push(Complex::new(0.0, 0.0))
            }
        }

        Self {
            matrix
        }
    }

    pub fn get_size(&self) -> usize {
        self.matrix.len()
    }

    pub fn copy(&self) -> Self {
        let mut m = Self::new(self.get_size());

        for i in 0..self.get_size() {
            for j in 0..self.get_size() {
                m.matrix[i][j] = self.matrix[i][j]
            }
        }

        m
    }

    #[allow(unused)]
    pub fn transpose(&self) -> Self {
        let mut transposed = Self::new(self.get_size());

        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                transposed.matrix[j][i] = self.matrix[i][j]
            }
        }

        transposed
    }

    pub fn conjugate(&self) -> Self {
        let mut conjugated = Self::new(self.get_size());

        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                conjugated.matrix[i][j] = self.matrix[i][j].conjugate();
            }
        }

        conjugated
    }

    pub fn adjoint(&self) -> Self {
        self.transpose().conjugate()
    }

    pub fn is_unitary(&self) -> bool {
        self.copy() * self.adjoint() == Self::identity_matrix(self.get_size())
    }

    // static methods
    pub fn tensor_product(matrix1: Self, matrix2: Self) -> Self {
        let size = matrix1.get_size() * matrix2.get_size();

        let (mat1_size, mat2_size) = (matrix1.get_size(), matrix2.get_size());

        let mut new_matrix = QubitMatrix::new(size);

        for row in 0..new_matrix.get_size() {
            let mat1_row = row / mat2_size;
            let mat2_row = row % mat2_size;
            for col in 0..new_matrix.get_size() {
                let mat1_col = col / mat1_size;
                let mat2_col = col % mat2_size;

                new_matrix.matrix[row][col] = matrix1.matrix[mat1_row][mat1_col] * matrix2.matrix[mat2_row][mat2_col];
            }
        }

        new_matrix
    }

    pub fn identity_matrix(size: usize) -> Self {
        let mut new_matrix = QubitMatrix::new(size);

        for row in 0..new_matrix.get_size() {
            for col in 0..new_matrix.get_size() {
                new_matrix.matrix[row][col] = if row == col { Complex { re: 1.0, im: 0.0 } } else { Complex { re: 0.0, im: 0.0 } };
            }
        }

        new_matrix
    }

    pub fn base_global_phase_matrix(theta: QFloat) -> Self {
        let mut new_matrix = QubitMatrix::identity_matrix(2);

        let phase_factor = Complex::new(0.0, theta).exp();

        for i in 0..new_matrix.get_size() {
            for j in 0..new_matrix.get_size() {
                new_matrix.matrix[i][j] *= phase_factor;
            }
        }

        new_matrix
    }

    fn base_x_matrix() -> Self {
        let mut x = QubitMatrix::new(2);

        x.matrix[0][0] = Complex::new(0.0, 0.0);
        x.matrix[0][1] = Complex::new(1.0, 0.0);
        x.matrix[1][0] = Complex::new(1.0, 0.0);
        x.matrix[1][1] = Complex::new(0.0, 0.0);

        x
    }

    fn base_y_matrix() -> Self {
        let mut y = QubitMatrix::new(2);

        y.matrix[0][0] = Complex::new(0.0, 0.0);
        y.matrix[0][1] = Complex::new(0.0, -1.0);
        y.matrix[1][0] = Complex::new(0.0, 1.0);
        y.matrix[1][1] = Complex::new(0.0, 0.0);

        y
    }

    fn base_z_matrix() -> Self {
        let mut z = QubitMatrix::new(2);

        z.matrix[0][0] = Complex::new(1.0, 0.0);
        z.matrix[0][1] = Complex::new(0.0, 0.0);
        z.matrix[1][0] = Complex::new(0.0, 0.0);
        z.matrix[1][1] = Complex::new(-1.0, 0.0);

        z
    }

    fn base_h_matrix() -> Self {
        let mut h = QubitMatrix::new(2);

        h.matrix[0][0] = Complex::new(1.0 / SQRT_2, 0.0);
        h.matrix[0][1] = Complex::new(1.0 / SQRT_2, 0.0);
        h.matrix[1][0] = Complex::new(1.0 / SQRT_2, 0.0);
        h.matrix[1][1] = Complex::new(-1.0 / SQRT_2, 0.0);

        h
    }

    fn base_rx_matrix(theta: QFloat) -> Self {
        let mut rx = QubitMatrix::new(2);

        rx.matrix[0][0] = Complex::new((theta / 2.0).cos(), 0.0);
        rx.matrix[0][1] = Complex::new(0.0, -(theta / 2.0).sin());
        rx.matrix[1][0] = Complex::new(0.0, -(theta / 2.0).sin());
        rx.matrix[1][1] = Complex::new((theta / 2.0).cos(), 0.0);

        rx
    }

    fn base_ry_matrix(theta: QFloat) -> Self {
        let mut ry = QubitMatrix::new(2);

        ry.matrix[0][0] = Complex::new((theta / 2.0).cos(), 0.0);
        ry.matrix[0][1] = Complex::new(-(theta / 2.0).sin(), 0.0);
        ry.matrix[1][0] = Complex::new((theta / 2.0).sin(), 0.0);
        ry.matrix[1][1] = Complex::new((theta / 2.0).cos(), 0.0);

        ry
    }

    fn base_rz_matrix(theta: QFloat) -> Self {
        let mut rz = QubitMatrix::new(2);

        rz.matrix[0][0] = Complex::new(0.0, -(theta / 2.0)).exp();
        rz.matrix[0][1] = Complex::new(0.0, 0.0);
        rz.matrix[1][0] = Complex::new(0.0, 0.0);
        rz.matrix[1][1] = Complex::new(0.0, theta / 2.0).exp();

        rz
    }

    pub fn global_phase_matrix(qubit_count: usize, qubit_index: usize, theta: QFloat) -> Self {
        let cur_matrix = if qubit_count == qubit_index { Self::base_global_phase_matrix(theta) } else { Self::identity_matrix(2) };

        if qubit_count <= 1 {
            return cur_matrix;
        }

        let prev_matrix = Self::global_phase_matrix(qubit_count - 1, qubit_index, theta);
        Self::tensor_product(prev_matrix, cur_matrix)
    }

    pub fn x_matrix(qubit_count: usize, qubit_index: usize) -> Self {
        let cur_matrix = if qubit_count == qubit_index { Self::base_x_matrix() } else { Self::identity_matrix(2) };

        if qubit_count <= 1 {
            return cur_matrix;
        }

        let prev_matrix = Self::x_matrix(qubit_count - 1, qubit_index);
        Self::tensor_product(prev_matrix, cur_matrix)
    }

    pub fn y_matrix(qubit_count: usize, qubit_index: usize) -> Self {
        let cur_matrix = if qubit_count == qubit_index { Self::base_y_matrix() } else { Self::identity_matrix(2) };

        if qubit_count <= 1 {
            return cur_matrix;
        }

        let prev_matrix = Self::y_matrix(qubit_count - 1, qubit_index);
        Self::tensor_product(prev_matrix, cur_matrix)
    }

    pub fn z_matrix(qubit_count: usize, qubit_index: usize) -> Self {
        let cur_matrix = if qubit_count == qubit_index { Self::base_z_matrix() } else { Self::identity_matrix(2) };

        if qubit_count <= 1 {
            return cur_matrix;
        }

        let prev_matrix = Self::z_matrix(qubit_count - 1, qubit_index);
        Self::tensor_product(prev_matrix, cur_matrix)
    }

    pub fn h_matrix(qubit_count: usize, qubit_index: usize) -> Self {
        let cur_matrix = if qubit_count == qubit_index { Self::base_h_matrix() } else { Self::identity_matrix(2) };

        if qubit_count <= 1 {
            return cur_matrix;
        }

        let prev_matrix = Self::h_matrix(qubit_count - 1, qubit_index);
        Self::tensor_product(prev_matrix, cur_matrix)
    }

    pub fn rx_matrix(qubit_count: usize, qubit_index: usize, theta: QFloat) -> Self {
        let cur_matrix = if qubit_count == qubit_index { Self::base_rx_matrix(theta) } else { Self::identity_matrix(2) };

        if qubit_count <= 1 {
            return cur_matrix;
        }

        let prev_matrix = Self::rx_matrix(qubit_count - 1, qubit_index, theta);
        Self::tensor_product(prev_matrix, cur_matrix)
    }

    pub fn ry_matrix(qubit_count: usize, qubit_index: usize, theta: QFloat) -> Self {
        let cur_matrix = if qubit_count == qubit_index { Self::base_ry_matrix(theta) } else { Self::identity_matrix(2) };

        if qubit_count <= 1 {
            return cur_matrix;
        }

        let prev_matrix = Self::ry_matrix(qubit_count - 1, qubit_index, theta);
        Self::tensor_product(prev_matrix, cur_matrix)
    }

    pub fn rz_matrix(qubit_count: usize, qubit_index: usize, theta: QFloat) -> Self {
        let cur_matrix = if qubit_count == qubit_index { Self::base_rz_matrix(theta) } else { Self::identity_matrix(2) };

        if qubit_count <= 1 {
            return cur_matrix;
        }

        let prev_matrix = Self::rz_matrix(qubit_count - 1, qubit_index, theta);
        Self::tensor_product(prev_matrix, cur_matrix)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiplication() {
        let mut m1 = QubitMatrix::new(2);
        m1.matrix[0][0] = Complex::new(1.0, 0.0);
        m1.matrix[0][1] = Complex::new(2.0, 0.0);
        m1.matrix[1][0] = Complex::new(3.0, 0.0);
        m1.matrix[1][1] = Complex::new(4.0, 0.0);

        let mut m2 = QubitMatrix::new(2);
        m2.matrix[0][0] = Complex::new(5.0, 0.0);
        m2.matrix[0][1] = Complex::new(6.0, 0.0);
        m2.matrix[1][0] = Complex::new(7.0, 0.0);
        m2.matrix[1][1] = Complex::new(8.0, 0.0);

        let m3 = m1 * m2;

        let mut expected = QubitMatrix::new(2);
        expected.matrix[0][0] = Complex::new(19.0, 0.0);
        expected.matrix[0][1] = Complex::new(22.0, 0.0);
        expected.matrix[1][0] = Complex::new(43.0, 0.0);
        expected.matrix[1][1] = Complex::new(50.0, 0.0);

        assert_eq!(m3, expected)
    }

    #[test]
    fn test_tensor_product() {
        let mut m1 = QubitMatrix::new(2);

        m1.matrix[0][0] = Complex::new(1.0, 0.0);
        m1.matrix[0][1] = Complex::new(2.0, 0.0);
        m1.matrix[1][0] = Complex::new(3.0, 0.0);
        m1.matrix[1][1] = Complex::new(4.0, 0.0);

        let mut m2 = QubitMatrix::new(2);

        m2.matrix[0][0] = Complex::new(5.0, 0.0);
        m2.matrix[0][1] = Complex::new(6.0, 0.0);
        m2.matrix[1][0] = Complex::new(7.0, 0.0);
        m2.matrix[1][1] = Complex::new(8.0, 0.0);
    
        let mut res = QubitMatrix::new(4);

        res.matrix[0][0] = Complex::new(5.0, 0.0);
        res.matrix[0][1] = Complex::new(6.0, 0.0);
        res.matrix[0][2] = Complex::new(10.0, 0.0);
        res.matrix[0][3] = Complex::new(12.0, 0.0);
        res.matrix[1][0] = Complex::new(7.0, 0.0);
        res.matrix[1][1] = Complex::new(8.0, 0.0);
        res.matrix[1][2] = Complex::new(14.0, 0.0);
        res.matrix[1][3] = Complex::new(16.0, 0.0);
        res.matrix[2][0] = Complex::new(15.0, 0.0);
        res.matrix[2][1] = Complex::new(18.0, 0.0);
        res.matrix[2][2] = Complex::new(20.0, 0.0);
        res.matrix[2][3] = Complex::new(24.0, 0.0);
        res.matrix[3][0] = Complex::new(21.0, 0.0);
        res.matrix[3][1] = Complex::new(24.0, 0.0);
        res.matrix[3][2] = Complex::new(28.0, 0.0);
        res.matrix[3][3] = Complex::new(32.0, 0.0);

        assert_eq!(QubitMatrix::tensor_product(m1, m2), res);
    }
}