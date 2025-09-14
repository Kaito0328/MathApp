use concrete_math::sequence::core::{ClosedForm, GeneralTerm};
use concrete_math::sequence::recurrence_relation::RecurrenceRelation;
use num_complex::Complex;
use poly::polynomial::Polynomial;

fn eval(cf: &ClosedForm, n: usize) -> Complex<f64> {
    cf.term(n as u32)
}

#[test]
fn fibonacci_homogeneous() {
    // a_n = a_{n-1} + a_{n-2}, a_0=0, a_1=1
    let rr = RecurrenceRelation::new(vec![1.0, 1.0], vec![], vec![0.0, 1.0]);
    let cf = rr.solve();
    let want = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
    for (i, &w) in want.iter().enumerate() {
        assert!((eval(&cf, i).re - w as f64).abs() < 1e-9);
        assert!(eval(&cf, i).im.abs() < 1e-9);
    }
}

#[test]
fn arithmetic_progression_inhomogeneous() {
    // a_n - a_{n-1} = 1  => a_n = n with a_0=0
    // non_homogeneous term: P(n) r^n with P(n)=1, r=1
    let nh = vec![GeneralTerm {
        polynomial: Polynomial::one().to_complex(),
        base: Complex::new(1.0, 0.0),
    }];
    let rr = RecurrenceRelation::new(vec![1.0], nh, vec![0.0]);
    let cf = rr.solve();
    for n in 0..10 {
        assert!((eval(&cf, n).re - n as f64).abs() < 1e-9);
    }
}

#[test]
fn geometric_weighted_linear_inhomogeneous() {
    // a_n - 2 a_{n-1} = n, a_0=0  => nonhomogeneous P(n) = n (係数 [0,1])
    let nh = vec![GeneralTerm {
        polynomial: Polynomial::new(vec![0.0, 1.0]).to_complex(),
        base: Complex::new(1.0, 0.0),
    }];
    let rr = RecurrenceRelation::new(vec![2.0], nh, vec![0.0]);
    let cf = rr.solve();

    // check first few terms from direct recurrence
    let mut a = [0.0f64; 12];
    for n in 1..12 {
        a[n] = 2.0 * a[n - 1] + (n as f64);
    }
    for (n, &val) in a.iter().enumerate() {
        assert!((eval(&cf, n).re - val).abs() < 1e-7);
    }
}

#[test]
fn quadratic_inhomogeneous_partial_sum() {
    // a_n - a_{n-1} = n^2, a_0 = 0
    // 非同次項: P(n) = n^2（係数 [0,0,1]）, base r = 1
    let nh = vec![GeneralTerm {
        polynomial: Polynomial::new(vec![0.0, 0.0, 1.0]).to_complex(),
        base: Complex::new(1.0, 0.0),
    }];
    let rr = RecurrenceRelation::new(vec![1.0], nh, vec![0.0]);
    let cf = rr.solve();

    // 期待値: n(n+1)(2n+1)/6
    for n in 0..50 {
        let got = eval(&cf, n).re;
        let n_f = n as f64;
        let want = n_f * (n_f + 1.0) * (2.0 * n_f + 1.0) / 6.0;
        let err = (got - want).abs();
        eprintln!("n={n}, got={got:.12e}, want={want:.12e}, err={err:.3e}");
        assert!(err < 1e-6);
    }
}

#[test]
fn exponential_inhomogeneous_partial_sum() {
    // a_n - a_{n-1} = 2^n, a_0 = 0  => a_n = sum_{k=0..n} 2^k = 2^{n+1} - 2
    let nh = vec![GeneralTerm {
        polynomial: Polynomial::one().to_complex(),
        base: Complex::new(2.0, 0.0),
    }];
    let rr = RecurrenceRelation::new(vec![1.0], nh, vec![0.0]);
    let cf = rr.solve();
    for n in 0..30 {
        let got = eval(&cf, n).re;
        let want = 2f64.powi((n as i32) + 1) - 2.0;
        let err = (got - want).abs();
        eprintln!("[exp] n={n}, got={got:.12e}, want={want:.12e}, err={err:.3e}");
        assert!(err < 1e-9);
    }
}

#[test]
fn mixed_linear_exponential_inhomogeneous_partial_sum() {
    // a_n - a_{n-1} = n * 2^n, a_0 = 0
    // 期待値: sum_{k=0..n} k 2^k = (n-1)2^{n+1} + 2  （n>=0 で成り立つ）
    let nh = vec![GeneralTerm {
        polynomial: Polynomial::new(vec![0.0, 1.0]).to_complex(), // P(n)=n
        base: Complex::new(2.0, 0.0),
    }];
    let rr = RecurrenceRelation::new(vec![1.0], nh, vec![0.0]);
    let cf = rr.solve();
    for n in 0..30 {
        let got = eval(&cf, n).re;
        let n_i = n as i32;
        let want = (n_i - 1) as f64 * 2f64.powi(n_i + 1) + 2.0;
        let err = (got - want).abs();
        eprintln!("[n*2^n] n={n}, got={got:.12e}, want={want:.12e}, err={err:.3e}");
        assert!(err < 1e-7);
    }
}
