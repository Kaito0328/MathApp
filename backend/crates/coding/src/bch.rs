use crate::Poly;
use crate::types::{Message, Codeword};
use linalg::{Field, Vector};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

// 最小限の BCH( n, k ) 構築: 基底体 F 上の拡大体 GF(q^m) の原始 n 次元の α を用い
// t 個の連続べき最小多項式の least common multiple を g(x) とし、CyclicCode 相当で符号化
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BCHCode<F: Field + Clone + PartialEq> {
    pub n: usize,
    pub t: usize,
    pub g: Poly<F>,
}

impl<F: Field + Clone + PartialEq + Zero + One> BCHCode<F> {
    pub fn new_from_minimal_polynomials(n: usize, polys: &[Poly<F>]) -> Self {
        let mut g = Poly::one();
        for p in polys {
            g = Poly::lcm(&g, p);
        }
        Self {
            n,
            t: (polys.len()) / 2,
            g,
        }
    }

    pub fn g(&self) -> &Poly<F> {
        &self.g
    }

    // 新API: Message -> Codeword（簡易: x^n≡1で縮約）
    pub fn encode(&self, u: &Message<F>) -> Codeword<F> {
        let k = self.k();
        assert_eq!(u.dim(), k);
        let mut v = vec![F::zero(); k + self.g.coeffs.len() - 1];
        for i in 0..k {
            for j in 0..self.g.coeffs.len() {
                v[i + j] = v[i + j].clone() + u[i].clone() * self.g.coeffs[j].clone();
            }
        }
        let mut c = vec![F::zero(); self.n];
        for (i, coef) in v.into_iter().enumerate() {
            c[i % self.n] = c[i % self.n].clone() + coef;
        }
        Codeword::from(Vector::new(c))
    }

    pub fn k(&self) -> usize {
        self.n - (self.g.coeffs.len() - 1)
    }
}
