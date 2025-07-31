use crate::{LinalgError, Matrix, Ring, Scalar};
use num_traits::{One, Zero};
use std::iter::Sum;
use std::ops::{Index, IndexMut, Mul};

/// Vector構造体の定義。Tは最低限Scalarであることを要求
#[derive(Debug, PartialEq, Clone)]
pub struct Vector<T: Scalar = f64> {
    pub data: Vec<T>,
}

// --- Level 1: Scalarに対する実装 (最も基本的な操作) ---
impl<T: Scalar> Vector<T> {
    /// 指定されたデータから新しいVectorを生成する
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    /// ベクトルの次元（要素数）を返す
    pub fn dim(&self) -> usize {
        self.data.len()
    }

    /// 転置して行ベクトル（1行のMatrix）を生成する
    pub fn transpose(&self) -> Matrix<T> {
        Matrix::new(1, self.dim(), self.data.clone()).unwrap()
    }

    /// ベクトルの最大値のインデックスを返す
    pub fn argmax(&self) -> usize
    where
        T: PartialOrd,
    {
        unimplemented!()
    }

    /// ベクトルの最小値のインデックスを返す
    pub fn argmin(&self) -> usize
    where
        T: PartialOrd,
    {
        unimplemented!()
    }

    pub fn max(&self) -> T
    where
        T: PartialOrd + Copy,
    {
        unimplemented!()
    }

    pub fn min(&self) -> T
    where
        T: PartialOrd + Copy,
    {
        unimplemented!()
    }
}

// --- Level 2: Ringに対する実装 (加減乗算などの代数的操作) ---
impl<T: Ring> Vector<T> {
    /// 全ての要素がゼロのVectorを生成する
    pub fn zeros(dim: usize) -> Self {
        Self::new(vec![T::zero(); dim])
    }

    /// 全ての要素がイチのVectorを生成する
    pub fn ones(dim: usize) -> Self {
        Self::new(vec![T::one(); dim])
    }

    /// 他のベクトルとの内積を計算する
    pub fn dot(&self, other: &Self) -> T {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a.clone() * b.clone())
            .sum()
    }

    /// 他のベクトルとのアダマール積（要素ごとの積）を計算する
    pub fn hadamard_product(&self, other: &Self) -> Vector<T> {
        // ... 以前の `*` の実装ロジック ...
        unimplemented!()
    }

    /// 他のベクトルとの外積を計算し、行列を返す
    pub fn outer_product(&self, other: &Self) -> Matrix<T> {
        // self (n x 1) * other.transpose() (1 x m) -> n x m Matrix
        unimplemented!()
    }

    pub fn cross(&self, other: &Self) -> Result<Vector<T>, LinalgError>
    where
        T: Ring + Copy,
    {
        if self.dim() != 3 || other.dim() != 3 {
            return Err(LinalgError::InvalidDimension {
                dim: self.dim(),
                text: "Cross product is only defined for 3D vectors.".to_string(),
            });
        }
        let a = self.data[0];
        let b = self.data[1];
        let c = self.data[2];
        let d = other.data[0];
        let e = other.data[1];
        let f = other.data[2];

        // 外積の計算
        let data = vec![b * f - c * e, c * d - a * f, a * e - b * d];
        Ok(Vector::new(data))
    }
}

// --- f64専用の高度な数値計算メソッド ---
impl Vector<f64> {
    /// 指定した範囲の値で初期化されたベクトルを生成する
    pub fn linspace(start: f64, end: f64, num: usize) -> Self {
        unimplemented!()
    }

    /// ベクトルのL2ノルム（大きさ）を計算する
    pub fn norm(&self) -> f64 {
        // self.dot(self)の結果の平方根
        unimplemented!()
    }

    /// ベクトルを正規化する（単位ベクトル化）
    pub fn normalize(&self) -> Vector<f64> {
        unimplemented!()
    }

    /// 他のベクトルとのコサイン類似度を計算する
    pub fn cosine_similarity(&self, other: &Self) -> f64 {
        unimplemented!()
    }
    /// ベクトルの平均値を計算する
    pub fn mean(&self) -> f64 {
        unimplemented!()
    }

    /// ベクトルの標準偏差を計算する
    pub fn std(&self) -> f64 {
        unimplemented!()
    }
}

// --- 添字アクセス (T: Scalarであれば可能) ---
impl<T: Scalar> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Scalar> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
