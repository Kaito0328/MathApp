use crate::{Matrix, Ring, Vector};
use std::ops::Neg;

// --- Unary Minus for Matrix ---

// - &Matrix<T> (中心となる実装)
impl<'a, T: Ring> Neg for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn neg(self) -> Self::Output {
        let data = self.data.iter().map(|v| -v.clone()).collect();
        Matrix::new(self.rows, self.cols, data).unwrap()
    }
}

// - Matrix<T>
impl<T: Ring> Neg for Matrix<T> {
    type Output = Matrix<T>;
    fn neg(self) -> Self::Output {
        -&self
    }
}

// --- Unary Minus for Vector ---

// - &Vector<T> (中心となる実装)
impl<'a, T: Ring> Neg for &'a Vector<T> {
    type Output = Vector<T>;
    fn neg(self) -> Self::Output {
        let data = self.data.iter().map(|v| -v.clone()).collect();
        Vector::new(data)
    }
}

// - Vector<T>
impl<T: Ring> Neg for Vector<T> {
    type Output = Vector<T>;
    fn neg(self) -> Self::Output {
        -&self
    }
}
