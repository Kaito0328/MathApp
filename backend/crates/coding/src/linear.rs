use crate::types::{Codeword, GeneratorMatrix, Message};
use linalg::{Field, Matrix, Vector};

#[derive(Debug, Clone)]
pub struct LinearCode<F: Field + Clone + PartialEq> {
    pub n: usize,
    pub k: usize,
    pub g: GeneratorMatrix<F>, // 生成行列 (k x n)
}

impl<F: Field + Clone + PartialEq> LinearCode<F> {
    pub fn new(g: Matrix<F>) -> Self {
        let k = g.rows;
        let n = g.cols;
        Self { n, k, g: GeneratorMatrix(g) }
    }

    pub fn encode(&self, u: &Message<F>) -> Codeword<F> {
        // u: 1 x k
        let v: Vector<F> = u.as_ref().clone();
        let g = &self.g.0;
        ((v * g).row(0).unwrap()).into()
    }
}
