use crate::{LinalgError, Matrix, Result, Ring, Vector};

#[cfg(test)]
mod tests;

impl<T: Ring> Vector<T> {
    pub fn zeros(dim: usize) -> Self {
        Self::new(vec![T::zero(); dim])
    }
    pub fn ones(dim: usize) -> Self {
        Self::new(vec![T::one(); dim])
    }

    pub fn checked_neg(&self) -> Vector<T> {
        let data = self.data.iter().map(|x| -x.clone()).collect();
        Vector::new(data)
    }

    pub fn checked_add(&self, rhs: &Vector<T>) -> Result<Vector<T>> {
        if self.data.len() != rhs.data.len() {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{}", self.data.len()),
                found: format!("{}", rhs.data.len()),
            });
        }
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();
        Ok(Vector::new(data))
    }

    pub fn checked_sub(&self, rhs: &Vector<T>) -> Result<Vector<T>> {
        if self.data.len() != rhs.data.len() {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{}", self.data.len()),
                found: format!("{}", rhs.data.len()),
            });
        }
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a.clone() - b.clone())
            .collect();
        Ok(Vector::new(data))
    }

    pub fn hadamard_product(&self, rhs: &Vector<T>) -> Result<Vector<T>> {
        if self.data.len() != rhs.data.len() {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{}", self.data.len()),
                found: format!("{}", rhs.data.len()),
            });
        }
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a.clone() * b.clone())
            .collect();
        Ok(Vector::new(data))
    }

    // 2つの解釈をサポート:
    // - 外積風: self (m) * rhs (1 x n) -> (m x n)
    // - 行ベクトル×行列: self (1 x m) * rhs (m x n) -> (1 x n)
    pub fn checked_mul_matrix(&self, rhs: &Matrix<T>) -> Result<Matrix<T>> {
        let m = self.data.len();
        // 外積風: rhs が 1 行の場合、各列と self の要素で (m x rhs.cols) を作る
        if rhs.rows == 1 {
            let mut out = Matrix::zeros(m, rhs.cols);
            for i in 0..m {
                for j in 0..rhs.cols {
                    out[(i, j)] = self.data[i].clone() * rhs[(0, j)].clone();
                }
            }
            return Ok(out);
        }

        // 行ベクトル×行列: self.len == rhs.rows のとき従来の (1 x n)
        if m == rhs.rows {
            let mut data = Vec::with_capacity(rhs.cols);
            for j in 0..rhs.cols {
                let mut acc = None;
                for i in 0..rhs.rows {
                    let val = self.data[i].clone() * rhs[(i, j)].clone();
                    acc = Some(match acc {
                        Some(x) => x + val,
                        None => val,
                    });
                }
                data.push(acc.unwrap());
            }
            return Matrix::new(1, rhs.cols, data);
        }

        Err(LinalgError::DimensionMismatch {
            expected: format!("rhs.rows == 1 or {m}"),
            found: format!("{}", rhs.rows),
        })
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

    pub fn dot(&self, other: &Self) -> T {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a.clone() * b.clone())
            .sum()
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
        let data = vec![b * f - c * e, c * d - a * f, a * e - b * d];
        Ok(Vector::new(data))
    }
}
