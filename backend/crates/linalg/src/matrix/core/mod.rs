use super::{LinalgError, Matrix, Result, Scalar};
pub use crate::vector::Vector;

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

    pub fn partial_row(
        &self,
        row_idx: usize,
        start_col: usize,
        end_col: usize,
    ) -> Result<Vector<T>> {
        if row_idx >= self.rows {
            return Err(LinalgError::IndexOutOfBounds {
                index: row_idx,
                size: self.rows,
            });
        }
        if end_col > self.cols || start_col > end_col {
            return Err(LinalgError::InvalidDimension {
                dim: (end_col),
                text: ("Invalid column range for row extraction".to_string()),
            });
        }

        let data = (start_col..end_col)
            .map(|c| self[(row_idx, c)].clone())
            .collect();

        Ok(Vector::new(data))
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

#[cfg(test)]
mod tests;
