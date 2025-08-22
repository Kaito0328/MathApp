use num_complex::Complex;
use signal_processing::dft::dft;
use signal_processing::fir::{
    design_fir_bandpass, design_fir_bandstop, design_fir_highpass, design_fir_lowpass,
};
use signal_processing::signal::Spectrum;
use signal_processing::window::WindowType;
use std::fs;

fn to_complex(v: &[f64]) -> Vec<Complex<f64>> {
    v.iter().map(|&r| Complex::new(r, 0.0)).collect()
}

fn zero_pad_complex(v: &[Complex<f64>], n: usize) -> Vec<Complex<f64>> {
    let mut data: Vec<Complex<f64>> = v.to_vec();
    data.resize(n, Complex::new(0.0, 0.0));
    data
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 出力先: クレート内 plot ディレクトリ
    let out_dir = format!("{}/plot", env!("CARGO_MANIFEST_DIR"));
    let _ = fs::create_dir_all(&out_dir);

    let n_fft = 1024usize; // パワーオブツーで高分解能

    let filters = vec![
        (
            "lowpass",
            design_fir_lowpass(101, 0.15, WindowType::Hamming),
        ),
        (
            "highpass",
            design_fir_highpass(101, 0.2, WindowType::Hamming),
        ),
        (
            "bandpass",
            design_fir_bandpass(101, 0.18, 0.3, WindowType::Hamming),
        ),
        (
            "bandstop",
            design_fir_bandstop(101, 0.18, 0.3, WindowType::Hamming),
        ),
    ];

    for (name, h) in filters {
        // ゼロパディングして周波数分解能を上げる
        let h_c = to_complex(&h);
        let h_zp = zero_pad_complex(&h_c, n_fft);
        let h_freq = dft(&h_zp);

        let svg_path = format!("{out_dir}/fir_{name}_magnitude.svg");
        // スペクトルとして包んで dB 表示の SVG を出力
        let sp = Spectrum::new(h_freq, 1.0);
        sp.save_svg_magnitude_db_with_axes(&svg_path, 900, 420, &format!("FIR {name}"))?;
        println!("Wrote {svg_path}");
    }

    Ok(())
}
