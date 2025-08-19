use linalg::Vector;
use num_complex::Complex;
use plotters::prelude::*;
use signal_processing::dft::dft;
use signal_processing::fir::{
    design_fir_bandpass, design_fir_bandstop, design_fir_highpass, design_fir_lowpass,
};
use signal_processing::window::WindowType;
use std::fs;

fn to_complex(v: &Vector<f64>) -> Vector<Complex<f64>> {
    Vector::new(v.iter().map(|&r| Complex::new(r, 0.0)).collect())
}

fn zero_pad_complex(v: &Vector<Complex<f64>>, n: usize) -> Vector<Complex<f64>> {
    let mut data: Vec<Complex<f64>> = v.iter().cloned().collect();
    data.resize(n, Complex::new(0.0, 0.0));
    Vector::new(data)
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

        // 0..Nyquist までの振幅スペクトル
        let half = n_fft / 2;
        let mut mags: Vec<f64> = Vec::with_capacity(half);
        for k in 0..half {
            mags.push(h_freq.iter().nth(k).unwrap().norm());
        }
        let max_mag = mags.iter().cloned().fold(0.0f64, f64::max);
        let y_top = if max_mag <= 0.0 { 1.0 } else { max_mag * 1.05 };

        // SVGへ描画
        let svg_path = format!("{out_dir}/fir_{name}_magnitude.svg");
        let root = SVGBackend::new(&svg_path, (900, 420)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .caption(
                format!("FIR {name} magnitude (0..Nyquist)"),
                ("sans-serif", 20),
            )
            .set_label_area_size(LabelAreaPosition::Left, 50)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            .build_cartesian_2d(0..half as i32, 0.0..y_top)?;
        chart
            .configure_mesh()
            .x_desc("bin")
            .y_desc("|H[k]|")
            .draw()?;

        // 折れ線で描画（LineSeriesが無効でもPathElementで代替）
        let points: Vec<(i32, f64)> = (0..half).map(|k| (k as i32, mags[k])).collect();
        chart.draw_series(points.windows(2).map(|w| {
            let p0 = w[0];
            let p1 = w[1];
            PathElement::new(vec![p0, p1], BLUE)
        }))?;

        root.present()?;
        println!("Wrote {svg_path}");
    }

    Ok(())
}
