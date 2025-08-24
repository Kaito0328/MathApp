use linalg::{Field, Vector};
use crate::types::{Message, Codeword};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

// 循環符号（生成多項式 g(x) に基づく、長さ n）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyclicCode<F: Field + Clone + PartialEq> {
    pub n: usize,
    pub g: Vec<F>, // 係数低次→高次、g(0) != 0 を想定
}

impl<F: Field + Clone + PartialEq + Zero + One> CyclicCode<F> {
    pub fn new(n: usize, g: Vec<F>) -> Self {
        Self { n, g }
    }

    // 新API: Message -> Codeword
    pub fn encode(&self, u: &Message<F>) -> Codeword<F> {
        let k = self.k();
        assert_eq!(u.dim(), k, "message length must be k");
        // 系統形 [I | parity] を構築する代わりに畳み込みして mod x^n-1
        let mut v = vec![F::zero(); k + self.g.len() - 1];
        for i in 0..k {
            for j in 0..self.g.len() {
                v[i + j] = v[i + j].clone() + u[i].clone() * self.g[j].clone();
            }
        }
        // x^n ≡ 1 で縮約
        let mut c = vec![F::zero(); self.n];
        for (i, coef) in v.into_iter().enumerate() {
            c[i % self.n] = c[i % self.n].clone() + coef;
        }
        Codeword::from(Vector::new(c))
    }

    pub fn k(&self) -> usize {
        self.n - (self.g.len() - 1)
    }

    // 旧API互換（将来削除予定）
    pub fn encode_poly(&self, u: &[F]) -> Vec<F> {
        let msg = Message::from(Vector::new(u.to_vec()));
        self.encode(&msg).0.data
    }
}
