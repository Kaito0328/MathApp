use crate::{Matrix, Ring, Vector};
use std::ops::Add;

// &Matrix<T> + &Matrix<T> の実装（これが中心ロジック）
impl<'a, 'b, T: Ring> Add<&'b Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: &'b Matrix<T>) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Dimension mismatch for matrix addition.");
        }
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();
        Matrix::new(self.rows, self.cols, data).unwrap()
    }
}

// 他の3パターンは、上記の実装を呼び出すだけ
impl<T: Ring> Add<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: Matrix<T>) -> Self::Output {
        &self + &rhs
    }
}
impl<T: Ring> Add<Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: Matrix<T>) -> Self::Output {
        self + &rhs
    }
}
impl<T: Ring> Add<&Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        &self + rhs
    }
}

// Vectorも同様に実装
// &Vector<T> + &Vector<T>
impl<T: Ring> Add<&Vector<T>> for &Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: &Vector<T>) -> Self::Output {
        unimplemented!()
    }
}

impl<T: Ring> Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        &self + &rhs
    }
}
impl<T: Ring> Add<Vector<T>> for &Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        self + &rhs
    }
}
impl<T: Ring> Add<&Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: &Vector<T>) -> Self::Output {
        &self + rhs
    }
}
// ... Vectorの他の3パターンも同様に実装 ...

// スカラー加算
// &Matrix<T> + T
impl<T: Ring> Add<T> for &Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: T) -> Self::Output {
        let data = self.data.iter().map(|v| v.clone() + rhs.clone()).collect();
        Matrix::new(self.rows, self.cols, data).unwrap()
    }
}
// ... Matrix + T, Vector + T なども同様に参照実装を呼び出す ...
// Matrix<T> + T (スカラー加算)
impl<T: Ring> Add<T> for Matrix<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn add(self, rhs: T) -> Self::Output {
        unimplemented!()
    }
}

// Vector<T> + T (スカラー加算)
impl<T: Ring> Add<T> for Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn add(self, rhs: T) -> Self::Output {
        unimplemented!()
    }
}
