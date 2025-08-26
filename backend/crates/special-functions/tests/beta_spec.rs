use special_functions::beta::{beta, log_beta, regularized_beta};

#[test]
fn beta_known_values() {
    // B(1,1) = 1
    assert!((beta(1.0, 1.0) - 1.0).abs() < 1e-12);
    // B(1/2,1/2) = pi
    assert!((beta(0.5, 0.5) - std::f64::consts::PI).abs() < 1e-12);
    // B(2,3) = 1/12
    assert!((beta(2.0, 3.0) - (1.0 / 12.0)).abs() < 1e-15);
}

#[test]
fn log_beta_consistency() {
    let a = 3.5;
    let b = 2.25;
    let lb = log_beta(a, b);
    let bv = beta(a, b);
    assert!(((lb.exp()) - bv).abs() < 1e-12);
}

#[test]
fn regularized_beta_basic_properties() {
    // I_x(a,b) in [0,1]
    let a = 2.5;
    let b = 1.5;
    for &x in &[0.0, 0.1, 0.5, 0.9, 1.0] {
        let v = regularized_beta(a, b, x);
        if x == 0.0 {
            assert!(v.abs() < 1e-15);
        } else if x == 1.0 {
            assert!((v - 1.0).abs() < 1e-14);
        } else {
            assert!(v.is_finite());
            assert!((0.0..=1.0).contains(&v));
        }
    }
}

#[test]
fn regularized_beta_symmetry() {
    // I_x(a,b) = 1 - I_{1-x}(b,a)
    let a = 3.2;
    let b = 0.9;
    let x = 0.7;
    let lhs = regularized_beta(a, b, x);
    let rhs = 1.0 - regularized_beta(b, a, 1.0 - x);
    assert!((lhs - rhs).abs() < 1e-12);
}

#[test]
fn regularized_beta_uniform_identity() {
    // For a=b=1, I_x(1,1) = x
    for &x in &[0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0] {
        let v = regularized_beta(1.0, 1.0, x);
        assert!((v - x).abs() < 1e-12);
    }
}
