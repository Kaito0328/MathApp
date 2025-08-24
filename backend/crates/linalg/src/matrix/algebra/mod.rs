use super::Matrix;
use crate::{LinalgError, Result, Ring, Vector};

impl<T: Ring> Matrix<T> {
	pub fn checked_add(&self, rhs: &Matrix<T>) -> Result<Matrix<T>> {
		if self.rows != rhs.rows || self.cols != rhs.cols {
			return Err(LinalgError::DimensionMismatch {
				expected: format!("{}x{}", self.rows, self.cols),
				found: format!("{}x{}", rhs.rows, rhs.cols),
			});
		}
		let data = self
			.data
			.iter()
			.zip(rhs.data.iter())
			.map(|(a, b)| a.clone() + b.clone())
			.collect();
		Matrix::new(self.rows, self.cols, data)
	}

	pub fn checked_sub(&self, rhs: &Matrix<T>) -> Result<Matrix<T>> {
		if self.rows != rhs.rows || self.cols != rhs.cols {
			return Err(LinalgError::DimensionMismatch {
				expected: format!("{}x{}", self.rows, self.cols),
				found: format!("{}x{}", rhs.rows, rhs.cols),
			});
		}
		let data = self
			.data
			.iter()
			.zip(rhs.data.iter())
			.map(|(a, b)| a.clone() - b.clone())
			.collect();
		Matrix::new(self.rows, self.cols, data)
	}

	pub fn checked_mul(&self, rhs: &Matrix<T>) -> Result<Matrix<T>> {
		if self.cols != rhs.rows {
			return Err(LinalgError::DimensionMismatch {
				expected: format!("left cols {} == right rows {}", self.cols, rhs.rows),
				found: format!("{} vs {}", self.cols, rhs.rows),
			});
		}
		let mut data = Vec::with_capacity(self.rows * rhs.cols);
		for i in 0..self.rows {
			for j in 0..rhs.cols {
				let mut acc = T::zero();
				for k in 0..self.cols {
					let val = self[(i, k)].clone() * rhs[(k, j)].clone();
					acc = acc + val;
				}
				data.push(acc);
			}
		}
		Matrix::new(self.rows, rhs.cols, data)
	}

	pub fn checked_mul_vector(&self, rhs: &Vector<T>) -> Result<Vector<T>> {
		if rhs.data.len() != self.cols {
			return Err(LinalgError::DimensionMismatch {
				expected: format!("vector dim {}", self.cols),
				found: format!("{}", rhs.data.len()),
			});
		}
		let mut out = Vec::with_capacity(self.rows);
		for i in 0..self.rows {
			let mut acc = T::zero();
			for k in 0..self.cols {
				let val = self[(i, k)].clone() * rhs.data[k].clone();
				acc = acc + val;
			}
			out.push(acc);
		}
		Ok(Vector::new(out))
	}
}

pub mod field;
pub mod ring;

#[cfg(test)]
mod tests;
