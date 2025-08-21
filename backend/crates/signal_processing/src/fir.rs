use std::f64::consts::PI;

use crate::signal::Signal;
use linalg::Vector;
use poly::Polynomial;

use crate::window::{generate_window, WindowType};

/// FIR フィルタ型（タップ係数を多項式で保持、低次→高次）。
#[derive(Clone, Debug, PartialEq)]
pub struct FIRFilter {
    pub taps: Polynomial<f64>,
}

impl FIRFilter {
    pub fn new_from_coeffs(coeffs: Vec<f64>) -> Self {
        Self {
            taps: Polynomial::new(coeffs),
        }
    }
    pub fn new_from_poly(p: Polynomial<f64>) -> Self {
        Self { taps: p }
    }
    pub fn len(&self) -> usize {
        self.taps.coeffs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.taps.coeffs.is_empty()
    }
    pub fn coeffs(&self) -> &[f64] {
        &self.taps.coeffs
    }
    /// 直接法で適用
    pub fn apply(&self, x: &Signal) -> Signal {
        apply_fir_signal(&self.taps, x)
    }
}

/// 窓関数法を用いてFIRローパスフィルタの係数を設計する。
///
/// # 引数
/// * `num_taps` - フィルタのタップ数（長さ）。奇数を推奨。
/// * `normalized_cutoff` - 正規化されたカットオフ周波数 (0.0 < f < 0.5)。
/// * `window_type` - 使用する窓関数の種類。
pub fn design_fir_lowpass(
    num_taps: usize,
    normalized_cutoff: f64,
    window_type: WindowType,
) -> Vector<f64> {
    // 前提条件のチェック
    assert!(num_taps > 0, "Number of taps must be positive.");
    assert!(
        normalized_cutoff > 0.0 && normalized_cutoff < 0.5,
        "Normalized cutoff must be between 0.0 and 0.5."
    );
    assert!(
        num_taps % 2 != 0,
        "Number of taps should be odd for a Type I filter."
    );

    // 係数をPolynomialで生成し、最終的にVectorへ変換
    let p = design_fir_lowpass_poly(num_taps, normalized_cutoff, window_type);
    Vector::new(p.coeffs)
}

/// 窓関数法を用いてFIRハイパスフィルタの係数を設計する。
///
/// # 引数
/// * `num_taps` - フィルタのタップ数（長さ）。奇数を推奨。
/// * `normalized_cutoff` - 正規化されたカットオフ周波数 (0.0 < f < 0.5)。
/// * `window_type` - 使用する窓関数の種類。
pub fn design_fir_highpass(
    num_taps: usize,
    normalized_cutoff: f64,
    window_type: WindowType,
) -> Vector<f64> {
    assert!(num_taps > 0, "Number of taps must be positive.");
    assert!(
        normalized_cutoff > 0.0 && normalized_cutoff < 0.5,
        "Normalized cutoff must be between 0.0 and 0.5."
    );
    assert!(
        num_taps % 2 != 0,
        "Number of taps should be odd for a Type I filter."
    );

    // 理想ハイパス: スペクトル反転（δ[n−M] − ローパス理想応答）
    let mut ideal_response = Vec::with_capacity(num_taps);
    let center = (num_taps - 1) as f64 / 2.0;
    for i in 0..num_taps {
        let t = i as f64 - center;
        if t == 0.0 {
            ideal_response.push(1.0 - 2.0 * normalized_cutoff);
        } else {
            let pi_t = PI * t;
            ideal_response.push(-((2.0 * normalized_cutoff * pi_t).sin() / pi_t));
        }
    }

    let p = design_fir_highpass_poly(num_taps, normalized_cutoff, window_type);
    Vector::new(p.coeffs)
}

/// 窓関数法を用いてFIRバンドパスフィルタの係数を設計する。
///
/// # 引数
/// * `num_taps` - フィルタのタップ数（長さ）。奇数を推奨。
/// * `normalized_f1` - 下側の正規化カットオフ (0.0 < f1 < 0.5)。
/// * `normalized_f2` - 上側の正規化カットオフ (f1 < f2 < 0.5)。
/// * `window_type` - 使用する窓関数の種類。
pub fn design_fir_bandpass(
    num_taps: usize,
    normalized_f1: f64,
    normalized_f2: f64,
    window_type: WindowType,
) -> Vector<f64> {
    assert!(num_taps > 0, "Number of taps must be positive.");
    assert!(
        normalized_f1 > 0.0 && normalized_f1 < 0.5,
        "normalized_f1 must be between 0.0 and 0.5."
    );
    assert!(
        normalized_f2 > 0.0 && normalized_f2 < 0.5,
        "normalized_f2 must be between 0.0 and 0.5."
    );
    assert!(
        normalized_f1 < normalized_f2,
        "require f1 < f2 for bandpass"
    );
    assert!(
        num_taps % 2 != 0,
        "Number of taps should be odd for a Type I filter."
    );

    // 理想バンドパス: h_bp = h_lp(f2) - h_lp(f1)
    let mut ideal_response = Vec::with_capacity(num_taps);
    let center = (num_taps - 1) as f64 / 2.0;
    for i in 0..num_taps {
        let t = i as f64 - center;
        if t == 0.0 {
            ideal_response.push(2.0 * (normalized_f2 - normalized_f1));
        } else {
            let pi_t = PI * t;
            let num = (2.0 * normalized_f2 * pi_t).sin() - (2.0 * normalized_f1 * pi_t).sin();
            ideal_response.push(num / pi_t);
        }
    }

    let p = design_fir_bandpass_poly(num_taps, normalized_f1, normalized_f2, window_type);
    Vector::new(p.coeffs)
}

/// 窓関数法を用いてFIRバンドストップ（帯域阻止）フィルタの係数を設計する。
///
/// # 引数
/// * `num_taps` - フィルタのタップ数（長さ）。奇数を推奨。
/// * `normalized_f1` - 下側の正規化カットオフ (0.0 < f1 < 0.5)。
/// * `normalized_f2` - 上側の正規化カットオフ (f1 < f2 < 0.5)。
/// * `window_type` - 使用する窓関数の種類。
pub fn design_fir_bandstop(
    num_taps: usize,
    normalized_f1: f64,
    normalized_f2: f64,
    window_type: WindowType,
) -> Vector<f64> {
    assert!(num_taps > 0, "Number of taps must be positive.");
    assert!(
        normalized_f1 > 0.0 && normalized_f1 < 0.5,
        "normalized_f1 must be between 0.0 and 0.5."
    );
    assert!(
        normalized_f2 > 0.0 && normalized_f2 < 0.5,
        "normalized_f2 must be between 0.0 and 0.5."
    );
    assert!(
        normalized_f1 < normalized_f2,
        "require f1 < f2 for bandstop"
    );
    assert!(
        num_taps % 2 != 0,
        "Number of taps should be odd for a Type I filter."
    );

    // 理想バンドストップ: h_bs = δ + h_lp(f1) - h_lp(f2)
    let mut ideal_response = Vec::with_capacity(num_taps);
    let center = (num_taps - 1) as f64 / 2.0;
    for i in 0..num_taps {
        let t = i as f64 - center;
        if t == 0.0 {
            ideal_response.push(1.0 - 2.0 * (normalized_f2 - normalized_f1));
        } else {
            let pi_t = PI * t;
            let num = (2.0 * normalized_f1 * pi_t).sin() - (2.0 * normalized_f2 * pi_t).sin();
            ideal_response.push(num / pi_t);
        }
    }

    let p = design_fir_bandstop_poly(num_taps, normalized_f1, normalized_f2, window_type);
    Vector::new(p.coeffs)
}

/// FIR フィルタを時間信号に適用（直接法）。既存の apply と同等の動作。
pub fn apply_fir_signal(taps: &Polynomial<f64>, x: &Signal) -> Signal {
    let h = &taps.coeffs; // 低次→高次 = h[0..]
    let n = x.len();
    let m = h.len();
    let xx = x.as_ref();
    let mut y = vec![0.0; n];
    for i in 0..n {
        let kmax = m.min(i + 1);
        let mut acc = 0.0;
        for k in 0..kmax {
            acc += h[k] * xx[i - k];
        }
        y[i] = acc;
    }
    Signal::new(y, x.sample_rate())
}

// ここからPolynomial版API。アルゴリズムは既存と同一で、係数列を多項式係数として保持する。

/// FIRローパス係数のPolynomial版。係数は低次→高次で h[0], h[1], ... を x^0, x^1, ... に対応付ける。
pub fn design_fir_lowpass_poly(
    num_taps: usize,
    normalized_cutoff: f64,
    window_type: WindowType,
) -> Polynomial<f64> {
    assert!(num_taps > 0, "Number of taps must be positive.");
    assert!(
        normalized_cutoff > 0.0 && normalized_cutoff < 0.5,
        "Normalized cutoff must be between 0.0 and 0.5."
    );
    assert!(
        num_taps % 2 != 0,
        "Number of taps should be odd for a Type I filter."
    );

    let mut ideal_response = Vec::with_capacity(num_taps);
    let center = (num_taps - 1) as f64 / 2.0;
    for i in 0..num_taps {
        let t = i as f64 - center;
        if t == 0.0 {
            ideal_response.push(2.0 * normalized_cutoff);
        } else {
            let pi_t = PI * t;
            ideal_response.push((2.0 * normalized_cutoff * pi_t).sin() / pi_t);
        }
    }
    let window = generate_window(num_taps, window_type);
    let coeffs: Vec<f64> = ideal_response
        .into_iter()
        .zip(window.iter().cloned())
        .map(|(h, w)| h * w)
        .collect();
    Polynomial::new(coeffs)
}

/// FIRハイパス係数のPolynomial版
pub fn design_fir_highpass_poly(
    num_taps: usize,
    normalized_cutoff: f64,
    window_type: WindowType,
) -> Polynomial<f64> {
    assert!(num_taps > 0, "Number of taps must be positive.");
    assert!(
        normalized_cutoff > 0.0 && normalized_cutoff < 0.5,
        "Normalized cutoff must be between 0.0 and 0.5."
    );
    assert!(
        num_taps % 2 != 0,
        "Number of taps should be odd for a Type I filter."
    );

    let mut ideal_response = Vec::with_capacity(num_taps);
    let center = (num_taps - 1) as f64 / 2.0;
    for i in 0..num_taps {
        let t = i as f64 - center;
        if t == 0.0 {
            ideal_response.push(1.0 - 2.0 * normalized_cutoff);
        } else {
            let pi_t = PI * t;
            ideal_response.push(-((2.0 * normalized_cutoff * pi_t).sin() / pi_t));
        }
    }
    let window = generate_window(num_taps, window_type);
    let coeffs: Vec<f64> = ideal_response
        .into_iter()
        .zip(window.iter().cloned())
        .map(|(h, w)| h * w)
        .collect();
    Polynomial::new(coeffs)
}

/// FIRバンドパス係数のPolynomial版
pub fn design_fir_bandpass_poly(
    num_taps: usize,
    normalized_f1: f64,
    normalized_f2: f64,
    window_type: WindowType,
) -> Polynomial<f64> {
    assert!(num_taps > 0, "Number of taps must be positive.");
    assert!(
        normalized_f1 > 0.0 && normalized_f1 < 0.5,
        "normalized_f1 must be between 0.0 and 0.5."
    );
    assert!(
        normalized_f2 > 0.0 && normalized_f2 < 0.5,
        "normalized_f2 must be between 0.0 and 0.5."
    );
    assert!(
        normalized_f1 < normalized_f2,
        "require f1 < f2 for bandpass"
    );
    assert!(
        num_taps % 2 != 0,
        "Number of taps should be odd for a Type I filter."
    );

    let mut ideal_response = Vec::with_capacity(num_taps);
    let center = (num_taps - 1) as f64 / 2.0;
    for i in 0..num_taps {
        let t = i as f64 - center;
        if t == 0.0 {
            ideal_response.push(2.0 * (normalized_f2 - normalized_f1));
        } else {
            let pi_t = PI * t;
            let num = (2.0 * normalized_f2 * pi_t).sin() - (2.0 * normalized_f1 * pi_t).sin();
            ideal_response.push(num / pi_t);
        }
    }
    let window = generate_window(num_taps, window_type);
    let coeffs: Vec<f64> = ideal_response
        .into_iter()
        .zip(window.iter().cloned())
        .map(|(h, w)| h * w)
        .collect();
    Polynomial::new(coeffs)
}

/// FIRバンドストップ係数のPolynomial版
pub fn design_fir_bandstop_poly(
    num_taps: usize,
    normalized_f1: f64,
    normalized_f2: f64,
    window_type: WindowType,
) -> Polynomial<f64> {
    assert!(num_taps > 0, "Number of taps must be positive.");
    assert!(
        normalized_f1 > 0.0 && normalized_f1 < 0.5,
        "normalized_f1 must be between 0.0 and 0.5."
    );
    assert!(
        normalized_f2 > 0.0 && normalized_f2 < 0.5,
        "normalized_f2 must be between 0.0 and 0.5."
    );
    assert!(
        normalized_f1 < normalized_f2,
        "require f1 < f2 for bandstop"
    );
    assert!(
        num_taps % 2 != 0,
        "Number of taps should be odd for a Type I filter."
    );

    let mut ideal_response = Vec::with_capacity(num_taps);
    let center = (num_taps - 1) as f64 / 2.0;
    for i in 0..num_taps {
        let t = i as f64 - center;
        if t == 0.0 {
            ideal_response.push(1.0 - 2.0 * (normalized_f2 - normalized_f1));
        } else {
            let pi_t = PI * t;
            let num = (2.0 * normalized_f1 * pi_t).sin() - (2.0 * normalized_f2 * pi_t).sin();
            ideal_response.push(num / pi_t);
        }
    }
    let window = generate_window(num_taps, window_type);
    let coeffs: Vec<f64> = ideal_response
        .into_iter()
        .zip(window.iter().cloned())
        .map(|(h, w)| h * w)
        .collect();
    Polynomial::new(coeffs)
}

// === 新 API: FIRFilter を返す設計関数 ===
pub fn design_fir_lowpass_filter(
    num_taps: usize,
    normalized_cutoff: f64,
    window_type: WindowType,
) -> FIRFilter {
    FIRFilter::new_from_poly(design_fir_lowpass_poly(
        num_taps,
        normalized_cutoff,
        window_type,
    ))
}

pub fn design_fir_highpass_filter(
    num_taps: usize,
    normalized_cutoff: f64,
    window_type: WindowType,
) -> FIRFilter {
    FIRFilter::new_from_poly(design_fir_highpass_poly(
        num_taps,
        normalized_cutoff,
        window_type,
    ))
}

pub fn design_fir_bandpass_filter(
    num_taps: usize,
    normalized_f1: f64,
    normalized_f2: f64,
    window_type: WindowType,
) -> FIRFilter {
    FIRFilter::new_from_poly(design_fir_bandpass_poly(
        num_taps,
        normalized_f1,
        normalized_f2,
        window_type,
    ))
}

pub fn design_fir_bandstop_filter(
    num_taps: usize,
    normalized_f1: f64,
    normalized_f2: f64,
    window_type: WindowType,
) -> FIRFilter {
    FIRFilter::new_from_poly(design_fir_bandstop_poly(
        num_taps,
        normalized_f1,
        normalized_f2,
        window_type,
    ))
}
