use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub};

// 素数位数 p に対する素体 GF(p)。p はコンパイル時定数。
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GFp<const P: u16>(pub u16);

impl<const P: u16> GFp<P> {
    #[inline]
    pub const fn modulus() -> u16 {
        P
    }

    #[inline]
    pub fn new(v: i64) -> Self {
        let p = P as i64;
        let mut r = v % p;
        if r < 0 {
            r += p;
        }
        GFp::<P>(r as u16)
    }

    #[inline]
    pub fn value(self) -> u16 {
        self.0
    }

    // 逆元（Result版：破壊的変更）
    pub fn inv(self) -> crate::prelude::FieldResult<Self> {
        if self.0 == 0 {
            return Err(crate::error::FieldError::DivisionByZero);
        }
        let (_x, y) = egcd(P as i64, self.0 as i64);
        Ok(GFp::<P>::new(y))
    }
    // 後方互換のための別名（将来削除可）
    pub fn try_inv(self) -> crate::prelude::FieldResult<Self> {
        self.inv()
    }
    #[inline]
    fn mul_i64(a: i64, b: i64) -> Self {
        // 内部表現の積に対して mod P を取って要素化
        GFp::<P>::new(a * b)
    }
}

fn egcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        return (1, 0);
    }
    let (x1, y1) = egcd(b, a % b);
    let x = y1;
    let y = x1 - (a / b) * y1;
    (x, y)
}

impl<const P: u16> Add for GFp<P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        GFp::<P>::new(self.0 as i64 + rhs.0 as i64)
    }
}
impl<const P: u16> Sub for GFp<P> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        GFp::<P>::new(self.0 as i64 - rhs.0 as i64)
    }
}
impl<const P: u16> Mul for GFp<P> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        GFp::<P>::new(self.0 as i64 * rhs.0 as i64)
    }
}
impl<const P: u16> Div for GFp<P> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        // rhs.inv() を明示的に計算し、内部表現で乗算→剰余化して返す
        let inv = rhs.inv().expect("GFp division by zero").0 as i64;
        GFp::<P>::mul_i64(self.0 as i64, inv)
    }
}

impl<const P: u16> GFp<P> {
    // 除算（Result版）
    pub fn checked_div(self, rhs: Self) -> crate::prelude::FieldResult<Self> {
        Ok(self * rhs.inv()?)
    }
}
impl<const P: u16> Neg for GFp<P> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        GFp::<P>::new(-(self.0 as i64))
    }
}

impl<const P: u16> Zero for GFp<P> {
    fn zero() -> Self {
        GFp::<P>(0)
    }
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
impl<const P: u16> One for GFp<P> {
    fn one() -> Self {
        GFp::<P>(1)
    }
}
impl<const P: u16> Sum for GFp<P> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, x| a + x)
    }
}

impl<const P: u16> Debug for GFp<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<const P: u16> Display for GFp<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Vector/Matrix の Display を活かすための表示要素トレイト実装
impl<const P: u16> linalg::matrix::DisplayElement for GFp<P> {
    fn to_formatted_string(&self) -> String {
        self.to_string()
    }
}
