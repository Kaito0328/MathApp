use num_complex::Complex;
use poly::polynomial::Polynomial;
use poly::rational_function::{PartialFractionExpansion, RationalFunction};

fn approx_eq(a: Complex<f64>, b: Complex<f64>, tol: f64) -> bool {
    (a - b).norm() <= tol
}

#[test]
fn partial_fraction_simple_real_poles() {
    // f(s) = (s + 3) / (s^2 + 3 s + 2) = 2/(s+1) - 1/(s+2)
    let num = Polynomial::new(vec![3.0, 1.0]);
    let den = Polynomial::new(vec![2.0, 3.0, 1.0]);
    let rf = RationalFunction::new(num, den);

    let pfe: PartialFractionExpansion = rf.partial_fraction_expansion();

    // polynomial part should be zero
    assert!(pfe.polynomial_part.is_zero());

    // Poles at -1 and -2
    assert_eq!(pfe.pole_terms.len(), 2);

    // Map poles to coefficients
    let mut coeff_at_neg1 = None;
    let mut coeff_at_neg2 = None;
    for term in &pfe.pole_terms {
        let p = term.pole;
        assert_eq!(term.coefficients.len(), 1); // simple poles
        if approx_eq(p, Complex::new(-1.0, 0.0), 1e-8) {
            coeff_at_neg1 = Some(term.coefficients[0]);
        } else if approx_eq(p, Complex::new(-2.0, 0.0), 1e-8) {
            coeff_at_neg2 = Some(term.coefficients[0]);
        }
    }
    let c1 = coeff_at_neg1.expect("missing pole at -1");
    let c2 = coeff_at_neg2.expect("missing pole at -2");

    assert!(approx_eq(c1, Complex::new(2.0, 0.0), 1e-8));
    assert!(approx_eq(c2, Complex::new(-1.0, 0.0), 1e-8));
}

#[test]
fn partial_fraction_repeated_pole() {
    // f(s) = 1 / (s+1)^2 = 1/(s+1)^2
    let num = Polynomial::new(vec![1.0]);
    let den = Polynomial::from_roots(vec![-1.0, -1.0]); // (s+1)^2
    let rf = RationalFunction::new(num, den);

    let pfe: PartialFractionExpansion = rf.partial_fraction_expansion();

    assert!(pfe.polynomial_part.is_zero());

    // One pole at -1 with multiplicity 2, so one PoleTerm with two coefficients [C1, C2]
    assert_eq!(pfe.pole_terms.len(), 1);
    let term = &pfe.pole_terms[0];
    assert!(approx_eq(term.pole, Complex::new(-1.0, 0.0), 1e-8));
    assert_eq!(term.coefficients.len(), 2);

    // Expected: 0/(s+1) + 1/(s+1)^2
    let c1 = term.coefficients[0]; // coefficient for 1/(s+1)
    let c2 = term.coefficients[1]; // coefficient for 1/(s+1)^2

    assert!(approx_eq(c1, Complex::new(0.0, 0.0), 1e-8));
    assert!(approx_eq(c2, Complex::new(1.0, 0.0), 1e-8));
}
