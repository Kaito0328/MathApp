use num_complex::Complex;
use poly::polynomial::Polynomial;

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

pub fn rising_factorial_poly(m: usize) -> Polynomial<Complex<f64>> {
    if m == 0 {
        return Polynomial::one();
    }
    let mut p = Polynomial::one();
    for i in 0..m {
        p = &p * &Polynomial::from_roots(vec![Complex::new(-(i as f64), 0.0)]);
    }
    p
}

pub fn shift_poly_x_plus_h(p: &Polynomial<Complex<f64>>, h: f64) -> Polynomial<Complex<f64>> {
    let n = p.deg();
    if n < 0 {
        return p.clone();
    }
    let n = n as usize;
    let mut out = vec![Complex::new(0.0, 0.0); n + 1];
    for (k, &ak) in p.coeffs.iter().enumerate() {
        if ak == Complex::new(0.0, 0.0) {
            continue;
        }
        for (j, coeff) in out.iter_mut().enumerate().take(k + 1) {
            let c = binom_usize(k, j) * h.powi((k - j) as i32);
            *coeff += ak * Complex::new(c, 0.0);
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

pub fn binom_x_plus_k_choose_k_poly(k: usize) -> Polynomial<Complex<f64>> {
    if k == 0 {
        return Polynomial::one();
    }
    let mut p = Polynomial::one();
    for j in 1..=k {
        p = &p * &Polynomial::from_roots(vec![Complex::new(-(j as f64), 0.0)]);
    }
    let mut fact = 1.0f64;
    for j in 1..=k {
        fact *= j as f64;
    }
    &p / Complex::new(fact, 0.0)
}
