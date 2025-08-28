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

    // 可読表示（上付き指数ON）
    println!("T4(x) = {}", t4.display_with("x").unicode_superscript(true));
    println!("U3(x) = {}", u3.display_with("x").unicode_superscript(true));
    println!("P3(x) = {}", p3.display_with("x").unicode_superscript(true));
    println!("H3(x) = {}", h3.display_with("x").unicode_superscript(true));
    println!("L3(x) = {}", l3.display_with("x").unicode_superscript(true));
}
