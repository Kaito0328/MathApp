use linalg::Field;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Poly<F: Field> {
	pub coeffs: Vec<F>, // 低次→高次
}

impl<F: Field> Poly<F> {
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
	pub fn deg(&self) -> isize {
		self.coeffs.len() as isize - 1
	}
	pub fn get(&self, i: usize) -> F {
		self.coeffs.get(i).cloned().unwrap_or_else(F::zero)
	}

	pub fn div_rem(&self, divisor: &Self) -> (Self, Self) {
		let mut r = self.coeffs.clone();
		while r.len() > 1 && r.last().map(|x| x.is_zero()).unwrap_or(false) {
			r.pop();
		}
		let mut rpoly = Poly::new(r);
		if divisor.coeffs.is_empty() || *divisor == Poly::zero() {
			return (Poly::zero(), self.clone());
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
			while rpoly.coeffs.len() > 1 && rpoly.coeffs.last().map(|x| x.is_zero()).unwrap_or(false) {
				rpoly.coeffs.pop();
			}
		}
		(Poly::new(q), rpoly)
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
			return Poly::zero();
		}
		let inv = F::one() / lc;
		Poly::new(self.coeffs.iter().map(|c| c.clone() * inv.clone()).collect())
	}
}

// 四則演算（演算子トレイト）
impl<F: Field> Add for &Poly<F> {
	type Output = Poly<F>;
	fn add(self, other: Self) -> Self::Output {
		let n = self.coeffs.len().max(other.coeffs.len());
		let mut v = vec![F::zero(); n];
		for (i, coeff) in v.iter_mut().enumerate() {
			*coeff = self.get(i) + other.get(i);
		}
		Poly::new(v)
	}
}

impl<F: Field> Sub for &Poly<F> {
	type Output = Poly<F>;
	fn sub(self, other: Self) -> Self::Output {
		let n = self.coeffs.len().max(other.coeffs.len());
		let mut v = vec![F::zero(); n];
		for (i, coeff) in v.iter_mut().enumerate() {
			*coeff = self.get(i) - other.get(i);
		}
		Poly::new(v)
	}
}

impl<F: Field> Mul for &Poly<F> {
	type Output = Poly<F>;
	fn mul(self, other: Self) -> Self::Output {
		if self.deg() < 0 || other.deg() < 0 {
			return Poly::zero();
		}
		let mut v = vec![F::zero(); self.coeffs.len() + other.coeffs.len() - 1];
		for i in 0..self.coeffs.len() {
			for j in 0..other.coeffs.len() {
				v[i + j] = v[i + j].clone() + self.coeffs[i].clone() * other.coeffs[j].clone();
			}
		}
		Poly::new(v)
	}
}

// スカラー倍や整除係数倍などの用途を想定しておく
impl<F: Field> Mul<F> for &Poly<F> {
	type Output = Poly<F>;
	fn mul(self, rhs: F) -> Self::Output {
		Poly::new(self.coeffs.iter().map(|c| c.clone() * rhs.clone()).collect())
	}
}

// 便宜上、整係数での除算をサポート（多項式同士の除算は div_rem を使用）
impl<F: Field> Div<F> for &Poly<F> {
	type Output = Poly<F>;
	fn div(self, rhs: F) -> Self::Output {
		Poly::new(self.coeffs.iter().map(|c| c.clone() / rhs.clone()).collect())
	}
}
