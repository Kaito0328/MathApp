use crate::{Matrix, Vector};
use std::ops::Sub;

// Matrix<T> - Matrix<T>
impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

// &Matrix<T> - &Matrix<T>
impl<T> Sub<&Matrix<T>> for &Matrix<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn sub(self, rhs: &Matrix<T>) -> Self::Output {
        unimplemented!()
    }
}

// Matrix<T> - &Matrix<T>
impl<T> Sub<&Matrix<T>> for Matrix<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn sub(self, rhs: &Matrix<T>) -> Self::Output {
        unimplemented!()
    }
}

// &Matrix<T> - Matrix<T>
impl<T> Sub<Matrix<T>> for &Matrix<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        unimplemented!()
    }
}

// Vector<T> - Vector<T>
impl<T> Sub for Vector<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

// &Vector<T> - &Vector<T>
impl<T> Sub<&Vector<T>> for &Vector<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        unimplemented!()
    }
}

// Vector<T> - &Vector<T>
impl<T> Sub<&Vector<T>> for Vector<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        unimplemented!()
    }
}

// &Vector<T> - Vector<T>
impl<T> Sub<Vector<T>> for &Vector<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        unimplemented!()
    }
}

// Matrix<T> - T (スカラー減算)
impl<T> Sub<T> for Matrix<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn sub(self, rhs: T) -> Self::Output {
        unimplemented!()
    }
}

// &Matrix<T> - T
impl<T> Sub<T> for &Matrix<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn sub(self, rhs: T) -> Self::Output {
        unimplemented!()
    }
}
// Vector<T> - T (スカラー減算)
impl<T> Sub<T> for Vector<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn sub(self, rhs: T) -> Self::Output {
        unimplemented!()
    }
}

// &Vector<T> - T
impl<T> Sub<T> for &Vector<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn sub(self, rhs: T) -> Self::Output {
        unimplemented!()
    }
}
// 単項マイナス演算子
impl<T> std::ops::Neg for Matrix<T>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn neg(self) -> Self::Output {
        unimplemented!()
    }
}

impl<T> std::ops::Neg for &Matrix<T>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn neg(self) -> Self::Output {
        unimplemented!()
    }
}

impl<T> std::ops::Neg for Vector<T>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn neg(self) -> Self::Output {
        unimplemented!()
    }
}

impl<T> std::ops::Neg for &Vector<T>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn neg(self) -> Self::Output {
        unimplemented!()
    }
}
