use crate::{LinalgError, Matrix, Result, Ring, Scalar};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vector<T: Scalar = f64> {
    pub data: Vec<T>,
}

// サブモジュールを宣言
mod algebra;
mod core;
pub mod numerical;
mod ops;

// 標準トレイト実装（利便性向上）
impl<T: Scalar> AsRef<[T]> for Vector<T> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}
impl<T: Scalar> AsMut<[T]> for Vector<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}
impl<T: Scalar> From<Vec<T>> for Vector<T> {
    fn from(v: Vec<T>) -> Self {
        Vector { data: v }
    }
}
