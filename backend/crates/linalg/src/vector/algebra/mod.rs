use super::{LinalgError, Matrix, Result, Ring, Vector};

#[cfg(test)]
mod tests;

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
