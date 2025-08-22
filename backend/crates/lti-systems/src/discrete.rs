use num_complex::Complex;
use poly::{polynomial::Polynomial, rational_function::RationalFunction};

/// 離散系の伝達関数 H(z) = B(z) / A(z)
#[derive(Clone, Debug, PartialEq)]
pub struct TransferFunction {
    pub ratio: RationalFunction<f64>,
    sample_rate: f64,
}

impl TransferFunction {
    /// 既定のサンプリング周波数 1.0 Hz を設定する互換コンストラクタ
    pub fn new(b: Polynomial<f64>, a: Polynomial<f64>) -> Self {
        Self::new_with_fs(b, a, 1.0)
    }
    /// 係数ベクタから生成（既定のサンプリング周波数 1.0 Hz）
    pub fn from_coeffs(b: Vec<f64>, a: Vec<f64>) -> Self {
        Self::new(Polynomial::new(b), Polynomial::new(a))
    }
    /// サンプリング周波数を指定して生成
    pub fn new_with_fs(b: Polynomial<f64>, a: Polynomial<f64>, sample_rate: f64) -> Self {
        assert!(sample_rate > 0.0, "sample_rate must be positive");
        let ratio = RationalFunction::new(b, a);
        Self { ratio, sample_rate }
    }
    /// サンプリング周波数の取得
    pub fn sample_rate(&self) -> f64 {
        self.sample_rate
    }
    /// サンプリング周波数の更新
    pub fn set_sample_rate(&mut self, fs: f64) {
        assert!(fs > 0.0, "fs must be positive");
        self.sample_rate = fs;
    }
    pub fn zeros(&self) -> Vec<Complex<f64>> {
        self.ratio.numerator.find_roots()
    }
    pub fn poles(&self) -> Vec<Complex<f64>> {
        self.ratio.denominator.find_roots()
    }
    pub fn is_stable(&self) -> bool {
        self.poles().into_iter().all(|p| p.norm() < 1.0)
    }
    pub fn eval_z(&self, z: Complex<f64>) -> Complex<f64> {
        let num = self.ratio.numerator.to_complex().eval(z);
        let den = self.ratio.denominator.to_complex().eval(z);
        num / den
    }
    pub fn b_coeffs(&self) -> &[f64] {
        &self.ratio.numerator.coeffs
    }
    pub fn a_coeffs(&self) -> &[f64] {
        &self.ratio.denominator.coeffs
    }

    /// 入力信号に伝達関数を適用する（直接形I）。
    /// FIR（a = [1]）/ IIR（a 長さ > 1）の両方に対応。
    pub fn apply(&self, input: &[f64]) -> Vec<f64> {
        apply_direct_form(
            &self.ratio.numerator.coeffs,
            &self.ratio.denominator.coeffs,
            input,
        )
    }
}

// --- 共通ロジック: 直接形Iの時間領域適用（離散系で使用） ---
fn apply_direct_form(b: &[f64], a: &[f64], input: &[f64]) -> Vec<f64> {
    let n = input.len();
    let mut y = vec![0.0; n];
    let a0 = if a.is_empty() { 1.0 } else { a[0] };
    assert!(a0 != 0.0, "a[0] must not be zero");
    for idx in 0..n {
        // 分子畳み込み部分（FIR 部分）
        let mut b_sum = 0.0;
        for (i, &bi) in b.iter().enumerate() {
            if idx >= i {
                b_sum += bi * input[idx - i];
            }
        }
        // 分母帰還部分（IIR 部分, a[0] は除外）
        let mut a_sum = 0.0;
        for (j, &aj) in a.iter().enumerate().skip(1) {
            if idx >= j {
                a_sum += aj * y[idx - j];
            }
        }
        y[idx] = (b_sum - a_sum) / a0;
    }
    y
}
