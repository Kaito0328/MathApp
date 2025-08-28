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
        const TOLERANCE: f64 = 1e-5; // 極の検出に使用する許容誤差（繰り返し根の数値分離を吸収）
        Polynomial::group_roots(&self.denominator.find_roots(), TOLERANCE)
            .into_iter()
            .map(Pole)
            .collect()
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
                continue; //例外的に無視
            }

            if pole.multiplicity == 1 {
                let num_eval = numerator_complex.eval(pole.value);
                // 分母を微分して、極の値を代入する
                let den_deriv_val = denominator_complex.differentiate().eval(pole.value);

                if den_deriv_val == Complex::new(0.0, 0.0) {
                    // 予期せぬエラーケース
                    continue;
                }

                let coefficient = num_eval / den_deriv_val;
                pole_terms.push(PoleTerm {
                    pole: pole.value,
                    coefficients: vec![coefficient],
                });

                continue;
            }

            let factor = Polynomial::from_roots(vec![pole.value]);
            let mut den_rem = denominator_complex.clone();
            for _ in 0..pole.multiplicity {
                den_rem = &den_rem / &factor; // 割り算で因子を取り除く
            }
            let mut g = RationalFunction::new_internal(numerator_complex.clone(), den_rem);

            let mut coefficients = vec![Complex::new(0.0, 0.0); pole.multiplicity];
            let mut deriv_order = 0usize; // 現在の微分階数 d
            let mut factorial = 1.0f64; // d! を保持（初期値 0! = 1）

            // C_{m}, C_{m-1}, ..., C_{1} の順に計算していき、配列の対応位置に格納
            for j in (1..=pole.multiplicity).rev() {
                // 現在の g の値を p に代入し、d! で割る
                let coeff = g.eval(pole.value).unwrap_or_default() / factorial;
                // j は 1..=m、配列indexは j-1（C_j を格納）
                coefficients[j - 1] = coeff;

                // 次の係数計算の準備（j>1 のときのみ微分して d と d! を更新）
                if j > 1 {
                    g = g.differentiate();
                    deriv_order += 1;
                    factorial *= deriv_order as f64; // d! = d * (d-1)! を更新
                }
            }
            pole_terms.push(PoleTerm {
                pole: pole.value,
                coefficients,
            });
        }

        PartialFractionExpansion {
            polynomial_part,
            pole_terms,
        }
    }
}
