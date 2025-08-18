use linalg::Vector;
use num_complex::Complex;
use signal_processing::dft::{dft, dft_simple};

fn approx_eq(a: Complex<f64>, b: Complex<f64>, tol: f64) -> bool {
    (a - b).norm() <= tol
}

#[test]
fn mixed_radix_fft_matches_dft_small() {
    let n = 12; // 12 = 3 * 2 * 2 (mixed radix)
    let x = Vector::new((0..n).map(|i| Complex::new(i as f64, 0.0)).collect());
    let y_dft = dft_simple(&x);
    let y_fft = dft(&x);
    assert_eq!(y_dft.dim(), y_fft.dim());
    for k in 0..n {
        assert!(approx_eq(
            y_dft.iter().nth(k).cloned().unwrap(),
            y_fft.iter().nth(k).cloned().unwrap(),
            1e-9
        ));
    }
}

#[test]
fn mixed_radix_fft_random_signal() {
    let n = 15; // 15 = 3 * 5
    let x = Vector::new(
        (0..n)
            .map(|i| Complex::new(((i * 7 + 3) % 11) as f64, ((i * 5 + 1) % 13) as f64))
            .collect(),
    );
    let y_dft = dft_simple(&x);
    let y_fft = dft(&x);
    for k in 0..n {
        assert!(approx_eq(
            y_dft.iter().nth(k).cloned().unwrap(),
            y_fft.iter().nth(k).cloned().unwrap(),
            1e-9
        ));
    }
}
