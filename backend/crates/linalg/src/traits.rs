use num_traits::{One, Signed, Zero};
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub}; // Negを追加

pub trait Scalar: Clone + Debug {}

pub trait Ring:
    Scalar
    + Zero
    + One
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
    + Sum<Self>
{
}

pub trait Field: Ring + Div<Output = Self> {}

impl<T> Scalar for T where T: Clone + Debug {}

impl<T> Ring for T where
    T: Scalar
        + Zero
        + One
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Sum<T>
{
}

// Ringであり、かつDiv<Output=T>を持つ型は、自動的にFieldトレイトを実装する
impl<T: Ring + Div<Output = T>> Field for T {}

pub trait LinalgField: Field + Signed + PartialOrd {
    // 必要に応じて、epsilon()のような小さな値を返すメソッドを定義
    fn epsilon() -> Self;
}

impl LinalgField for f64 {
    fn epsilon() -> Self {
        1e-12
    }
}

impl LinalgField for f32 {
    fn epsilon() -> Self {
        1e-6
    }
}
