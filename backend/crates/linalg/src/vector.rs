use crate::{LinalgError, Matrix, Result, Ring, Scalar};
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

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

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn argmax(&self) -> Option<usize>
    where
        T: PartialOrd,
    {
        if self.data.is_empty() {
            return None;
        }

        // 0番目のインデックスと値を初期値とする
        let mut max_index = 0;
        let mut max_value = &self.data[0];

        // 1番目からループを開始 (skip(1))
        for (i, current_value) in self.data.iter().enumerate().skip(1) {
            // 値を直接比較する
            if current_value > max_value {
                max_value = current_value;
                max_index = i;
            }
        }
        Some(max_index)
    }

    /// ベクトルの最小値のインデックスを返す
    pub fn argmin(&self) -> Option<usize>
    where
        T: PartialOrd,
    {
        if self.data.is_empty() {
            return None; // 空のベクトルの場合はNoneを返す
        }
        let mut min_index = 0;
        let mut min_value = &self.data[0];
        for (i, value) in self.data.iter().enumerate().skip(1) {
            if value < min_value {
                min_index = i;
                min_value = value;
            }
        }
        Some(min_index)
    }

    pub fn max(&self) -> Option<T>
    where
        T: PartialOrd + Copy,
    {
        if self.data.is_empty() {
            return None; // 空のベクトルの場合はNoneを返す
        }
        let mut max_value = self.data[0];
        for &value in self.data.iter().skip(1) {
            if value > max_value {
                max_value = value;
            }
        }
        Some(max_value)
    }

    pub fn min(&self) -> Option<T>
    where
        T: PartialOrd + Copy,
    {
        if self.data.is_empty() {
            return None; // 空のベクトルの場合はNoneを返す
        }
        let mut min_value = self.data[0];
        for &value in self.data.iter().skip(1) {
            if value < min_value {
                min_value = value;
            }
        }
        Some(min_value)
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

    pub fn checked_add(&self, other: &Self) -> Result<Self> {
        if self.dim() != other.dim() {
            return Err(LinalgError::InvalidDimension {
                dim: self.dim(),
                text: "Vector dimensions must match for addition.".to_string(),
            });
        }
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();
        Ok(Vector::new(data))
    }

    pub fn checked_sub(&self, other: &Self) -> Result<Self> {
        if self.dim() != other.dim() {
            return Err(LinalgError::InvalidDimension {
                dim: self.dim(),
                text: "Vector dimensions must match for subtraction.".to_string(),
            });
        }
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a.clone() - b.clone())
            .collect();
        Ok(Vector::new(data))
    }

    pub fn checked_mul_matrix(&self, matrix: &Matrix<T>) -> Result<Matrix<T>> {
        if matrix.rows != 1 {
            return Err(LinalgError::InvalidDimension {
                dim: 1,
                text: "Matrix rows must be 1 for vector multiplication.".to_string(),
            });
        }
        let data: Vec<T> = (0..self.dim())
            .flat_map(|i| {
                (0..matrix.cols).map(move |j| self.data[i].clone() * matrix[(0, j)].clone())
            })
            .collect();

        Matrix::new(self.dim(), matrix.cols, data)
    }

    pub fn checked_add_scalar(&self, scalar: T) -> Self {
        let data = self
            .data
            .iter()
            .map(|x| x.clone() + scalar.clone())
            .collect();
        Vector::new(data)
    }

    pub fn checked_sub_scalar(&self, scalar: T) -> Self {
        let data = self
            .data
            .iter()
            .map(|x| x.clone() - scalar.clone())
            .collect();
        Vector::new(data)
    }

    pub fn checked_mul_scalar(&self, scalar: T) -> Self {
        let data = self
            .data
            .iter()
            .map(|x| x.clone() * scalar.clone())
            .collect();
        Vector::new(data)
    }

    pub fn checked_neg(&self) -> Self {
        let data = self.data.iter().map(|v| -v.clone()).collect();
        Vector::new(data)
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
    pub fn hadamard_product(&self, other: &Self) -> Result<Vector<T>> {
        if self.dim() != other.dim() {
            return Err(LinalgError::InvalidDimension {
                dim: self.dim(),
                text: "Vector dimensions must match for Hadamard product.".to_string(),
            });
        }
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a.clone() * b.clone())
            .collect::<Vec<T>>();
        Ok(Vector::new(data))
    }

    pub fn cross(&self, other: &Self) -> Result<Vector<T>>
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
    pub fn linspace(start: f64, end: f64, num: usize) -> Result<Self> {
        if num <= 1 || start >= end {
            return Err(LinalgError::InvalidArgument {
                text: "num must be greater than 1 and start must be less than end".to_string(),
            });
        }
        let step = (end - start) / (num as f64 - 1.0);
        let data = (0..num)
            .map(|i| start + (i as f64 * step))
            .collect::<Vec<f64>>();
        Ok(Vector::new(data))
    }

    /// ベクトルのL2ノルム（大きさ）を計算する
    pub fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }

    /// ベクトルを正規化する（単位ベクトル化）
    pub fn normalize(&self) -> Vector<f64> {
        let norm = self.norm();
        if norm == 0.0 {
            return Vector::zeros(self.dim());
        }
        let data = self.data.iter().map(|x| x / norm).collect::<Vec<f64>>();
        Vector::new(data)
    }

    /// 他のベクトルとのコサイン類似度を計算する
    pub fn cosine_similarity(&self, other: &Self) -> f64 {
        let dot_product = self.dot(other);
        let norm_self = self.norm();
        let norm_other = other.norm();
        if norm_self == 0.0 || norm_other == 0.0 {
            return 0.0; // ゼロベクトルとの類似度は定義しない
        }
        dot_product / (norm_self * norm_other)
    }
    /// ベクトルの平均値を計算する
    pub fn mean(&self) -> Option<f64> {
        if self.data.is_empty() {
            return None;
        }
        let sum: f64 = self.data.iter().sum();
        Some(sum / self.data.len() as f64)
    }

    /// ベクトルの標準偏差を計算する
    pub fn std(&self) -> f64 {
        if self.data.len() < 2 {
            return 0.0;
        }
        let mean = self.mean().unwrap();
        let squared_diffs: Vec<f64> = self
            .data
            .iter()
            .map(|x| {
                let diff = *x - mean;
                diff * diff
            })
            .collect();
        let variance = squared_diffs.iter().sum::<f64>() / self.data.len() as f64;
        variance.sqrt()
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

impl<'a, T: Scalar> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<T: Ring> Neg for &Vector<T> {
    type Output = Vector<T>;
    fn neg(self) -> Self::Output {
        self.checked_neg()
    }
}

impl<T: Ring> Neg for Vector<T> {
    type Output = Vector<T>;
    fn neg(self) -> Self::Output {
        -&self
    }
}

impl<'b, T: Ring> Add<&'b Vector<T>> for &Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: &'b Vector<T>) -> Self::Output {
        let result = self.checked_add(rhs);
        match result {
            Ok(vec) => vec,
            Err(e) => panic!("Vector addition failed: {e}"),
        }
    }
}

impl<'b, T: Ring> Sub<&'b Vector<T>> for &Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: &'b Vector<T>) -> Self::Output {
        let result = self.checked_sub(rhs);
        match result {
            Ok(vec) => vec,
            Err(e) => panic!("Vector subtraction failed: {e}"),
        }
    }
}

impl<'b, T: Ring> Mul<&'b Vector<T>> for &Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: &'b Vector<T>) -> Self::Output {
        let result = self.hadamard_product(rhs);
        match result {
            Ok(vec) => vec,
            Err(e) => panic!("Vector multiplication failed: {e}"),
        }
    }
}

impl<'b, T: Ring> Mul<&'b Matrix<T>> for &Vector<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        let result = self.checked_mul_matrix(rhs);
        match result {
            Ok(mat) => mat,
            Err(e) => panic!("Vector-matrix multiplication failed: {e}"),
        }
    }
}

impl<'b, T: Ring> Add<&'b T> for &Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: &'b T) -> Self::Output {
        let data = self.data.iter().map(|x| x.clone() + rhs.clone()).collect();
        Vector::new(data)
    }
}

impl<'b, T: Ring> Sub<&'b T> for &Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: &'b T) -> Self::Output {
        let data = self.data.iter().map(|x| x.clone() - rhs.clone()).collect();
        Vector::new(data)
    }
}

impl<'b, T: Ring> Mul<&'b T> for &Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: &'b T) -> Self::Output {
        let data = self.data.iter().map(|x| x.clone() * rhs.clone()).collect();
        Vector::new(data)
    }
}

impl<'b, T: Ring> Add<&'b Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: &'b Vector<T>) -> Self::Output {
        &self + rhs
    }
}

impl<'b, T: Ring> Sub<&'b Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: &'b Vector<T>) -> Self::Output {
        &self - rhs
    }
}

impl<'b, T: Ring> Mul<&'b Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: &'b Vector<T>) -> Self::Output {
        &self * rhs
    }
}

impl<'b, T: Ring> Mul<&'b Matrix<T>> for Vector<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        &self * rhs
    }
}

impl<'b, T: Ring> Add<&'b T> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: &'b T) -> Self::Output {
        &self + rhs
    }
}

impl<'b, T: Ring> Sub<&'b T> for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: &'b T) -> Self::Output {
        &self - rhs
    }
}

impl<'b, T: Ring> Mul<&'b T> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: &'b T) -> Self::Output {
        &self * rhs
    }
}

impl<T: Ring> Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        &self + &rhs
    }
}

impl<T: Ring> Sub<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        &self - &rhs
    }
}

impl<T: Ring> Mul<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Ring> Mul<Matrix<T>> for Vector<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Ring> Add<T> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: T) -> Self::Output {
        &self + &rhs
    }
}

impl<T: Ring> Sub<T> for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: T) -> Self::Output {
        &self - &rhs
    }
}

impl<T: Ring> Mul<T> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Ring> Add<Vector<T>> for &Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        self + &rhs
    }
}

impl<T: Ring> Sub<Vector<T>> for &Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        self - &rhs
    }
}

impl<T: Ring> Mul<Vector<T>> for &Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T: Ring> Mul<Matrix<T>> for &Vector<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T: Ring> Add<T> for &Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: T) -> Self::Output {
        self + &rhs
    }
}

impl<T: Ring> Sub<T> for &Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: T) -> Self::Output {
        self - &rhs
    }
}

impl<T: Ring> Mul<T> for &Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        self * &rhs
    }
}
