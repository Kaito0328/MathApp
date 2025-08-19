use linalg::Vector;
use num_complex::Complex;
use signal_processing::dft::conv_with_dft;

fn to_complex_real(v: &[f64]) -> Vector<Complex<f64>> {
    Vector::new(v.iter().cloned().map(|r| Complex::new(r, 0.0)).collect())
}

fn to_complex(vr: &[f64], vi: &[f64]) -> Vector<Complex<f64>> {
    assert_eq!(vr.len(), vi.len());
    Vector::new(
        vr.iter()
            .zip(vi.iter())
            .map(|(r, i)| Complex::new(*r, *i))
            .collect(),
    )
}

fn assert_vec_approx(a: &Vector<Complex<f64>>, b: &Vector<Complex<f64>>, eps: f64) {
    assert_eq!(a.dim(), b.dim());
    for k in 0..a.dim() {
        let da = a.iter().nth(k).unwrap();
        let db = b.iter().nth(k).unwrap();
        let diff = (*da - *db).norm();
        assert!(diff <= eps, "idx {k}: {da} vs {db} (|diff|={diff})");
    }
}

#[test]
fn conv_real_small_matches_naive() {
    // [1,2,3] * [4,5] = [4,13,22,15]
    let x = to_complex_real(&[1.0, 2.0, 3.0]);
    let h = to_complex_real(&[4.0, 5.0]);
    let via_dft = conv_with_dft(&x, &h);
    let naive = x.conv(&h);
    assert_vec_approx(&via_dft, &naive, 1e-9);
}

#[test]
fn conv_real_mixed_lengths() {
    // mixed lengths hitting non-power-of-two DFT size
    let x = to_complex_real(&[0.5, -1.0, 2.0, 0.25, -0.75]); // len=5
    let h = to_complex_real(&[1.0, 0.0, -0.5, 0.25, 0.75, -0.25, 0.5]); // len=7, out len=11 (prime)
    let via_dft = conv_with_dft(&x, &h);
    let naive = x.conv(&h);
    assert_eq!(via_dft.dim(), 11);
    assert_vec_approx(&via_dft, &naive, 1e-9);
}

#[test]
fn conv_complex_sequence_matches_naive() {
    // complex-valued sequences
    let xr = [1.0, 2.0, -1.0];
    let xi = [1.0, -1.0, 0.5];
    let hr = [0.5, -0.25];
    let hi = [0.0, 1.0];
    let x = to_complex(&xr, &xi);
    let h = to_complex(&hr, &hi);
    let via_dft = conv_with_dft(&x, &h);
    let naive = x.conv(&h);
    assert_vec_approx(&via_dft, &naive, 1e-9);
}
