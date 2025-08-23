use polynomial::Polynomial;
use signal_processing::adaptive_filter::{AdaptiveFilterLMS, AdaptiveFilterNLMS};
use signal_processing::plot::{save_svg_time_series, Series};
use signal_processing::signal::Signal;

fn main() {
    let fs = 1000.0;
    let n = 1200usize;

    // 目的信号 s: 正弦波
    let f0 = 50.0;
    let mut s = vec![0.0; n];
    for (i, si) in s.iter_mut().enumerate() {
        *si = (2.0 * std::f64::consts::PI * f0 * (i as f64) / fs).sin();
    }

    // 参照雑音 v: 単純なFIRを通した白色雑音
    let mut v = vec![0.0; n];
    let mut state = 2025u64;
    for vi in v.iter_mut() {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let r = ((state >> 33) as u32) as f64 / (u32::MAX as f64);
        *vi = r - 0.5;
    }
    let h = Polynomial::new(vec![0.4, 0.3, 0.2, 0.1]);
    let mut n_correlated = vec![0.0; n];
    for i in 0..n {
        let mut acc = 0.0;
        for (k, &hk) in h.coeffs.iter().enumerate() {
            if i >= k {
                acc += hk * v[i - k];
            }
        }
        n_correlated[i] = acc;
    }

    // 観測 d = s + n_correlated
    let mut d = vec![0.0; n];
    for i in 0..n {
        d[i] = s[i] + n_correlated[i];
    }

    // 自由雑音入力は参照 v、目的は d。
    let xsig = Signal::new(v, fs);
    let dsig = Signal::new(d, fs);

    let mut lms = AdaptiveFilterLMS::new(16, 0.05);
    let mut nlms = AdaptiveFilterNLMS::new(16, 0.5, 1e-6);

    let mut mse_points: Vec<(f32, f32)> = Vec::new();
    let mut mse_points_nlms: Vec<(f32, f32)> = Vec::new();
    let mut acc = 0.0f64;
    let mut acc_nlms = 0.0f64;
    let mut y_out = vec![0.0; n];
    let mut y_out_nlms = vec![0.0; n];
    for (i, yi) in y_out.iter_mut().enumerate() {
        let (_y, e) = lms.process_sample(xsig.data()[i], dsig.data()[i]); // _y: 推定雑音
        let (_y_nlms, e_nlms) = nlms.process_sample(xsig.data()[i], dsig.data()[i]);
        *yi = e; // ノイズキャンセル後の推定クリーン信号
        y_out_nlms[i] = e_nlms;
        acc = 0.99 * acc + 0.01 * (e * e);
        acc_nlms = 0.99 * acc_nlms + 0.01 * (e_nlms * e_nlms);
        mse_points.push((i as f32 / n as f32, acc as f32));
        mse_points_nlms.push((i as f32 / n as f32, acc_nlms as f32));
    }

    let ys: Vec<f64> = mse_points.iter().map(|&(_, y)| y as f64).collect();
    let ys_nlms: Vec<f64> = mse_points_nlms.iter().map(|&(_, y)| y as f64).collect();
    let series = [
        Series {
            y: &ys,
            label: "LMS EWMA MSE",
        },
        Series {
            y: &ys_nlms,
            label: "NLMS EWMA MSE",
        },
    ];
    let _ = save_svg_time_series(
        "crates/signal_processing/plot/lms_noise_cancellation_mse.svg",
        800,
        300,
        &series,
        Some(fs),
    );
    println!("saved: crates/signal_processing/plot/lms_noise_cancellation_mse.svg");
}
