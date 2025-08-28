use polynomial::Polynomial;
use signal_processing::adaptive_filter::{AdaptiveFilterLMS, AdaptiveFilterNLMS};
use signal_processing::plot::{save_svg_series_scaled, Series};
use signal_processing::signal::Signal;

fn main() {
    let fs = 1000.0;
    let h = Polynomial::new(vec![0.25, -0.15, 0.1, 0.05, -0.02]);
    let taps = h.coeffs.len();

    // 擬似乱数入力
    let n = 1500usize;
    let mut x = vec![0.0; n];
    let mut state = 42u64;
    for xi in x.iter_mut() {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let r = ((state >> 33) as u32) as f64 / (u32::MAX as f64);
        *xi = r - 0.5;
    }

    // 目的信号 d = h * x
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

    let xsig = Signal::new(x.clone(), fs);
    let dsig = Signal::new(d.clone(), fs);

    let mut lms = AdaptiveFilterLMS::new(taps, 0.01);
    let mut nlms = AdaptiveFilterNLMS::new(taps, 0.5, 1e-6);

    let mut mse_lms: Vec<(f32, f32)> = Vec::new();
    let mut mse_nlms: Vec<(f32, f32)> = Vec::new();
    let mut acc_l = 0.0f64;
    let mut acc_n = 0.0f64;

    for i in 0..n {
        let (_yl, el) = lms.process_sample(xsig.data()[i], dsig.data()[i]);
        let (_yn, en) = nlms.process_sample(xsig.data()[i], dsig.data()[i]);
        acc_l = 0.99 * acc_l + 0.01 * (el * el);
        acc_n = 0.99 * acc_n + 0.01 * (en * en);
        let t = i as f32 / n as f32;
        mse_lms.push((t, acc_l as f32));
        mse_nlms.push((t, acc_n as f32));
    }

    let ys_l: Vec<f64> = mse_lms.iter().map(|&(_, y)| y as f64).collect();
    let ys_n: Vec<f64> = mse_nlms.iter().map(|&(_, y)| y as f64).collect();
    let series = [
        Series {
            y: &ys_l,
            label: "LMS EWMA MSE",
        },
        Series {
            y: &ys_n,
            label: "NLMS EWMA MSE",
        },
    ];
    let _ = save_svg_series_scaled(
        "crates/signal_processing/plot/lms_vs_nlms_mse.svg",
        800,
        300,
        &series,
        "normalized time",
        1.0,
    );
    println!("saved: crates/signal_processing/plot/lms_vs_nlms_mse.svg");
}
