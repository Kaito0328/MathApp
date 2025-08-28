use num_complex::Complex;

#[test]
fn continuous_zpk_to_tf_basic_first_order_lowpass() {
    // G(s) = 1 / (s + 1)
    let zpk = lti_systems::zpk::ContinuousZpk::new(vec![], vec![Complex::new(-1.0, 0.0)], 1.0);
    let tf = zpk.to_transfer_function();
    // Expect B(s) = 1, A(s) = s + 1
    assert_eq!(tf.b_coeffs(), &[1.0]);
    assert_eq!(tf.a_coeffs(), &[1.0, 1.0]);

    // Round-trip TF -> ZPK -> TF should preserve leading-coefficient ratio (gain)
    let zpk_rt = lti_systems::zpk::ContinuousZpk::from_transfer_function(&tf);
    let tf_rt = zpk_rt.to_transfer_function();
    assert_eq!(tf_rt.b_coeffs(), tf.b_coeffs());
    assert_eq!(tf_rt.a_coeffs(), tf.a_coeffs());
}

#[test]
fn discrete_zpk_to_tf_basic_differentiator() {
    // H(z) = (1 - z^{-1})  => numerator (z - 1), denominator 1
    let zpk = lti_systems::zpk::DiscreteZpk::new(vec![Complex::new(1.0, 0.0)], vec![], 1.0);
    let tf = zpk.to_transfer_function(1.0);
    // Expect B(z) = z - 1 => coeffs [-1, 1], A(z) = 1
    assert_eq!(tf.b_coeffs(), &[-1.0, 1.0]);
    assert_eq!(tf.a_coeffs(), &[1.0]);

    // Round-trip TF -> ZPK -> TF
    let zpk_rt = lti_systems::zpk::DiscreteZpk::from_transfer_function(&tf);
    let tf_rt = zpk_rt.to_transfer_function(1.0);
    assert_eq!(tf_rt.b_coeffs(), tf.b_coeffs());
    assert_eq!(tf_rt.a_coeffs(), tf.a_coeffs());
}
