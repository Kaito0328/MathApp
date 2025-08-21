use num_complex::Complex;
use poly::polynomial::Polynomial;
use poly::sequence::core::{ClosedForm, GeneralTerm};
use poly::sequence::recurrence_relation::RecurrenceRelation;

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
