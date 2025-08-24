use num_complex::Complex;

use poly::{
    polynomial::{self, Polynomial},
    rational_function::RationalFunction,
};

use crate::sequence::core::{ClosedForm, GeneralTerm};

pub struct RecurrenceRelation {
    pub coeffs: Vec<f64>,
    pub non_homogeneous: Vec<GeneralTerm>,
    pub initial_values: Vec<f64>,
}

impl RecurrenceRelation {
    pub fn new(
        coeffs: Vec<f64>,
        non_homogeneous: Vec<GeneralTerm>,
        initial_values: Vec<f64>,
    ) -> Self {
        RecurrenceRelation {
            coeffs,
            non_homogeneous,
            initial_values,
        }
    }

    pub fn solve(&self) -> ClosedForm {
        let k = self.coeffs.len();
        if k == 0 {
            return ClosedForm::zero();
        }

        let mut d = vec![0.0f64; k + 1];
        d[0] = 1.0;
        for i in 0..k {
            d[i + 1] = -self.coeffs[i];
        }
        let den = Polynomial::new(d);

        let mut p = vec![0.0f64; k];
        let mut a = self.initial_values.clone();
        if a.len() < k {
            a.resize(k, 0.0);
        }
        if k >= 1 {
            p[0] = a[0];
            for j in 1..k {
                let mut v = a[j];
                for i in 1..=j {
                    v -= self.coeffs[i - 1] * a[j - i];
                }
                p[j] = v;
            }
        }
        let num = Polynomial::new(p);
        let den_poly = den.clone();
        let rf = RationalFunction::new(num, den);
        let mut rf_total = rf;
        for term in &self.non_homogeneous {
            if let Some(gx) = Self::general_term_generating_function(term) {
                let prefix = Self::prefix_polynomial(term, k);
                let adj = &gx - &RationalFunction::new(prefix, Polynomial::one());
                rf_total = &rf_total + &adj.div_poly(&den_poly);
            }
        }

        Self::rational_function_to_closed_form(rf_total)
    }

    fn general_term_generating_function(term: &GeneralTerm) -> Option<RationalFunction<f64>> {
        const EPS: f64 = 1e-12;
        if term.base.im.abs() > EPS {
            return None;
        }
        let r = term.base.re;

        let mut coeffs: Vec<f64> = Vec::with_capacity((term.polynomial.deg().max(0)) as usize + 1);
        for c in &term.polynomial.coeffs {
            if c.im.abs() > EPS {
                return None;
            }
            coeffs.push(c.re);
        }
        let m = if coeffs.is_empty() {
            0
        } else {
            coeffs.len() - 1
        };

        let t_poly: Polynomial<f64> = Polynomial::new(vec![0.0, 1.0]);
        let one: Polynomial<f64> = Polynomial::one();
        let one_minus_t: Polynomial<f64> = &one - &t_poly;

        let mut numerators: Vec<Polynomial<f64>> = Vec::with_capacity(m + 1);
        numerators.push(Polynomial::one());
        for k in 0..m {
            let nk = numerators[k].clone();
            let nk_deriv = nk.differentiate();
            let t1 = &(&t_poly * &one_minus_t) * nk_deriv;
            let t2 = &(&t_poly * Polynomial::new(vec![k as f64 + 1.0])) * nk;
            numerators.push(&t1 + &t2);
        }

        let mut pow_one_minus_t: Vec<Polynomial<f64>> = Vec::with_capacity(m + 1);
        let mut prev = Polynomial::one();
        pow_one_minus_t.push(prev.clone());
        for _p in 1..=m {
            prev = &prev * &one_minus_t;
            pow_one_minus_t.push(prev.clone());
        }

        let mut num_t = Polynomial::zero();
        for (k, &ak) in coeffs.iter().enumerate() {
            if ak == 0.0 {
                continue;
            }
            let scale = &pow_one_minus_t[m - k];
            let contrib = &numerators[k] * scale;
            num_t = &num_t + &(&contrib * ak);
        }
        let mut den_t = Polynomial::one();
        for _ in 0..=m {
            den_t = &den_t * &one_minus_t;
        }

        fn subst_rx(mut poly_t: Polynomial<f64>, r: f64) -> Polynomial<f64> {
            for (i, c) in poly_t.coeffs.iter_mut().enumerate() {
                *c *= r.powi(i as i32);
            }
            poly_t
        }
        let num_x = subst_rx(num_t, r);
        let den_x = subst_rx(den_t, r);

        Some(RationalFunction::new(num_x, den_x))
    }

    fn prefix_polynomial(term: &GeneralTerm, k: usize) -> Polynomial<f64> {
        let mut coeffs = vec![0.0f64; k.max(1)];
        for (n, c) in coeffs.iter_mut().enumerate() {
            let v = term.polynomial.eval(Complex::new(n as f64, 0.0)) * term.base.powu(n as u32);
            *c = v.re;
        }
        Polynomial::new(coeffs)
    }

    fn rational_function_to_closed_form(g: RationalFunction<f64>) -> ClosedForm {
        let p = g.partial_fraction_expansion();
        let polynomial_part = p.polynomial_part;
        let pole_terms = p.pole_terms;

        let mut terms: Vec<GeneralTerm> = Vec::new();
        terms.push(GeneralTerm {
            polynomial: polynomial_part.to_complex(),
            base: Complex::new(1.0, 0.0),
        });

        for pole_term in pole_terms.iter() {
            let base = Complex::new(1.0, 0.0) / pole_term.pole;
            let mut binomial: Polynomial<Complex<f64>> = polynomial::Polynomial::one();

            for (idx, &c) in pole_term.coefficients.iter().enumerate() {
                let j = idx + 1;
                let scale = Complex::new(-1.0, 0.0).powu(j as u32) * base.powu(j as u32);
                terms.push(GeneralTerm {
                    polynomial: &binomial * (c * scale),
                    base,
                });

                let k_next = idx + 1;
                if k_next < pole_term.coefficients.len() {
                    let factor_poly =
                        Polynomial::from_roots(vec![Complex::new(-(k_next as f64), 0.0)]);
                    binomial = &binomial * factor_poly / Complex::new(k_next as f64, 0.0);
                }
            }
        }

        ClosedForm::new(terms)
    }
}
