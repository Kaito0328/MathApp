use linalg::Vector;

use crate::{
    dft::conv_with_dft_for_f64, fir::design_fir_lowpass, signal::Signal, window::WindowType,
};

/// 信号を整数倍でダウンサンプリングする。
///
/// 内部で自動的にアンチエイリアシング用のFIRローパスフィルタを設計・適用する。
///
/// # 引数
/// * `signal` - 入力信号。
/// * `factor` - ダウンサンプリングする倍率（2以上の整数）。
/// * `filter_taps` - 内部で使用するFIRフィルタのタップ数。奇数を推奨。
///
/// # 戻り値
/// * ダウンサンプリングされた信号。長さは元信号の約 1/factor になる。
pub fn down_sample(
    signal: &Vector<f64>,
    factor: usize,
    filter_taps: usize,
    window_type: WindowType,
) -> Vector<f64> {
    if factor == 0 {
        return Vector::new(vec![]);
    }

    let normalized_cutoff = 0.5 / factor as f64; // 正規化されたカットオフ周波数

    let lowpass_fil = design_fir_lowpass(filter_taps, normalized_cutoff, window_type);
    let signal_filtered = conv_with_dft_for_f64(signal, &lowpass_fil);
    decimate(&signal_filtered, factor)
}

/// 信号を整数倍でアップサンプリングする。
///
/// 内部で自動的に補間用のFIRローパスフィルタを設計・適用する。
///
/// # 引数
/// * `signal` - 入力信号。
/// * `factor` - アップサンプリングする倍率（2以上の整数）。
/// * `filter_taps` - 内部で使用するFIRフィルタのタップ数。奇数を推奨。
///
/// # 戻り値
/// * アップサンプリングされた信号。長さは元信号の factor 倍になる。
pub fn upsample(
    signal: &Vector<f64>,
    factor: usize,
    filter_taps: usize,
    window_type: WindowType,
) -> Vector<f64> {
    let zero_inserted_signal = expand(signal, factor);

    let normalized_cutoff = 0.5 / factor as f64; // 正規化されたカットオフ周波数
    let interpolation_filter =
        design_fir_lowpass(filter_taps, normalized_cutoff, window_type) * factor as f64;

    conv_with_dft_for_f64(&zero_inserted_signal, &interpolation_filter)
}

pub fn resample(
    signal: &Vector<f64>,
    upsample_factor: usize,
    downsample_factor: usize,
    filter_taps: usize,
    window_type: WindowType,
) -> Vector<f64> {
    let upsampled = upsample(signal, upsample_factor, filter_taps, window_type);
    down_sample(&upsampled, downsample_factor, filter_taps, window_type)
}

/// 信号を整数倍で間引く（デシメーション）。
/// 注意：この関数はアンチエイリアシングフィルタを適用しないため、
/// 事前に手動でローパスフィルタをかける必要がある。
pub fn decimate(signal: &Vector<f64>, factor: usize) -> Vector<f64> {
    if factor == 0 {
        return Vector::new(vec![]);
    }
    Vector::new(signal.iter().step_by(factor).cloned().collect())
}

/// 信号の各サンプルの間にゼロを挿入する（エキスパンド）。
///
/// 注意：この関数は補間フィルタを適用しないため、
/// この関数の後に手動でローパスフィルタをかける必要がある。
///
/// # 引数
/// * `signal` - 入力信号。
/// * `factor` - 拡張する倍率（2以上の整数）。
///
/// # 戻り値
/// * ゼロが挿入された信号。長さは元信号の factor 倍に近くなる。
pub fn expand(signal: &Vector<f64>, factor: usize) -> Vector<f64> {
    if factor <= 1 {
        return signal.clone();
    }

    let mut output = vec![0.0; signal.dim() * factor];
    for (i, &sample) in signal.iter().enumerate() {
        // 元のサンプルを追加
        output[i * factor] = sample;
    }
    Vector::new(output)
}

// ===== Signal フレンドリー API（impl メソッド） =====
impl Signal {
    /// ダウンサンプリング（内部でアンチエイリアスのローパス適用）。
    pub fn downsample(&self, factor: usize, filter_taps: usize, window_type: WindowType) -> Signal {
        let v = Vector::new(self.data().to_vec());
        let y = down_sample(&v, factor, filter_taps, window_type);
        // サンプルレートは 1/factor
        let sr = if factor > 0 {
            self.sample_rate() / factor as f64
        } else {
            self.sample_rate()
        };
        Signal::new(y.data, sr)
    }

    /// アップサンプリング（ゼロ挿入 + ローパス補間）。
    pub fn upsample(&self, factor: usize, filter_taps: usize, window_type: WindowType) -> Signal {
        let v = Vector::new(self.data().to_vec());
        let y = upsample(&v, factor, filter_taps, window_type);
        let sr = self.sample_rate() * factor as f64;
        Signal::new(y.data, sr)
    }

    /// 有理リサンプリング（L/M）。
    pub fn resample(
        &self,
        upsample_factor: usize,
        downsample_factor: usize,
        filter_taps: usize,
        window_type: WindowType,
    ) -> Signal {
        let v = Vector::new(self.data().to_vec());
        let y = resample(
            &v,
            upsample_factor,
            downsample_factor,
            filter_taps,
            window_type,
        );
        let sr = self.sample_rate() * upsample_factor as f64 / downsample_factor as f64;
        Signal::new(y.data, sr)
    }

    /// デシメーション（フィルタなし）。
    pub fn decimate(&self, factor: usize) -> Signal {
        let v = Vector::new(self.data().to_vec());
        let y = decimate(&v, factor);
        let sr = if factor > 0 {
            self.sample_rate() / factor as f64
        } else {
            self.sample_rate()
        };
        Signal::new(y.data, sr)
    }

    /// エキスパンド（ゼロ挿入のみ）。
    pub fn expand(&self, factor: usize) -> Signal {
        let v = Vector::new(self.data().to_vec());
        let y = expand(&v, factor);
        let sr = self.sample_rate() * factor as f64;
        Signal::new(y.data, sr)
    }
}
