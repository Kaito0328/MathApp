use linalg::Field;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Polynomial<F: Field> {
    pub coeffs: Vec<F>, // 低次→高次
}

impl<F: Field> Polynomial<F> {
    pub fn new(mut c: Vec<F>) -> Self {
        Self {
            coeffs: Self::trim(&mut c),
        }
    }
    fn trim(v: &mut Vec<F>) -> Vec<F> {
        while v.len() > 1 && v.last().map(|x| x.is_zero()).unwrap_or(false) {
            v.pop();
        }
        if v.is_empty() {
            v.push(F::zero());
        }
        v.clone()
    }
    pub fn zero() -> Self {
        Self {
            coeffs: vec![F::zero()],
        }
    }
    pub fn one() -> Self {
        Self {
            coeffs: vec![F::one()],
        }
    }
    pub fn is_zero(&self) -> bool {
        self.coeffs.len() == 1 && self.coeffs[0].is_zero()
    }
    pub fn deg(&self) -> isize {
        // 慣習に従い、ゼロ多項式 (coeffs == [0]) の次数は -1 とする
        if self.coeffs.len() == 1 && self.coeffs[0].is_zero() {
            -1
        } else {
            self.coeffs.len() as isize - 1
        }
    }
    pub fn get(&self, i: usize) -> F {
        self.coeffs.get(i).cloned().unwrap_or_else(F::zero)
    }

    pub fn div_rem(&self, divisor: &Self) -> (Self, Self) {
        let mut r = self.coeffs.clone();
        while r.len() > 1 && r.last().map(|x| x.is_zero()).unwrap_or(false) {
            r.pop();
        }
        let mut rpoly = Polynomial::new(r);
        // ゼロ多項式の判定は係数で直接行う（PartialEq不要）
        if divisor.coeffs.is_empty() || (divisor.coeffs.len() == 1 && divisor.coeffs[0].is_zero()) {
            return (Polynomial::zero(), self.clone());
        }
        let dl = divisor.coeffs.len();
        let lead = divisor.coeffs[dl - 1].clone();
        let mut q = vec![F::zero(); self.coeffs.len().saturating_sub(dl) + 1];
        while rpoly.coeffs.len() >= dl && !(rpoly.coeffs.len() == 1 && rpoly.coeffs[0].is_zero()) {
            let shift = rpoly.coeffs.len() - dl;
            let coef = rpoly.coeffs.last().unwrap().clone() / lead.clone();
            q[shift] = coef.clone();
            for i in 0..dl {
                let idx = i + shift;
                let val = rpoly.coeffs[idx].clone() - coef.clone() * divisor.coeffs[i].clone();
                rpoly.coeffs[idx] = val;
            }
            while rpoly.coeffs.len() > 1
                && rpoly.coeffs.last().map(|x| x.is_zero()).unwrap_or(false)
            {
                rpoly.coeffs.pop();
            }
        }
        (Polynomial::new(q), rpoly)
    }
    pub fn eval(&self, x: F) -> F {
        let mut acc = F::zero();
        for c in self.coeffs.iter().rev() {
            acc = acc * x.clone() + c.clone();
        }
        acc
    }

    pub fn monic(&self) -> Self {
        if self.deg() < 0 {
            return self.clone();
        }
        let lc = self.coeffs.last().unwrap().clone();
        if lc.is_zero() {
            return Polynomial::zero();
        }
        let inv = F::one() / lc;
        Polynomial::new(
            self.coeffs
                .iter()
                .map(|c| c.clone() * inv.clone())
                .collect(),
        )
    }
}

impl<F: Field + FromPrimitive> Polynomial<F> {
    pub fn differentiate(&self) -> Self {
        let deg = self.deg();
        if deg <= 0 {
            return Polynomial::zero();
        }
        let mut new_coeffs = Vec::with_capacity(deg as usize);
        for (i, coeff) in self.coeffs.iter().enumerate().skip(1) {
            new_coeffs.push(coeff.clone() * F::from_usize(i).unwrap());
        }
        Polynomial::new(new_coeffs)
    }

    pub fn integrate(&self) -> Self {
        let deg = self.deg();
        let mut new_coeffs: Vec<F> = Vec::with_capacity(deg as usize + 2);
        new_coeffs.push(F::zero());
        for (i, coeff) in self.coeffs.iter().enumerate() {
            new_coeffs.push(coeff.clone() / F::from_usize(i + 1).unwrap());
        }
        Polynomial::new(new_coeffs)
    }
}

// 四則演算（演算子トレイト）
impl<F: Field> Add for &Polynomial<F> {
    type Output = Polynomial<F>;
    fn add(self, other: Self) -> Self::Output {
        let n = self.coeffs.len().max(other.coeffs.len());
        let mut v = vec![F::zero(); n];
        for (i, coeff) in v.iter_mut().enumerate() {
            *coeff = self.get(i) + other.get(i);
        }
        Polynomial::new(v)
    }
}

impl<F: Field> Sub for &Polynomial<F> {
    type Output = Polynomial<F>;
    fn sub(self, other: Self) -> Self::Output {
        let n = self.coeffs.len().max(other.coeffs.len());
        let mut v = vec![F::zero(); n];
        for (i, coeff) in v.iter_mut().enumerate() {
            *coeff = self.get(i) - other.get(i);
        }
        Polynomial::new(v)
    }
}

impl<F: Field> Mul for &Polynomial<F> {
    type Output = Polynomial<F>;
    fn mul(self, other: Self) -> Self::Output {
        if self.deg() < 0 || other.deg() < 0 {
            return Polynomial::zero();
        }
        let mut v = vec![F::zero(); self.coeffs.len() + other.coeffs.len() - 1];
        for i in 0..self.coeffs.len() {
            for j in 0..other.coeffs.len() {
                v[i + j] = v[i + j].clone() + self.coeffs[i].clone() * other.coeffs[j].clone();
            }
        }
        Polynomial::new(v)
    }
}

// 速度重視の f64 専用: 素朴/FFT/自動
impl Polynomial<f64> {
    pub fn mul_simple(&self, other: &Self) -> Self {
        if self.deg() < 0 || other.deg() < 0 {
            return Polynomial::zero();
        }
        let mut v = vec![0f64; self.coeffs.len() + other.coeffs.len() - 1];
        for i in 0..self.coeffs.len() {
            for j in 0..other.coeffs.len() {
                v[i + j] += self.coeffs[i] * other.coeffs[j];
            }
        }
        Polynomial::new(v)
    }
    pub fn mul_fft(&self, other: &Self) -> Self {
        if self.deg() < 0 || other.deg() < 0 {
            return Polynomial::zero();
        }
        let y = convolution::convolve_fft_f64(&self.coeffs, &other.coeffs);
        Polynomial::new(y)
    }
    pub fn mul_auto(&self, other: &Self) -> Self {
        if self.deg() < 0 || other.deg() < 0 {
            return Polynomial::zero();
        }
        let work = self.coeffs.len() * other.coeffs.len();
        if work <= 2048 {
            self.mul_simple(other)
        } else {
            self.mul_fft(other)
        }
    }
}

// スカラー倍や整除係数倍などの用途を想定しておく
impl<F: Field> Mul<F> for &Polynomial<F> {
    type Output = Polynomial<F>;
    fn mul(self, rhs: F) -> Self::Output {
        Polynomial::new(
            self.coeffs
                .iter()
                .map(|c| c.clone() * rhs.clone())
                .collect(),
        )
    }
}

// 便宜上、整係数での除算をサポート（多項式同士の除算は下の Div 実装を使用）
impl<F: Field> Div<F> for &Polynomial<F> {
    type Output = Polynomial<F>;
    fn div(self, rhs: F) -> Self::Output {
        Polynomial::new(
            self.coeffs
                .iter()
                .map(|c| c.clone() / rhs.clone())
                .collect(),
        )
    }
}

// 所有型に対するスカラー除算（テスト利便性のため）
impl<F: Field> Div<F> for Polynomial<F> {
    type Output = Polynomial<F>;
    fn div(self, rhs: F) -> Self::Output {
        (&self).div(rhs)
    }
}

// 多項式 ÷ 多項式 は商のみを返す（余りは `div_rem` を使用）
impl<F: Field> Div for &Polynomial<F> {
    type Output = Polynomial<F>;
    fn div(self, rhs: Self) -> Self::Output {
        let (q, _r) = self.div_rem(rhs);
        q
    }
}

// 所有/参照の3パターン (Poly op &Poly, &Poly op Poly, Poly op Poly) を自動実装
crate::impl_ops_by_ref_variants!(Polynomial<F>, Add, add, linalg::Field);
crate::impl_ops_by_ref_variants!(Polynomial<F>, Sub, sub, linalg::Field);
crate::impl_ops_by_ref_variants!(Polynomial<F>, Mul, mul, linalg::Field);
crate::impl_ops_by_ref_variants!(Polynomial<F>, Div, div, linalg::Field);
