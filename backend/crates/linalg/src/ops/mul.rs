use crate::{Matrix, Vector};
use num_traits::{One, Zero};
use std::ops::Mul;

// Matrix<T> * Matrix<T>
impl<T> Mul for Matrix<T>
where
    T: Mul<Output = T> + std::iter::Sum + Copy + Zero,
{
    type Output = Matrix<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

// &Matrix<T> * &Matrix<T>
impl<T> Mul<&Matrix<T>> for &Matrix<T>
where
    T: Mul<Output = T> + std::iter::Sum + Copy + Zero,
{
    type Output = Matrix<T>;
    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        unimplemented!()
    }
}

// Matrix<T> * Vector<T>
impl<T> Mul<Vector<T>> for Matrix<T>
where
    T: Mul<Output = T> + std::iter::Sum + Copy + Zero,
{
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        unimplemented!()
    }
}

// &Matrix<T> * &Vector<T>
impl<T> Mul<&Vector<T>> for &Matrix<T>
where
    T: Mul<Output = T> + std::iter::Sum + Copy + Zero,
{
    type Output = Vector<T>;
    fn mul(self, rhs: &Vector<T>) -> Self::Output {
        unimplemented!()
    }
}

// Vector<T> * Vector<T> (外積または要素ごとの積)
impl<T> Mul for Vector<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

// &Vector<T> * &Vector<T>
impl<T> Mul<&Vector<T>> for &Vector<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn mul(self, rhs: &Vector<T>) -> Self::Output {
        unimplemented!()
    }
}

// Matrix<T> * T (スカラー倍)
impl<T> Mul<T> for Matrix<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        unimplemented!()
    }
}

// &Matrix<T> * T
impl<T> Mul<T> for &Matrix<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        unimplemented!()
    }
}

// Vector<T> * T (スカラー倍)
impl<T> Mul<T> for Vector<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        unimplemented!()
    }
}

// &Vector<T> * T
impl<T> Mul<T> for &Vector<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        unimplemented!()
    }
}
