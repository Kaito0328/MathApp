use crate::{Matrix, Ring, Vector};
use std::{clone, ops::Mul};

// --- Matrix * Matrix ---

// &Matrix<T> * &Matrix<T> (中心となる実装)
impl<'a, 'b, T: Ring> Mul<&'b Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("Matrix dimensions are incompatible for multiplication.");
        }
        let mut data = vec![T::zero(); self.rows * rhs.cols];
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let sum = (0..self.cols)
                    .map(|k| self[(i, k)].clone() * rhs[(k, j)].clone())
                    .sum();
                data[i * rhs.cols + j] = sum;
            }
        }
        Matrix::new(self.rows, rhs.cols, data).unwrap()
    }
}

// 他の3パターンは上記の実装を呼び出す
impl<T: Ring> Mul<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        &self * &rhs
    }
}
impl<'a, T: Ring> Mul<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        self * &rhs
    }
}
impl<'b, T: Ring> Mul<&'b Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        &self * rhs
    }
}

// --- Matrix * Vector ---

// &Matrix<T> * &Vector<T> (中心となる実装)
impl<'a, 'b, T: Ring> Mul<&'b Vector<T>> for &'a Matrix<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: &'b Vector<T>) -> Self::Output {
        unimplemented!()
    }
}

// 他の3パターンも同様に実装
impl<T: Ring> Mul<Vector<T>> for Matrix<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        &self * &rhs
    }
}
// ...など

// --- Vector * Vector (要素ごとの積) ---

// Vector<T> * Vector<T> は外積を返す
impl<'a, 'b, T: Ring> Mul<&'b Vector<T>> for &'a Vector<T> {
    // 返り値の型を行列に変更
    type Output = Matrix<T>;
    fn mul(self, rhs: &'b Vector<T>) -> Self::Output {
        // 新設したメソッドを呼び出す
        self.outer_product(rhs)
    }
}
// ... 他の3パターンも同様に実装 ...

// --- Matrix * Scalar ---

// &Matrix<T> * T (中心となる実装)
impl<'a, T: Ring> Mul<T> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        let data = self
            .data
            .iter()
            .map(|val| val.clone() * rhs.clone())
            .collect();
        Matrix::new(self.rows, self.cols, data).unwrap()
    }
}

// Matrix<T> * T
impl<T: Ring> Mul<T> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        &self * rhs
    }
}

// --- Vector * Scalar ---

// &Vector<T> * T (中心となる実装)
impl<'a, T: Ring> Mul<T> for &'a Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        let data = self
            .data
            .iter()
            .map(|val| val.clone() * rhs.clone())
            .collect();
        Vector::new(data)
    }
}

// Vector<T> * T
impl<T: Ring> Mul<T> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        &self * rhs
    }
}

// 注意: T * Matrix<T> や T * Vector<T> のような実装は孤児のルールにより定義できません。
// スカラー倍を行いたい場合は、`matrix * scalar` のように必ず右側にスカラーを記述してください。
