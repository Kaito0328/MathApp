use crate::{LinalgError, Result, Scalar};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction {
    Left,
    Right,
}

// サブモジュールを宣言
mod algebra;
mod core;
pub mod numerical;
mod ops;

// パブリックな再エクスポート
pub use ops::DisplayElement;
