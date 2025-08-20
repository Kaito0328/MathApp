use num_complex::Complex;

use crate::{
    polynomial::{self, Polynomial},
    rational_function::RationalFunction,
    sequence::core::{ClosedForm, GeneralTerm},
};

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
            // 係数がない場合、定義不明なのでゼロ列を返しておく
            return ClosedForm::zero();
        }

        // 分母 D(x) = 1 - c1 x - c2 x^2 - ... - ck x^k
        let mut d = vec![0.0f64; k + 1];
        d[0] = 1.0;
        for i in 0..k {
            d[i + 1] = -self.coeffs[i];
        }
        let den = Polynomial::new(d);

        // 分子 P(x) = a0 + (a1 - c1 a0)x + (a2 - c1 a1 - c2 a0)x^2 + ...
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
        // 非同次項を D(x) で割って加算
        let mut rf_total = rf;
        for term in &self.non_homogeneous {
            if let Some(gx) = Self::general_term_generating_function(term) {
                // A(x) += (G(x) - prefix(x)) / D(x)
                let prefix = Self::prefix_polynomial(term, k);
                let adj = &gx - &RationalFunction::new(prefix, Polynomial::one());
                rf_total = &rf_total + &adj.div_poly(&den_poly);
            }
        }

        Self::rational_function_to_closed_form(rf_total)
    }

    /// 一般項 P(n) * r^n の生成関数 G(x) を返す。
    /// ここでは r と P の係数が実数（虚部が十分小さい）である場合のみ対応する。
    fn general_term_generating_function(term: &GeneralTerm) -> Option<RationalFunction<f64>> {
        const EPS: f64 = 1e-12;
        // r は実数限定
        if term.base.im.abs() > EPS {
            return None;
        }
        let r = term.base.re;

        // P(n) の係数（n^k）を実数列として取り出す
        let mut coeffs: Vec<f64> = Vec::with_capacity((term.polynomial.deg().max(0)) as usize + 1);
        for c in &term.polynomial.coeffs {
            if c.im.abs() > EPS {
                return None;
            }
            coeffs.push(c.re);
        }
        // 次数 m
        let m = if coeffs.is_empty() {
            0
        } else {
            coeffs.len() - 1
        };

        // 変数 t の多項式として分子 N_k を再帰で生成
        // N_0(t) = 1, N_{k+1} = t(1 - t) N_k'(t) + t (k+1) N_k(t)
        let t_poly: Polynomial<f64> = Polynomial::new(vec![0.0, 1.0]);
        let one: Polynomial<f64> = Polynomial::one();
        let one_minus_t: Polynomial<f64> = &one - &t_poly;

        let mut numerators: Vec<Polynomial<f64>> = Vec::with_capacity(m + 1);
        numerators.push(Polynomial::one()); // N_0
        for k in 0..m {
            let nk = numerators[k].clone();
            let nk_deriv = nk.differentiate();
            let t1 = &(&t_poly * &one_minus_t) * nk_deriv; // t(1-t) N_k'
            let t2 = &(&t_poly * Polynomial::new(vec![k as f64 + 1.0])) * nk; // t (k+1) N_k
            numerators.push(&t1 + &t2);
        }

        // 共通分母 (1 - t)^{m+1} を作り、各 k の寄与を (1 - t)^{m-k} 倍して合成
        // (1 - t)^p を反復で構成
        let mut pow_one_minus_t: Vec<Polynomial<f64>> = Vec::with_capacity(m + 1);
        pow_one_minus_t.push(Polynomial::one()); // p=0
        for _p in 1..=m {
            let prev = pow_one_minus_t.last().unwrap().clone();
            pow_one_minus_t.push(&prev * &one_minus_t);
        }

        let mut num_t = Polynomial::zero();
        for (k, &ak) in coeffs.iter().enumerate() {
            if ak == 0.0 {
                continue;
            }
            // (1 - t)^{m-k}
            let scale = &pow_one_minus_t[m - k];
            let contrib = &numerators[k] * scale;
            num_t = &num_t + &(&contrib * ak);
        }
        // 分母 D_t(t) = (1 - t)^{m+1}
        let mut den_t = Polynomial::one();
        for _ in 0..=m {
            den_t = &den_t * &one_minus_t;
        }

        // 置換 t = r x により、x 多項式へ変換（t^i -> r^i x^i）
        fn subst_rx(mut poly_t: Polynomial<f64>, r: f64) -> Polynomial<f64> {
            // 係数を r^i でスケール
            for (i, c) in poly_t.coeffs.iter_mut().enumerate() {
                *c *= r.powi(i as i32);
            }
            poly_t
        }
        let num_x = subst_rx(num_t, r);
        let den_x = subst_rx(den_t, r);

        Some(RationalFunction::new(num_x, den_x))
    }

    /// 再帰次数 k のとき、非同次項の最初の k 項の寄与を多項式 prefix(x) として返す。
    /// prefix(x) = Σ_{n=0}^{k-1} f(n) x^n
    fn prefix_polynomial(term: &GeneralTerm, k: usize) -> Polynomial<f64> {
        let mut coeffs = vec![0.0f64; k.max(1)];
        for n in 0..k {
            let v = term.polynomial.eval(Complex::new(n as f64, 0.0)) * term.base.powu(n as u32);
            coeffs[n] = v.re; // 実数のみ使用
        }
        Polynomial::new(coeffs)
    }

    fn rational_function_to_closed_form(g: RationalFunction<f64>) -> ClosedForm {
        let p = g.partial_fraction_expansion();
        let polynomial_part = p.polynomial_part;
        let pole_terms = p.pole_terms;

        let mut terms: Vec<GeneralTerm> = Vec::new();

        // 有理関数の部分を閉じた形に変換
        terms.push(GeneralTerm {
            polynomial: polynomial_part.to_complex(),
            base: Complex::new(1.0, 0.0),
        });

        for pole_term in pole_terms.iter() {
            // base = 1 / pole （複素数の逆数）
            let base = Complex::new(1.0, 0.0) / pole_term.pole;

            // 係数 C_j に対応する係数列は C_j * C(n + j - 1, j - 1) * base^n
            // ここで C(n + k, k) を k=0,1,... と更新的に生成する。
            // 初期: k=0 -> 1
            let mut binomial: Polynomial<Complex<f64>> = polynomial::Polynomial::one();

            for (idx, &c) in pole_term.coefficients.iter().enumerate() {
                // idx = j-1, まず現状の binomial を係数 c でスケールして追加
                let j = idx + 1; // 重複次数
                let scale = Complex::new(-1.0, 0.0).powu(j as u32) * base.powu(j as u32);
                terms.push(GeneralTerm {
                    polynomial: &binomial * (c * scale),
                    base,
                });

                // 次の k=idx+1 用に更新: B_{k} = B_{k-1} * (n + k) / k
                // ただし最後の係数処理後には更新不要
                let k_next = idx + 1;
                if k_next < pole_term.coefficients.len() {
                    let factor_poly =
                        Polynomial::from_roots(vec![Complex::new(-(k_next as f64), 0.0)]); // (n + k) = x - (-(k))
                    binomial = &binomial * factor_poly / Complex::new(k_next as f64, 0.0);
                }
            }
        }

        ClosedForm::new(terms)
    }
}
