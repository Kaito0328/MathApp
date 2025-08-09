use crate::{Ring, Vector};
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

// 添字アクセス
impl<T: crate::Scalar> Index<(usize, usize)> for super::Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}

impl<T: crate::Scalar> IndexMut<(usize, usize)> for super::Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.cols + index.1]
    }
}

// 負号演算子
impl<T: Ring> Neg for &super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn neg(self) -> Self::Output {
        self.checked_neg()
    }
}

impl<T: Ring> Neg for super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn neg(self) -> Self::Output {
        -&self
    }
}

// 行列同士の演算
impl<'b, T: Ring> Add<&'b super::Matrix<T>> for &super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn add(self, rhs: &'b super::Matrix<T>) -> Self::Output {
        let result = self.checked_add(rhs);
        match result {
            Ok(mat) => mat,
            Err(e) => panic!("Matrix addition failed: {e}"),
        }
    }
}

impl<'b, T: Ring> Sub<&'b super::Matrix<T>> for &super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn sub(self, rhs: &'b super::Matrix<T>) -> Self::Output {
        let result = self.checked_sub(rhs);
        match result {
            Ok(mat) => mat,
            Err(e) => panic!("Matrix subtraction failed: {e}"),
        }
    }
}

impl<'b, T: Ring> Mul<&'b super::Matrix<T>> for &super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn mul(self, rhs: &'b super::Matrix<T>) -> Self::Output {
        let result = self.checked_mul(rhs);
        match result {
            Ok(mat) => mat,
            Err(e) => panic!("Matrix multiplication failed: {e}"),
        }
    }
}

// 行列とベクトルの演算
impl<'b, T: Ring> Mul<&'b Vector<T>> for &super::Matrix<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: &'b Vector<T>) -> Self::Output {
        let result = self.checked_mul_vector(rhs);
        match result {
            Ok(vec) => vec,
            Err(e) => panic!("Matrix-vector multiplication failed: {e}"),
        }
    }
}

// 行列とスカラーの演算
impl<'b, T: Ring> Mul<&'b T> for &super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn mul(self, rhs: &'b T) -> Self::Output {
        let data = self.data.iter().map(|x| x.clone() * rhs.clone()).collect();
        super::Matrix::new(self.rows, self.cols, data).unwrap()
    }
}

impl<'b, T: Ring> Add<&'b T> for &super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn add(self, rhs: &'b T) -> Self::Output {
        let data = self.data.iter().map(|x| x.clone() + rhs.clone()).collect();
        super::Matrix::new(self.rows, self.cols, data).unwrap()
    }
}

impl<'b, T: Ring> Sub<&'b T> for &super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn sub(self, rhs: &'b T) -> Self::Output {
        let data = self.data.iter().map(|x| x.clone() - rhs.clone()).collect();
        super::Matrix::new(self.rows, self.cols, data).unwrap()
    }
}

// 所有権を取る演算子のオーバーロード
impl<T: Ring> Add<super::Matrix<T>> for &super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn add(self, rhs: super::Matrix<T>) -> Self::Output {
        self + &rhs
    }
}

impl<T: Ring> Sub<super::Matrix<T>> for &super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn sub(self, rhs: super::Matrix<T>) -> Self::Output {
        self - &rhs
    }
}

impl<T: Ring> Mul<super::Matrix<T>> for &super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn mul(self, rhs: super::Matrix<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T: Ring> Mul<Vector<T>> for &super::Matrix<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T: Ring> Mul<T> for &super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        self * &rhs
    }
}

impl<T: Ring> Add<T> for &super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn add(self, rhs: T) -> Self::Output {
        self + &rhs
    }
}

impl<T: Ring> Sub<T> for &super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn sub(self, rhs: T) -> Self::Output {
        self - &rhs
    }
}

// 値型に対する演算子
impl<'b, T: Ring> Add<&'b super::Matrix<T>> for super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn add(self, rhs: &'b super::Matrix<T>) -> Self::Output {
        &self + rhs
    }
}

impl<'b, T: Ring> Sub<&'b super::Matrix<T>> for super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn sub(self, rhs: &'b super::Matrix<T>) -> Self::Output {
        &self - rhs
    }
}

impl<'b, T: Ring> Mul<&'b super::Matrix<T>> for super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn mul(self, rhs: &'b super::Matrix<T>) -> Self::Output {
        &self * rhs
    }
}

impl<'b, T: Ring> Mul<&'b Vector<T>> for super::Matrix<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: &'b Vector<T>) -> Self::Output {
        &self * rhs
    }
}

impl<'b, T: Ring> Mul<&'b T> for super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn mul(self, rhs: &'b T) -> Self::Output {
        &self * rhs
    }
}

impl<'b, T: Ring> Add<&'b T> for super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn add(self, rhs: &'b T) -> Self::Output {
        &self + rhs
    }
}

impl<'b, T: Ring> Sub<&'b T> for super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn sub(self, rhs: &'b T) -> Self::Output {
        &self - rhs
    }
}

impl<T: Ring> Add<super::Matrix<T>> for super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn add(self, rhs: super::Matrix<T>) -> Self::Output {
        &self + &rhs
    }
}

impl<T: Ring> Sub<super::Matrix<T>> for super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn sub(self, rhs: super::Matrix<T>) -> Self::Output {
        &self - &rhs
    }
}

impl<T: Ring> Mul<super::Matrix<T>> for super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn mul(self, rhs: super::Matrix<T>) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Ring> Mul<Vector<T>> for super::Matrix<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Ring> Mul<T> for super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Ring> Add<T> for super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn add(self, rhs: T) -> Self::Output {
        &self + &rhs
    }
}

impl<T: Ring> Sub<T> for super::Matrix<T> {
    type Output = super::Matrix<T>;
    fn sub(self, rhs: T) -> Self::Output {
        &self - &rhs
    }
}

// Display トレイト実装
use core::fmt;

pub trait DisplayElement {
    fn to_formatted_string(&self) -> String;
}

macro_rules! impl_default_display_element {
    ($($t:ty),*) => {
        $(
            impl DisplayElement for $t {
                fn to_formatted_string(&self) -> String {
                    self.to_string()
                }
            }
        )*
    };
}

const DISPLAY_PRECISION: i32 = 4;

impl DisplayElement for f64 {
    fn to_formatted_string(&self) -> String {
        let factor = 10.0_f64.powi(DISPLAY_PRECISION);
        let mut rounded_val = (self * factor).round() / factor;

        if rounded_val == 0.0 {
            rounded_val = 0.0;
        }

        rounded_val.to_string()
    }
}

impl_default_display_element!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, String, &str, bool, char
);

impl<T: crate::Scalar + DisplayElement> fmt::Display for super::Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_strings: Vec<String> = self
            .data
            .iter()
            .map(|val| val.to_formatted_string())
            .collect();

        let max_width = formatted_strings.iter().map(|s| s.len()).max().unwrap_or(0);

        writeln!(f, "rows: {}, cols: {}", self.rows, self.cols)?;
        for r in 0..self.rows {
            for c in 0..self.cols {
                let s = &formatted_strings[r * self.cols + c];
                write!(f, "{s:>max_width$}")?;
                if c < self.cols - 1 {
                    write!(f, "\t")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
