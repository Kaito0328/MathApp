use linalg::Vector;
use num_complex::Complex;
use signal_processing::dft::{dft, dft_simple, ift, ift_simple};

fn approx_eq(a: Complex<f64>, b: Complex<f64>, tol: f64) -> bool {
    (a - b).norm() <= tol
}

fn approx_vec(a: &Vector<Complex<f64>>, b: &Vector<Complex<f64>>, tol: f64) -> bool {
    if a.dim() != b.dim() {
        return false;
    }
    for i in 0..a.dim() {
        if !approx_eq(
            a.iter().nth(i).cloned().unwrap(),
            b.iter().nth(i).cloned().unwrap(),
            tol,
        ) {
            return false;
        }
    }
    true
}

#[test]
fn ift_simple_inverts_dft_simple_n4() {
    let x = Vector::new(vec![
        Complex::new(1.0, -0.5),
        Complex::new(0.5, 2.0),
        Complex::new(-1.25, 0.75),
        Complex::new(0.0, -1.0),
    ]);
    let x_freq = dft_simple(&x);
    let x_rec = ift_simple(&x_freq);
    assert!(approx_vec(&x, &x_rec, 1e-9));
}

#[test]
fn ift_inverts_dft_power_of_two_n8() {
    let n = 8;
    let x = Vector::new(
        (0..n)
            .map(|i| {
                Complex::new(
                    ((i * 7 + 3) % 11) as f64 * 0.3,
                    ((i * 5 + 1) % 13) as f64 * -0.2,
                )
            })
            .collect(),
    );
    let x_freq = dft(&x);
    let x_rec = ift(&x_freq);
    assert!(approx_vec(&x, &x_rec, 1e-9));
}

#[test]
fn ift_inverts_dft_mixed_radix_n15() {
    let n = 15; // 3 * 5
    let x = Vector::new(
        (0..n)
            .map(|i| {
                Complex::new(
                    ((i * 11 + 4) % 17) as f64 * 0.1,
                    ((i * 9 + 2) % 19) as f64 * 0.05,
                )
            })
            .collect(),
    );
    let x_freq = dft(&x);
    let x_rec = ift(&x_freq);
    assert!(approx_vec(&x, &x_rec, 1e-9));
}
