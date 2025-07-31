use crate::{Matrix, Vector};
use std::ops::Add; // `crate::`でライブラリのルートからインポート

// Matrix + Matrix
impl Add for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

// Vector + Vector
impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
