use crate::combinatorics::numbers::stirling2;
use crate::combinatorics::polynomials::{falling_factorial_poly, shift_poly_x_plus_h};
use crate::polynomial::Polynomial;
use num_complex::Complex;

/// 離散微分 Δp(n) = p(n+1) - p(n)
pub fn discrete_diff(p: &Polynomial<Complex<f64>>) -> Polynomial<Complex<f64>> {
    // p(x+1) - p(x)
    let shifted = super::super::combinatorics::polynomials::shift_poly_x_plus_h(p, 1.0);
    &shifted - p
}

/// 離散積分（左リーマン和）: 累積和 Q(n) s.t. ΔQ(n) = p(n), Q(0) = 0
/// 多項式の離散積分は上昇階乗基底で一意に決まる
pub fn discrete_sum(p: &Polynomial<Complex<f64>>) -> Polynomial<Complex<f64>> {
    // 多項式 p(x) = Σ a_k x^k を (x)_j に展開: x^k = Σ_{j=0..k} S2(k,j) (x)_j
    // 和: Σ_{i=0..n} (i)_j = (n+1)_{j+1} / (j+1)
    // よって Q(n) = Σ_k a_k Σ_{j=0..k} S2(k,j) (n+1)_{j+1} / (j+1)
    if p.deg() < 0 {
        return Polynomial::zero();
    }
    let mut acc = Polynomial::zero();
    for (k, ak) in p.coeffs.iter().enumerate() {
        if *ak == Complex::new(0.0, 0.0) {
            continue;
        }
        for j in 0..=k {
            let s2 = stirling2(k, j); // f64
            if s2 == 0.0 {
                continue;
            }
            // (n+1)_{j+1}
            let ff = falling_factorial_poly(j + 1);
            let ff_shift = shift_poly_x_plus_h(&ff, 1.0);
            let scale = (s2 / (j as f64 + 1.0)) * ak.re; // 現状: 実係数のみ対応
            acc = &acc + &ff_shift * Complex::new(scale, 0.0);
        }
    }
    acc
}
