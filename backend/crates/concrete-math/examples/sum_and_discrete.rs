use concrete_math::sequence::core::ClosedForm;
use concrete_math::sum::discrete::{discrete_diff, discrete_sum};
use concrete_math::sum::partial_sum::partial_sum;
use num_complex::Complex;
use poly::polynomial::Polynomial;

fn main() {
    // 部分和の例: a_n = 2^n
    let cf = ClosedForm::single(Polynomial::one().to_complex(), Complex::new(2.0, 0.0));
    let s = partial_sum(&cf);
    for n in 0..5 {
        println!("S({}) = {}", n, s.term(n).re);
    }

    // 離散微分/積分の例
    let p = Polynomial::new(vec![
        Complex::new(3.0, 0.0),
        Complex::new(2.0, 0.0),
        Complex::new(1.0, 0.0),
    ]); // x^2 + 2x + 3
    let d = discrete_diff(&p); // 2x + 3
    println!("Δp coeffs: {:?}", d.coeffs);

    let lin = Polynomial::new(vec![Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)]); // x+1
    let sum = discrete_sum(&lin);
    for n in 0..5 {
        println!(
            "Σ_0^{} (i+1) = {}",
            n,
            sum.eval(Complex::new(n as f64, 0.0)).re
        );
    }
}
