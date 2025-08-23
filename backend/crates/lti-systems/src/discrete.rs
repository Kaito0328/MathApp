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

    /// インパルス応答 h[n]（長さ `len`）を返す。
    /// 最初のサンプルにのみ 1 を与えた入力に対する出力。
    pub fn impulse_response(&self, len: usize) -> Vec<f64> {
        crate::responses::impulse_response_discrete_tf(self, len)
    }

    /// ステップ応答 s[n]（長さ `len`）を返す。
    /// 全サンプル 1 の入力（単位ステップ）に対する出力。
    pub fn step_response(&self, len: usize) -> Vec<f64> {
        crate::responses::step_response_discrete_tf(self, len)
    }

    /// 周波数応答 H(e^{jω}) を `n_freqs` 個の等間隔グリッドで返す（0..2π）。
    /// 返り値の k 番目は ω_k = 2π k / n_freqs における値。
    pub fn frequency_response(&self, n_freqs: usize) -> Vec<Complex<f64>> {
        if n_freqs == 0 {
            return Vec::new();
        }
        (0..n_freqs)
            .map(|k| {
                let omega = 2.0 * std::f64::consts::PI * (k as f64) / (n_freqs as f64);
                let z = Complex::from_polar(1.0, omega);
                self.eval_z(z)
            })
            .collect()
    }

    /// ZPK 変換（離散）
    pub fn to_zpk(&self) -> crate::zpk::DiscreteZpk {
        crate::zpk::DiscreteZpk::from_transfer_function(self)
    }
    pub fn from_zpk(z: &crate::zpk::DiscreteZpk, sample_rate: f64) -> Self {
        z.to_transfer_function(sample_rate)
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
