use crate::polynomial::Polynomial;
use num_complex::Complex;

/// 下降階乗 (x)_m = x(x-1)...(x-m+1) を多項式として返す（複素係数）。
pub fn falling_factorial_poly(m: usize) -> Polynomial<Complex<f64>> {
    if m == 0 {
        return Polynomial::one();
    }
    let mut p = Polynomial::one();
    for i in 0..m {
        p = &p * &Polynomial::from_roots(vec![Complex::new(i as f64, 0.0)]);
    }
    p
}

/// 上昇階乗 (x)^{(m)} = x(x+1)...(x+m-1) を多項式として返す（複素係数）。
pub fn rising_factorial_poly(m: usize) -> Polynomial<Complex<f64>> {
    if m == 0 {
        return Polynomial::one();
    }
    let mut p = Polynomial::one();
    for i in 0..m {
        // (x + i) = x - (-(i))
        p = &p * &Polynomial::from_roots(vec![Complex::new(-(i as f64), 0.0)]);
    }
    p
}

/// 多項式 p(x) に対し、q(x) = p(x + h) を計算（h は実数）。
pub fn shift_poly_x_plus_h(p: &Polynomial<Complex<f64>>, h: f64) -> Polynomial<Complex<f64>> {
    let n = p.deg();
    if n < 0 {
        return p.clone();
    }
    let n = n as usize;
    let mut out = vec![Complex::new(0.0, 0.0); n + 1];
    // p(x) = sum_{k=0..n} a_k x^k
    // p(x+h) = sum_{k} a_k sum_{j=0..k} C(k,j) h^{k-j} x^j
    for (k, &ak) in p.coeffs.iter().enumerate() {
        if ak == Complex::new(0.0, 0.0) {
            continue;
        }
        for j in 0..=k {
            // 係数 C(k,j) h^{k-j}
            let c = binom_usize(k, j) * h.powi((k - j) as i32);
            out[j] += ak * Complex::new(c, 0.0);
        }
    }
    Polynomial::new(out)
}

fn binom_usize(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    if k == 0 || k == n {
        return 1.0;
    }
    let k = k.min(n - k);
    let mut num = 1.0f64;
    let mut den = 1.0f64;
    for i in 1..=k {
        num *= (n - (k - i)) as f64;
        den *= i as f64;
    }
    num / den
}
/// 二項係数 C(x + k, k) を x の多項式として返す。
/// 恒等式 C(x + k, k) = (x+1)(x+2)...(x+k)/k! に基づき構成。
pub fn binom_x_plus_k_choose_k_poly(k: usize) -> Polynomial<Complex<f64>> {
    if k == 0 {
        return Polynomial::one();
    }
    let mut p = Polynomial::one();
    for j in 1..=k {
        // (x + j) = x - (-(j))
        p = &p * &Polynomial::from_roots(vec![Complex::new(-(j as f64), 0.0)]);
    }
    // 除算で k! による正規化
    let mut fact = 1.0f64;
    for j in 1..=k {
        fact *= j as f64;
    }
    &p / Complex::new(fact, 0.0)
}
