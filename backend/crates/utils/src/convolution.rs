use num_complex::Complex;

pub fn convolve_simple_f64(x: &[f64], h: &[f64]) -> Vec<f64> {
    let mut y = vec![0.0f64; x.len() + h.len() - 1];
    for i in 0..x.len() {
        for j in 0..h.len() {
            y[i + j] += x[i] * h[j];
        }
    }
    y
}

pub fn convolve_fft_f64(x: &[f64], h: &[f64]) -> Vec<f64> {
    let n = x.len() + h.len() - 1;
    let mut x_pad: Vec<Complex<f64>> = x.iter().map(|&v| Complex::new(v, 0.0)).collect();
    let mut h_pad: Vec<Complex<f64>> = h.iter().map(|&v| Complex::new(v, 0.0)).collect();
    x_pad.resize(n, Complex::new(0.0, 0.0));
    h_pad.resize(n, Complex::new(0.0, 0.0));
    let x_fft = fft_core::dft(&x_pad);
    let h_fft = fft_core::dft(&h_pad);
    let y_fft: Vec<Complex<f64>> = x_fft.into_iter().zip(h_fft).map(|(a, b)| a * b).collect();
    let y = fft_core::ift(&y_fft);
    y.into_iter().map(|c| c.re).collect()
}

pub fn convolve_auto_f64(x: &[f64], h: &[f64]) -> Vec<f64> {
    let work = x.len() * h.len();
    if work <= 2048 {
        convolve_simple_f64(x, h)
    } else {
        convolve_fft_f64(x, h)
    }
}
