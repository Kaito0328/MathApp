use crate::vector::Vector;
use crate::{LinalgError, Matrix, Result, Ring};

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

    pub fn form_diag(rows: usize, cols: usize, diag_elements: &Vector<T>) -> Self {
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
