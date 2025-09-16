use crate::Poly;
use linalg::Field;
use num_traits::{Zero, One};

// Berlekamp–Massey: 与えられたシンドローム列 s[0..m-1]（m>=1）から、接続多項式 Sigma(x) を求める。
// 返り値は Sigma(x) の係数（低次→高次）。
pub fn berlekamp_massey<F>(s: &[F]) -> Vec<F>
where
    F: Field + Clone + PartialEq + Zero + One,
{
    let mut c: Vec<F> = vec![F::one()];      // C(x)
    let mut b: Vec<F> = vec![F::one()];      // B(x)
    let mut l: usize = 0;                    // 現次数
    let mut m: usize = 1;
    let mut bb = F::one();                   // 前回の非零不一致

    for n in 0..s.len() {
        // 不一致 d = s[n] + sum_{i=1..L} C[i]*s[n-i]
        let mut d = s[n].clone();
        for i in 1..=l { d = d + c[i].clone() * s[n - i].clone(); }
        if d == F::zero() {
            m += 1;
            continue;
        }
        // T(x) = C(x) - d/bb * x^m * B(x)
        let inv_bb = F::one() / bb.clone();
        let coef = d.clone() * inv_bb; // d/bb
        let mut t = c.clone();
        let mut xb = vec![F::zero(); m + b.len()];
        for (i, bi) in b.iter().enumerate() { xb[m + i] = bi.clone(); }
        // t = c - coef * xb
        let len = t.len().max(xb.len());
        t.resize(len, F::zero());
        let mut scaled_xb = xb.clone();
        for x in scaled_xb.iter_mut() { *x = x.clone() * coef.clone(); }
        for i in 0..len { let xi = if i < scaled_xb.len() { scaled_xb[i].clone() } else { F::zero() }; t[i] = t[i].clone() - xi; }

        if 2*l <= n {
            l = n + 1 - l;
            b = c;
            bb = d;
            c = t;
            m = 1;
        } else {
            c = t;
            m += 1;
        }
    }
    // 正規化（最高次係数を1に）
    if let Some(lc) = c.last().cloned() { if lc != F::one() { let inv = F::one()/lc; for x in c.iter_mut(){ *x = x.clone()*inv.clone(); } } }
    c
}

// シンドローム多項式 S(x) と Σ(x) から、Ω(x) = [S(x) * Σ(x)] mod x^d を作る
pub fn error_evaluator_from_syndromes<F>(syndromes: &[F], sigma: &Poly<F>, d: usize) -> Poly<F>
where
    F: Field + Clone + PartialEq + Zero + One,
{
    let s_poly: Poly<F> = Poly::new(syndromes.to_vec());
    let prod = &s_poly * sigma;
    let mut coeffs = prod.coeffs.clone();
    if coeffs.len() > d { coeffs.truncate(d); }
    Poly::new(coeffs)
}

// 多項式の形式微分（一般体）: 係数 i*c_i を i 回の加算で生成
pub fn formal_derivative<F>(p: &Poly<F>) -> Poly<F>
where
    F: Field + Clone + PartialEq + Zero + One,
{
    let mut out: Vec<F> = Vec::with_capacity(p.coeffs.len().saturating_sub(1));
    for i in 1..p.coeffs.len() {
        // i を体要素に変換（1 を i 回足す）
        let mut k = F::zero();
        for _ in 0..i { k = k + F::one(); }
        out.push(p.coeffs[i].clone() * k);
    }
    Poly::new(out)
}
