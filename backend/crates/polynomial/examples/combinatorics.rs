use num_complex::Complex;
use poly::combinatorics::numbers::{binom, stirling2};
use poly::combinatorics::polynomials::{
    binom_x_plus_k_choose_k_poly, falling_factorial_poly, rising_factorial_poly,
    shift_poly_x_plus_h,
};

fn main() {
    println!("binom(10,3) = {}", binom(10, 3));
    println!("S2(5,2) = {}", stirling2(5, 2));

    let ff = falling_factorial_poly(4); // x(x-1)(x-2)(x-3)
    let rf = rising_factorial_poly(4); // x(x+1)(x+2)(x+3)
    println!("(x)_4 coeffs:   {:?}", ff.coeffs);
    println!("(x)^(4) coeffs: {:?}", rf.coeffs);

    let c = binom_x_plus_k_choose_k_poly(3); // C(x+3,3)
    for x in 0..5 {
        println!("C({}+3,3) = {}", x, c.eval(Complex::new(x as f64, 0.0)).re);
    }

    // シフト: p(x+h)
    let p = rf.clone();
    let q = shift_poly_x_plus_h(&p, 1.0); // p(x+1)
    println!("shifted (x)^(4) by +1 coeffs: {:?}", q.coeffs);
}
