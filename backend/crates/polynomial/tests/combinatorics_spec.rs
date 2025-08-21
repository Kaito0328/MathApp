use num_complex::Complex;
use poly::combinatorics::numbers::{binom, stirling2};
use poly::combinatorics::polynomials::{binom_x_plus_k_choose_k_poly, falling_factorial_poly};

#[test]
fn binom_small_values() {
    assert_eq!(binom(5, 0), 1.0);
    assert_eq!(binom(5, 1), 5.0);
    assert_eq!(binom(5, 2), 10.0);
    assert_eq!(binom(5, 3), 10.0);
    assert_eq!(binom(5, 4), 5.0);
    assert_eq!(binom(5, 5), 1.0);
}

#[test]
fn stirling2_small_values() {
    assert_eq!(stirling2(0, 0), 1.0);
    assert_eq!(stirling2(3, 1), 1.0);
    assert_eq!(stirling2(3, 2), 3.0);
    assert_eq!(stirling2(3, 3), 1.0);
}

#[test]
fn falling_factorial_poly_degree_and_roots() {
    let p = falling_factorial_poly(3);
    // roots at 0,1,2
    let r = vec![0.0, 1.0, 2.0];
    for x in r {
        let v = p.eval(Complex::new(x, 0.0));
        assert!(v.norm() < 1e-9);
    }
}

#[test]
fn binom_x_plus_k_choose_k_poly_eval() {
    // For k=3: C(x+3,3) at x=0..3 equals 1,4,10,20
    let p = binom_x_plus_k_choose_k_poly(3);
    let vals = [1.0, 4.0, 10.0, 20.0];
    for (x, &e) in (0..4).zip(vals.iter()) {
        let v = p.eval(Complex::new(x as f64, 0.0)).re;
        assert!((v - e).abs() < 1e-9);
    }
}
