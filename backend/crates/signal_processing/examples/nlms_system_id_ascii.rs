use polynomial::Polynomial;
use signal_processing::adaptive_filter::AdaptiveFilterNLMS;
use signal_processing::plot::{save_svg_series_scaled, Series};
use signal_processing::signal::Signal;

fn main() {
    let fs = 1000.0;
    let h = Polynomial::new(vec![0.3, -0.2, 0.1, 0.05]);
    let taps = h.coeffs.len();

    // 擬似乱数入力
    let n = 1000usize;
    let mut x = vec![0.0; n];
    let mut state = 1u64;
    for xi in x.iter_mut() {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let r = ((state >> 33) as u32) as f64 / (u32::MAX as f64);
        *xi = r - 0.5;
    }

    // 目的信号
    let mut d = vec![0.0; n];
    for i in 0..n {
        let mut acc = 0.0;
        for (k, &hk) in h.coeffs.iter().enumerate() {
            if i >= k {
                acc += hk * x[i - k];
            }
        }
        d[i] = acc;
    }

    let xsig = Signal::new(x, fs);
    let dsig = Signal::new(d, fs);

    let mut nlms = AdaptiveFilterNLMS::new(taps, 0.5, 1e-6);

    let mut mse_points: Vec<(f32, f32)> = Vec::new();
    let mut acc = 0.0f64;
    for i in 0..n {
        let (y, e) = nlms.process_sample(xsig.data()[i], dsig.data()[i]);
        let _ = y; // unused
        acc = 0.99 * acc + 0.01 * (e * e);
        mse_points.push((i as f32 / n as f32, acc as f32));
    }

    let ys: Vec<f64> = mse_points.iter().map(|&(_, y)| y as f64).collect();
    let series = [Series {
        y: &ys,
        label: "NLMS EWMA MSE",
    }];
    let _ = save_svg_series_scaled(
        "crates/signal_processing/plot/nlms_mse.svg",
        800,
        300,
        &series,
        "normalized time",
        1.0,
    );
    println!("saved: crates/signal_processing/plot/nlms_mse.svg");
}
