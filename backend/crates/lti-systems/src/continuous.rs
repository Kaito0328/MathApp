use crate::conversions::{bilinear_transform, bilinear_transform_prewarp, inverse_tustin};
use crate::statespace::{tf_c2d_zoh_siso, DiscreteStateSpace};
use num_complex::Complex;
use poly::{polynomial::Polynomial, rational_function::RationalFunction};

/// 連続系の伝達関数 G(s) = B(s)/A(s)
#[derive(Clone, Debug, PartialEq)]
pub struct TransferFunction {
    pub ratio: RationalFunction<f64>,
}

impl TransferFunction {
    pub fn new(b: Polynomial<f64>, a: Polynomial<f64>) -> Self {
        Self {
            ratio: RationalFunction::new(b, a),
        }
    }
    pub fn from_coeffs(b: Vec<f64>, a: Vec<f64>) -> Self {
        Self::new(Polynomial::new(b), Polynomial::new(a))
    }
    pub fn zeros(&self) -> Vec<Complex<f64>> {
        self.ratio.numerator.find_roots()
    }
    pub fn poles(&self) -> Vec<Complex<f64>> {
        self.ratio.denominator.find_roots()
    }
    pub fn is_stable(&self) -> bool {
        self.poles().into_iter().all(|p| p.re < 0.0)
    }
    pub fn eval_s(&self, s: Complex<f64>) -> Complex<f64> {
        let num = self.ratio.numerator.to_complex().eval(s);
        let den = self.ratio.denominator.to_complex().eval(s);
        num / den
    }
    pub fn b_coeffs(&self) -> &[f64] {
        &self.ratio.numerator.coeffs
    }
    pub fn a_coeffs(&self) -> &[f64] {
        &self.ratio.denominator.coeffs
    }

    /// 双一次変換で離散系へ変換（Tustin）。戻り値は離散系 TransferFunction（Z 変換領域）。
    pub fn to_discrete_bilinear(&self, fs: f64) -> crate::discrete::TransferFunction {
    bilinear_transform(self, fs)
    }

    /// プリワープ付き双一次（指定周波数 f_warp[Hz]）
    pub fn to_discrete_bilinear_prewarp(
        &self,
        fs: f64,
        f_warp_hz: f64,
    ) -> crate::discrete::TransferFunction {
    bilinear_transform_prewarp(self, fs, f_warp_hz)
    }

    /// 逆Tustin（離散TF → 連続TF）: k を直接指定
    pub fn from_discrete_inverse_tustin_k(
        discrete: &crate::discrete::TransferFunction,
        k: f64,
    ) -> Self {
    inverse_tustin(discrete, k)
    }

    /// 厳密ZOH離散化（SISO前提）: 離散の状態空間を返す
    pub fn to_discrete_zoh_statespace(&self, fs: f64) -> DiscreteStateSpace {
        tf_c2d_zoh_siso(&self.ratio.numerator, &self.ratio.denominator, fs)
    }
}
