use special_functions::gamma::gamma;
use special_functions::gamma::regularized_gamma;

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-12
}

#[test]
fn gamma_integers() {
    assert!(approx(gamma(1.0), 1.0));
    assert!(approx(gamma(2.0), 1.0));
    assert!(approx(gamma(3.0), 2.0));
    assert!(approx(gamma(4.0), 6.0));
}

#[test]
fn gamma_half_integers() {
    let rt_pi = std::f64::consts::PI.sqrt();
    assert!(approx(gamma(0.5), rt_pi));
    assert!(approx(gamma(1.5), 0.5 * rt_pi));
    assert!(approx(gamma(2.5), 1.5 * 0.5 * rt_pi));
}

#[test]
fn gamma_functional_equation() {
    // gamma(z+1) = z * gamma(z)
    let z = 2.3;
    assert!(approx(gamma(z + 1.0), z * gamma(z)));
}

#[test]
fn gamma_reflection_basic() {
    // reflection consistency near 0<z<1
    let z = 0.3;
    let lhs = gamma(z);
    let rhs = std::f64::consts::PI / ((std::f64::consts::PI * z).sin() * gamma(1.0 - z));
    assert!(approx(lhs, rhs));
}

#[test]
fn lower_regularized_gamma_series_branch() {
    let s = 2.5;
    let x = 1.0; // x < s+1 → series branch
    let p = regularized_gamma(s, x);
    assert!(p.is_finite());
    assert!(p > 0.0 && p < 1.0);
}

#[test]
fn lower_regularized_gamma_cf_branch() {
    let s = 2.5;
    let x = 10.0; // x > s+1 → continued-fraction branch
    let p = regularized_gamma(s, x);
    assert!(p.is_finite());
    assert!(p > 0.0 && p < 1.0);
}

#[test]
fn lower_regularized_gamma_bounds() {
    let s = 3.0;
    assert!(approx(regularized_gamma(s, 0.0), 0.0));
    let p = regularized_gamma(s, 100.0);
    assert!(p > 0.999);
}
