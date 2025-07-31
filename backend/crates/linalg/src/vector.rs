use crate::{LinalgError, Matrix};
use num_traits::{One, Zero};
use std::iter::Sum;
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<T = f64> {
    pub data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    pub fn zeros(dim: usize) -> Self
    where
        T: Zero + Copy,
    {
        Self::new(vec![T::zero(); dim])
    }

    pub fn ones(dim: usize) -> Self
    where
        T: One + Copy,
    {
        Self::new(vec![T::one(); dim])
    }

    pub fn dim(&self) -> usize {
        self.data.len()
    }

    pub fn dot(&self, other: &Self) -> T
    where
        T: Sum + Copy + Mul<Output = T>,
    {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }

    pub fn transpose(&self) -> Matrix<T>
    where
        T: Copy,
    {
        Matrix::new(1, self.dim(), self.data.clone())
    }

    /// ベクトルの最大値のインデックスを返す
    pub fn argmax(&self) -> usize {
        unimplemented!()
    }

    /// ベクトルの最小値のインデックスを返す
    pub fn argmin(&self) -> usize {
        unimplemented!()
    }
}

// --- f64専用のメソッドを実装するブロック ---
impl Vector<f64> {
    pub fn linspace(start: f64, end: f64, num: usize) -> Self {
        unimplemented!()
    }

    pub fn norm(&self) -> f64 {
        unimplemented!()
    }

    pub fn cross(&self, other: &Self) -> Result<Vector<f64>, LinalgError> {
        unimplemented!()
    }

    pub fn normalize(&self) -> Vector<f64> {
        unimplemented!()
    }

    pub fn cosine_similarity(&self, other: &Self) -> f64 {
        unimplemented!()
    }

    pub fn max(&self) -> f64 {
        unimplemented!()
    }
    pub fn min(&self) -> f64 {
        unimplemented!()
    }
    pub fn mean(&self) -> f64 {
        unimplemented!()
    }
    pub fn std(&self) -> f64 {
        unimplemented!()
    }
}

// --- 添字アクセスのためのトレイト実装 ---
impl<T> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
