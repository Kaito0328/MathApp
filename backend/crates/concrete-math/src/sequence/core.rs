use num_complex::Complex;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::error::{ConcreteMathError, Result as ConcreteMathResult};
use poly::polynomial::Polynomial;

#[derive(Clone, Debug, PartialEq)]
pub struct GeneralTerm {
    pub polynomial: Polynomial<Complex<f64>>,
    pub base: Complex<f64>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ClosedForm {
    pub terms: Vec<GeneralTerm>,
}

impl ClosedForm {
    pub fn new(terms: Vec<GeneralTerm>) -> Self {
        let mut cf = ClosedForm { terms };
        cf.simplify();
        cf
    }

    pub fn term(&self, n: u32) -> Complex<f64> {
        let n_complex = Complex::new(n as f64, 0.0);
        const EPSILON: f64 = 1e-15;
        self.terms
            .iter()
            .map(|term| {
                let poly_val = term.polynomial.eval(n_complex);
                if (term.base - 1.0).norm() < EPSILON {
                    poly_val
                } else {
                    poly_val * term.base.powu(n)
                }
            })
            .sum()
    }

    pub fn zero() -> Self {
        ClosedForm { terms: Vec::new() }
    }

    pub fn single(polynomial: Polynomial<Complex<f64>>, base: Complex<f64>) -> Self {
        ClosedForm::new(vec![GeneralTerm { polynomial, base }])
    }

    pub fn is_zero(&self) -> bool {
        self.terms.is_empty() || self.terms.iter().all(|t| t.polynomial.is_zero())
    }

    pub fn simplify(&mut self) {
        const EPS: f64 = 1e-12;
        let mut acc: Vec<GeneralTerm> = Vec::new();
        for t in self.terms.drain(..) {
            if t.polynomial.is_zero() {
                continue;
            }
            if let Some(existing) = acc.iter_mut().find(|u| (u.base - t.base).norm() < EPS) {
                existing.polynomial = &existing.polynomial + &t.polynomial;
            } else {
                acc.push(t);
            }
        }
        acc.retain(|t| !t.polynomial.is_zero());
        self.terms = acc;
    }

    pub fn simplified(mut self) -> Self {
        self.simplify();
        self
    }

    /// Checked division by a complex scalar. Returns an error on zero divisor.
    pub fn try_div_scalar(&self, scalar: Complex<f64>) -> ConcreteMathResult<ClosedForm> {
        if scalar == Complex::new(0.0, 0.0) {
            return Err(ConcreteMathError::InvalidArgument {
                text: "division by zero scalar".into(),
            });
        }
        Ok(ClosedForm::new(
            self.terms
                .iter()
                .cloned()
                .map(|mut t| {
                    t.polynomial = &t.polynomial / scalar;
                    t
                })
                .collect(),
        ))
    }
}

macro_rules! impl_ops_ref_variants_for_nongen {
    ($Type:ty, $Trait:ident, $method:ident) => {
        #[allow(clippy::suspicious_arithmetic_impl)]
        impl ::std::ops::$Trait<&$Type> for $Type {
            type Output = $Type;
            #[inline]
            fn $method(self, rhs: &$Type) -> Self::Output {
                (&self).$method(rhs)
            }
        }
        #[allow(clippy::suspicious_arithmetic_impl)]
        impl ::std::ops::$Trait<$Type> for &$Type {
            type Output = $Type;
            #[inline]
            fn $method(self, rhs: $Type) -> Self::Output {
                self.$method(&rhs)
            }
        }
        #[allow(clippy::suspicious_arithmetic_impl)]
        impl ::std::ops::$Trait<$Type> for $Type {
            type Output = $Type;
            #[inline]
            fn $method(self, rhs: $Type) -> Self::Output {
                (&self).$method(&rhs)
            }
        }
    };
}

macro_rules! impl_scalar_rhs_ref_variants_for_nongen {
    ($Type:ty, $Scalar:ty, $Trait:ident, $method:ident) => {
        impl ::std::ops::$Trait<&$Scalar> for $Type {
            type Output = $Type;
            #[inline]
            fn $method(self, rhs: &$Scalar) -> Self::Output {
                (&self).$method(*rhs)
            }
        }
        impl ::std::ops::$Trait<$Scalar> for $Type {
            type Output = $Type;
            #[inline]
            fn $method(self, rhs: $Scalar) -> Self::Output {
                (&self).$method(rhs)
            }
        }
        impl ::std::ops::$Trait<&$Scalar> for &$Type {
            type Output = $Type;
            #[inline]
            fn $method(self, rhs: &$Scalar) -> Self::Output {
                self.$method(*rhs)
            }
        }
    };
}

impl Add for &ClosedForm {
    type Output = ClosedForm;
    fn add(self, rhs: Self) -> Self::Output {
        let mut terms = Vec::with_capacity(self.terms.len() + rhs.terms.len());
        terms.extend(self.terms.iter().cloned());
        terms.extend(rhs.terms.iter().cloned());
        ClosedForm::new(terms)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for &ClosedForm {
    type Output = ClosedForm;
    fn sub(self, rhs: Self) -> Self::Output {
        self + &(-rhs)
    }
}

impl Mul for &ClosedForm {
    type Output = ClosedForm;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut terms: Vec<GeneralTerm> = Vec::with_capacity(self.terms.len() * rhs.terms.len());
        for lt in &self.terms {
            for rt in &rhs.terms {
                terms.push(GeneralTerm {
                    polynomial: &lt.polynomial * &rt.polynomial,
                    base: lt.base * rt.base,
                });
            }
        }
        ClosedForm::new(terms)
    }
}

impl Neg for &ClosedForm {
    type Output = ClosedForm;
    fn neg(self) -> Self::Output {
        let m1 = Complex::new(-1.0, 0.0);
        ClosedForm::new(
            self.terms
                .iter()
                .cloned()
                .map(|mut t| {
                    t.polynomial = &t.polynomial * m1;
                    t
                })
                .collect(),
        )
    }
}

impl Mul<Complex<f64>> for &ClosedForm {
    type Output = ClosedForm;
    fn mul(self, scalar: Complex<f64>) -> Self::Output {
        ClosedForm::new(
            self.terms
                .iter()
                .cloned()
                .map(|mut t| {
                    t.polynomial = &t.polynomial * scalar;
                    t
                })
                .collect(),
        )
    }
}

impl Div<Complex<f64>> for &ClosedForm {
    type Output = ClosedForm;
    fn div(self, scalar: Complex<f64>) -> Self::Output {
        // 非チェック演算子: 0除算はゼロを返す（落とさない挙動）。
        // 例外のないAPIを維持しつつ、エラー検出が必要な場合は try_div_scalar を使用してください。
        self.try_div_scalar(scalar)
            .unwrap_or_else(|_| ClosedForm::zero())
    }
}

impl Neg for ClosedForm {
    type Output = ClosedForm;
    #[inline]
    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl_ops_ref_variants_for_nongen!(ClosedForm, Add, add);
impl_ops_ref_variants_for_nongen!(ClosedForm, Sub, sub);
impl_ops_ref_variants_for_nongen!(ClosedForm, Mul, mul);

impl_scalar_rhs_ref_variants_for_nongen!(ClosedForm, Complex<f64>, Mul, mul);
impl_scalar_rhs_ref_variants_for_nongen!(ClosedForm, Complex<f64>, Div, div);
