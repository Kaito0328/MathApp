use num_complex::Complex;
use signal_processing::dft::dft;
use signal_processing::fir::{
    design_fir_bandpass, design_fir_bandstop, design_fir_highpass, design_fir_lowpass,
};
use signal_processing::window::WindowType;

fn apply_fir(h: &[f64], x: &[f64]) -> Vec<f64> {
    let m = h.len();
    let n = x.len();
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut acc = 0.0;
        let kmax = m.min(i + 1);
        for k in 0..kmax {
            acc += h[k] * x[i - k];
        }
        y[i] = acc;
    }
    y
}

fn to_complex(v: &[f64]) -> Vec<Complex<f64>> {
    v.iter().map(|&r| Complex::new(r, 0.0)).collect()
}

fn mag_at_bin(v: &[Complex<f64>], k: usize) -> f64 {
    v[k].norm()
}

#[test]
fn lowpass_attenuates_high_tone() {
    let n = 256usize; // power of two for FFT
                      // Build signal: low tone (k=5), high tone (k=60)
    let x: Vec<f64> = (0..n)
        .map(|i| {
            let t = i as f64;
            (2.0 * std::f64::consts::PI * (5.0 * t / n as f64)).sin()
                + (2.0 * std::f64::consts::PI * (60.0 * t / n as f64)).sin()
        })
        .collect();
    let h = design_fir_lowpass(101, 0.05, WindowType::Hamming);
    let y = apply_fir(&h, &x);
    let y_c = to_complex(&y);
    let y_dft = dft(&y_c);
    let low = mag_at_bin(&y_dft, 5);
    let high = mag_at_bin(&y_dft, 60);
    assert!(
        low > high * 5.0,
        "low band not dominant: low={low} high={high}",
    );
}

#[test]
fn highpass_attenuates_low_tone() {
    let n = 256usize;
    let x: Vec<f64> = (0..n)
        .map(|i| {
            let t = i as f64;
            (2.0 * std::f64::consts::PI * (5.0 * t / n as f64)).sin()
                + (2.0 * std::f64::consts::PI * (60.0 * t / n as f64)).sin()
        })
        .collect();
    let h = design_fir_highpass(101, 0.2, WindowType::Hamming);
    let y = apply_fir(&h, &x);
    let y_dft = dft(&to_complex(&y));
    let low = mag_at_bin(&y_dft, 5);
    let high = mag_at_bin(&y_dft, 60);
    assert!(
        high > low * 5.0,
        "high band not dominant: low={low} high={high}",
    );
}

#[test]
fn bandpass_passes_mid_band() {
    let n = 256usize;
    // in-band tone k=50 (~0.195), out-band low k=5
    let x: Vec<f64> = (0..n)
        .map(|i| {
            let t = i as f64;
            (2.0 * std::f64::consts::PI * (5.0 * t / n as f64)).sin()
                + (2.0 * std::f64::consts::PI * (50.0 * t / n as f64)).sin()
        })
        .collect();
    let h = design_fir_bandpass(101, 0.18, 0.3, WindowType::Hamming);
    let y = apply_fir(&h, &x);
    let y_dft = dft(&to_complex(&y));
    let out_low = mag_at_bin(&y_dft, 5);
    let in_mid = mag_at_bin(&y_dft, 50);
    assert!(
        in_mid > out_low * 5.0,
        "bandpass failed: in_mid={in_mid} out_low={out_low}",
    );
}

#[test]
fn bandstop_rejects_mid_band() {
    let n = 256usize;
    // mid-band tone k=50 in stopband, low tone k=5 should pass
    let x: Vec<f64> = (0..n)
        .map(|i| {
            let t = i as f64;
            (2.0 * std::f64::consts::PI * (5.0 * t / n as f64)).sin()
                + (2.0 * std::f64::consts::PI * (50.0 * t / n as f64)).sin()
        })
        .collect();
    let h = design_fir_bandstop(101, 0.18, 0.3, WindowType::Hamming);
    let y = apply_fir(&h, &x);
    let y_dft = dft(&to_complex(&y));
    let pass_low = mag_at_bin(&y_dft, 5);
    let stop_mid = mag_at_bin(&y_dft, 50);
    assert!(
        pass_low > stop_mid * 5.0,
        "bandstop failed: pass_low={pass_low} stop_mid={stop_mid}",
    );
}
