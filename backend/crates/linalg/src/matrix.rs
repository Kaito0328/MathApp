use crate::{Field, LinalgError, Result, Ring, Scalar, Vector};
use core::fmt;
use num_complex::Complex;
use std::cmp::min;
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

/// 固有値と固有ベクトルのペアを格納するジェネリックな構造体
#[derive(Debug, PartialEq)]
pub struct EigenDecomposition {
    pub eigenvalues: Vec<Complex<f64>>,
    pub eigenvectors: Matrix<f64>,
}

#[derive(Debug, PartialEq)]
pub struct SVD {
    /// 左特異ベクトル行列 U
    pub u: Matrix<f64>,
    /// 特異値ベクトル Σ (対角行列の対角成分)
    pub sigma: Vector<f64>,
    /// 右特異ベクトル行列 V
    pub v: Matrix<f64>,
}

/// Matrix構造体の定義。Tは最低限Scalarであることを要求
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T: Scalar = f64> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

// --- Level 1: Scalarに対する実装 (最も基本的な操作) ---
impl<T: Scalar> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Result<Self> {
        if rows * cols != data.len() {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{}x{} ({} elements)", rows, cols, rows * cols),
                found: format!("{} elements", data.len()),
            });
        }
        Ok(Self { rows, cols, data })
    }

    pub fn with_default(rows: usize, cols: usize) -> Self
    where
        T: Default,
    {
        let data = (0..rows * cols).map(|_| T::default()).collect();
        Self::new(rows, cols, data).unwrap()
    }

    pub fn transpose(&self) -> Matrix<T> {
        if self.rows == 0 || self.cols == 0 {
            return Matrix::new(self.cols, self.rows, vec![]).unwrap();
        }
        let data = (0..self.cols)
            .flat_map(|j| (0..self.rows).map(move |i| self[(i, j)].clone()))
            .collect();
        Matrix::new(self.cols, self.rows, data).unwrap()
    }

    pub fn swap_rows(&mut self, r1: usize, r2: usize) -> Result<()> {
        let row1 = self.row(r1)?;
        let row2 = self.row(r2)?;
        self.set_row(r1, &row2)?;
        self.set_row(r2, &row1)?;
        Ok(())
    }

    pub fn col(&self, c: usize) -> Result<Vector<T>> {
        if c >= self.cols {
            return Err(LinalgError::IndexOutOfBounds {
                index: c,
                size: self.cols,
            });
        }
        let col_data = (0..self.rows).map(|r| self[(r, c)].clone()).collect();
        Ok(Vector::new(col_data))
    }

    pub fn partial_col(
        &self,
        col_idx: usize,
        start_row: usize,
        end_row: usize,
    ) -> Result<Vector<T>> {
        if col_idx >= self.cols {
            return Err(LinalgError::IndexOutOfBounds {
                index: col_idx,
                size: self.cols,
            });
        }
        if end_row > self.rows || start_row > end_row {
            return Err(LinalgError::InvalidDimension {
                dim: (end_row),
                text: ("Invalid row range for column extraction".to_string()),
            });
        }

        let data = (start_row..end_row)
            .map(|r| self[(r, col_idx)].clone())
            .collect();

        Ok(Vector::new(data))
    }

    pub fn row(&self, r: usize) -> Result<Vector<T>> {
        if r >= self.rows {
            return Err(LinalgError::IndexOutOfBounds {
                index: r,
                size: self.rows,
            });
        }
        let row_data = (0..self.cols).map(|c| self[(r, c)].clone()).collect();
        Ok(Vector::new(row_data))
    }
    pub fn set_col(&mut self, c: usize, col_vec: &Vector<T>) -> Result<()> {
        if c >= self.cols {
            return Err(LinalgError::IndexOutOfBounds {
                index: c,
                size: self.cols,
            });
        }
        if col_vec.dim() != self.rows {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{} rows", self.rows),
                found: format!("{} rows", col_vec.dim()),
            });
        }
        for r in 0..self.rows {
            self[(r, c)] = col_vec[r].clone();
        }
        Ok(())
    }
    pub fn set_row(&mut self, r: usize, row_vec: &Vector<T>) -> Result<()> {
        if r >= self.rows {
            return Err(LinalgError::IndexOutOfBounds {
                index: r,
                size: self.rows,
            });
        }

        if row_vec.dim() != self.cols {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{} columns", self.cols),
                found: format!("{} columns", row_vec.dim()),
            });
        }
        let row_start = r * self.cols;
        let row_end = row_start + self.cols;

        self.data[row_start..row_end].clone_from_slice(&row_vec.data);

        Ok(())
    }
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }
    pub fn submatrix(
        &self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> Matrix<T> {
        let data = (start_row..end_row)
            .flat_map(|i| (start_col..end_col).map(move |j| self[(i, j)].clone()))
            .collect();
        Matrix::new(end_row - start_row, end_col - start_col, data).unwrap()
    }

    pub fn set_submatrix(
        &mut self,
        start_row: usize,
        start_col: usize,
        submatrix: &Matrix<T>,
    ) -> Result<()> {
        if start_row + submatrix.rows > self.rows || start_col + submatrix.cols > self.cols {
            return Err(LinalgError::IndexOutOfBounds {
                index: start_row + submatrix.rows,
                size: self.rows,
            });
        }
        for i in 0..submatrix.rows {
            for j in 0..submatrix.cols {
                self[(start_row + i, start_col + j)] = submatrix[(i, j)].clone();
            }
        }
        Ok(())
    }

    pub fn hstack(&self, other: &Matrix<T>) -> Result<Matrix<T>> {
        if other.rows != self.rows {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{} rows", self.rows),
                found: format!("{} rows", other.rows),
            });
        }
        let self_rows = self.data.chunks(self.cols);
        let other_rows = other.data.chunks(other.cols);
        let combined_data = self_rows
            .zip(other_rows)
            .flat_map(|(self_row, other_row)| {
                self_row.iter().cloned().chain(other_row.iter().cloned())
            })
            .collect();
        Matrix::new(self.rows, self.cols + other.cols, combined_data)
    }
    pub fn vstack(&self, other: &Matrix<T>) -> Result<Matrix<T>> {
        if other.cols != self.cols {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{} columns", self.cols),
                found: format!("{} columns", other.cols),
            });
        }
        let data = self
            .data
            .iter()
            .cloned()
            .chain(other.data.iter().cloned())
            .collect();
        Matrix::new(self.rows + other.rows, self.cols, data)
    }
}

// --- Level 2: Ringに対する実装 (加減乗算などの代数的操作) ---
impl<T: Ring> Matrix<T> {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols, vec![T::zero(); rows * cols]).unwrap()
    }

    pub fn identity(size: usize) -> Self {
        let mut matrix = Self::zeros(size, size);
        for i in 0..size {
            matrix[(i, i)] = T::one();
        }
        matrix
    }

    pub fn diag(rows: usize, cols: usize, diag_elements: &Vector<T>) -> Self {
        let mut matrix = Matrix::zeros(rows, cols);
        // 対角線の長さは、行数と列数の小さい方
        let diag_len = std::cmp::min(rows, cols);
        // ベクトルの長さと対角線の長さの、さらに小さい方までを埋める
        let fill_len = std::cmp::min(diag_len, diag_elements.dim());

        for i in 0..fill_len {
            matrix[(i, i)] = diag_elements[i].clone();
        }
        matrix
    }

    pub fn checked_add(&self, other: &Matrix<T>) -> Result<Matrix<T>> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{}x{}", self.rows, self.cols),
                found: format!("{}x{}", other.rows, other.cols),
            });
        }
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();
        Matrix::new(self.rows, self.cols, data)
    }

    pub fn checked_sub(&self, other: &Matrix<T>) -> Result<Matrix<T>> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{}x{}", self.rows, self.cols),
                found: format!("{}x{}", other.rows, other.cols),
            });
        }
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a.clone() - b.clone())
            .collect();
        Matrix::new(self.rows, self.cols, data)
    }

    pub fn checked_mul(&self, rhs: &Matrix<T>) -> Result<Matrix<T>> {
        if self.cols != rhs.rows {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{}x{}", self.rows, self.cols),
                found: format!("{}x{}", rhs.rows, rhs.cols),
            });
        }
        let mut data = vec![T::zero(); self.rows * rhs.cols];
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let sum = (0..self.cols)
                    .map(|k| self[(i, k)].clone() * rhs[(k, j)].clone())
                    .sum();
                data[i * rhs.cols + j] = sum;
            }
        }
        Matrix::new(self.rows, rhs.cols, data)
    }

    pub fn checked_mul_vector(&self, rhs: &Vector<T>) -> Result<Vector<T>> {
        if self.cols != rhs.dim() {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{} columns", self.cols),
                found: format!("{} elements in vector", rhs.dim()),
            });
        }
        let mut data = vec![T::zero(); self.rows];
        for i in 0..self.rows {
            data[i] = (0..self.cols)
                .map(|j| self[(i, j)].clone() * rhs[j].clone())
                .sum();
        }
        Ok(Vector::new(data))
    }

    pub fn checked_mul_scalar(&self, scalar: T) -> Matrix<T> {
        let data = self
            .data
            .iter()
            .map(|x| x.clone() * scalar.clone())
            .collect();
        Matrix::new(self.rows, self.cols, data).unwrap()
    }

    pub fn checked_add_scalar(&self, scalar: T) -> Matrix<T> {
        let data = self
            .data
            .iter()
            .map(|x| x.clone() + scalar.clone())
            .collect();
        Matrix::new(self.rows, self.cols, data).unwrap()
    }

    pub fn checked_sub_scalar(&self, scalar: T) -> Matrix<T> {
        let data = self
            .data
            .iter()
            .map(|x| x.clone() - scalar.clone())
            .collect();
        Matrix::new(self.rows, self.cols, data).unwrap()
    }

    pub fn checked_neg(&self) -> Matrix<T> {
        let data = self.data.iter().map(|v| -v.clone()).collect();
        Matrix::new(self.rows, self.cols, data).unwrap()
    }

    pub fn scale_row(&mut self, r: usize, scalar: T) -> Result<()> {
        if r >= self.rows {
            return Err(LinalgError::IndexOutOfBounds {
                index: r,
                size: self.rows,
            });
        }
        for c in 0..self.cols {
            self[(r, c)] = self[(r, c)].clone() * scalar.clone();
        }
        Ok(())
    }

    pub fn scale_col(&mut self, c: usize, scalar: T) -> Result<()> {
        if c >= self.cols {
            return Err(LinalgError::IndexOutOfBounds {
                index: c,
                size: self.cols,
            });
        }
        for r in 0..self.rows {
            self[(r, c)] = self[(r, c)].clone() * scalar.clone();
        }
        Ok(())
    }
    pub fn add_scaled_row_to_row(
        &mut self,
        source_row: usize,
        dest_row: usize,
        scalar: T,
    ) -> Result<()> {
        if source_row >= self.rows || dest_row >= self.rows {
            return Err(LinalgError::IndexOutOfBounds {
                index: source_row,
                size: self.rows,
            });
        }
        for c in 0..self.cols {
            self[(dest_row, c)] =
                self[(dest_row, c)].clone() + self[(source_row, c)].clone() * scalar.clone();
        }
        Ok(())
    }

    pub fn trace(&self) -> T {
        if !self.is_square() {
            // 正方行列でない場合はパニックさせるか、エラーを返す
            panic!("Trace is only defined for square matrices.");
        }
        (0..self.rows).map(|i| self[(i, i)].clone()).sum()
    }
}

// --- Level 3: Fieldに対する実装 (除算を必要とする操作) ---
impl<T: Field> Matrix<T> {
    fn _gauss_elimination(&self) -> Result<(Self, T)> {
        let mut pivot_row = 0;
        let mut det_factor = T::one();
        let mut rref_matrix = self.clone();

        for c in 0..self.cols {
            if pivot_row >= self.rows {
                break;
            }

            // ピボット要素を見つける
            let mut not_zero_index = None;
            for r in pivot_row..self.rows {
                if !rref_matrix[(r, c)].clone().is_zero() {
                    not_zero_index = Some(r);
                    break;
                }
            }

            if not_zero_index.is_none() {
                det_factor = T::zero(); // ピボットが見つからない場合、行列式はゼロ
                continue; // この列にはピボットがない
            }

            if pivot_row != not_zero_index.unwrap() {
                rref_matrix.swap_rows(pivot_row, not_zero_index.unwrap())?;
                det_factor = -det_factor;
            }

            let pivot_value = rref_matrix[(pivot_row, c)].clone();
            rref_matrix.scale_row(pivot_row, T::one() / pivot_value.clone())?;
            det_factor = det_factor * pivot_value.clone();

            for r in 0..self.rows {
                if r == pivot_row {
                    continue; // ピボット行はスキップ
                }

                let scale = rref_matrix[(r, c)].clone();
                rref_matrix.add_scaled_row_to_row(pivot_row, r, -scale)?;
            }

            pivot_row += 1; // 次のピボット行へ
        }

        Ok((rref_matrix, det_factor))
    }

    pub fn rref(&self) -> Result<Matrix<T>> {
        let (mat, _) = self._gauss_elimination()?;
        Ok(mat)
    }

    pub fn rank(&self) -> Result<usize> {
        let rref = self.rref()?;
        let rank = Self::rank_from_rref(&rref);
        Ok(rank)
    }

    pub fn rank_from_rref(rref_matrix: &Matrix<T>) -> usize {
        let mut rank = 0;
        for r in 0..rref_matrix.rows {
            // is_zero()を実装している前提
            if rref_matrix.row(r).unwrap().iter().any(|x| !x.is_zero()) {
                rank += 1;
            }
        }
        rank
    }

    pub fn determinant(&self) -> Result<T> {
        if !self.is_square() {
            return Err(LinalgError::DimensionMismatch {
                expected: "square matrix".to_string(),
                found: format!("{}x{}", self.rows, self.cols),
            });
        }
        let (_, det) = self._gauss_elimination()?;
        Ok(det)
    }

    pub fn inverse(&self) -> Option<Matrix<T>> {
        if !self.is_square() {
            return None; // 正方行列でない場合は逆行列は存在しない
        }

        let i_mat = Self::identity(self.rows);
        let augmented = self.hstack(&i_mat).unwrap();

        let (rref, _) = augmented._gauss_elimination().ok()?;
        let left_half = rref.submatrix(0, self.rows, 0, self.cols);

        if Self::rank_from_rref(&left_half) != self.rows {
            return None; // 逆行列が存在しない
        }

        Some(rref.submatrix(0, self.rows, self.cols, self.cols * 2))
    }
}

// --- f64専用の高度な数値計算メソッド ---
impl Matrix<f64> {
    fn householder_reflection_matrix(x: &Vector<f64>) -> Option<Matrix<f64>> {
        let norm_x = x.norm();
        let mut u = x.clone();

        let sign = if u[0] < 0.0 { -1.0 } else { 1.0 };
        u[0] += sign * norm_x;

        let norm_u = u.norm();
        if norm_u < 1e-10 {
            // 変換不要、またはゼロベクトルなので単位行列を返すかNoneを返す
            return Some(Matrix::identity(x.dim()));
        }

        let u_normalized = &u * (1.0 / norm_u);
        let outer_prod = u_normalized.clone() * u_normalized.transpose();
        let h_prime = Matrix::identity(x.dim()) - &outer_prod * 2.0;

        Some(h_prime)
    }
    fn to_hessenberg(&self) -> Option<(Matrix<f64>, Matrix<f64>)> {
        if !self.is_square() {
            return None; // 正方行列でない場合は変換できない
        }
        let mut h = self.clone();
        // v は変換行列を累積していくので、mut をつける
        let mut v = Matrix::identity(self.rows);

        for k in 0..self.rows - 2 {
            // k列目の下の部分をゼロにする
            let x: Vector<f64> = h.partial_col(k, k + 1, self.rows).unwrap();
            let h_prime = Self::householder_reflection_matrix(&x)?;

            // H' を n x n の行列 H に埋め込む
            let mut full_h = Matrix::identity(self.rows);
            full_h.set_submatrix(k + 1, k + 1, &h_prime).unwrap();

            // Aを変換
            h = &(&full_h * &h) * &full_h;

            // ★★★★★ ここが修正箇所 ★★★★★
            // 固有ベクトルを計算するために、変換行列を累積していく
            v = &v * &full_h;
        }
        Some((h, v))
    }

    fn solve_2x2_eigenvalues(a: f64, b: f64, c: f64, d: f64) -> (Complex<f64>, Complex<f64>) {
        let trace = a + d;
        let det = a * d - b * c;
        let discriminant = trace * trace - 4.0 * det;

        if discriminant >= 0.0 {
            // 実数固有値の場合
            let sqrt_disc = discriminant.sqrt();
            let lambda1 = (trace + sqrt_disc) / 2.0;
            let lambda2 = (trace - sqrt_disc) / 2.0;
            (Complex::new(lambda1, 0.0), Complex::new(lambda2, 0.0))
        } else {
            // 複素共役な固有値の場合
            let real_part = trace / 2.0;
            let imag_part = (-discriminant).sqrt() / 2.0;
            (
                Complex::new(real_part, imag_part),
                Complex::new(real_part, -imag_part),
            )
        }
    }

    pub fn eigen_decomposition(&self) -> Option<EigenDecomposition> {
        // --- エッジケースの事前処理 ---
        if self.rows == 0 {
            return Some(EigenDecomposition {
                eigenvalues: vec![],
                // 空のMatrixを返す
                eigenvectors: Matrix::new(0, 0, vec![]).unwrap(),
            });
        }
        if !self.is_square() {
            return None;
        }
        if self.rows == 1 {
            return Some(EigenDecomposition {
                eigenvalues: vec![Complex::new(self[(0, 0)], 0.0)],
                // 1x1の単位行列を返す
                eigenvectors: Matrix::identity(1),
            });
        }

        const MAX_ITERATIONS: usize = 1000;
        let n = self.rows;

        let (mut h, mut v) = self.to_hessenberg()?;
        let mut end = n;

        // ... while end > 0 ループの中身は変更なし ...
        while end > 0 {
            let mut iter = 0;
            loop {
                if iter >= MAX_ITERATIONS * n {
                    println!("Maximum iterations reached, returning None.");
                    return None;
                }
                iter += 1;
                let m = end - 1;

                // 2x2 ブロックに分割された場合の処理
                if end == 2 {
                    // このブロックは固有値を計算するだけなので、このまま残しても良いが、
                    // ループ後の対角成分から取得する方が一貫性がある。
                    // ただし、2x2のままループが終わる場合もあるため、この処理は安全策として有効。
                    let (lambda1, lambda2) =
                        Self::solve_2x2_eigenvalues(h[(0, 0)], h[(0, 1)], h[(1, 0)], h[(1, 1)]);
                    h[(0, 0)] = lambda1.re;
                    h[(1, 1)] = lambda2.re;
                    h[(0, 1)] = 0.0;
                    h[(1, 0)] = 0.0;
                    end = 0;
                    break;
                }
                let tol = f64::EPSILON * (h[(m, m)].abs() + h[(m - 1, m - 1)].abs());
                if h[(m, m - 1)].abs() <= tol {
                    // 分離成功。問題サイズを1小さくして、内側ループを抜ける
                    end -= 1;
                    break;
                } else {
                    let s = h[(m, m)];
                    let t = h[(m - 1, m - 1)];
                    let u = h[(m - 1, m)];
                    let p = h[(m, m - 1)];
                    let trace = t + s;
                    let det = t * s - u * p;
                    let discriminant = (trace * trace / 4.0) - det;
                    let mu1_denom = trace / 2.0 + discriminant.abs().sqrt().copysign(trace);
                    let mu1 = if mu1_denom.abs() > 1e-14 {
                        mu1_denom
                    } else {
                        0.0
                    };
                    let mu2 = if mu1.abs() > 1e-14 { det / mu1 } else { 0.0 };
                    let shift = if (mu1 - s).abs() < (mu2 - s).abs() {
                        mu1
                    } else {
                        mu2
                    };
                    let mut shifted_h = h.clone();
                    for j in 0..end {
                        shifted_h[(j, j)] -= shift;
                    }
                    if let Some((q, _)) = shifted_h.submatrix(0, end, 0, end).qr_decomposition() {
                        let mut q_full = Matrix::identity(n);
                        q_full.set_submatrix(0, 0, &q).ok()?;
                        h = &(&q_full.transpose() * &h) * &q_full;
                        v = &v * &q_full;
                    } else {
                        return None;
                    }
                }
            }
        }

        let eigenvalues: Vec<Complex<f64>> = (0..n).map(|i| Complex::new(h[(i, i)], 0.0)).collect();

        // v は、h の対角成分（固有値）に対応する固有ベクトル行列になっている。
        let eigenvectors_matrix = v;

        println!("Eigenvalues: {eigenvectors_matrix}");

        Some(EigenDecomposition {
            eigenvalues,
            eigenvectors: eigenvectors_matrix,
        })
    }

    pub fn frobenius_norm(&self) -> f64 {
        if self.rows == 0 || self.cols == 0 {
            return 0.0; // 空の行列の場合は0を返す
        }
        let mut sum = 0.0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self[(i, j)].is_nan() || self[(i, j)].is_infinite() {
                    return f64::NAN; // NaNまたは無限大が含まれている場合はNaNを返す
                }
                sum += self[(i, j)].powi(2);
            }
        }
        sum.sqrt()
    }
    /// PA = LU 分解を行う.
    /// 返り値は成功した場合 Some((P, L, U))、失敗した場合は None
    /// PA = LU 分解を行う.
    /// 返り値は成功した場合 Some((P, L, U))、失敗した場合は None
    pub fn lu_decomposition(&self) -> Option<(Matrix<f64>, Matrix<f64>, Matrix<f64>)> {
        if !self.is_square() {
            return None;
        }
        let n = self.rows;
        // Lはゼロ行列から始め、UはAのコピーから始めるのがシンプル
        let mut l = Matrix::zeros(n, n);
        let mut u = self.clone();
        let mut p = Matrix::identity(n);

        for k in 0..n {
            // --- 1. ピボット選択 ---
            let mut max_val = 0.0;
            let mut pivot_row = k;
            for i in k..n {
                if u[(i, k)].abs() > max_val {
                    max_val = u[(i, k)].abs();
                    pivot_row = i;
                }
            }

            if max_val < 1e-10 {
                return None;
            }

            // --- 2. 行の交換 ---
            if pivot_row != k {
                let _ = p.swap_rows(k, pivot_row);
                let _ = u.swap_rows(k, pivot_row);
                // L行列の計算済み部分(k列目より前)だけを交換する
                for j in 0..k {
                    let temp = l[(k, j)];
                    l[(k, j)] = l[(pivot_row, j)];
                    l[(pivot_row, j)] = temp;
                }
            }

            // --- 3. LとUの計算 (ガウスの消去法) ---
            // Lの対角成分は1
            l[(k, k)] = 1.0;

            // Lのk列目の計算
            for i in k + 1..n {
                l[(i, k)] = u[(i, k)] / u[(k, k)];
            }

            // Uの更新 (前方消去)
            for i in k + 1..n {
                for j in k..n {
                    u[(i, j)] -= l[(i, k)] * u[(k, j)];
                }
            }
        }

        Some((p, l, u))
    }

    pub fn qr_decomposition(&self) -> Option<(Matrix<f64>, Matrix<f64>)> {
        let (rows, cols) = (self.rows, self.cols);

        let mut r = self.clone();
        let mut q = Matrix::identity(rows);

        for k in 0..min(rows, cols) {
            // --- 1. 部分ベクトルを抽出 ---
            let x: Vector<f64> = r.partial_col(k, k, rows).unwrap();

            let h_prime = Self::householder_reflection_matrix(&x)?;

            // --- 4. H' を n x n の行列 H に埋め込む ---
            let mut h = Matrix::identity(rows);
            h.set_submatrix(k, k, &h_prime).unwrap();

            // --- 5. QとRを更新 ---
            r = &h * &r;
            q = &q * &h;
        }

        for k in 0..min(rows, cols) {
            if r[(k, k)] >= 0.0 {
                continue;
            }

            let _ = q.scale_col(k, -1.0);
            let _ = r.scale_row(k, -1.0);
        }

        Some((q, r))
    }

    /// 特異値分解 A = UΣV^T を計算します
    pub fn svd(&self) -> Option<SVD> {
        if self.rows < self.cols {
            // Aが横長の行列(m < n)の場合、A^TのSVDを計算して結果を変換する
            let svd_t = self.transpose().svd()?;
            return Some(SVD {
                u: svd_t.v,
                sigma: svd_t.sigma,
                v: svd_t.u,
            });
        }

        // 1. A^T * A の固有値問題を解く
        let ata = &self.transpose() * self;
        let eigen_decomp = ata.eigen_decomposition()?;
        let eigenvalues = eigen_decomp.eigenvalues;
        let v_raw = eigen_decomp.eigenvectors; // 固有値分解直後のV

        // 2. 固有値を降順にソートし、対応する固有ベクトルも並べ替える
        let mut pairs: Vec<_> = eigenvalues.into_iter().zip(0..v_raw.cols).collect();
        pairs.sort_by(|a, b| {
            b.0.re
                .partial_cmp(&a.0.re)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut sorted_v = Matrix::zeros(v_raw.rows, v_raw.cols);
        let mut sigma_vec = Vec::with_capacity(self.cols);

        for (i, (eigenval, original_idx)) in pairs.iter().enumerate() {
            let v_col = v_raw.col(*original_idx).unwrap();
            sorted_v.set_col(i, &v_col).ok()?; // この時点ではまだ正規化も直交化も不完全
            sigma_vec.push(eigenval.re.sqrt());
        }

        // ★★★ 変更点 ① ★★★
        // ソート後のV行列の直交性が崩れている可能性があるため、QR分解で直交性を回復させる
        // v_final は V^T ではなく V なので注意
        let (v_final, _) = sorted_v.qr_decomposition()?;

        // 3. 特異値ベクトル Σ と 左特異ベクトル U を計算する
        let sigma = Vector::new(sigma_vec);
        let mut u = Matrix::zeros(self.rows, self.rows);

        for i in 0..self.cols {
            let sigma_i = sigma[i];
            let v_i = v_final.col(i).unwrap();

            if sigma_i.abs() < 1e-14 {
                // 特異値がゼロの場合、グラム・シュミット法でUの基底を補充する
                // (この部分は元の堅牢な実装をそのまま利用)
                let mut new_basis_found = false;
                for k in 0..self.rows {
                    let mut candidate_vec = Vector::zeros(self.rows);
                    candidate_vec[k] = 1.0;
                    for j in 0..i {
                        let u_j = u.col(j).unwrap();
                        let proj = u_j.dot(&candidate_vec);
                        candidate_vec = &candidate_vec - &(&u_j * proj);
                    }
                    let norm = candidate_vec.norm();
                    if norm > 1e-12 {
                        u.set_col(i, &(&candidate_vec * (1.0 / norm))).unwrap();
                        new_basis_found = true;
                        break;
                    }
                }
                if !new_basis_found {
                    u.set_col(i, &Vector::zeros(self.rows)).unwrap();
                }
            } else {
                // u_i = A * v_i / sigma_i
                let u_i = self * &v_i * (1.0 / sigma_i);
                u.set_col(i, &u_i).unwrap();
            }
        }

        // ★★★ 変更点 ② ★★★
        // 計算されたU行列の直交性が崩れている可能性があるため、QR分解で直交性を回復させる
        let (u_final, _) = if self.rows == self.cols {
            // Aが正方行列の場合、uは正方行列なのでそのままQR分解
            u.qr_decomposition()?
        } else {
            // Aが縦長行列(m > n)の場合、uはm x mだが最初のn列しか計算していない。
            // そのため、計算済みのn列部分だけを直交化し、残りのm-n列を補完する必要がある。
            // ここでは簡単のため、m x n のU行列を返すことを想定し、
            // Uの最初のn列部分でQR分解を行う。
            // (もしm x mのUが必要な場合は、残りの列を埋める処理が必要)
            let u_sub = u.submatrix(0, self.rows, 0, self.cols);
            u_sub.qr_decomposition()?
        };

        Some(SVD {
            u: u_final,
            sigma,
            v: v_final,
        })
    }
}

// 1. トレイトの定義を変更
pub trait DisplayElement {
    fn to_formatted_string(&self) -> String;
}

// 2. マクロも String を返すように修正
macro_rules! impl_default_display_element {
    ($($t:ty),*) => {
        $(
            impl DisplayElement for $t {
                fn to_formatted_string(&self) -> String {
                    self.to_string()
                }
            }
        )*
    };
}
const DISPLAY_PRECISION: i32 = 4;

impl DisplayElement for f64 {
    fn to_formatted_string(&self) -> String {
        let factor = 10.0_f64.powi(DISPLAY_PRECISION);
        let mut rounded_val = (self * factor).round() / factor;

        if rounded_val == 0.0 {
            rounded_val = 0.0;
        }

        rounded_val.to_string()
    }
}

// 4. マクロを呼び出して、他の型に対する実装を自動生成
impl_default_display_element!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, String, &str, bool, char
);
impl<T: Scalar + DisplayElement> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // --- 1. 事前計算パス：全要素のフォーマット済み文字列を先に作ってしまう ---
        let formatted_strings: Vec<String> = self
            .data
            .iter()
            .map(|val| val.to_formatted_string())
            .collect();

        // --- 2. 文字列から最大幅を計算 ---
        let max_width = formatted_strings.iter().map(|s| s.len()).max().unwrap_or(0);

        // --- 3. 書き込みパス ---
        writeln!(f, "rows: {}, cols: {}", self.rows, self.cols)?;
        for r in 0..self.rows {
            for c in 0..self.cols {
                // 事前計算した文字列を取得
                let s = &formatted_strings[r * self.cols + c];
                // 右揃えで表示
                write!(f, "{s:>max_width$}")?;
                // セパレータ（タブ文字）
                if c < self.cols - 1 {
                    write!(f, "\t")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// --- 添字アクセス (T: Scalarであれば可能) ---
impl<T: Scalar> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}

impl<T: Scalar> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.cols + index.1]
    }
}

impl<T: Ring> Neg for &Matrix<T> {
    type Output = Matrix<T>;
    fn neg(self) -> Self::Output {
        self.checked_neg()
    }
}

impl<T: Ring> Neg for Matrix<T> {
    type Output = Matrix<T>;
    fn neg(self) -> Self::Output {
        -&self
    }
}

impl<'b, T: Ring> Add<&'b Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: &'b Matrix<T>) -> Self::Output {
        let result = self.checked_add(rhs);
        match result {
            Ok(mat) => mat,
            Err(e) => panic!("Matrix addition failed: {e}"),
        }
    }
}

impl<'b, T: Ring> Sub<&'b Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: &'b Matrix<T>) -> Self::Output {
        let result = self.checked_sub(rhs);
        match result {
            Ok(mat) => mat,
            Err(e) => panic!("Matrix subtraction failed: {e}"),
        }
    }
}

impl<'b, T: Ring> Mul<&'b Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        let result = self.checked_mul(rhs);
        match result {
            Ok(mat) => mat,
            Err(e) => panic!("Matrix multiplication failed: {e}"),
        }
    }
}

impl<'b, T: Ring> Mul<&'b Vector<T>> for &Matrix<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: &'b Vector<T>) -> Self::Output {
        let result = self.checked_mul_vector(rhs);
        match result {
            Ok(vec) => vec,
            Err(e) => panic!("Matrix-vector multiplication failed: {e}"),
        }
    }
}

impl<'b, T: Ring> Mul<&'b T> for &Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: &'b T) -> Self::Output {
        let data = self.data.iter().map(|x| x.clone() * rhs.clone()).collect();
        Matrix::new(self.rows, self.cols, data).unwrap()
    }
}

impl<'b, T: Ring> Add<&'b T> for &Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: &'b T) -> Self::Output {
        let data = self.data.iter().map(|x| x.clone() + rhs.clone()).collect();
        Matrix::new(self.rows, self.cols, data).unwrap()
    }
}

impl<'b, T: Ring> Sub<&'b T> for &Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: &'b T) -> Self::Output {
        let data = self.data.iter().map(|x| x.clone() - rhs.clone()).collect();
        Matrix::new(self.rows, self.cols, data).unwrap()
    }
}

impl<T: Ring> Add<Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: Matrix<T>) -> Self::Output {
        self + &rhs
    }
}

impl<T: Ring> Sub<Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        self - &rhs
    }
}

impl<T: Ring> Mul<Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T: Ring> Mul<Vector<T>> for &Matrix<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T: Ring> Mul<T> for &Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        self * &rhs
    }
}

impl<T: Ring> Add<T> for &Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: T) -> Self::Output {
        self + &rhs
    }
}

impl<T: Ring> Sub<T> for &Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: T) -> Self::Output {
        self - &rhs
    }
}

impl<'b, T: Ring> Add<&'b Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: &'b Matrix<T>) -> Self::Output {
        &self + rhs
    }
}

impl<'b, T: Ring> Sub<&'b Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: &'b Matrix<T>) -> Self::Output {
        &self - rhs
    }
}

impl<'b, T: Ring> Mul<&'b Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        &self * rhs
    }
}

impl<'b, T: Ring> Mul<&'b Vector<T>> for Matrix<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: &'b Vector<T>) -> Self::Output {
        &self * rhs
    }
}

impl<'b, T: Ring> Mul<&'b T> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: &'b T) -> Self::Output {
        &self * rhs
    }
}

impl<'b, T: Ring> Add<&'b T> for Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: &'b T) -> Self::Output {
        &self + rhs
    }
}

impl<'b, T: Ring> Sub<&'b T> for Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: &'b T) -> Self::Output {
        &self - rhs
    }
}

impl<T: Ring> Add<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: Matrix<T>) -> Self::Output {
        &self + &rhs
    }
}

impl<T: Ring> Sub<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        &self - &rhs
    }
}

impl<T: Ring> Mul<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Ring> Mul<Vector<T>> for Matrix<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Ring> Mul<T> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Ring> Add<T> for Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: T) -> Self::Output {
        &self + &rhs
    }
}

impl<T: Ring> Sub<T> for Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: T) -> Self::Output {
        &self - &rhs
    }
}
