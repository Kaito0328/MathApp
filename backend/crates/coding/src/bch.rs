use crate::error::{CodingError, Result as CodingResult};
use crate::types::{Codeword, Message};
use crate::Poly;
use linalg::{Field, Vector};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use std::fmt;

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
    pub fn encode(&self, u: &Message<F>) -> CodingResult<Codeword<F>> {
        let k = self.k();
        if u.dim() != k {
            return Err(CodingError::InvalidParameters {
                text: format!("message length {} must be k {}", u.dim(), k),
            });
        }
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
        Ok(Codeword::from(Vector::new(c)))
    }

    pub fn k(&self) -> usize {
        self.n - (self.g.coeffs.len() - 1)
    }
}

impl<F> fmt::Display for BCHCode<F>
where
    F: Field + Clone + PartialEq + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 試しに f64 専用の Display は polynomial 側にあるが、ここは一般 F を前提に簡易表記
        // g(x) の表示は係数から直接組み立て
        let mut terms: Vec<String> = Vec::new();
        for (i, c) in self.g.coeffs.iter().enumerate() {
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
        write!(
            f,
            "BCH(n={}, k={}, t={}, g(x)={})",
            self.n,
            self.k(),
            self.t,
            gstr
        )
    }
}
