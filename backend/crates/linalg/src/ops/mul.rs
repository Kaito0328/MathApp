use crate::{Matrix, Vector};
use std::ops::Mul;

// Matrix * Matrix
impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

// Matrix * Vector
impl Mul<Vector> for Matrix {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        unimplemented!()
    }
}

// スカラー倍 (f64 * Matrix)
impl Mul<Matrix> for f64 {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        unimplemented!()
    }
}
