use crate::{Matrix, Ring, Vector};
use std::ops::Sub;

// --- Matrix - Matrix ---

// &Matrix<T> - &Matrix<T> (中心となる実装)
impl<'a, 'b, T: Ring> Sub<&'b Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: &'b Matrix<T>) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Dimension mismatch for matrix subtraction.");
        }
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a.clone() - b.clone())
            .collect();
        Matrix::new(self.rows, self.cols, data).unwrap()
    }
}

// 他の3パターンは上記の実装を呼び出す
impl<T: Ring> Sub<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        &self - &rhs
    }
}
impl<'a, T: Ring> Sub<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        self - &rhs
    }
}
impl<'b, T: Ring> Sub<&'b Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: &'b Matrix<T>) -> Self::Output {
        &self - rhs
    }
}

// --- Vector - Vector ---

// &Vector<T> - &Vector<T> (中心となる実装)
impl<'a, 'b, T: Ring> Sub<&'b Vector<T>> for &'a Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: &'b Vector<T>) -> Self::Output {
        if self.dim() != rhs.dim() {
            panic!("Vector dimensions must match for subtraction.");
        }
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a.clone() - b.clone())
            .collect();
        Vector::new(data)
    }
}

// 他の3パターンは上記の実装を呼び出す
impl<T: Ring> Sub<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        &self - &rhs
    }
}
impl<'a, T: Ring> Sub<Vector<T>> for &'a Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        self - &rhs
    }
}
impl<'b, T: Ring> Sub<&'b Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: &'b Vector<T>) -> Self::Output {
        &self - rhs
    }
}

// --- Matrix - Scalar ---

// &Matrix<T> - T (中心となる実装)
impl<'a, T: Ring> Sub<T> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: T) -> Self::Output {
        let data = self.data.iter().map(|v| v.clone() - rhs.clone()).collect();
        Matrix::new(self.rows, self.cols, data).unwrap()
    }
}

// Matrix<T> - T
impl<T: Ring> Sub<T> for Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: T) -> Self::Output {
        &self - rhs
    }
}

// --- Vector - Scalar ---

// &Vector<T> - T (中心となる実装)
impl<'a, T: Ring> Sub<T> for &'a Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: T) -> Self::Output {
        let data = self.data.iter().map(|v| v.clone() - rhs.clone()).collect();
        Vector::new(data)
    }
}

// Vector<T> - T
impl<T: Ring> Sub<T> for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: T) -> Self::Output {
        &self - rhs
    }
}
