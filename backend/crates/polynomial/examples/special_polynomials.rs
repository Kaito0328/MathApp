use poly::polynomial::special::{
    chebyshev_first_kind, chebyshev_second_kind, hermite_physicists, laguerre, legendre,
};

fn main() {
    // いくつかの特別多項式を表示
    let t4 = chebyshev_first_kind::<f64>(4); // 8x^4 - 8x^2 + 1
    let u3 = chebyshev_second_kind::<f64>(3); // 8x^3 - 4x
    let p3 = legendre::<f64>(3); // (5x^3 - 3x)/2
    let h3 = hermite_physicists::<f64>(3); // 8x^3 - 12x
    let l3 = laguerre::<f64>(3); // (1/6)(-x^3 + 9x^2 - 18x + 6)

    println!("T4 coeffs:   {:?}", t4.coeffs);
    println!("U3 coeffs:   {:?}", u3.coeffs);
    println!("P3 coeffs:   {:?}", p3.coeffs);
    println!("H3 coeffs:   {:?}", h3.coeffs);
    println!("L3 coeffs:   {:?}", l3.coeffs);
}
