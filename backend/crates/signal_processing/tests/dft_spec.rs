use num_complex::Complex;
use signal_processing::dft::dft_simple;

fn approx_eq(a: Complex<f64>, b: Complex<f64>, tol: f64) -> bool {
    (a - b).norm() <= tol
}

#[test]
fn dft_of_impulse_is_constant() {
    // x = [1, 0, 0, 0]
    let x = vec![
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ];
    let y = dft_simple(&x);
    assert_eq!(y.len(), 4);
    for k in 0..4 {
        assert!(approx_eq(y[k], Complex::new(1.0, 0.0), 1e-12));
    }
}

#[test]
fn dft_of_single_tone_matches_analytical() {
    // x_n = exp(j*2pi*n/N) for N=4 -> spectrum is a delta at k=1
    let n = 4usize;
    let x: Vec<Complex<f64>> = (0..n)
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
            Complex::new(angle.cos(), angle.sin())
        })
        .collect();
    let y = dft_simple(&x);
    assert_eq!(y.len(), n);
    // Expected: bin 1 is N, others ~0 (depending on scaling convention)
    // Our dft_simple uses non-normalized forward DFT
    for k in 0..n {
        let expected = if k == 1 {
            Complex::new(n as f64, 0.0)
        } else {
            Complex::new(0.0, 0.0)
        };
        assert!(approx_eq(y[k], expected, 1e-9));
    }
}
