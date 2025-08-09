use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

use super::*;

#[cfg(test)]
mod tests;

impl<T: Scalar> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Scalar> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<'a, T: Scalar> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<T: Ring> Neg for &Vector<T> {
    type Output = Vector<T>;
    fn neg(self) -> Self::Output {
        self.checked_neg()
    }
}

impl<T: Ring> Neg for Vector<T> {
    type Output = Vector<T>;
    fn neg(self) -> Self::Output {
        -&self
    }
}

impl<'b, T: Ring> Add<&'b Vector<T>> for &Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: &'b Vector<T>) -> Self::Output {
        let result = self.checked_add(rhs);
        match result {
            Ok(vec) => vec,
            Err(e) => panic!("Vector addition failed: {e}"),
        }
    }
}

impl<'b, T: Ring> Sub<&'b Vector<T>> for &Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: &'b Vector<T>) -> Self::Output {
        let result = self.checked_sub(rhs);
        match result {
            Ok(vec) => vec,
            Err(e) => panic!("Vector subtraction failed: {e}"),
        }
    }
}

impl<'b, T: Ring> Mul<&'b Vector<T>> for &Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: &'b Vector<T>) -> Self::Output {
        let result = self.hadamard_product(rhs);
        match result {
            Ok(vec) => vec,
            Err(e) => panic!("Vector multiplication failed: {e}"),
        }
    }
}

impl<'b, T: Ring> Mul<&'b Matrix<T>> for &Vector<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        let result = self.checked_mul_matrix(rhs);
        match result {
            Ok(mat) => mat,
            Err(e) => panic!("Vector-matrix multiplication failed: {e}"),
        }
    }
}

impl<'b, T: Ring> Add<&'b T> for &Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: &'b T) -> Self::Output {
        let data = self.data.iter().map(|x| x.clone() + rhs.clone()).collect();
        Vector::new(data)
    }
}

impl<'b, T: Ring> Sub<&'b T> for &Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: &'b T) -> Self::Output {
        let data = self.data.iter().map(|x| x.clone() - rhs.clone()).collect();
        Vector::new(data)
    }
}

impl<'b, T: Ring> Mul<&'b T> for &Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: &'b T) -> Self::Output {
        let data = self.data.iter().map(|x| x.clone() * rhs.clone()).collect();
        Vector::new(data)
    }
}

impl<'b, T: Ring> Add<&'b Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: &'b Vector<T>) -> Self::Output {
        &self + rhs
    }
}

impl<'b, T: Ring> Sub<&'b Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: &'b Vector<T>) -> Self::Output {
        &self - rhs
    }
}

impl<'b, T: Ring> Mul<&'b Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: &'b Vector<T>) -> Self::Output {
        &self * rhs
    }
}

impl<'b, T: Ring> Mul<&'b Matrix<T>> for Vector<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        &self * rhs
    }
}

impl<'b, T: Ring> Add<&'b T> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: &'b T) -> Self::Output {
        &self + rhs
    }
}

impl<'b, T: Ring> Sub<&'b T> for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: &'b T) -> Self::Output {
        &self - rhs
    }
}

impl<'b, T: Ring> Mul<&'b T> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: &'b T) -> Self::Output {
        &self * rhs
    }
}

impl<T: Ring> Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        &self + &rhs
    }
}

impl<T: Ring> Sub<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        &self - &rhs
    }
}

impl<T: Ring> Mul<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Ring> Mul<Matrix<T>> for Vector<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Ring> Add<T> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: T) -> Self::Output {
        &self + &rhs
    }
}

impl<T: Ring> Sub<T> for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: T) -> Self::Output {
        &self - &rhs
    }
}

impl<T: Ring> Mul<T> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Ring> Add<Vector<T>> for &Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        self + &rhs
    }
}

impl<T: Ring> Sub<Vector<T>> for &Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        self - &rhs
    }
}

impl<T: Ring> Mul<Vector<T>> for &Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T: Ring> Mul<Matrix<T>> for &Vector<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T: Ring> Add<T> for &Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: T) -> Self::Output {
        self + &rhs
    }
}

impl<T: Ring> Sub<T> for &Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: T) -> Self::Output {
        self - &rhs
    }
}

impl<T: Ring> Mul<T> for &Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        self * &rhs
    }
}
