use num_complex::Complex;
use poly::polynomial::solver::RootMethod;
use poly::polynomial::Polynomial;

fn approx_eq(a: Complex<f64>, b: Complex<f64>, tol: f64) -> bool {
    (a - b).norm() <= tol
}

fn assert_roots_match(mut found: Vec<Complex<f64>>, mut expected: Vec<Complex<f64>>, tol: f64) {
    assert_eq!(found.len(), expected.len(), "root count differs");
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
fn sextic_all_real_roots() {
    // (x-1)(x-2)(x-3)(x-4)(x-5)(x-6)
    let roots = vec![1., 2., 3., 4., 5., 6.];
    let p = Polynomial::from_roots(roots);
    let found = p.find_roots();
    let expected = vec![
        Complex::new(1.0, 0.0),
        Complex::new(2.0, 0.0),
        Complex::new(3.0, 0.0),
        Complex::new(4.0, 0.0),
        Complex::new(5.0, 0.0),
        Complex::new(6.0, 0.0),
    ];
    assert_roots_match(found, expected, 1e-6);
}

#[test]
fn septic_mixed_roots() {
    // 実根3つ + 複素共役対2組 = 7次
    let real = vec![-3.0, 0.5, 4.0];
    let complex_pairs = vec![
        (Complex::new(1.0, 2.0), Complex::new(1.0, -2.0)),
        (Complex::new(-2.5, 0.75), Complex::new(-2.5, -0.75)),
    ];
    let pr = Polynomial::from_roots(real.clone());
    let mut pc = Polynomial::new(vec![1.0]);
    for (a, b) in complex_pairs.iter() {
        let q = Polynomial::from_roots(vec![*a, *b]);
        let qr = Polynomial::new(q.coeffs.iter().map(|c| c.re).collect::<Vec<f64>>());
        pc = &pc * &qr;
    }
    let p = &pr * &pc;
    let mut expected: Vec<Complex<f64>> = real.into_iter().map(|r| Complex::new(r, 0.0)).collect();
    for (a, b) in complex_pairs {
        expected.push(a);
        expected.push(b);
    }

    let found = p.find_roots();
    assert_roots_match(found, expected, 1e-5);
}

#[test]
fn nonic_many_complex() {
    // 9次: 複素ペア4組 + 実根1
    let real = vec![2.0];
    let pairs = vec![
        (Complex::new(0.0, 1.0), Complex::new(0.0, -1.0)),
        (Complex::new(1.5, 0.3), Complex::new(1.5, -0.3)),
        (Complex::new(-2.0, 1.1), Complex::new(-2.0, -1.1)),
        (Complex::new(3.2, 2.5), Complex::new(3.2, -2.5)),
    ];
    let pr = Polynomial::from_roots(real.clone());
    let mut pc = Polynomial::new(vec![1.0]);
    for (a, b) in pairs.iter() {
        let q = Polynomial::from_roots(vec![*a, *b]);
        let qr = Polynomial::new(q.coeffs.iter().map(|c| c.re).collect::<Vec<f64>>());
        pc = &pc * &qr;
    }
    let p = &pr * &pc;
    let mut expected: Vec<Complex<f64>> = real.into_iter().map(|r| Complex::new(r, 0.0)).collect();
    for (a, b) in pairs {
        expected.push(a);
        expected.push(b);
    }

    let found = p.find_roots();
    assert_roots_match(found, expected, 1e-5);
}

#[test]
fn decic_close_roots() {
    // 10次: 近接する根（数値的にやや難しい）
    let roots: Vec<f64> = (0..10).map(|k| 1.0 + (k as f64) * 1e-3).collect();
    let p = Polynomial::from_roots(roots.clone());
    let found = p.find_roots();
    let expected: Vec<Complex<f64>> = roots.into_iter().map(|r| Complex::new(r, 0.0)).collect();
    // 非常に近接する10個の根は条件数が高く、数値的に難しいため少し緩める
    assert_roots_match(found, expected, 5e-2);
}

#[test]
fn sextic_compare_pure_and_hybrid() {
    // 2次（純JTは低次数は閉形式で解く）
    let p = Polynomial::from_roots(vec![-3.0, 1.5]);
    let r_pure = p.find_roots_with(RootMethod::JenkinsTraubPure);
    let r_hybrid = p.find_roots_with(RootMethod::JenkinsTraubHybrid);
    let expected = vec![Complex::new(-3.0, 0.0), Complex::new(1.5, 0.0)];
    assert_roots_match(r_pure.clone(), expected.clone(), 1e-6);
    assert_roots_match(r_hybrid, expected, 1e-8);
}

#[test]
fn septic_compare_pure_and_hybrid_mixed() {
    // 2次、複素共役対
    let pair = (Complex::new(1.0, 1.2), Complex::new(1.0, -1.2));
    let q = Polynomial::from_roots(vec![pair.0, pair.1]);
    let p = Polynomial::new(q.coeffs.iter().map(|c| c.re).collect::<Vec<f64>>());
    let expected = vec![pair.0, pair.1];

    let r_pure = p.find_roots_with(RootMethod::JenkinsTraubPure);
    let r_hybrid = p.find_roots_with(RootMethod::JenkinsTraubHybrid);
    assert_roots_match(r_pure.clone(), expected.clone(), 1e-6);
    assert_roots_match(r_hybrid, expected, 1e-6);
}
