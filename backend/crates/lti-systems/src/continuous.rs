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

    /// ZPK 変換（連続）
    pub fn to_zpk(&self) -> crate::zpk::ContinuousZpk {
        crate::zpk::ContinuousZpk::from_transfer_function(self)
    }
    pub fn from_zpk(z: &crate::zpk::ContinuousZpk) -> Self {
        z.to_transfer_function()
    }

    /// インパルス応答（連続系）をサンプリングして長さ len を返す。
    /// 実装は SISO連続TFを ZOH 厳密離散化して離散状態空間でシミュレーションします。
    pub fn impulse_response(&self, fs: f64, len: usize) -> Vec<f64> {
        let ss = self.to_discrete_zoh_statespace(fs);
        crate::responses::impulse_response_discrete_ss(&ss, len)
    }

    /// ステップ応答（連続系）をサンプリングして長さ len を返す。
    pub fn step_response(&self, fs: f64, len: usize) -> Vec<f64> {
        let ss = self.to_discrete_zoh_statespace(fs);
        crate::responses::step_response_discrete_ss(&ss, len)
    }

    /// 周波数応答 G(jω) を n_freqs 点で返す（ω [rad/s] を等間隔サンプル）。
    /// k番目は ω_k = ω_max * k/(n_freqs-1)。ω_max は引数で指定。
    pub fn frequency_response(
        &self,
        omega_max: f64,
        n_freqs: usize,
    ) -> Vec<num_complex::Complex<f64>> {
        use num_complex::Complex;
        if n_freqs == 0 {
            return Vec::new();
        }
        let mut out = Vec::with_capacity(n_freqs);
        for k in 0..n_freqs {
            let t = if n_freqs == 1 {
                0.0
            } else {
                k as f64 / (n_freqs - 1) as f64
            };
            let omega = omega_max.max(0.0) * t;
            let s = Complex::new(0.0, omega);
            out.push(self.eval_s(s));
        }
        out
    }
}
