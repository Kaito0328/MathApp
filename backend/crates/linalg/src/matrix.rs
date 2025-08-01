use crate::{Field, LinalgError, Result, Ring, Scalar, Vector};
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

/// 固有値と固有ベクトルのペアを格納するジェネリックな構造体
#[derive(Debug, PartialEq)]
pub struct EigenDecomposition<T: Scalar> {
    pub eigenvalues: Vec<T>,
    pub eigenvectors: Vec<Vector<T>>,
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
