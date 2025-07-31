use crate::{Matrix, Vector};
use std::ops::Sub;

// Matrix - Matrix
impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

// Vector - Vector
impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
