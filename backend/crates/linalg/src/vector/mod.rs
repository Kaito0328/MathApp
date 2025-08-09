use crate::{LinalgError, Matrix, Result, Ring, Scalar};

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<T: Scalar = f64> {
    pub data: Vec<T>,
}

// サブモジュールを宣言
mod algebra;
mod core;
pub mod numerical;
mod ops;
