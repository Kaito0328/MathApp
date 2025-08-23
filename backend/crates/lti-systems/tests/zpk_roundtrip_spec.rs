use num_complex::Complex;

#[test]
fn zpk_roundtrip_continuous() {
    // G(s) = 2 (s+1) / (s+2)
    let zeros = vec![Complex::new(-1.0, 0.0)];
    let poles = vec![Complex::new(-2.0, 0.0)];
    let zpk = lti_systems::zpk::ContinuousZpk::new(zeros, poles, 2.0);
    let tf = zpk.to_transfer_function();
    // 戻す
    let zpk_rt = lti_systems::zpk::ContinuousZpk::from_transfer_function(&tf);
    let tf_rt = zpk_rt.to_transfer_function();
    assert_eq!(tf_rt.b_coeffs(), tf.b_coeffs());
    assert_eq!(tf_rt.a_coeffs(), tf.a_coeffs());
}

#[test]
fn zpk_roundtrip_discrete() {
    // H(z) = (z-1)/(z-0.5)
    let zeros = vec![Complex::new(1.0, 0.0)];
    let poles = vec![Complex::new(0.5, 0.0)];
    let zpk = lti_systems::zpk::DiscreteZpk::new(zeros, poles, 1.0);
    let tf = zpk.to_transfer_function(1.0);
    let zpk_rt = lti_systems::zpk::DiscreteZpk::from_transfer_function(&tf);
    let tf_rt = zpk_rt.to_transfer_function(1.0);
    assert_eq!(tf_rt.b_coeffs(), tf.b_coeffs());
    assert_eq!(tf_rt.a_coeffs(), tf.a_coeffs());
}
