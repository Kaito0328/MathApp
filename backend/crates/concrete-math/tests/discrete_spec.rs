use concrete_math::sum::discrete::{discrete_diff, discrete_sum};
use num_complex::Complex;
use poly::polynomial::Polynomial;

#[test]
fn discrete_diff_of_quadratic() {
    // p(x) = x^2 + 2x + 3 => Δp = (x+1)^2 - x^2 + 2 => 2x + 1 + 2 = 2x + 3
    let p = Polynomial::new(vec![
        Complex::new(3.0, 0.0),
        Complex::new(2.0, 0.0),
        Complex::new(1.0, 0.0),
    ]);
    let d = discrete_diff(&p);
    for x in 0..5 {
        let v = d.eval(Complex::new(x as f64, 0.0)).re;
        assert!((v - (2.0 * x as f64 + 3.0)).abs() < 1e-9);
    }
}

#[test]
fn discrete_sum_of_linear() {
    // p(x) = x+1 => 累積和 Q(n) = n(n+1)/2 + n
    let p = Polynomial::new(vec![Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)]);
    let s = discrete_sum(&p);
    for n in 0..6 {
        // 直接和
        let mut q = 0.0;
        for i in 0..=n {
            q += i as f64 + 1.0;
        }
        let v = s.eval(Complex::new(n as f64, 0.0)).re;
        assert!((v - q).abs() < 1e-7);
    }
}
