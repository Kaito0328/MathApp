use linalg::Vector;
use num_complex::Complex;
use plotters::prelude::*;
use signal_processing::dft::dft;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a sample time signal: sum of two sinusoids
    let n = 256usize;
    let fs = 256.0f64; // sample rate
    let f1 = 10.0f64;
    let f2 = 40.0f64;
    let x = Vector::new(
        (0..n)
            .map(|i| {
                let t = i as f64 / fs;
                let s = (2.0 * std::f64::consts::PI * f1 * t).sin()
                    + 0.5 * (2.0 * std::f64::consts::PI * f2 * t).sin();
                Complex::new(s, 0.0)
            })
            .collect(),
    );

    // Compute spectrum (magnitude)
    let x_freq = dft(&x);
    let mag: Vec<f64> = x_freq.iter().map(|c| c.norm()).collect();

    // Resolve output directory inside this crate
    let out_dir = format!("{}/plot", env!("CARGO_MANIFEST_DIR"));
    let _ = fs::create_dir_all(&out_dir);
    // Plot time-domain signal to SVG
    let time_svg = format!("{out_dir}/time_signal.svg");
    let root = SVGBackend::new(&time_svg, (800, 400)).into_drawing_area();
    root.fill(&WHITE)?;
    let y_min = x.iter().map(|c| c.re).fold(f64::INFINITY, f64::min);
    let y_max = x.iter().map(|c| c.re).fold(f64::NEG_INFINITY, f64::max);
    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Time Signal", ("sans-serif", 20))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0..n as i32, y_min..y_max)?;
    chart.configure_mesh().draw()?;
    let line_points: Vec<(i32, f64)> = (0..n)
        .map(|i| (i as i32, x.iter().nth(i).unwrap().re))
        .collect();
    chart.draw_series(line_points.windows(2).map(|w| {
        let p0 = w[0];
        let p1 = w[1];
        PathElement::new(vec![p0, p1], BLUE)
    }))?;
    root.present()?;

    // Plot magnitude spectrum (only first half due to symmetry for real signals)
    let nh = n / 2;
    let max_mag = mag.iter().take(nh).cloned().fold(0.0f64, f64::max);
    let y_top = if max_mag <= 0.0 { 1.0 } else { max_mag * 1.05 };
    let spec_svg = format!("{out_dir}/magnitude_spectrum.svg");
    let root2 = SVGBackend::new(&spec_svg, (800, 400)).into_drawing_area();
    root2.fill(&WHITE)?;
    let mut chart2 = ChartBuilder::on(&root2)
        .margin(20)
        .caption("Magnitude Spectrum", ("sans-serif", 20))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0..nh as i32, 0.0..y_top)?;
    chart2
        .configure_mesh()
        .x_desc("bin")
        .y_desc("|X[k]|")
        .draw()?;
    chart2.draw_series((0..nh).map(|k| {
        let x0 = k as i32;
        let x1 = x0 + 1; // non-zero width bar for visibility
        Rectangle::new([(x0, 0.0), (x1, mag[k])], BLUE.filled())
    }))?;
    root2.present()?;

    println!("Wrote {out_dir}/time_signal.svg and {out_dir}/magnitude_spectrum.svg");
    Ok(())
}
