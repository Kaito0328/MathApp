#[cfg(test)]
mod tests {
    use lti_systems::discrete::TransferFunction;
    use poly::polynomial::Polynomial;

    #[test]
    fn impulse_and_step_fir() {
        // 簡単な FIR: h = [1, 2, 3]
        let tf = TransferFunction::new(
            Polynomial::new(vec![1.0, 2.0, 3.0]),
            Polynomial::new(vec![1.0]),
        );
        let h = tf.impulse_response(5);
        assert_eq!(h, vec![1.0, 2.0, 3.0, 0.0, 0.0]);

        let s = tf.step_response(4);
        // y[n] = conv(step, h): [1, 3, 6, 6]
        assert_eq!(s, vec![1.0, 3.0, 6.0, 6.0]);
    }

    #[test]
    fn frequency_response_dc_gain() {
        // H(z) = (1 + z^-1) / 1 -> DC ゲインは 2
        let tf = TransferFunction::new(Polynomial::new(vec![1.0, 1.0]), Polynomial::new(vec![1.0]));
        let h0 = tf.frequency_response(8)[0];
        assert!((h0.re - 2.0).abs() < 1e-12 && h0.im.abs() < 1e-12);
    }
}
