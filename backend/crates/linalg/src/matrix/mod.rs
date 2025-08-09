use crate::{LinalgError, Result, Scalar};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T: Scalar = f64> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

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
