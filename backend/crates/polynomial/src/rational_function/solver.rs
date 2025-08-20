use std::ops::Deref;

use crate::{
    polynomial::{solver::Root, Polynomial},
    rational_function::core::RationalFunction,
};
use linalg::Field;
use num_complex::Complex;

impl<F: Field> RationalFunction<F> {
    pub fn eval(&self, x: F) -> Option<F> {
        let numerator_value = self.numerator.eval(x.clone());
        let denominator_value = self.denominator.eval(x.clone());
        if denominator_value.is_zero() {
            return None;
        }
        Some(numerator_value / denominator_value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pole(Root);

impl Deref for Pole {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PoleTerm {
    /// 極の値 p
    pub pole: Complex<f64>,
    /// 係数のリスト [C₁, C₂, ..., Cₘ]
    /// Cⱼ は (s - p)ʲ の項の分子係数。
    pub coefficients: Vec<Complex<f64>>, // ← 各係数は複素数の定数なので、これで正しい
}

/// 有理関数の部分分数分解の結果全体を表現する構造体。
#[derive(Debug, Clone, PartialEq)]
pub struct PartialFractionExpansion {
    /// 多項式部分。分子の次数 < 分母の次数 の場合はゼロ多項式。
    pub polynomial_part: Polynomial<Complex<f64>>,
    /// 極と係数の項のリスト。
    pub pole_terms: Vec<PoleTerm>,
}

impl RationalFunction<f64> {
    pub fn differentiate(&self) -> Self {
        let numerator_deriv = self.numerator.differentiate();
        let denominator_deriv = self.denominator.differentiate();
        Self {
            numerator: numerator_deriv * &self.denominator - &self.numerator * denominator_deriv,
            denominator: &self.denominator * &self.denominator,
        }
    }
    pub fn find_poles(&self) -> Vec<Pole> {
        const TOLERANCE: f64 = 1e-12; // 極の検出に使用する許容誤差
        Polynomial::group_roots(&self.denominator.find_roots(), TOLERANCE)
            .into_iter()
            .map(Pole)
            .collect()
    }

    pub fn partial_fraction_expansion(&self) -> PartialFractionExpansion {
        let mut numerator = self.numerator.clone();
        if self.denominator.deg() < self.numerator.deg() {
            numerator = numerator.div_rem(&self.denominator).0;
        }
        let poles = self.find_poles();

        unimplemented!("Partial fraction expansion is not yet implemented");
    }
}
