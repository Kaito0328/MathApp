use polynomial::Polynomial;
use signal_processing::iir::IIRFilter;
use signal_processing::signal::Signal;

fn approx_eq(a: f64, b: f64, eps: f64) {
    let diff = (a - b).abs();
    assert!(diff <= eps, "expected {b} got {a} (|diff|={diff})");
}

#[test]
fn iir_impulse_response_known_first_order() {
    // 離散一次 IIR: H(z) = (1 - r) / (1 - r z^{-1}) → h[n] = (1 - r) r^n
    let fs: f64 = 1000.0;
    let a: f64 = 5.0; // continuous pole at -a → r = e^{-aT}
    let r = (-a / fs).exp();
    let b = Polynomial::new(vec![1.0 - r]);
    let a = Polynomial::new(vec![1.0, -r]);
    let filt = IIRFilter::new_with_fs(b, a, fs);

    // 単位インパルス
    let n = 32;
    let mut x = vec![0.0; n];
    x[0] = 1.0;
    let xsig = Signal::new(x, fs);
    let y = filt.apply(&xsig);
    let yh = y.data();

    for (i, &v) in yh.iter().enumerate() {
        let expected = (1.0 - r) * r.powi(i as i32);
        approx_eq(v, expected, 1e-12);
    }
}

#[test]
fn iir_butterworth_lowpass_high_freq_attenuation() {
    let fs = 1000.0;
    let fc = 100.0;
    let filt = IIRFilter::design_digital_butterworth(
        4,
        fs,
        signal_processing::iir::DigitalFilterSpec::Lowpass { fc_hz: fc },
    );
    let tf = filt.as_transfer();

    // 直流と高域(0.45*fs) の振幅を比較
    use num_complex::Complex;
    let z_dc = Complex::new(1.0, 0.0);
    let h_dc = tf.eval_z(z_dc).norm();
    // ω = 2π f / fs, f=0.45*fs
    let omega = 2.0 * std::f64::consts::PI * 0.45;
    let z_hf = Complex::new(omega.cos(), omega.sin());
    let h_hf = tf.eval_z(z_hf).norm();

    assert!(
        h_hf < h_dc * 0.5,
        "expected noticeable attenuation: hf {h_hf} < dc {h_dc}"
    );
}
