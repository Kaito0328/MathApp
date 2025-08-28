use polynomial::Polynomial;
use signal_processing::adaptive_filter::nlms_filter;
use signal_processing::signal::Signal;

fn approx_le(a: f64, b: f64, eps: f64) {
    assert!(a <= b + eps, "{a} !<= {b} (eps {eps})");
}

#[test]
fn nlms_converges_on_fir_system_identification() {
    // 既知のFIRシステム h を NLMS で同定できるか（平均二乗誤差が減少するか）
    let fs = 1000.0;
    let h = Polynomial::new(vec![0.3, -0.2, 0.1, 0.05]);
    let taps = h.coeffs.len();

    // 入力: 白色雑音
    let n = 2000usize;
    let mut x = vec![0.0; n];
    // シード固定
    let mut state = 1u64;
    for xi in x.iter_mut() {
        // 線形合同法で擬似乱数 (0,1) → (-0.5,0.5)
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let r = ((state >> 33) as u32) as f64 / (u32::MAX as f64);
        *xi = r - 0.5;
    }

    // 目標信号: y = h * x（直接形）
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut acc = 0.0;
        for (k, &hk) in h.coeffs.iter().enumerate() {
            if i >= k {
                acc += hk * x[i - k];
            }
        }
        y[i] = acc;
    }

    let xsig = Signal::new(x.clone(), fs);
    let dsig = Signal::new(y, fs);

    let (_yo, eo) = nlms_filter(&xsig, &dsig, taps, 0.5, 1e-6);
    // 前半・後半の平均二乗誤差を比較（後半の方が小さい）
    let e = eo.data();
    let mid = n / 2;
    let mse_first: f64 = e[..mid].iter().map(|v| v * v).sum::<f64>() / (mid as f64);
    let mse_second: f64 = e[mid..].iter().map(|v| v * v).sum::<f64>() / ((n - mid) as f64);

    approx_le(mse_second, mse_first, 1e-6);
}
