use crate::combinatorics::numbers::stirling2;
use crate::combinatorics::polynomials::{falling_factorial_poly, shift_poly_x_plus_h};
use num_complex::Complex;
use poly::polynomial::Polynomial;

pub fn discrete_diff(p: &Polynomial<Complex<f64>>) -> Polynomial<Complex<f64>> {
    let shifted = super::super::combinatorics::polynomials::shift_poly_x_plus_h(p, 1.0);
    &shifted - p
}

pub fn discrete_sum(p: &Polynomial<Complex<f64>>) -> Polynomial<Complex<f64>> {
    if p.deg() < 0 {
        return Polynomial::zero();
    }
    let mut acc = Polynomial::zero();
    for (k, ak) in p.coeffs.iter().enumerate() {
        if *ak == Complex::new(0.0, 0.0) {
            continue;
        }
        for j in 0..=k {
            let s2 = stirling2(k, j);
            if s2 == 0.0 {
                continue;
            }
            let ff = falling_factorial_poly(j + 1);
            let ff_shift = shift_poly_x_plus_h(&ff, 1.0);
            let scale = (s2 / (j as f64 + 1.0)) * ak.re;
            acc = &acc + &ff_shift * Complex::new(scale, 0.0);
        }
    }
    acc
}
