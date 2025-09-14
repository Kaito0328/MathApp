use std::ops::Deref;

use crate::{
    polynomial::{solver::Root, Polynomial},
    rational_function::core::RationalFunction,
};
use linalg::Field;
use num_complex::Complex;
use num_traits::FromPrimitive;

impl<F: Field> RationalFunction<F> {
    pub fn eval(&self, x: F) -> Option<F> {
        let numerator_value = self.numerator.eval(x.clone());
        let denominator_value = self.denominator.eval(x.clone());
        if denominator_value.is_zero() {
            return None;
        }
        Some(numerator_value / denominator_value)
    }
}

impl<F: Field + FromPrimitive> RationalFunction<F> {
    pub fn differentiate(&self) -> Self {
        let numerator_deriv = self.numerator.differentiate();
        let denominator_deriv = self.denominator.differentiate();
        Self {
            numerator: numerator_deriv * &self.denominator - &self.numerator * denominator_deriv,
            denominator: &self.denominator * &self.denominator,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pole(Root);

impl Deref for Pole {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PoleTerm {
    /// 極の値 p
    pub pole: Complex<f64>,
    /// 係数のリスト [C₁, C₂, ..., Cₘ]
    /// Cⱼ は (s - p)ʲ の項の分子係数。
    pub coefficients: Vec<Complex<f64>>, // ← 各係数は複素数の定数なので、これで正しい
}

/// 有理関数の部分分数分解の結果全体を表現する構造体。
#[derive(Debug, Clone, PartialEq)]
pub struct PartialFractionExpansion {
    /// 多項式部分。分子の次数 < 分母の次数 の場合はゼロ多項式。
    pub polynomial_part: Polynomial<f64>,
    /// 極と係数の項のリスト。
    pub pole_terms: Vec<PoleTerm>,
}

impl RationalFunction<f64> {
    pub fn find_poles(&self) -> Vec<Pole> {
        // ルートを求め、数値誤差の影響を最小化するための前処理と段階的クラスタリングを行う
        let mut roots = self.denominator.find_roots();

        // 実軸に近い虚部を0にスナップ、特に 1 と -1 付近は強めにスナップ
        for z in &mut roots {
            if z.im.abs() < 1e-12 {
                z.im = 0.0;
            }
            if (z.re - 1.0).abs() < 1e-8 && z.im == 0.0 {
                z.re = 1.0;
            } else if (z.re + 1.0).abs() < 1e-8 && z.im == 0.0 {
                z.re = -1.0;
            }
        }

        // 段階的にしきい値を上げながらグルーピングを試みる
        let tolerances = [1e-12, 1e-10, 1e-8, 1e-6, 2e-5, 5e-5, 1e-4, 5e-4, 1e-3];
        let total = roots.len();
        let mut best = Polynomial::group_roots(&roots, 1e-12);
        let mut best_score = (best.len(), 0usize); // (cluster count, max multiplicity)
        for tol in tolerances {
            let mut grouped = Polynomial::group_roots(&roots, tol);
            for g in &mut grouped {
                if g.value.im.abs() < tol {
                    g.value.im = 0.0;
                }
                if (g.value.re - 1.0).abs() < 10.0 * tol && g.value.im == 0.0 {
                    g.value.re = 1.0;
                } else if (g.value.re + 1.0).abs() < 10.0 * tol && g.value.im == 0.0 {
                    g.value.re = -1.0;
                }
            }
            let sum_mult: usize = grouped.iter().map(|g| g.multiplicity).sum();
            if sum_mult != total { continue; }
            let max_mult = grouped.iter().map(|g| g.multiplicity).max().unwrap_or(1);
            let score = (grouped.len(), total - max_mult); // prefer fewer clusters, larger max mult
            if score < best_score {
                best = grouped;
                best_score = score;
            }
        }
        best.into_iter().map(Pole).collect()
    }

    pub fn partial_fraction_expansion(&self) -> PartialFractionExpansion {
        let mut numerator = self.numerator.clone();
        let mut polynomial_part = Polynomial::zero();
        if self.denominator.deg() < self.numerator.deg() {
            (numerator, polynomial_part) = numerator.div_rem(&self.denominator);
        }

        let numerator_complex = numerator.to_complex();
        let denominator_complex = self.denominator.to_complex();
        let poles = self.find_poles();

        let mut pole_terms = Vec::with_capacity(poles.len());

        for pole in poles {
            if pole.multiplicity < 1 {
                continue; // 例外的に無視
            }

            // 安定化戦略（単純極も含め統一処理）
            // g(x) = N(x) / (D(x)/(x-p)^m) = (x-p)^m * N(x) / D(x)
            // 係数は C_j = g^{(m-j)}(p) / (m-j)!（j = m, m-1, ..., 1）
            let factor = Polynomial::from_roots(vec![pole.value]);
            let mut den_rem = denominator_complex.clone();
            for _ in 0..pole.multiplicity {
                den_rem = &den_rem / &factor; // (x-p)^m を取り除く
            }
            let mut g = RationalFunction::new_internal(numerator_complex.clone(), den_rem);

            let mut coefficients = vec![Complex::new(0.0, 0.0); pole.multiplicity];
            let mut deriv_order = 0usize; // d = 0 から開始
            let mut factorial = 1.0f64; // d!

            // j = m..1 に対して、C_j = g^{(m-j)}(p) / (m-j)! を計算
            for j in (1..=pole.multiplicity).rev() {
                let val = g.eval(pole.value).unwrap_or_default();
                let coeff = val / factorial; // g^{(d)}(p) / d!
                coefficients[j - 1] = coeff;

                if j > 1 {
                    g = g.differentiate();
                    deriv_order += 1;
                    factorial *= deriv_order as f64; // d! を更新
                }
            }
            pole_terms.push(PoleTerm {
                pole: pole.value,
                coefficients,
            });
        }

        let pfe = PartialFractionExpansion {
            polynomial_part,
            pole_terms,
        };
        if std::env::var("PFE_DEBUG").is_ok() {
            eprintln!("[PFE] poly_part={}", pfe.polynomial_part);
            for (i, t) in pfe.pole_terms.iter().enumerate() {
                eprintln!("[PFE] term#{i} pole=({:.6},{:.6}) mult={} coeffs={:?}", t.pole.re, t.pole.im, t.coefficients.len(), t.coefficients);
            }
        }
        pfe
    }
}
