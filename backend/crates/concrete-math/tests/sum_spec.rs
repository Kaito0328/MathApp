use concrete_math::sequence::core::ClosedForm;
use concrete_math::sum::partial_sum::partial_sum;
use num_complex::Complex;
use poly::polynomial::Polynomial;

#[test]
fn sum_of_ones_is_linear() {
    // a_n = 1 => S_n = n+1
    let cf = ClosedForm::single(Polynomial::one().to_complex(), Complex::new(1.0, 0.0));
    let s = partial_sum(&cf);
    for n in 0..10 {
        let v = s.term(n).re;
        assert!((v - (n as f64 + 1.0)).abs() < 1e-9);
    }
}

#[test]
fn sum_of_geometric_r_is() {
    // a_n = 2^n => S_n = 2^{n+1} - 1
    let cf = ClosedForm::single(Polynomial::one().to_complex(), Complex::new(2.0, 0.0));
    let s = partial_sum(&cf);
    for n in 0..10 {
        let v = s.term(n).re;
        let expect = 2f64.powi((n as i32) + 1) - 1.0;
        assert!((v - expect).abs() < 1e-7);
    }
}
