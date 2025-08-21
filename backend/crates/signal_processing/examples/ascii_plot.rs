use signal_processing::signal::Signal;
use textplots::{Chart, Plot, Shape};

fn main() {
    // Create a simple sinusoid
    let n = 128usize;
    let fs = 128.0f64;
    let f1 = 10.0f64;
    let x = Signal::new(
        (0..n)
            .map(|i| {
                let t = i as f64 / fs;
                (2.0 * std::f64::consts::PI * f1 * t).sin()
            })
            .collect(),
        fs,
    );

    // Compute magnitude spectrum
    let x_freq = x.dft();
    let mag: Vec<f64> = x_freq.magnitudes();
    let nh = n / 2;

    // ASCII plot of time signal
    println!("Time signal (first 128 samples)");
    let time_points: Vec<(f32, f32)> = (0..n).map(|i| (i as f32, x.data()[i] as f32)).collect();
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
