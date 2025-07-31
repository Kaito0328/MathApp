use super::matrix::Matrix;
use std::ops::{Index, IndexMut}; // 親モジュールのmatrixをインポート

pub struct Vector {
    pub data: Vec<f64>,
}

impl Vector {
    // --- コンストラクタ ---

    /// 指定されたデータから新しいVectorを生成する
    pub fn new(data: Vec<f64>) -> Self {
        Self { data }
    }

    /// 全ての要素がゼロのVectorを生成する
    pub fn zeros(dim: usize) -> Self {
        unimplemented!()
    }

    // --- 基本操作 ---

    /// ベクトルの次元（要素数）を返す
    pub fn dim(&self) -> usize {
        self.data.len()
    }

    /// ベクトルのL2ノルム（大きさ）を計算する
    pub fn norm(&self) -> f64 {
        unimplemented!()
    }

    /// 他のベクトルとの内積を計算する
    pub fn dot(&self, other: &Self) -> f64 {
        unimplemented!()
    }

    /// 転置して行ベクトル（1行のMatrix）を生成する
    pub fn transpose(&self) -> Matrix {
        unimplemented!()
    }

    pub fn ones(dim: usize) -> Self {
        unimplemented!()
    }

    /// 指定した範囲の値で初期化されたベクトルを生成する
    pub fn linspace(start: f64, end: f64, num: usize) -> Self {
        unimplemented!()
    }

    /// 外積を計算する（3次元ベクトルのみ）
    pub fn cross(&self, other: &Self) -> Result<Vector, String> {
        unimplemented!()
    }

    /// ベクトルを正規化する
    pub fn normalize(&self) -> Vector {
        unimplemented!()
    }

    /// 他のベクトルとのコサイン類似度を計算する
    pub fn cosine_similarity(&self, other: &Self) -> f64 {
        unimplemented!()
    }

    /// ベクトルの最大値のインデックスを返す
    pub fn argmax(&self) -> usize {
        unimplemented!()
    }

    /// ベクトルの最小値のインデックスを返す
    pub fn argmin(&self) -> usize {
        unimplemented!()
    }

    /// ベクトルの最大値を返す
    pub fn max(&self) -> f64 {
        unimplemented!()
    }

    /// ベクトルの最小値を返す
    pub fn min(&self) -> f64 {
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

// --- 添字アクセスのためのトレイト実装 ---

impl Index<usize> for Vector {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
