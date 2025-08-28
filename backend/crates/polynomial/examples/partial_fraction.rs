use poly::format::{fmt_complex, fmt_real};
use poly::polynomial::Polynomial;
use poly::rational_function::RationalFunction;

fn main() {
    // Example: f(s) = (s^2 + 1) / (s^3 + 3 s^2 + 3 s + 1) = (s^2 + 1) / (s+1)^3
    // Decomposition should be: A/(s+1) + B/(s+1)^2 + C/(s+1)^3
    let num = Polynomial::new(vec![1.0, 0.0, 1.0]);
    let den = Polynomial::from_roots(vec![-1.0, -1.0, -1.0]); // (s+1)^3
    let rf = RationalFunction::new(num, den);

    let pfe = rf.partial_fraction_expansion();

    // 元の有理式を可読表示
    println!("f(s) = {}", rf.display_with("s").unicode_superscript(true));

    // Polynomial part (real coefficients)
    let poly_str = pfe
        .polynomial_part
        .coeffs
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if *c == 0.0 {
                None
            } else if i == 0 {
                Some(fmt_real(*c))
            } else {
                Some(format!("{} s^{i}", fmt_real(*c)))
            }
        })
        .collect::<Vec<_>>()
        .join(" + ");
    println!(
        "polynomial part (coeff form): {}",
        if poly_str.is_empty() {
            "0".to_string()
        } else {
            poly_str
        }
    );
    println!(
        "polynomial part (pretty): {}",
        pfe.polynomial_part
            .display_with("s")
            .unicode_superscript(true)
    );
    for term in pfe.pole_terms {
        print!("pole at s = {}: ", fmt_complex(term.pole));
        for (j, c) in term.coefficients.iter().enumerate() {
            print!("C{} = {}  ", j + 1, fmt_complex(*c));
        }
        println!();
    }
}
