use num_complex::Complex;
use signal_processing::dft::dft;
use signal_processing::iir::{DigitalFilterSpec, IIRFilter};
use signal_processing::plot::{save_svg_series_scaled, Series};

fn to_complex(v: &[f64]) -> Vec<Complex<f64>> {
    v.iter().map(|&r| Complex::new(r, 0.0)).collect()
}

fn main() {
    let fs = 1000.0;
    let fc = 100.0;
    let filt =
        IIRFilter::design_digital_butterworth(4, fs, DigitalFilterSpec::Lowpass { fc_hz: fc });
    let tf = filt.as_transfer();

    // インパルス応答の先頭を出して周波数応答をざっくり
    let n = 256usize;
    let mut x = vec![0.0; n];
    x[0] = 1.0;
    let y = tf.apply(&x);

    let yf = dft(&to_complex(&y));
    let mags: Vec<f32> = yf.iter().take(n / 2).map(|c| c.norm() as f32).collect();

    let half = mags.len();
    let ys: Vec<f64> = (0..half).map(|k| mags[k] as f64).collect();
    let series = [Series {
        y: &ys,
        label: "|H(e^{jω})|",
    }];
    let _ = save_svg_series_scaled(
        "crates/signal_processing/plot/iir_butter_mag.svg",
        800,
        300,
        &series,
        "normalized frequency (×π rad/sample)",
        1.0,
    );
    println!("saved: crates/signal_processing/plot/iir_butter_mag.svg");
}
