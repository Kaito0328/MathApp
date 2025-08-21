use linalg::Vector;
use num_complex::Complex;
use signal_processing::dft::dft;
use signal_processing::fir::{design_fir_bandpass, design_fir_highpass, design_fir_lowpass};
use signal_processing::signal::{Signal, Spectrum};
use signal_processing::window::{calc_beta, WindowType};
use std::fs;

fn to_complex(v: &Vector<f64>) -> Vector<Complex<f64>> {
    Vector::new(v.iter().map(|&r| Complex::new(r, 0.0)).collect())
}

fn zero_pad_complex(v: &Vector<Complex<f64>>, n: usize) -> Vector<Complex<f64>> {
    let mut data: Vec<Complex<f64>> = v.iter().cloned().collect();
    data.resize(n, Complex::new(0.0, 0.0));
    Vector::new(data)
}

fn apply_fir(h: &Vector<f64>, x: &Vector<f64>) -> Vector<f64> {
    let m = h.dim();
    let n = x.dim();
    let hh: Vec<f64> = h.iter().cloned().collect();
    let xx: Vec<f64> = x.iter().cloned().collect();
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut acc = 0.0;
        let kmax = m.min(i + 1);
        for k in 0..kmax {
            acc += hh[k] * xx[i - k];
        }
        y[i] = acc;
    }
    Vector::new(y)
}

fn make_signal(n: usize, f1: f64, f2: f64, fs: f64) -> Vector<f64> {
    Vector::new(
        (0..n)
            .map(|i| {
                let t = i as f64 / fs;
                (2.0 * std::f64::consts::PI * f1 * t).sin()
                    + 0.5 * (2.0 * std::f64::consts::PI * f2 * t).sin()
            })
            .collect(),
    )
}

// note: we inline simple path drawing to avoid type gymnastics

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 出力先
    let out_dir = format!("{}/plot", env!("CARGO_MANIFEST_DIR"));
    let _ = fs::create_dir_all(&out_dir);

    // 共通パラメータ
    let num_taps = 101usize;
    let n_fft = 2048usize; // 周波数分解能

    // 比較する窓（Kaiserは60dB相当）
    let windows = [
        ("Rectangular", WindowType::Rectangular),
        ("Hann", WindowType::Hann),
        ("Hamming", WindowType::Hamming),
        ("Blackman", WindowType::Blackman),
        (
            "Kaiser(60dB)",
            WindowType::Kaiser {
                beta: calc_beta(60.0),
            },
        ),
    ];

    // 1) ローパス: fc = 0.15
    {
        let fc = 0.15;
        let svg_path = format!("{out_dir}/compare_windows_lowpass.svg");
        // 各窓の周波数応答を Spectrum にしてまとめて描画（dB）。
        let mut specs: Vec<(Spectrum, &str)> = Vec::new();
        for &(name, w) in windows.iter() {
            let h = design_fir_lowpass(num_taps, fc, w);
            let h_freq = dft(&zero_pad_complex(&to_complex(&h), n_fft));
            let sp = Spectrum::new(h_freq.iter().cloned().collect(), 1.0);
            specs.push((sp, name));
        }
        // 参照の配列を作って API に渡す
        let refs: Vec<(&Spectrum, &str)> = specs.iter().map(|(s, name)| (s, *name)).collect();
        Spectrum::save_svg_magnitude_db_multi(&svg_path, 900, 420, &refs)?;
        println!("Wrote {svg_path}");
    }

    // 2) ハイパス: fc = 0.25
    {
        let fc = 0.25;
        let svg_path = format!("{out_dir}/compare_windows_highpass.svg");
        let mut specs: Vec<(Spectrum, &str)> = Vec::new();
        for &(name, w) in windows.iter() {
            let h = design_fir_highpass(num_taps, fc, w);
            let h_freq = dft(&zero_pad_complex(&to_complex(&h), n_fft));
            let sp = Spectrum::new(h_freq.iter().cloned().collect(), 1.0);
            specs.push((sp, name));
        }
        let refs: Vec<(&Spectrum, &str)> = specs.iter().map(|(s, name)| (s, *name)).collect();
        Spectrum::save_svg_magnitude_db_multi(&svg_path, 900, 420, &refs)?;
        println!("Wrote {svg_path}");
    }

    // 3) バンドパス: f1=0.18, f2=0.30
    {
        let f1 = 0.18;
        let f2 = 0.30;
        let svg_path = format!("{out_dir}/compare_windows_bandpass.svg");
        let mut specs: Vec<(Spectrum, &str)> = Vec::new();
        for &(name, w) in windows.iter() {
            let h = design_fir_bandpass(num_taps, f1, f2, w);
            let h_freq = dft(&zero_pad_complex(&to_complex(&h), n_fft));
            let sp = Spectrum::new(h_freq.iter().cloned().collect(), 1.0);
            specs.push((sp, name));
        }
        let refs: Vec<(&Spectrum, &str)> = specs.iter().map(|(s, name)| (s, *name)).collect();
        Spectrum::save_svg_magnitude_db_multi(&svg_path, 900, 420, &refs)?;
        println!("Wrote {svg_path}");
    }

    // 4) 時間信号: 入力と出力（ローパス: 窓を比較）
    {
        let fs = 256.0;
        let n = 512usize;
        let x = make_signal(n, 10.0, 40.0, fs); // 10Hz + 40Hz/2
        let svg_path = format!("{out_dir}/compare_windows_lowpass_timesignal.svg");
        let input_sig = Signal::new(x.iter().cloned().collect(), fs);

        // いくつかの窓の出力を重ね描き
        let fc = 0.15;
        let choose = vec![
            ("Hamming", WindowType::Hamming),
            ("Blackman", WindowType::Blackman),
            (
                "Kaiser(60dB)",
                WindowType::Kaiser {
                    beta: calc_beta(60.0),
                },
            ),
        ];

        let mut labeled: Vec<(Signal, &str)> = Vec::new();
        // 先頭に入力信号
        labeled.push((input_sig.clone(), "Input"));
        for (name, w) in choose.into_iter() {
            let h = design_fir_lowpass(num_taps, fc, w);
            let y = apply_fir(&h, &x);
            let y_sig = Signal::new(y.iter().cloned().collect(), fs);
            labeled.push((y_sig, name));
        }
        let refs: Vec<(&Signal, &str)> = labeled.iter().map(|(s, name)| (s, *name)).collect();
        // 先頭を基準にマルチ描画
        labeled[0].0.save_svg_multi(&svg_path, 900, 420, &refs)?;
        println!("Wrote {svg_path}");
    }

    Ok(())
}
