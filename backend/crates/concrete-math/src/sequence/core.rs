use num_complex::Complex;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::error::{ConcreteMathError, Result as ConcreteMathResult};
use poly::format::fmt_complex;
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

// ---------------- Display helpers ----------------
fn fmt_poly_complex(
    poly: &Polynomial<Complex<f64>>,
    var: &str,
    unicode_superscript: bool,
) -> String {
    // 係数が複素数の多項式を "a_k var^k + ... + a_0" 形式で表示（'*' なし）
    if poly.is_zero() {
        return "0".to_string();
    }
    let tol = 1e-12;
    let mut out: Vec<String> = Vec::new();
    for (k, c) in poly.coeffs.iter().enumerate().rev() {
        if c.norm() <= tol {
            continue;
        }
        let deg = k;
        let abs_c = if c.re.abs() <= tol && c.im.abs() <= tol {
            Complex::new(0.0, 0.0)
        } else {
            *c
        };
        let c_str = fmt_complex(abs_c);
        let term = if deg == 0 {
            c_str
        } else if deg == 1 {
            if (abs_c - Complex::new(1.0, 0.0)).norm() <= tol {
                var.to_string()
            } else if (abs_c + Complex::new(-1.0, 0.0)).norm() <= tol {
                // -1
                format!("-{var}")
            } else {
                format!("{c_str}{var}")
            }
        } else if (abs_c - Complex::new(1.0, 0.0)).norm() <= tol {
            if unicode_superscript {
                format!("{var}{}", to_superscript(deg as isize))
            } else {
                format!("{var}^{deg}")
            }
        } else if (abs_c + Complex::new(-1.0, 0.0)).norm() <= tol {
            if unicode_superscript {
                format!("-{var}{}", to_superscript(deg as isize))
            } else {
                format!("-{var}^{deg}")
            }
        } else if unicode_superscript {
            format!("{c_str}{var}{}", to_superscript(deg as isize))
        } else {
            format!("{c_str}{var}^{deg}")
        };
        out.push(term);
    }
    // 結合：符号が文字列に含まれている場合があるので、適切にスペースを入れる
    let mut s = String::new();
    for (i, t) in out.into_iter().enumerate() {
        if i == 0 {
            s.push_str(&t);
        } else if t.starts_with('-') {
            s.push_str(" - ");
            s.push_str(t.trim_start_matches('-'));
        } else {
            s.push_str(" + ");
            s.push_str(&t);
        }
    }
    s
}

impl fmt::Display for GeneralTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.polynomial.is_zero() {
            return write!(f, "0");
        }
        let p = fmt_poly_complex(&self.polynomial, "n", false);
        let eps = 1e-12;
        if (self.base - Complex::new(1.0, 0.0)).norm() < eps {
            // P(n)
            write!(f, "{p}")
        } else {
            // P(n) (r)^n  （'*' は付けない）
            let r = fmt_complex(self.base);
            write!(f, "{p} ({r})^n")
        }
    }
}

impl fmt::Display for ClosedForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        let mut parts: Vec<String> = Vec::new();
        for t in &self.terms {
            let s = t.to_string();
            if s == "0" {
                continue;
            }
            parts.push(s);
        }
        if parts.is_empty() {
            return write!(f, "0");
        }
        let mut out = String::new();
        for (i, s) in parts.into_iter().enumerate() {
            if i == 0 {
                out.push_str(&s);
            } else if s.starts_with('-') {
                out.push_str(" - ");
                out.push_str(s.trim_start_matches('-'));
            } else {
                out.push_str(" + ");
                out.push_str(&s);
            }
        }
        write!(f, "{out}")
    }
}

// --------- 可読表示ラッパ（変数名と上付き指数の選択） ---------
#[derive(Clone, Copy, Debug, Default)]
pub struct SeqStyle {
    pub unicode_superscript: bool,
}

pub struct ClosedFormDisplay<'a> {
    pub cf: &'a ClosedForm,
    pub var: &'static str,
    pub style: SeqStyle,
}

impl<'a> ClosedFormDisplay<'a> {
    pub fn new(cf: &'a ClosedForm, var: &'static str) -> Self {
        Self {
            cf,
            var,
            style: SeqStyle::default(),
        }
    }
    pub fn unicode_superscript(mut self, on: bool) -> Self {
        self.style.unicode_superscript = on;
        self
    }
}

fn to_superscript(n: isize) -> String {
    let digits = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    let minus = '⁻';
    let mut x = n;
    if x == 0 {
        return digits[0].to_string();
    }
    let mut out = String::new();
    if x < 0 {
        out.push(minus);
        x = -x;
    }
    let mut buf = Vec::new();
    while x > 0 {
        buf.push((x % 10) as usize);
        x /= 10;
    }
    for d in buf.iter().rev() {
        out.push(digits[*d]);
    }
    out
}

impl<'a> fmt::Display for ClosedFormDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cf.is_zero() {
            return write!(f, "0");
        }
        let mut parts: Vec<String> = Vec::new();
        for t in &self.cf.terms {
            if t.polynomial.is_zero() {
                continue;
            }
            let p = fmt_poly_complex(&t.polynomial, self.var, self.style.unicode_superscript);
            let eps = 1e-12;
            let s = if (t.base - Complex::new(1.0, 0.0)).norm() < eps {
                p
            } else {
                let r = fmt_complex(t.base);
                // base^n（n は記号のため Unicode 上付きは使わず ^n とする）
                format!("{p} ({r})^n")
            };
            parts.push(s);
        }
        if parts.is_empty() {
            return write!(f, "0");
        }
        let mut out = String::new();
        for (i, s) in parts.into_iter().enumerate() {
            if i == 0 {
                out.push_str(&s);
            } else if s.starts_with('-') {
                out.push_str(" - ");
                out.push_str(s.trim_start_matches('-'));
            } else {
                out.push_str(" + ");
                out.push_str(&s);
            }
        }
        write!(f, "{out}")
    }
}

impl ClosedForm {
    /// 表示ラッパ（変数名: 既定は "n"）
    pub fn display(&self) -> ClosedFormDisplay<'_> {
        ClosedFormDisplay::new(self, "n")
    }
    /// 変数名指定付き表示ラッパ
    pub fn display_with(&self, var: &'static str) -> ClosedFormDisplay<'_> {
        ClosedFormDisplay::new(self, var)
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
