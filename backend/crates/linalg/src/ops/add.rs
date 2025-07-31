use crate::{Matrix, Vector};
use std::ops::Add;

// Matrix<T> + Matrix<T>
impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn add(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

// &Matrix<T> + &Matrix<T>
impl<T> Add<&Matrix<T>> for &Matrix<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        unimplemented!()
    }
}

// Matrix<T> + &Matrix<T>
impl<T> Add<&Matrix<T>> for Matrix<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        unimplemented!()
    }
}

// &Matrix<T> + Matrix<T>
impl<T> Add<Matrix<T>> for &Matrix<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn add(self, rhs: Matrix<T>) -> Self::Output {
        unimplemented!()
    }
}

// Vector<T> + Vector<T>
impl<T> Add for Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

// &Vector<T> + &Vector<T>
impl<T> Add<&Vector<T>> for &Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn add(self, rhs: &Vector<T>) -> Self::Output {
        unimplemented!()
    }
}

// Vector<T> + &Vector<T>
impl<T> Add<&Vector<T>> for Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn add(self, rhs: &Vector<T>) -> Self::Output {
        unimplemented!()
    }
}

// &Vector<T> + Vector<T>
impl<T> Add<Vector<T>> for &Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        unimplemented!()
    }
}

// Matrix<T> + T (スカラー加算)
impl<T> Add<T> for Matrix<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn add(self, rhs: T) -> Self::Output {
        unimplemented!()
    }
}

// &Matrix<T> + T
impl<T> Add<T> for &Matrix<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Matrix<T>;
    fn add(self, rhs: T) -> Self::Output {
        unimplemented!()
    }
}

// Vector<T> + T (スカラー加算)
impl<T> Add<T> for Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn add(self, rhs: T) -> Self::Output {
        unimplemented!()
    }
}

// &Vector<T> + T
impl<T> Add<T> for &Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn add(self, rhs: T) -> Self::Output {
        unimplemented!()
    }
}
