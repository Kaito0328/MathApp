use linalg::Field;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Poly<F: Field + Clone + PartialEq> {
    pub coeffs: Vec<F>, // 低次→高次
}

impl<F: Field + Clone + PartialEq + Zero + One> Poly<F> {
    pub fn new(mut c: Vec<F>) -> Self {
        Self {
            coeffs: Self::trim(&mut c),
        }
    }
    fn trim(v: &mut Vec<F>) -> Vec<F> {
        while v.len() > 1 && v.last().map(|x| x.is_zero()).unwrap_or(false) {
            v.pop();
        }
        if v.is_empty() {
            v.push(F::zero());
        }
        v.clone()
    }
    pub fn zero() -> Self {
        Self {
            coeffs: vec![F::zero()],
        }
    }
    pub fn one() -> Self {
        Self {
            coeffs: vec![F::one()],
        }
    }
    pub fn deg(&self) -> isize {
        self.coeffs.len() as isize - 1
    }
    pub fn get(&self, i: usize) -> F {
        self.coeffs.get(i).cloned().unwrap_or_else(F::zero)
    }

    pub fn add(&self, other: &Self) -> Self {
        let n = self.coeffs.len().max(other.coeffs.len());
        let mut v = vec![F::zero(); n];
        for (i, coeff) in v.iter_mut().enumerate() {
            *coeff = self.get(i) + other.get(i);
        }
        Poly::new(v)
    }
    pub fn sub(&self, other: &Self) -> Self {
        let n = self.coeffs.len().max(other.coeffs.len());
        let mut v = vec![F::zero(); n];
        for (i, coeff) in v.iter_mut().enumerate() {
            *coeff = self.get(i) - other.get(i);
        }
        Poly::new(v)
    }
    pub fn mul(&self, other: &Self) -> Self {
        if self.deg() < 0 || other.deg() < 0 {
            return Poly::zero();
        }
        let mut v = vec![F::zero(); self.coeffs.len() + other.coeffs.len() - 1];
        for i in 0..self.coeffs.len() {
            for j in 0..other.coeffs.len() {
                v[i + j] = v[i + j].clone() + self.coeffs[i].clone() * other.coeffs[j].clone();
            }
        }
        Poly::new(v)
    }
    pub fn div_rem(&self, divisor: &Self) -> (Self, Self) {
        let mut r = self.coeffs.clone();
        while r.len() > 1 && r.last().map(|x| x.is_zero()).unwrap_or(false) {
            r.pop();
        }
        let mut rpoly = Poly::new(r);
        if divisor.coeffs.is_empty() || *divisor == Poly::zero() {
            return (Poly::zero(), self.clone());
        }
        let dl = divisor.coeffs.len();
        let lead = divisor.coeffs[dl - 1].clone();
        let mut q = vec![F::zero(); self.coeffs.len().saturating_sub(dl) + 1];
        while rpoly.coeffs.len() >= dl && !(rpoly.coeffs.len() == 1 && rpoly.coeffs[0].is_zero()) {
            let shift = rpoly.coeffs.len() - dl;
            let coef = rpoly.coeffs.last().unwrap().clone() / lead.clone();
            q[shift] = coef.clone();
            for i in 0..dl {
                let idx = i + shift;
                let val = rpoly.coeffs[idx].clone() - coef.clone() * divisor.coeffs[i].clone();
                rpoly.coeffs[idx] = val;
            }
            while rpoly.coeffs.len() > 1
                && rpoly.coeffs.last().map(|x| x.is_zero()).unwrap_or(false)
            {
                rpoly.coeffs.pop();
            }
        }
        (Poly::new(q), rpoly)
    }
    pub fn eval(&self, x: F) -> F {
        let mut acc = F::zero();
        for c in self.coeffs.iter().rev() {
            acc = acc * x.clone() + c.clone();
        }
        acc
    }

    pub fn monic(&self) -> Self {
        if self.deg() < 0 {
            return self.clone();
        }
        let lc = self.coeffs.last().unwrap().clone();
        if lc.is_zero() {
            return Poly::zero();
        }
        let inv = F::one() / lc;
        Poly::new(
            self.coeffs
                .iter()
                .map(|c| c.clone() * inv.clone())
                .collect(),
        )
    }
}

impl<F: Field + Clone + PartialEq + Zero + One> Poly<F> {
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
        let g = Poly::gcd(a, b);
        let ab = a.mul(b);
        let (q, _r) = ab.div_rem(&g);
        q.monic()
    }
}
