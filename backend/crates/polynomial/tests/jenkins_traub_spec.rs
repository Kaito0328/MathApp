use num_complex::Complex;
use poly::polynomial::Polynomial;

fn approx_eq(a: Complex<f64>, b: Complex<f64>, tol: f64) -> bool {
    (a - b).norm() <= tol
}

fn assert_roots_match(mut found: Vec<Complex<f64>>, mut expected: Vec<Complex<f64>>, tol: f64) {
    assert_eq!(found.len(), expected.len(), "root count differs");

    // Greedy matching: for each expected, find a close found root and remove it
    for e in expected.drain(..) {
        let mut idx = None;
        for (i, f) in found.iter().enumerate() {
            if approx_eq(*f, e, tol) {
                idx = Some(i);
                break;
            }
        }
        assert!(
            idx.is_some(),
            "expected root {e:?} not found within tol {tol} in {found:?}"
        );
    found.remove(idx.expect("matching root index"));
    }
}

#[test]
fn linear_root() {
    // p(x) = x - 3
    let p = Polynomial::new(vec![-3.0, 1.0]);
    let roots = p.find_roots();
    assert_roots_match(roots, vec![Complex::new(3.0, 0.0)], 1e-10);
}

#[test]
fn quadratic_real_roots() {
    // p(x) = x^2 - 5x + 6 = (x-2)(x-3)
    let p = Polynomial::new(vec![6.0, -5.0, 1.0]);
    let roots = p.find_roots();
    assert_roots_match(
        roots,
        vec![Complex::new(2.0, 0.0), Complex::new(3.0, 0.0)],
        1e-10,
    );
}

#[test]
fn quadratic_complex_roots() {
    // p(x) = x^2 + 2x + 5 -> -1 ± 2j
    let p = Polynomial::new(vec![5.0, 2.0, 1.0]);
    let roots = p.find_roots();
    assert_roots_match(
        roots,
        vec![Complex::new(-1.0, 2.0), Complex::new(-1.0, -2.0)],
        1e-8,
    );
}

#[test]
fn cubic_three_real_roots() {
    // p(x) = (x-1)(x-2)(x-3) = x^3 - 6x^2 + 11x - 6
    let p = Polynomial::new(vec![-6.0, 11.0, -6.0, 1.0]);
    let roots = p.find_roots();
    assert_roots_match(
        roots,
        vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
        ],
        1e-8,
    );
}

#[test]
fn quintic_mixed_roots_from_roots_roundtrip() {
    // Construct polynomial from known roots and verify we get them back
    let expected: Vec<Complex<f64>> = vec![
        Complex::new(-2.0, 0.0),
        Complex::new(0.5, 0.0),
        Complex::new(1.0, 1.5),
        Complex::new(1.0, -1.5),
        Complex::new(3.0, 0.0),
    ];
    let real_roots: Vec<f64> = expected
        .iter()
        .filter(|c| c.im.abs() < 1e-12)
        .map(|c| c.re)
        .collect();

    // Build polynomial with complex and real factors: (x - r) for each root
    // We multiply real-root polynomial and quadratic factors for complex pairs.
    let real_poly = Polynomial::from_roots(real_roots);
    let complex_pair_poly =
        Polynomial::from_roots(vec![Complex::new(1.0, 1.5), Complex::new(1.0, -1.5)]);
    let complex_pair_poly_real = complex_pair_poly
        .coeffs
        .iter()
        .map(|c| c.re)
        .collect::<Vec<f64>>();
    let p = &real_poly * &Polynomial::new(complex_pair_poly_real);

    let roots = p.find_roots();
    assert_eq!(roots.len(), expected.len());
    assert_roots_match(roots, expected, 1e-6);
}

#[test]
fn residuals_are_small_at_found_roots() {
    // p(x) = x^4 + x^3 - 7x^2 - x + 6 = (x-1)(x-2)(x+1)(x+3)
    let p = Polynomial::new(vec![6.0, -1.0, -7.0, 1.0, 1.0]);
    let roots = p.find_roots();
    let pc = p.to_complex();
    for r in roots {
        let val = pc.eval(r);
        assert!(val.norm() < 1e-8, "residual too large at root {r:?}: {val}");
    }
}
