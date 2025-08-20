use std::ops::{Add, Div, Mul, Sub};

use crate::polynomial::Polynomial;
use linalg::Field;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct RationalFunction<F: Field> {
    pub numerator: Polynomial<F>,
    pub denominator: Polynomial<F>,
}

impl<F: Field> RationalFunction<F> {
    pub fn new(numerator: Polynomial<F>, denominator: Polynomial<F>) -> Self {
        if denominator.is_zero() {
            panic!("Denominator cannot be zero");
        }
        let (numerator, denominator) = Self::simplify_internal(&numerator, &denominator);
        Self {
            numerator,
            denominator,
        }
    }

    fn new_internal(numerator: Polynomial<F>, denominator: Polynomial<F>) -> Self {
        if denominator.is_zero() {
            panic!("Denominator cannot be zero");
        }
        Self {
            numerator,
            denominator,
        }
    }

    fn simplify_internal(
        numerator: &Polynomial<F>,
        denominator: &Polynomial<F>,
    ) -> (Polynomial<F>, Polynomial<F>) {
        let gcd = Polynomial::gcd(numerator, denominator);
        (numerator / &gcd, denominator / &gcd)
    }

    pub fn simplify(&mut self) {
        let (numerator, denominator) = Self::simplify_internal(&self.numerator, &self.denominator);
        self.numerator = numerator;
        self.denominator = denominator;
    }

    pub fn zero() -> Self {
        Self {
            numerator: Polynomial::zero(),
            denominator: Polynomial::one(),
        }
    }

    pub fn one() -> Self {
        Self {
            numerator: Polynomial::one(),
            denominator: Polynomial::one(),
        }
    }

    pub fn inverse(&self) -> Self {
        if self.is_zero() {
            panic!("Cannot invert zero rational function");
        }
        Self {
            numerator: self.denominator.clone(),
            denominator: self.numerator.clone(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.numerator.is_zero()
    }

    /// 多項式を乗算する。
    pub fn mul_poly(&self, poly: &Polynomial<F>) -> Self {
        // 既存の実装ロジック
        let g = Polynomial::gcd(&self.denominator, poly);
        let num = &self.numerator * &(poly / &g); // poly.div_rem(&g)の方が良いかも
        let den = &self.denominator / &g;
        Self::new_internal(num, den)
    }

    /// 多項式で除算する。
    pub fn div_poly(&self, poly: &Polynomial<F>) -> Self {
        if poly.is_zero() {
            panic!("Division by zero polynomial");
        }
        // (B/A) / P = B / (A*P)
        let g = Polynomial::gcd(&self.numerator, poly);
        let num = &self.numerator / &g;
        let den = &self.denominator * &(poly / &g);
        Self::new_internal(num, den)
    }
}

// 四則演算（演算子トレイト）(加減算計算効率のため、既約化されていない)
impl<F: Field> Add for &RationalFunction<F> {
    type Output = RationalFunction<F>;
    fn add(self, other: Self) -> Self::Output {
        let numerator = &self.numerator * &other.denominator + &other.numerator * &self.denominator;
        let denominator = &self.denominator * &other.denominator;
        RationalFunction::new_internal(numerator, denominator)
    }
}

impl<F: Field> Sub for &RationalFunction<F> {
    type Output = RationalFunction<F>;
    fn sub(self, other: Self) -> Self::Output {
        let numerator = &self.numerator * &other.denominator - &other.numerator * &self.denominator;
        let denominator = &self.denominator * &other.denominator;
        RationalFunction::new_internal(numerator, denominator)
    }
}

impl<F: Field> Mul for &RationalFunction<F> {
    type Output = RationalFunction<F>;
    fn mul(self, other: Self) -> Self::Output {
        let g1 = Polynomial::gcd(&self.numerator, &other.denominator);
        let g2 = Polynomial::gcd(&other.numerator, &self.denominator);

        let num1 = &self.numerator / &g1;
        let den2 = &other.denominator / &g1;

        let num2 = &other.numerator / &g2;
        let den1 = &self.denominator / &g2;
        // ------------------------------------------

        let new_numerator = num1 * num2;
        let new_denominator = den1 * den2;

        // 計算過程での部分的な既約化は行ったが、
        // 最終的な既約化は行わずに返す（遅延正規化の方針を維持）
        RationalFunction::new_internal(new_numerator, new_denominator)
    }
}

impl<F: Field> Div for &RationalFunction<F> {
    type Output = RationalFunction<F>;
    fn div(self, other: Self) -> Self::Output {
        if other.is_zero() {
            panic!("Division by zero in rational function");
        }
        let g1 = Polynomial::gcd(&self.numerator, &other.numerator);
        let g2 = Polynomial::gcd(&self.denominator, &other.denominator);

        let num1 = &self.numerator / &g1;
        let den2 = &other.numerator / &g1;

        let num2 = &other.denominator / &g2;
        let den1 = &self.denominator / &g2;
        // ------------------------------------------

        let new_numerator = num1 * num2;
        let new_denominator = den1 * den2;

        // 計算過程での部分的な既約化は行ったが、
        // 最終的な既約化は行わずに返す（遅延正規化の方針を維持）
        RationalFunction::new_internal(new_numerator, new_denominator)
    }
}

impl<F: Field> Mul<F> for &RationalFunction<F> {
    type Output = RationalFunction<F>;
    fn mul(self, scalar: F) -> Self::Output {
        let numerator = &self.numerator * scalar;
        RationalFunction::new_internal(numerator, self.denominator.clone())
    }
}

impl<F: Field> Div<F> for &RationalFunction<F> {
    type Output = RationalFunction<F>;
    fn div(self, scalar: F) -> Self::Output {
        if scalar.is_zero() {
            panic!("Division by zero in rational function");
        }
        let numerator = &self.numerator / scalar;
        RationalFunction::new_internal(numerator, self.denominator.clone())
    }
}

// 所有/参照の3パターン (RF op &RF, &RF op RF, RF op RF) を自動実装
impl_ops_by_ref_variants!(RationalFunction<F>, Add, add, linalg::Field);
impl_ops_by_ref_variants!(RationalFunction<F>, Sub, sub, linalg::Field);
impl_ops_by_ref_variants!(RationalFunction<F>, Mul, mul, linalg::Field);
impl_ops_by_ref_variants!(RationalFunction<F>, Div, div, linalg::Field);
