use crate::polynomial::Polynomial;
use linalg::Field;

impl<F: Field> Polynomial<F> {
    pub fn gcd(a: &Self, b: &Self) -> Self {
        let mut r0 = a.clone();
        let mut r1 = b.clone();
        while r1.deg() >= 0 && !(r1.coeffs.len() == 1 && r1.coeffs[0].is_zero()) {
            let (_q, r) = r0.div_rem(&r1);
            r0 = r1;
            r1 = r;
        }
        r0.monic()
    }

    pub fn lcm(a: &Self, b: &Self) -> Self {
        if a.deg() < 0 {
            return b.clone();
        }
        if b.deg() < 0 {
            return a.clone();
        }
        let g = Polynomial::gcd(a, b);
        let ab = a * b; // Mul演算子
        let (q, _r) = ab.div_rem(&g);
        q.monic()
    }
}
