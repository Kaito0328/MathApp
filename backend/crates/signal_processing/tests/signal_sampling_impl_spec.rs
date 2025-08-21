use signal_processing::signal::Signal;
use signal_processing::window::WindowType;
use signal_processing::{dft::conv_with_dft_for_f64, fir, sampling};

fn vec_close(a: &[f64], b: &[f64], eps: f64) {
    assert_eq!(a.len(), b.len());
    for i in 0..a.len() {
        assert!(
            (a[i] - b[i]).abs() <= eps,
            "idx {}: {} vs {}",
            i,
            a[i],
            b[i]
        );
    }
}

#[test]
fn upsample_impl_matches_reference() {
    let x = Signal::new(vec![1.0, 0.5, -1.0, 0.25], 48_000.0);
    let l = 2usize;
    let taps = 41usize;
    let win = WindowType::Hamming;

    let y = x.upsample(l, taps, win);

    // Reference using sampling::expand + LPF
    let v = linalg::Vector::new(x.data().to_vec());
    let xp = sampling::expand(&v, l);
    let h = fir::design_fir_lowpass(taps, 0.5 / l as f64, win) * l as f64;
    let y_ref = conv_with_dft_for_f64(&xp, &h);

    vec_close(y.data(), &y_ref.data, 1e-9);
    assert!((y.sample_rate() - x.sample_rate() * l as f64).abs() < 1e-12);
}

#[test]
fn downsample_impl_matches_reference() {
    let x = Signal::new(vec![1.0, -1.0, 0.5, -0.5, 2.0, -2.0, 1.5, -1.5], 48_000.0);
    let m = 2usize;
    let taps = 31usize;
    let win = WindowType::Hamming;

    let y = x.downsample(m, taps, win);

    // Reference: LPF then decimate
    let v = linalg::Vector::new(x.data().to_vec());
    let h = fir::design_fir_lowpass(taps, 0.5 / m as f64, win);
    let xf = conv_with_dft_for_f64(&v, &h);
    let y_ref = sampling::decimate(&xf, m);

    vec_close(y.data(), &y_ref.data, 1e-9);
    assert!((y.sample_rate() - x.sample_rate() / m as f64).abs() < 1e-12);
}

#[test]
fn resample_impl_matches_reference() {
    let x = Signal::new(
        (0..16)
            .map(|i| (2.0 * std::f64::consts::PI * (i as f64) / 8.0).sin())
            .collect(),
        32_000.0,
    );
    let l = 3usize;
    let m = 2usize;
    let taps = 41usize;
    let win = WindowType::Hamming;
    let y = x.resample(l, m, taps, win);

    let v = linalg::Vector::new(x.data().to_vec());
    let up = sampling::upsample(&v, l, taps, win);
    let y_ref = sampling::down_sample(&up, m, taps, win);

    vec_close(y.data(), &y_ref.data, 1e-9);
    assert!((y.sample_rate() - x.sample_rate() * l as f64 / m as f64).abs() < 1e-12);
}
