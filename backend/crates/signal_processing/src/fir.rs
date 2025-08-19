use std::f64::consts::PI;

use linalg::Vector;

use crate::window::{generate_window, WindowType};

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

    // --- ステップ1: 理想フィルタ（Sinc関数）のインパルス応答を計算 ---
    let mut ideal_response = Vec::with_capacity(num_taps);
    let center = (num_taps - 1) as f64 / 2.0;

    for i in 0..num_taps {
        let t = i as f64 - center;
        if t == 0.0 {
            // 中心点 (t=0) の場合
            ideal_response.push(2.0 * normalized_cutoff);
        } else {
            // それ以外の点
            let pi_t = PI * t;
            ideal_response.push((2.0 * normalized_cutoff * pi_t).sin() / pi_t);
        }
    }

    // --- ステップ2: 窓関数を生成 ---
    let window = generate_window(num_taps, window_type);

    // --- ステップ3: 要素ごとに掛け合わせて最終的な係数を計算 ---
    Vector::new(ideal_response) * window
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

    let window = generate_window(num_taps, window_type);
    Vector::new(ideal_response) * window
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

    let window = generate_window(num_taps, window_type);
    Vector::new(ideal_response) * window
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

    let window = generate_window(num_taps, window_type);
    Vector::new(ideal_response) * window
}
