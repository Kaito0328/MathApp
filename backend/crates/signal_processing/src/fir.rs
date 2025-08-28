use std::f64::consts::PI;

use crate::signal::Signal;
use lti_systems::{DiscreteTransferFunction, Polynomial};

use crate::window::{generate_window, WindowType};

/// FIR フィルタ型（タップ係数を多項式で保持、低次→高次）。
#[derive(Clone, Debug, PartialEq)]
pub struct FIRFilter {
    tf: DiscreteTransferFunction,
}

impl FIRFilter {
    pub fn new_from_coeffs(coeffs: Vec<f64>) -> Self {
        let b = Polynomial::new(coeffs);
        let a = Polynomial::new(vec![1.0]);
        Self {
            tf: DiscreteTransferFunction::new(b, a),
        }
    }
    /// サンプリング周波数を保持するバージョン
    pub fn new_from_coeffs_with_fs(coeffs: Vec<f64>, fs: f64) -> Self {
        let b = Polynomial::new(coeffs);
        let a = Polynomial::new(vec![1.0]);
        Self {
            tf: DiscreteTransferFunction::new_with_fs(b, a, fs),
        }
    }
    pub fn new_from_poly(p: Polynomial<f64>) -> Self {
        let a = Polynomial::new(vec![1.0]);
        Self {
            tf: DiscreteTransferFunction::new(p, a),
        }
    }
    /// サンプリング周波数を保持するバージョン
    pub fn new_from_poly_with_fs(p: Polynomial<f64>, fs: f64) -> Self {
        let a = Polynomial::new(vec![1.0]);
        Self {
            tf: DiscreteTransferFunction::new_with_fs(p, a, fs),
        }
    }
    pub fn len(&self) -> usize {
        self.tf.b_coeffs().len()
    }
    pub fn is_empty(&self) -> bool {
        self.tf.b_coeffs().is_empty()
    }
    pub fn coeffs(&self) -> &[f64] {
        self.tf.b_coeffs()
    }
    /// 直接法で適用
    pub fn apply(&self, x: &Signal) -> Signal {
        let y = self.tf.apply(x.data());
        Signal::new(y, x.sample_rate())
    }

    // --- static design methods (wrapping existing poly-based designers) ---
    pub fn design_lowpass(
        num_taps: usize,
        normalized_cutoff: f64,
        window_type: WindowType,
    ) -> Self {
        Self::new_from_poly(design_fir_lowpass_poly(
            num_taps,
            normalized_cutoff,
            window_type,
        ))
    }
    pub fn design_highpass(
        num_taps: usize,
        normalized_cutoff: f64,
        window_type: WindowType,
    ) -> Self {
        Self::new_from_poly(design_fir_highpass_poly(
            num_taps,
            normalized_cutoff,
            window_type,
        ))
    }
    pub fn design_bandpass(
        num_taps: usize,
        normalized_f1: f64,
        normalized_f2: f64,
        window_type: WindowType,
    ) -> Self {
        Self::new_from_poly(design_fir_bandpass_poly(
            num_taps,
            normalized_f1,
            normalized_f2,
            window_type,
        ))
    }
    pub fn design_bandstop(
        num_taps: usize,
        normalized_f1: f64,
        normalized_f2: f64,
        window_type: WindowType,
    ) -> Self {
        Self::new_from_poly(design_fir_bandstop_poly(
            num_taps,
            normalized_f1,
            normalized_f2,
            window_type,
        ))
    }
}

// Signal側の apply_fir_filter は signal.rs に実装済み

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
) -> Vec<f64> {
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
    p.coeffs
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
) -> Vec<f64> {
    assert!(num_taps > 0, "Number of taps must be positive.");
    assert!(
        normalized_cutoff > 0.0 && normalized_cutoff < 0.5,
        "Normalized cutoff must be between 0.0 and 0.5."
    );
    assert!(
        num_taps % 2 != 0,
        "Number of taps should be odd for a Type I filter."
    );

    let p = design_fir_highpass_poly(num_taps, normalized_cutoff, window_type);
    p.coeffs
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
) -> Vec<f64> {
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

    let p = design_fir_bandpass_poly(num_taps, normalized_f1, normalized_f2, window_type);
    p.coeffs
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
) -> Vec<f64> {
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

    let p = design_fir_bandstop_poly(num_taps, normalized_f1, normalized_f2, window_type);
    p.coeffs
}

/// FIR フィルタを時間信号に適用（直接法）。既存の apply と同等の動作。
pub fn apply_fir_signal(taps: &Polynomial<f64>, x: &Signal) -> Signal {
    // DiscreteTransferFunction に委譲（a = [1]）。入力信号のFsを保持。
    let tf = DiscreteTransferFunction::new_with_fs(
        taps.clone(),
        Polynomial::new(vec![1.0]),
        x.sample_rate(),
    );
    let y = tf.apply(x.data());
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
    FIRFilter::design_lowpass(num_taps, normalized_cutoff, window_type)
}

pub fn design_fir_highpass_filter(
    num_taps: usize,
    normalized_cutoff: f64,
    window_type: WindowType,
) -> FIRFilter {
    FIRFilter::design_highpass(num_taps, normalized_cutoff, window_type)
}

pub fn design_fir_bandpass_filter(
    num_taps: usize,
    normalized_f1: f64,
    normalized_f2: f64,
    window_type: WindowType,
) -> FIRFilter {
    FIRFilter::design_bandpass(num_taps, normalized_f1, normalized_f2, window_type)
}

pub fn design_fir_bandstop_filter(
    num_taps: usize,
    normalized_f1: f64,
    normalized_f2: f64,
    window_type: WindowType,
) -> FIRFilter {
    FIRFilter::design_bandstop(num_taps, normalized_f1, normalized_f2, window_type)
}
