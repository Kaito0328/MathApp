use crate::gfp::GFp; // for convenience helpers; generic impl works for any Field
use linalg::Field;
use num_traits::{One, Zero};
use poly::Polynomial;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::sync::Arc;

// Polynomial に委譲した拡張ユークリッド互除法
fn poly_ext_gcd_poly<F: Field>(
    mut a: Polynomial<F>,
    mut b: Polynomial<F>,
) -> (Polynomial<F>, Polynomial<F>, Polynomial<F>) {
    // 返り値: (s, t, g) で s*a0 + t*b0 = g = gcd(a0, b0)
    let mut s0 = Polynomial::one();
    let mut t0 = Polynomial::zero();
    let mut s1 = Polynomial::zero();
    let mut t1 = Polynomial::one();

    while !b.is_zero() {
        let (q, r) = a.div_rem(&b);
        let ns = &s0 - &(&q * &s1);
        let nt = &t0 - &(&q * &t1);
        a = b;
        b = r;
        s0 = s1;
        t0 = t1;
        s1 = ns;
        t1 = nt;
    }
    (s0, t0, a) // a が gcd
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
        let r = Polynomial::new(v)
            .div_rem(&Polynomial::new((**px).clone()))
            .1;
        r.coeffs
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
        let px_poly = Polynomial::new((*(self.px)).clone());
        let val_poly = Polynomial::new(self.coeffs.clone());
        let (_s, t, g) = poly_ext_gcd_poly(px_poly, val_poly);
        // g は定数のはず（既約多項式を法とするため）。t / g を逆元とする。
        if g.deg() == 0 {
            let c = g.get(0);
            let cinv = F::one() / c;
            let tnorm = &t * cinv;
            return GFExt::new(self.px.clone(), tnorm.coeffs);
        }
        panic!("GFExt inverse: gcd is not constant; modulus may not be irreducible");
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
        // Polynomial 加算に委譲
        let s = &Polynomial::new(self.coeffs) + &Polynomial::new(rhs.coeffs);
        GFExt::new(px, s.coeffs)
    }
}
impl<F: Field + Clone + PartialEq + Zero> Sub for GFExt<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_zero() {
            let s = &Polynomial::zero() - &Polynomial::new(rhs.coeffs);
            return GFExt::new(rhs.px.clone(), s.coeffs);
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
        let s = &Polynomial::new(self.coeffs) - &Polynomial::new(rhs.coeffs);
        GFExt::new(px, s.coeffs)
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
        let prod = &Polynomial::new(self.coeffs) * &Polynomial::new(rhs.coeffs);
        // 剰余は new() 側で取られるが、係数が大きいときもあるため一旦生の係数を渡す
        GFExt::new(px, prod.coeffs)
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
