use linalg::Vector;
use num_complex::Complex;
use plotters::prelude::*;
use signal_processing::dft::dft;
use signal_processing::fir::{design_fir_bandpass, design_fir_highpass, design_fir_lowpass};
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
        ("Rectangular", WindowType::Rectangular, RED),
        ("Hann", WindowType::Hann, BLUE),
        ("Hamming", WindowType::Hamming, GREEN),
        ("Blackman", WindowType::Blackman, MAGENTA),
        (
            "Kaiser(60dB)",
            WindowType::Kaiser {
                beta: calc_beta(60.0),
            },
            CYAN,
        ),
    ];

    // 1) ローパス: fc = 0.15
    {
        let fc = 0.15;
        let svg_path = format!("{out_dir}/compare_windows_lowpass.svg");
        let root = SVGBackend::new(&svg_path, (900, 420)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .caption("Lowpass magnitude (0..Nyquist)", ("sans-serif", 20))
            .set_label_area_size(LabelAreaPosition::Left, 50)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            .build_cartesian_2d(0..(n_fft / 2) as i32, 0.0..1.2)?;
        chart
            .configure_mesh()
            .x_desc("bin")
            .y_desc("|H[k]|")
            .draw()?;

        for &(name, w, color) in windows.iter() {
            let h = design_fir_lowpass(num_taps, fc, w);
            let h_freq = dft(&zero_pad_complex(&to_complex(&h), n_fft));
            let half = n_fft / 2;
            let pts: Vec<(i32, f64)> = (0..half)
                .map(|k| (k as i32, h_freq.iter().nth(k).unwrap().norm()))
                .collect();
            chart
                .draw_series(std::iter::once(PathElement::new(pts, color)))?
                .label(name)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperRight)
            .border_style(BLACK)
            .background_style(WHITE.mix(0.8))
            .draw()?;
        root.present()?;
        println!("Wrote {svg_path}");
    }

    // 2) ハイパス: fc = 0.25
    {
        let fc = 0.25;
        let svg_path = format!("{out_dir}/compare_windows_highpass.svg");
        let root = SVGBackend::new(&svg_path, (900, 420)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .caption("Highpass magnitude (0..Nyquist)", ("sans-serif", 20))
            .set_label_area_size(LabelAreaPosition::Left, 50)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            .build_cartesian_2d(0..(n_fft / 2) as i32, 0.0..1.2)?;
        chart
            .configure_mesh()
            .x_desc("bin")
            .y_desc("|H[k]|")
            .draw()?;

        for &(name, w, color) in windows.iter() {
            let h = design_fir_highpass(num_taps, fc, w);
            let h_freq = dft(&zero_pad_complex(&to_complex(&h), n_fft));
            let half = n_fft / 2;
            let pts: Vec<(i32, f64)> = (0..half)
                .map(|k| (k as i32, h_freq.iter().nth(k).unwrap().norm()))
                .collect();
            chart
                .draw_series(std::iter::once(PathElement::new(pts, color)))?
                .label(name)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperRight)
            .border_style(BLACK)
            .background_style(WHITE.mix(0.8))
            .draw()?;
        root.present()?;
        println!("Wrote {svg_path}");
    }

    // 3) バンドパス: f1=0.18, f2=0.30
    {
        let f1 = 0.18;
        let f2 = 0.30;
        let svg_path = format!("{out_dir}/compare_windows_bandpass.svg");
        let root = SVGBackend::new(&svg_path, (900, 420)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .caption("Bandpass magnitude (0..Nyquist)", ("sans-serif", 20))
            .set_label_area_size(LabelAreaPosition::Left, 50)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            .build_cartesian_2d(0..(n_fft / 2) as i32, 0.0..1.2)?;
        chart
            .configure_mesh()
            .x_desc("bin")
            .y_desc("|H[k]|")
            .draw()?;

        for &(name, w, color) in windows.iter() {
            let h = design_fir_bandpass(num_taps, f1, f2, w);
            let h_freq = dft(&zero_pad_complex(&to_complex(&h), n_fft));
            let half = n_fft / 2;
            let pts: Vec<(i32, f64)> = (0..half)
                .map(|k| (k as i32, h_freq.iter().nth(k).unwrap().norm()))
                .collect();
            chart
                .draw_series(std::iter::once(PathElement::new(pts, color)))?
                .label(name)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperRight)
            .border_style(BLACK)
            .background_style(WHITE.mix(0.8))
            .draw()?;
        root.present()?;
        println!("Wrote {svg_path}");
    }

    // 4) 時間信号: 入力と出力（ローパス: 窓を比較）
    {
        let fs = 256.0;
        let n = 512usize;
        let x = make_signal(n, 10.0, 40.0, fs); // 10Hz + 40Hz/2
        let svg_path = format!("{out_dir}/compare_windows_lowpass_timesignal.svg");
        let root = SVGBackend::new(&svg_path, (900, 420)).into_drawing_area();
        root.fill(&WHITE)?;
        let min_y = x.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_y = x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .caption("Time signal (lowpass outputs)", ("sans-serif", 20))
            .set_label_area_size(LabelAreaPosition::Left, 50)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            .build_cartesian_2d(0..n as i32, (min_y - 0.2)..(max_y + 0.2))?;
        chart
            .configure_mesh()
            .x_desc("sample")
            .y_desc("amplitude")
            .draw()?;

        // 入力信号（グレー）
        let input_pts: Vec<(i32, f64)> = (0..n)
            .map(|i| (i as i32, *x.iter().nth(i).unwrap()))
            .collect();
        chart
            .draw_series(std::iter::once(PathElement::new(
                input_pts,
                RGBColor(160, 160, 160),
            )))?
            .label("Input")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RGBColor(160, 160, 160)));

        // いくつかの窓の出力を重ね描き
        let fc = 0.15;
        let choose = vec![
            ("Hamming", WindowType::Hamming, BLUE),
            ("Blackman", WindowType::Blackman, RED),
            (
                "Kaiser(60dB)",
                WindowType::Kaiser {
                    beta: calc_beta(60.0),
                },
                GREEN,
            ),
        ];
        for (name, w, color) in choose.into_iter() {
            let h = design_fir_lowpass(num_taps, fc, w);
            let y = apply_fir(&h, &x);
            let pts: Vec<(i32, f64)> = (0..n)
                .map(|i| (i as i32, *y.iter().nth(i).unwrap()))
                .collect();
            chart
                .draw_series(std::iter::once(PathElement::new(pts, color)))?
                .label(name)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperRight)
            .border_style(BLACK)
            .background_style(WHITE.mix(0.8))
            .draw()?;
        root.present()?;
        println!("Wrote {svg_path}");
    }

    Ok(())
}
