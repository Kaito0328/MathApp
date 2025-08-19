use linalg::Vector;
use signal_processing::iir::{
    bilinear_transform, find_polynomial_coefficient, find_polynomial_coefficient_fast, iir_filter,
    BinomialCoeffsCache,
};

fn approx_eq_vec(a: &Vector<f64>, b: &Vector<f64>, eps: f64) {
    assert_eq!(a.dim(), b.dim());
    for i in 0..a.dim() {
        let da = a[i];
        let db = b[i];
        let diff = (da - db).abs();
        assert!(diff <= eps, "idx {i}: {da} vs {db} (|diff|={diff})");
    }
}

#[test]
fn binomial_coeffs_plus_and_minus() {
    let mut cache = BinomialCoeffsCache::new();
    // (x+1)^0 = [1]
    assert_eq!(cache.get_x_plus_1(0).data, vec![1.0]);
    // (x+1)^3 = [1,3,3,1]
    assert_eq!(cache.get_x_plus_1(3).data, vec![1.0, 3.0, 3.0, 1.0]);

    // (x-1)^3 = [(-1)^3, 3(-1)^2, 3(-1)^1, 1] with ascending order
    // Ascending coefficients: [ -1, 3, -3, 1 ]
    assert_eq!(cache.get_x_minus_1(3).data, vec![-1.0, 3.0, -3.0, 1.0]);

    // symmetry check for k=4: (x-1)^4 ascending: [1,-4,6,-4,1]
    assert_eq!(cache.get_x_minus_1(4).data, vec![1.0, -4.0, 6.0, -4.0, 1.0]);
}

#[test]
fn polynomial_from_roots_matches_expected() {
    // Roots a,b => (x-a)(x-b) = x^2 - (a+b)x + ab
    let roots = [2.0, -3.0];
    let coeff = find_polynomial_coefficient(&roots);
    let expected = Vector::new(vec![
        roots[0] * roots[1],    // x^0
        -(roots[0] + roots[1]), // x^1
        1.0,                    // x^2
    ]);
    approx_eq_vec(&coeff, &expected, 1e-12);

    let coeff_fast = find_polynomial_coefficient_fast(&roots);
    approx_eq_vec(&coeff_fast, &expected, 1e-12);
}

#[test]
fn polynomial_from_roots_three_terms() {
    // Roots [1,2,3] => x^3 - 6x^2 + 11x - 6
    let roots = [1.0, 2.0, 3.0];
    let coeff = find_polynomial_coefficient(&roots);
    let expected = Vector::new(vec![
        -6.0, // x^0
        11.0, // x^1
        -6.0, // x^2
        1.0,  // x^3
    ]);
    approx_eq_vec(&coeff, &expected, 1e-12);

    let coeff_fast = find_polynomial_coefficient_fast(&roots);
    approx_eq_vec(&coeff_fast, &expected, 1e-12);
}

#[test]
fn bilinear_transform_integrator() {
    // Analog H(s) = 1 / s
    // analog_b(s) = 1, analog_a(s) = s  => ascending coeffs: b=[1], a=[0,1]
    let analog_b = Vector::new(vec![1.0]);
    let analog_a = Vector::new(vec![0.0, 1.0]);

    // Using bilinear with fs -> T = 1/fs.
    // Expected discrete H(z) ≈ (T/2) * (1 + z^-1) / (1 - z^-1)
    // So (after normalizing a0 to 1): a=[1, -1], b=[T/2, T/2]
    let fs = 2.0; // T=0.5 => T/2=0.25
    let (mut b_d, mut a_d) = bilinear_transform(&analog_b, &analog_a, fs);

    // Normalize so that a0 == 1
    let a0 = a_d[0];
    for i in 0..b_d.dim() {
        b_d[i] /= a0;
    }
    for i in 0..a_d.dim() {
        a_d[i] /= a0;
    }

    let expected_b = Vector::new(vec![0.25, 0.25]);
    let expected_a = Vector::new(vec![1.0, -1.0]);

    approx_eq_vec(&b_d, &expected_b, 1e-9);
    approx_eq_vec(&a_d, &expected_a, 1e-9);
}

#[test]
fn iir_filter_impulse_first_order() {
    // y[n] - 0.5 y[n-1] = 0.5 x[n]
    // b=[0.5], a=[1.0, -0.5]
    let b = [0.5];
    let a = [1.0, -0.5];
    let impulse = vec![1.0, 0.0, 0.0, 0.0, 0.0];
    let y = iir_filter(&impulse, &b, &a);

    // Expected impulse response: 0.5, 0.25, 0.125, 0.0625, 0.03125
    let expected = Vector::new(vec![0.5, 0.25, 0.125, 0.0625, 0.03125]);
    approx_eq_vec(&y, &expected, 1e-12);
}
