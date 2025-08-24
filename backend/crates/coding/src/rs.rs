use crate::Poly;
use crate::types::{Codeword, Message};
use linalg::{Field, Matrix, Vector};

#[derive(Debug, Clone)]
pub struct RSDecodeResult<F: Field + Clone> {
    pub decoded: Message<F>,
}

// C++版の RS: k, elements(alpha^i), px はGF(256)では簡約
#[derive(Debug, Clone)]
pub struct ReedSolomon<F: Field + Clone + PartialEq> {
    pub k: usize,
    pub n: usize,
    pub t: usize,
    pub alphas: Vec<F>,
    pub g: Matrix<F>,
}

impl<F: Field + Clone + PartialEq> ReedSolomon<F> {
    pub fn new(k: usize, alphas: Vec<F>) -> Self {
        let n = alphas.len();
        let t = (n - k).div_ceil(2); // ceil((n-k)/2) の簡易版
                                     // 生成行列 G (k x n): G[i,j] = (alpha_j)^i
        let mut data = Vec::with_capacity(k * n);
        for i in 0..k {
            for a in &alphas {
                // 素朴に累乗
                let mut acc = F::one();
                for _ in 0..i {
                    acc = acc * a.clone();
                }
                data.push(acc);
            }
        }
        let g = Matrix::new(k, n, data).unwrap();
        Self { k, n, t, alphas, g }
    }

    pub fn encode(&self, f: &Message<F>) -> Codeword<F> {
        // 行ベクトル f (1 x k) と G (k x n) の積 => (1 x n)
        let v: Vector<F> = (f.as_ref().clone() * &self.g).row(0).unwrap();
        Codeword::from(v)
    }

    // 簡易デコード（Berlekamp–Welch相当は未実装）: 連立を作り最小二乗的に復元
    // まずはユーティリティ的に placeholder を返す
    pub fn decode(&self, r: &Codeword<F>) -> RSDecodeResult<F> {
        // A = [A0 | r .* A1] を作って RREF、q0, q1 を抽出し q0 / q1 を行う
        // A0: n x (n-t), A1: n x (t+1) with exp_table
        let max_pow = (self.k).max(self.n - self.t).max(self.t + 1);
        // exp_table[j][i] = (alpha_j)^i
        let mut exp = Vec::with_capacity(self.n * max_pow);
        for j in 0..self.n {
            let a = self.alphas[j].clone();
            for i in 0..max_pow {
                let mut acc = F::one();
                for _ in 0..i {
                    acc = acc * a.clone();
                }
                exp.push(acc);
            }
        }
        let exp_table = Matrix::new(self.n, max_pow, exp).unwrap();

        // A0, A1
        let a0 = exp_table.submatrix(0, self.n, 0, self.n - self.t);
        let a1 = exp_table.submatrix(0, self.n, 0, self.t + 1);

        // A = [A0 | diag(r) * A1]
        let mut data = Vec::with_capacity(self.n * (self.n + 1));
    for i in 0..self.n {
            // A0 部分
            for j in 0..(self.n - self.t) {
                data.push(a0[(i, j)].clone());
            }
            // A1 部分（スカラー倍）
            for j in 0..(self.t + 1) {
                data.push(r[i].clone() * a1[(i, j)].clone());
            }
        }
        let a = Matrix::new(self.n, self.n + 1, data).unwrap();

        // 右辺は 0 ベクトル（RREFの抽出に使うため、単に a をRREF化し最後列を見ればよい）
        let (qmat, _rhs) = a
            .rref_with(&Matrix::new(self.n, 1, vec![F::zero(); self.n]).unwrap())
            .unwrap();

        // q0: 前半(n-t)列の最下段の成分、q1: 後半(t+1)列の最下段成分から係数抽出…
        // C++の実装では Q の最右列を使って q0, q1 を拾っている。ここでは単純化して、
        // 左側(n-t)行の最右列を q0、右側t行の最右列を -q1 とみなす。
        let mut q0 = vec![F::zero(); self.n - self.t];
        let mut q1 = vec![F::zero(); self.t + 1];
        for i in 0..(self.n - self.t) {
            q0[i] = qmat[(i, self.n)].clone();
        }
        for i in 0..self.t {
            q1[i] = F::zero() - qmat[(self.n - self.t + i, self.n)].clone();
        }
        q1[self.t] = F::one();

        let q0p: Poly<F> = Poly::new(q0);
        let q1p: Poly<F> = Poly::new(q1);
        let (fpoly, _rem) = q0p.div_rem(&q1p);
    RSDecodeResult { decoded: Message::from(Vector::new(fpoly.coeffs)) }
    }
}
