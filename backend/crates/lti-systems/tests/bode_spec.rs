use lti_systems::{continuous::TransferFunction as CTF, discrete::TransferFunction as DTF};
use num_complex::Complex;

#[test]
fn discrete_bode_dc_gain_and_phase() {
    // H(z) = 1 + z^{-1}
    let tf = DTF::from_coeffs(vec![1.0, 1.0], vec![1.0]);
    let h0 = tf.eval_z(Complex::new(1.0, 0.0));
    // DC での振幅/位相
    assert!((h0.norm() - 2.0).abs() < 1e-12);
    assert!(h0.im.abs() < 1e-12);
}

#[test]
fn continuous_bode_lowpass_limits() {
    // G(s) = 1 / (s + 1)
    let tf = CTF::from_coeffs(vec![1.0], vec![1.0, 1.0]);
    // 低周波(ω→0)で |G|→1、位相→0
    let g0 = tf.eval_s(Complex::new(0.0, 1e-9));
    assert!((g0.norm() - 1.0).abs() < 1e-6);
    assert!(g0.im.abs() < 1e-6);
    // 高周波(ω→∞)で |G|→0
    let g_inf = tf.eval_s(Complex::new(0.0, 1e9));
    assert!(g_inf.norm() < 1e-6);
}
