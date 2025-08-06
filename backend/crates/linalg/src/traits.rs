use num_traits::{One, Zero};
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

// f64はFieldの全ての条件を満たす
impl Scalar for f64 {}
impl Ring for f64 {}
impl Field for f64 {}

impl Scalar for f32 {}
impl Ring for f32 {}
impl Field for f32 {}

// i32はRingの条件を満たすが、Fieldではない
impl Scalar for i32 {}
impl Ring for i32 {}

// StringはCloneとDebugを持つので、最低限のScalarにはなれる
impl Scalar for String {}
