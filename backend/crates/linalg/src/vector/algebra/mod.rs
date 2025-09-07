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

    /// 要素の総和を返す
    pub fn sum(&self) -> T {
        self.data
            .iter()
            .cloned()
            .fold(T::zero(), |acc, x| acc + x)
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
                let mut acc = T::zero();
                for i in 0..rhs.rows {
                    let val = self.data[i].clone() * rhs[(i, j)].clone();
                    acc = acc + val;
                }
                data.push(acc);
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

    /// 素朴な 1D 畳み込み（時間領域）。
    pub fn conv(&self, other: &Self) -> Vector<T> {
        let mut result = vec![T::zero(); self.data.len() + other.data.len() - 1];
        for (i, a) in self.data.iter().enumerate() {
            for (j, b) in other.data.iter().enumerate() {
                result[i + j] = result[i + j].clone() + a.clone() * b.clone();
            }
        }
        Vector::new(result)
    }

    // f64専用APIは Vector<f64> impl に分離

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

// f64 専用の畳み込み API
impl Vector<f64> {
    /// 素朴な 1D 畳み込み（時間領域）。
    pub fn conv_simple(&self, other: &Self) -> Vector<f64> {
        let mut y = vec![0.0f64; self.data.len() + other.data.len() - 1];
        for (i, &a) in self.data.iter().enumerate() {
            for (j, &b) in other.data.iter().enumerate() {
                y[i + j] += a * b;
            }
        }
        Vector::new(y)
    }

    /// FFT 畳み込み
    pub fn conv_fft(&self, other: &Self) -> crate::Result<Vector<f64>> {
        use num_complex::Complex;
        let x = &self.data;
        let h = &other.data;
        let n = x.len() + h.len() - 1;
        let mut x_pad: Vec<Complex<f64>> = x.iter().map(|&v| Complex::new(v, 0.0)).collect();
        let mut h_pad: Vec<Complex<f64>> = h.iter().map(|&v| Complex::new(v, 0.0)).collect();
        x_pad.resize(n, Complex::new(0.0, 0.0));
        h_pad.resize(n, Complex::new(0.0, 0.0));
    let x_fft = fft_core::dft(&x_pad);
    let h_fft = fft_core::dft(&h_pad);
        let y_fft: Vec<Complex<f64>> = x_fft.into_iter().zip(h_fft).map(|(a, b)| a * b).collect();
    let y = fft_core::ift(&y_fft);
    Ok(Vector::new(y.into_iter().map(|c| c.re).collect()))
    }

    /// サイズにより素朴/FFT を自動切替。
    pub fn conv_auto(&self, other: &Self) -> crate::Result<Vector<f64>> {
        let work = self.data.len() * other.data.len();
        if work <= 2048 {
            Ok(self.conv_simple(other))
        } else {
            self.conv_fft(other)
        }
    }
}
