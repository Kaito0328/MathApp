use lti_systems::continuous::TransferFunction as CTF;

#[test]
fn impulse_and_step_match_statespace_sim() {
    // G(s) = 1 / (s + 1)
    let tf = CTF::from_coeffs(vec![1.0], vec![1.0, 1.0]);
    let fs = 100.0; // サンプリング周波数(離散化の密度)
    let n = 32;

    // 連続応答（ZOH離散化＋離散SSシミュレーション）
    let h = tf.impulse_response(fs, n);
    let s = tf.step_response(fs, n);

    // 1) 解析解（t = n/fs における 1 - e^{-t}）と比較
    let mut max_err = 0.0_f64;
    for (i, &sn) in s.iter().enumerate() {
        let t = (i as f64) / fs;
        let s_exact = 1.0 - (-t).exp();
        max_err = max_err.max((sn - s_exact).abs());
    }
    assert!(max_err < 5e-3, "max |s - (1-e^-t)| = {max_err}");

    // 2) 離散系の関係 s[n] - s[n-1] ≈ h[n] を確認（ZOH 離散化経由でも成立）
    let mut mae = 0.0_f64;
    let mut cnt = 0;
    let mut prev = 0.0;
    for (i, &sn) in s.iter().enumerate() {
        let d = sn - prev; // 離散微分
        let hn = h[i];
        mae += (d - hn).abs();
        cnt += 1;
        prev = sn;
    }
    let mae = mae / (cnt as f64);
    assert!(mae < 1e-2, "avg |Δs - h| = {mae}");
}
