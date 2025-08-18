use linalg::Vector;
use num_complex::Complex;
use signal_processing::dft::dft;
use textplots::{Chart, Plot, Shape};

fn main() {
    // Create a simple sinusoid
    let n = 128usize;
    let fs = 128.0f64;
    let f1 = 10.0f64;
    let x = Vector::new(
        (0..n)
            .map(|i| {
                let t = i as f64 / fs;
                let s = (2.0 * std::f64::consts::PI * f1 * t).sin();
                Complex::new(s, 0.0)
            })
            .collect(),
    );

    // Compute magnitude spectrum
    let x_freq = dft(&x);
    let mag: Vec<f64> = x_freq.iter().map(|c| c.norm()).collect();
    let nh = n / 2;

    // ASCII plot of time signal
    println!("Time signal (first 128 samples)");
    let time_points: Vec<(f32, f32)> = (0..n)
        .map(|i| (i as f32, x.iter().nth(i).unwrap().re as f32))
        .collect();
    Chart::new(120, 20, 0.0, n as f32)
        .lineplot(&Shape::Lines(&time_points[..]))
        .display();

    // ASCII plot of magnitude spectrum (first half)
    println!("Magnitude spectrum (0..Nyquist)");
    let spec_points: Vec<(f32, f32)> = (0..nh).map(|k| (k as f32, mag[k] as f32)).collect();
    Chart::new(120, 20, 0.0, nh as f32)
        .lineplot(&Shape::Lines(&spec_points[..]))
        .display();
}
