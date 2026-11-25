use std::fmt::{Debug, Write};

use crate::{complex::{Complex, QFloat}, qubit_matrix::QubitMatrix};

pub struct QubitVector {
    pub vector: Vec<Complex>
}

impl Debug for QubitVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        res.push_str("[");
        for i in 0..self.vector.len() {
            write!(&mut res, "{}", self.vector[i]).unwrap();
            if i < self.vector.len()-1 {
                res.push_str(", ");
            }
        }
        res.push_str("]");

        write!(f, "{}", res)
    }
}

impl PartialEq for QubitVector {
    fn eq(&self, other: &Self) -> bool {
        if self.vector.len() != other.vector.len() {
            return false;
        }

        for i in 0..self.vector.len() {
            if self.vector[i] != other.vector[i] { return false; }
        }
        
        true
    }

    fn ne(&self, other: &Self) -> bool {
        if self.vector.len() != other.vector.len() {
            return true;
        }

        for i in 0..self.vector.len() {
            if self.vector[i] != other.vector[i] { return true; }
        }
        
        false
    }
}

impl QubitVector {
    pub fn new(size: usize) -> Self {
        let mut vector = vec![Complex::new(0.0, 0.0); size];

        vector[0].re = 1.0;

        Self {
            vector
        }
    }

    pub fn len(&self) -> usize {
        self.vector.len()
    }

    pub fn normalize(&mut self) {
        let len: QFloat = self.vector.iter()
            .map(|&x| (x * x).modulus())
            .fold(0.0, |acc, x| acc + x)
            .sqrt();

        for i in self.vector.iter_mut() {
            *i = (*i / len) * 1_000_000.0;
            i.re = (i.re.round()) / 1_000_000.0;
            i.im = (i.im.round()) / 1_000_000.0;
        }
    }

    pub fn apply_matrix(&mut self, m: QubitMatrix) -> Result<(), ()> {
        if m.get_size() != self.len() {
            return Err(());
        }

        let mut new_vector = self.vector.clone();

        for row in 0..new_vector.len() {
            let mut total = Complex::new(0.0, 0.0);
            for col in 0..m.get_size() {
                total += m.matrix[row][col] * self.vector[col];
            }

            new_vector[row] = total;
        }

        self.vector = new_vector;
        
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_apply_matrix() {
        let m = QubitMatrix::x_matrix(1, 1);
        let mut v = QubitVector::new(2);

        let _ = v.apply_matrix(m);

        let mut result = QubitVector::new(2);
        result.vector[0] = Complex::new(0.0, 0.0);
        result.vector[1] = Complex::new(1.0, 0.0);

        assert_eq!(result, v);
    }
}