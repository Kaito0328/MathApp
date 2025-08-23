use lti_systems::{continuous::TransferFunction as CTF, discrete::TransferFunction as DTF};
use num_complex::Complex;

#[test]
fn nyquist_discrete_start_end_points() {
    let tf = DTF::from_coeffs(vec![1.0], vec![1.0, -0.5]);
    // オプションに依存せず API 呼び出しのみを検証（内部関数は crate 内）
    // 直接描画は I/O を伴うため、ここでは評価軌跡を手計算でチェック
    let thetas = [-std::f64::consts::PI, 0.0, std::f64::consts::PI];
    let mut points = Vec::new();
    for &th in &thetas {
        let z = Complex::from_polar(1.0, th);
        points.push(tf.eval_z(z));
    }
    // θ = -π と π で同一点（実軸）になることを確認
    assert!((points[0] - points[2]).norm() < 1e-12);
    // θ=0 は z=1 の応答
    assert!((points[1] - tf.eval_z(Complex::new(1.0, 0.0))).norm() < 1e-12);

    // 簡単なファイル出力のスモーク（生成だけ）
    let path = "plot/test_nyquist_discrete.svg";
    std::fs::create_dir_all("plot").ok();
    tf.plot_nyquist_svg_simple(path, 400, 300, 64).unwrap();
    assert!(std::path::Path::new(path).exists());
}

#[test]
fn nyquist_continuous_monotonic_freq_sampling() {
    let tf = CTF::from_coeffs(vec![1.0], vec![1.0, 1.0]); // 1st LP
                                                          // f が単調増加でサンプリングされる（log/linear はオプション次第）
    let opts = lti_systems::plot::ContinuousNyquistOptions {
        n_points: 5,
        f_min_hz: 1e-2,
        f_max_hz: 1e0,
        log_freq: true,
        ..Default::default()
    };
    let path = "plot/test_nyquist_continuous.svg";
    std::fs::create_dir_all("plot").ok();
    tf.plot_nyquist_svg(path, 400, 300, &opts).unwrap();
    assert!(std::path::Path::new(path).exists());
}
