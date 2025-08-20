use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

use super::super::Matrix;
use super::*;

#[cfg(test)]
mod tests;

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

// マクロで所有/参照の派生を一括実装
impl_ops_by_ref_variants!(Vector<T>, Add, add, crate::Ring);
impl_ops_by_ref_variants!(Vector<T>, Sub, sub, crate::Ring);
impl_ops_by_ref_variants!(Vector<T>, Mul, mul, crate::Ring); // ベクトル同士（Hadamard）

// ベクトル×行列（結果は行列）
impl_mixed_ops_by_ref_variants!(Vector<T>, Matrix<T>, Matrix<T>, Mul, mul, crate::Ring);

// スカラー右辺（結果はベクトル）
impl_scalar_rhs_by_ref_variants!(Vector<T>, Add, add, crate::Ring);
impl_scalar_rhs_by_ref_variants!(Vector<T>, Sub, sub, crate::Ring);
impl_scalar_rhs_by_ref_variants!(Vector<T>, Mul, mul, crate::Ring);

// Display トレイト実装（Matrix と同様の整形ロジックを利用）
use crate::matrix::DisplayElement;
use ::core::fmt;

impl<T: crate::Scalar + DisplayElement> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self
            .data
            .iter()
            .map(|val| val.to_formatted_string())
            .collect();
        write!(f, "dim: {} [", self.dim())?;
        for (i, s) in parts.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{s}")?;
        }
        write!(f, "]")
    }
}
