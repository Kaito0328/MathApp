use linalg::Vector;
use plotters::prelude::*;
use signal_processing::sampling::{down_sample, resample, upsample};
use signal_processing::window::WindowType;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 元信号：サンプル数256、10Hz + 40Hz/2 の合成正弦波
    let n = 256usize;
    let fs = 256.0f64;
    let f1 = 10.0f64;
    let f2 = 40.0f64;
    let x = Vector::new(
        (0..n)
            .map(|i| {
                let t = i as f64 / fs;
                (2.0 * std::f64::consts::PI * f1 * t).sin()
                    + 0.5 * (2.0 * std::f64::consts::PI * f2 * t).sin()
            })
            .collect(),
    );

    let taps = 41usize;
    let win = WindowType::Hamming;

    let l = 3usize; // upsample factor
    let m = 2usize; // downsample factor

    let x_up = upsample(&x, l, taps, win);
    let _x_down = down_sample(&x, m, taps, win);
    let x_resamp = resample(&x, l, m, taps, win);

    let out_dir = format!("{}/plot", env!("CARGO_MANIFEST_DIR"));
    let _ = fs::create_dir_all(&out_dir);
    let svg_path = format!("{out_dir}/sampling_demo.svg");

    let root = SVGBackend::new(&svg_path, (1000, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // 3段に分けて時間波形を描画
    let areas = root.split_evenly((3, 1));

    let draw_series = |area: &DrawingArea<SVGBackend, plotters::coord::Shift>,
                       title: &str,
                       y: &Vector<f64>|
     -> Result<(), Box<dyn std::error::Error>> {
        let y_min = y.iter().cloned().fold(f64::INFINITY, f64::min);
        let y_max = y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let x_max = y.dim() as i32;
        let mut chart = ChartBuilder::on(area)
            .margin(10)
            .caption(title, ("sans-serif", 16))
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 30)
            .build_cartesian_2d(0..x_max, (y_min - 0.2)..(y_max + 0.2))?;
        chart
            .configure_mesh()
            .x_desc("sample")
            .y_desc("amp")
            .draw()?;
        let pts: Vec<(i32, f64)> = (0..y.dim()).map(|i| (i as i32, y[i])).collect();
        chart.draw_series(std::iter::once(PathElement::new(pts, BLUE)))?;
        Ok(())
    };

    draw_series(&areas[0], "Original", &x)?;
    draw_series(&areas[1], &format!("Upsampled x{l}"), &x_up)?;
    draw_series(&areas[2], &format!("Resampled x{l}/{m}"), &x_resamp)?;

    root.present()?;
    println!("Wrote {svg_path}");
    Ok(())
}
