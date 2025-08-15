use crate::prime::GFp; // base prime example, but works with any Field
use linalg::Field;
use num_traits::{One, Zero};
// use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::sync::Arc;

// 多項式演算（係数低次→高次）
fn zero_pop<F: Field + Clone + PartialEq + Zero>(mut v: Vec<F>) -> Vec<F> {
    while v.len() > 1 && v.last().map(|c| c.is_zero()).unwrap_or(false) {
        v.pop();
    }
    if v.is_empty() {
        v.push(F::zero());
    }
    v
}
fn poly_add<F: Field + Clone + PartialEq + Zero>(a: &[F], b: &[F]) -> Vec<F> {
    let n = a.len().max(b.len());
    let mut v = vec![F::zero(); n];
    for (i, val) in v.iter_mut().enumerate() {
        let ai = a.get(i).cloned().unwrap_or_else(F::zero);
        let bi = b.get(i).cloned().unwrap_or_else(F::zero);
        *val = ai + bi;
    }
    zero_pop(v)
}
fn poly_sub<F: Field + Clone + PartialEq + Zero>(a: &[F], b: &[F]) -> Vec<F> {
    let n = a.len().max(b.len());
    let mut v = vec![F::zero(); n];
    for (i, val) in v.iter_mut().enumerate() {
        let ai = a.get(i).cloned().unwrap_or_else(F::zero);
        let bi = b.get(i).cloned().unwrap_or_else(F::zero);
        *val = ai - bi;
    }
    zero_pop(v)
}
fn poly_mul<F: Field + Clone + PartialEq + Zero>(a: &[F], b: &[F]) -> Vec<F> {
    let mut v = vec![F::zero(); a.len() + b.len() - 1];
    for i in 0..a.len() {
        for j in 0..b.len() {
            let t = a[i].clone() * b[j].clone();
            v[i + j] = v[i + j].clone() + t;
        }
    }
    zero_pop(v)
}
fn poly_div_rem<F: Field + Clone + PartialEq + Zero>(a: &[F], b: &[F]) -> (Vec<F>, Vec<F>) {
    let mut r = zero_pop(a.to_vec());
    let b = zero_pop(b.to_vec());
    if b.len() == 1 && b[0].is_zero() {
        return (vec![F::zero()], r);
    }
    if r.len() < b.len() {
        return (vec![F::zero()], r);
    }
    let mut q = vec![F::zero(); r.len() - b.len() + 1];
    let lead = b[b.len() - 1].clone();
    while r.len() >= b.len() && !(r.len() == 1 && r[0].is_zero()) {
        let shift = r.len() - b.len();
        let coef = r[r.len() - 1].clone() / lead.clone();
        q[shift] = coef.clone();
        for (i, val) in b.iter().enumerate() {
            let idx = i + shift;
            let r_val = r[idx].clone() - coef.clone() * val.clone();
            r[idx] = r_val;
        }
        r = zero_pop(r);
    }
    (zero_pop(q), zero_pop(r))
}
fn poly_ext_gcd<F: Field + Clone + PartialEq + Zero>(
    mut a: Vec<F>,
    mut b: Vec<F>,
) -> (Vec<F>, Vec<F>) {
    // return (s, t) such that s*a + t*b = gcd(a,b)
    let mut s0: Vec<F> = vec![F::one()];
    let mut t0: Vec<F> = vec![F::zero()];
    let mut s1: Vec<F> = vec![F::zero()];
    let mut t1: Vec<F> = vec![F::one()];
    while !(b.len() == 1 && b[0].is_zero()) {
        let (q, r) = poly_div_rem(&a, &b);
        let ns = poly_sub(&s0, &poly_mul(&q, &s1));
        let nt = poly_sub(&t0, &poly_mul(&q, &t1));
        a = b;
        b = r;
        s0 = s1;
        t0 = t1;
        s1 = ns;
        t1 = nt;
    }
    // gcd = a; return (s0, t0)
    (s0, t0)
}

// 拡大体: 既約多項式 px を法とする多項式係数ベクトル
#[derive(Clone)]
pub struct GFExt<F: Field + Clone + PartialEq> {
    px: Arc<Vec<F>>, // 既約多項式
    coeffs: Vec<F>,  // 低次→高次
}

impl<F: Field + Clone + PartialEq + Zero> GFExt<F> {
    pub fn new(px: Arc<Vec<F>>, coeffs: Vec<F>) -> Self {
        Self {
            px: px.clone(),
            coeffs: Self::mod_poly(&px, coeffs),
        }
    }
    fn mod_poly(px: &Arc<Vec<F>>, v: Vec<F>) -> Vec<F> {
        let (_q, r) = poly_div_rem(&v, px.as_ref());
        r
    }
    pub fn from_base(px: Arc<Vec<F>>, a: F) -> Self {
        Self::new(px, vec![a])
    }
    pub fn is_zero(&self) -> bool {
        self.coeffs.len() == 1 && self.coeffs[0].is_zero()
    }
    pub fn is_one(&self) -> bool {
        self.coeffs.len() == 1 && self.coeffs[0] == F::one()
    }
    pub fn px(&self) -> Arc<Vec<F>> {
        self.px.clone()
    }
    pub fn coeffs(&self) -> &[F] {
        &self.coeffs
    }

    pub fn inv(&self) -> Self {
        assert!(!self.is_zero(), "GFExt zero has no inverse");
        let (_s, t) = poly_ext_gcd((*(self.px)).clone(), self.coeffs.clone());
        // t is such that s*px + t*val = 1 => t is inverse mod px
        GFExt::new(self.px.clone(), t)
    }
}

impl<F: Field + Clone + PartialEq + Zero> Debug for GFExt<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.coeffs)
    }
}
impl<F: Field + Clone + PartialEq + Zero + One + Display> Display for GFExt<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 多項式表示: c0 + c1*x + c2*x^2 + ...（係数0は省略、係数1は x のみ）
        let mut terms: Vec<String> = Vec::new();
        for (i, c) in self.coeffs.iter().enumerate() {
            if c.is_zero() {
                continue;
            }
            let term = if i == 0 {
                format!("{c}")
            } else if *c == F::one() {
                if i == 1 {
                    "x".to_string()
                } else {
                    format!("x^{i}")
                }
            } else if i == 1 {
                format!("{c}*x")
            } else {
                format!("{c}*x^{i}")
            };
            terms.push(term);
        }
        if terms.is_empty() {
            write!(f, "0")
        } else {
            write!(f, "{}", terms.join(" + "))
        }
    }
}

// Vector/Matrix の Display を活かすための表示要素トレイト実装
impl<F> linalg::matrix::DisplayElement for GFExt<F>
where
    F: Field + Clone + PartialEq + Zero + Display,
{
    fn to_formatted_string(&self) -> String {
        self.to_string()
    }
}

impl<F: Field + Clone + PartialEq + Zero> PartialEq for GFExt<F> {
    fn eq(&self, other: &Self) -> bool {
        // ゼロ要素は px に依らず同一とみなす
        if self.is_zero() && other.is_zero() {
            return true;
        }
        // それ以外は係数が一致かつ px も同一（内容一致）
        self.coeffs == other.coeffs && *self.px == *other.px
    }
}
impl<F: Field + Clone + PartialEq + Zero> Eq for GFExt<F> {}

impl<F: Field + Clone + PartialEq + Zero> Add for GFExt<F> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_zero() {
            return GFExt::new(rhs.px.clone(), rhs.coeffs);
        }
        if rhs.is_zero() {
            return GFExt::new(self.px.clone(), self.coeffs);
        }
        let px = if self.px.is_empty() {
            rhs.px.clone()
        } else {
            self.px.clone()
        };
        assert!(
            rhs.px.is_empty() || *px == *rhs.px,
            "GFExt add: px mismatch"
        );
        GFExt::new(px, poly_add(&self.coeffs, &rhs.coeffs))
    }
}
impl<F: Field + Clone + PartialEq + Zero> Sub for GFExt<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_zero() {
            return GFExt::new(rhs.px.clone(), poly_sub(&[F::zero()], &rhs.coeffs));
        }
        if rhs.is_zero() {
            return GFExt::new(self.px.clone(), self.coeffs);
        }
        let px = if self.px.is_empty() {
            rhs.px.clone()
        } else {
            self.px.clone()
        };
        assert!(
            rhs.px.is_empty() || *px == *rhs.px,
            "GFExt sub: px mismatch"
        );
        GFExt::new(px, poly_sub(&self.coeffs, &rhs.coeffs))
    }
}
impl<F: Field + Clone + PartialEq + Zero> Mul for GFExt<F> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_zero() {
            return GFExt::new(rhs.px.clone(), vec![F::zero()]);
        }
        if rhs.is_zero() {
            return GFExt::new(self.px.clone(), vec![F::zero()]);
        }
        if self.is_one() {
            return GFExt::new(rhs.px.clone(), rhs.coeffs);
        }
        if rhs.is_one() {
            return GFExt::new(self.px.clone(), self.coeffs);
        }
        let px = if self.px.is_empty() {
            rhs.px.clone()
        } else {
            self.px.clone()
        };
        assert!(
            rhs.px.is_empty() || *px == *rhs.px,
            "GFExt mul: px mismatch"
        );
        GFExt::new(px, poly_mul(&self.coeffs, &rhs.coeffs))
    }
}
impl<F: Field + Clone + PartialEq + Zero> Div for GFExt<F> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if self.is_zero() {
            return GFExt::new(rhs.px.clone(), vec![F::zero()]);
        }
        if rhs.is_one() {
            return GFExt::new(self.px.clone(), self.coeffs);
        }
        let px = if self.px.is_empty() {
            rhs.px.clone()
        } else {
            self.px.clone()
        };
        assert!(
            rhs.px.is_empty() || *px == *rhs.px,
            "GFExt div: px mismatch"
        );
        GFExt::new(px.clone(), self.coeffs) * GFExt::new(px, rhs.coeffs).inv()
    }
}
impl<F: Field + Clone + PartialEq + Zero> Neg for GFExt<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        GFExt::new(
            self.px.clone(),
            self.coeffs.into_iter().map(|c| -c).collect(),
        )
    }
}

impl<F: Field + Clone + PartialEq + Zero> Sum for GFExt<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        // Sum by folding with additive identity
        iter.fold(
            GFExt {
                px: Arc::new(vec![]),
                coeffs: vec![F::zero()],
            },
            |acc, x| acc + x,
        )
    }
}

// 注意: Zero/One は px 無しで返るため、演算側で px を合わせる特別扱いを実装済み
impl<F: Field + Clone + PartialEq + Zero> Zero for GFExt<F> {
    fn zero() -> Self {
        GFExt {
            px: Arc::new(vec![]),
            coeffs: vec![F::zero()],
        }
    }
    fn is_zero(&self) -> bool {
        <GFExt<F>>::is_zero(self)
    }
}
impl<F: Field + Clone + PartialEq + Zero + One> One for GFExt<F> {
    fn one() -> Self {
        GFExt {
            px: Arc::new(vec![]),
            coeffs: vec![F::one()],
        }
    }
}

// linalg トレイト
impl<F: Field + Clone + PartialEq + Zero> linalg::Scalar for GFExt<F> {}
impl<F: Field + Clone + PartialEq + Zero> linalg::Ring for GFExt<F> {}
impl<F: Field + Clone + PartialEq + Zero> linalg::Field for GFExt<F> {}

// 補助: GF(2^m) 用（u8 との相互変換, AES 多項式など）
impl GFExt<GFp<2>> {
    pub fn from_u8(px: Arc<Vec<GFp<2>>>, x: u8) -> Self {
        let mut coeffs = Vec::new();
        for i in 0..8 {
            if ((x >> i) & 1) == 1 {
                coeffs.resize(i + 1, GFp::<2>(0));
                coeffs[i] = GFp::<2>(1);
            }
        }
        if coeffs.is_empty() {
            coeffs.push(GFp::<2>(0));
        }
        GFExt::new(px, coeffs)
    }
    pub fn to_u8(&self) -> u8 {
        let mut v = 0u8;
        for (i, c) in self.coeffs.iter().enumerate() {
            if i < 8 && c.0 == 1 {
                v |= 1 << i;
            }
        }
        v
    }
}
