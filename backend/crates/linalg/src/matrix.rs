use crate::{LinalgError, Vector};
use num_traits::{One, Zero};
use std::iter::Sum;
use std::ops::{Add, Index, IndexMut, Mul};

/// 固有値と固有ベクトルのペアを格納するジェネリックな構造体
#[derive(Debug, PartialEq)]
pub struct EigenDecomposition<T> {
    pub eigenvalues: Vec<T>,
    pub eigenvectors: Vec<Vector<T>>,
}

/// Matrix構造体の定義。Tが省略された場合はf64として扱われる
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T = f64> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

// --- ジェネリックなTに対する実装 ---
impl<T> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        // TODO: rows * cols == data.len() のチェック
        Self { rows, cols, data }
    }

    pub fn zeros(rows: usize, cols: usize) -> Self
    where
        T: Zero + Copy,
    {
        Self::new(rows, cols, vec![T::zero(); rows * cols])
    }

    pub fn identity(size: usize) -> Self
    where
        T: Zero + One + Copy,
    {
        let mut matrix = Self::zeros(size, size);
        for i in 0..size {
            matrix[(i, i)] = T::one();
        }
        matrix
    }

    pub fn transpose(&self) -> Matrix<T>
    where
        T: Copy,
    {
        unimplemented!()
    }

    pub fn swap_rows(&mut self, r1: usize, r2: usize) {
        unimplemented!()
    }

    pub fn scale_row(&mut self, r: usize, scalar: T)
    where
        T: Mul<Output = T> + Copy,
    {
        unimplemented!()
    }

    pub fn add_scaled_row_to_row(&mut self, source_row: usize, dest_row: usize, scalar: T)
    where
        T: Add<Output = T> + Mul<Output = T> + Copy,
    {
        unimplemented!()
    }

    pub fn col(&self, c: usize) -> Vector<T>
    where
        T: Copy,
    {
        unimplemented!()
    }

    pub fn row(&self, r: usize) -> Matrix<T>
    where
        T: Copy,
    {
        unimplemented!()
    }

    pub fn set_col(&mut self, c: usize, col_vec: &Vector<T>)
    where
        T: Copy,
    {
        unimplemented!()
    }

    pub fn set_row(&mut self, r: usize, row_vec: &Matrix<T>)
    where
        T: Copy,
    {
        unimplemented!()
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn trace(&self) -> T
    where
        T: Sum + Copy,
    {
        unimplemented!()
    }

    pub fn submatrix(
        &self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> Matrix<T>
    where
        T: Copy,
    {
        unimplemented!()
    }

    pub fn hstack(&self, other: &Matrix<T>) -> Result<Matrix<T>, LinalgError>
    where
        T: Copy,
    {
        unimplemented!()
    }

    pub fn vstack(&self, other: &Matrix<T>) -> Result<Matrix<T>, LinalgError>
    where
        T: Copy,
    {
        unimplemented!()
    }
}

// --- f64専用のメソッドを実装するブロック ---
impl Matrix<f64> {
    pub fn rank(&self) -> usize {
        unimplemented!()
    }

    pub fn determinant(&self) -> Result<f64, LinalgError> {
        unimplemented!()
    }

    pub fn inverse(&self) -> Option<Matrix<f64>> {
        unimplemented!()
    }

    pub fn eigen_decomposition(&self) -> Option<EigenDecomposition<f64>> {
        unimplemented!()
    }

    pub fn frobenius_norm(&self) -> f64 {
        unimplemented!()
    }

    pub fn lu_decomposition(&self) -> Option<(Matrix<f64>, Matrix<f64>)> {
        unimplemented!()
    }

    pub fn qr_decomposition(&self) -> Option<(Matrix<f64>, Matrix<f64>)> {
        unimplemented!()
    }

    pub fn svd(&self) -> Option<(Matrix<f64>, Matrix<f64>, Matrix<f64>)> {
        unimplemented!()
    }
}

// --- 添字アクセス ---
impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.cols + index.1]
    }
}
