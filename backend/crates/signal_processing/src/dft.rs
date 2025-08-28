use crate::signal::{Signal, Spectrum};
use fft_core as fftc;
use num_complex::Complex;

// ---- 型メソッド拡張 ----
impl Signal {
    /// この信号の DFT/FFT を計算して Spectrum を返す。
    pub fn dft(&self) -> Spectrum {
        dft_signal(self)
    }

    /// この信号と h の線形畳み込み（自動: small=時間領域, large=FFT）。
    pub fn convolve(&self, h: &Signal) -> Signal {
        conv_signal_auto(self, h)
    }
}

impl Spectrum {
    /// 逆変換で時間信号へ戻す。
    pub fn ift(&self) -> Signal {
        ift_spectrum(self)
    }
}

pub fn dft_simple(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = x.len();
    let mut dft_result: Vec<Complex<f64>> = Vec::with_capacity(n);
    for i in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for (j, &x_j) in x.iter().enumerate() {
            let angle = -2.0 * std::f64::consts::PI * (i as f64 * j as f64) / (n as f64);
            let w = Complex::new(angle.cos(), angle.sin());
            sum += x_j * w;
        }
        dft_result.push(sum);
    }
    dft_result
}

fn base_ift(dft_result: &mut [Complex<f64>], n: usize) {
    if dft_result.len() > 1 {
        dft_result[1..].reverse();
    }

    let n_float = n as f64;
    for i in dft_result.iter_mut() {
        *i /= n_float;
    }
}

pub fn ift_simple(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let mut dft_result = dft_simple(x);
    base_ift(&mut dft_result, x.len());
    dft_result
}

// 公開API: 実体は fft-core に委譲
pub fn dft(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    match fftc::dft(x) {
        Ok(v) => v,
        Err(e) => panic!("DFT failed: {e}"),
    }
}
pub fn ift(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    match fftc::ift(x) {
        Ok(v) => v,
        Err(e) => panic!("IFT failed: {e}"),
    }
}

// ===== Signal/Spectrum フレンドリーAPI（内部アルゴリズムは既存関数を使用） =====

/// 実信号から複素スペクトルを得る（DFT/FFT 自動選択）。
pub fn dft_signal(x: &Signal) -> Spectrum {
    let y = dft(&x.to_complex_vec());
    Spectrum::new(y, x.sample_rate())
}

/// 複素スペクトルから実信号へ（逆変換）。
pub fn ift_spectrum(x: &Spectrum) -> Signal {
    let v: Vec<Complex<f64>> = x.clone().into();
    let y = ift(&v);
    let data: Vec<f64> = y.into_iter().map(|c| c.re).collect();
    Signal::new(data, x.sample_rate())
}
/// 1D 線形畳み込み（時間領域、素朴法）。
pub fn conv_simple(x: &[Complex<f64>], h: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let mut y = vec![Complex::new(0.0, 0.0); x.len() + h.len() - 1];
    for i in 0..x.len() {
        for j in 0..h.len() {
            y[i + j] += x[i] * h[j];
        }
    }
    y
}

/// 1D 線形畳み込み（FFT ベース）。
pub fn conv_fft(x: &[Complex<f64>], h: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = x.len() + h.len() - 1;
    let mut x_padded = x.to_vec();
    x_padded.resize(n, Complex::new(0.0, 0.0));
    let mut h_padded = h.to_vec();
    h_padded.resize(n, Complex::new(0.0, 0.0));

    let dft_x = dft(&x_padded);
    let dft_h = dft(&h_padded);

    let dft_result: Vec<Complex<f64>> = dft_x.into_iter().zip(dft_h).map(|(a, b)| a * b).collect();

    ift(&dft_result)
}

/// 1D 線形畳み込み（サイズにより素朴/FFT を自動切替）。
pub fn conv_auto(x: &[Complex<f64>], h: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let work = x.len() * h.len();
    if work <= 2048 {
        conv_simple(x, h)
    } else {
        conv_fft(x, h)
    }
}

/// 実数列のバリアント（素朴）。
pub fn conv_simple_f64(x: &[f64], h: &[f64]) -> Vec<f64> {
    let mut y = vec![0.0f64; x.len() + h.len() - 1];
    for i in 0..x.len() {
        for j in 0..h.len() {
            y[i + j] += x[i] * h[j];
        }
    }
    y
}
/// 実数列のバリアント（FFT）。
pub fn conv_fft_f64(x: &[f64], h: &[f64]) -> Vec<f64> {
    let cx: Vec<Complex<f64>> = x.iter().map(|&v| Complex::new(v, 0.0)).collect();
    let ch: Vec<Complex<f64>> = h.iter().map(|&v| Complex::new(v, 0.0)).collect();
    conv_fft(&cx, &ch).into_iter().map(|c| c.re).collect()
}
/// 実数列のバリアント（自動）。
pub fn conv_auto_f64(x: &[f64], h: &[f64]) -> Vec<f64> {
    let work = x.len() * h.len();
    if work <= 2048 {
        conv_simple_f64(x, h)
    } else {
        conv_fft_f64(x, h)
    }
}

/// Signal 同士の線形畳み込み（自動）。
pub fn conv_signal_auto(x: &Signal, h: &Signal) -> Signal {
    let vx = x.to_complex_vec();
    let vh = h.to_complex_vec();
    let y = conv_auto(&vx, &vh);
    let data: Vec<f64> = y.into_iter().map(|c| c.re).collect();
    Signal::new(data, x.sample_rate())
}

// 以降の詳細アルゴリズムは fft-core に移設
