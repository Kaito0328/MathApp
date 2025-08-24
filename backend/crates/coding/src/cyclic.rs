use crate::error::{CodingError, Result as CodingResult};
use crate::types::{Codeword, Message};
use linalg::{Field, Vector};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use std::fmt;

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
    pub fn encode(&self, u: &Message<F>) -> CodingResult<Codeword<F>> {
        let k = self.k();
        if u.dim() != k {
            return Err(CodingError::InvalidParameters {
                text: format!("message length {} must be k {}", u.dim(), k),
            });
        }
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
        Ok(Codeword::from(Vector::new(c)))
    }

    pub fn k(&self) -> usize {
        self.n - (self.g.len() - 1)
    }

    // 旧API互換（将来削除予定）
    pub fn encode_poly(&self, u: &[F]) -> Vec<F> {
        let msg = Message::from(Vector::new(u.to_vec()));
        // keep legacy API panicking behavior by unwrapping internally
        match self.encode(&msg) {
            Ok(c) => c.0.data,
            Err(e) => panic!("valid message length: {e}"),
        }
    }
}

impl<F> fmt::Display for CyclicCode<F>
where
    F: Field + Clone + PartialEq + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // g(x) の簡易表示: 低次→高次の係数を人が読む形に
        // 係数が 0 の項は省略。係数 1 は x^k のみ（k>=1）。
        let mut terms: Vec<String> = Vec::new();
        for (i, c) in self.g.iter().enumerate() {
            if c.is_zero() {
                continue;
            }
            let term = if i == 0 {
                format!("{c}")
            } else if *c == F::one() {
                if i == 1 {
                    "x".to_string()
                } else {
                    format!("x^{i}")
                }
            } else if i == 1 {
                format!("{c}x")
            } else {
                format!("{c}x^{i}")
            };
            terms.push(term);
        }
        let gstr = if terms.is_empty() {
            "0".to_string()
        } else {
            terms.join(" + ")
        };
        write!(f, "CyclicCode(n={}, k={}, g(x)={})", self.n, self.k(), gstr)
    }
}
