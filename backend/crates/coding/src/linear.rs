use linalg::{Field, Matrix, Vector};

#[derive(Debug, Clone)]
pub struct LinearCode<F: Field + Clone + PartialEq> {
    pub n: usize,
    pub k: usize,
    pub g: Matrix<F>, // 生成行列 (k x n)
}

impl<F: Field + Clone + PartialEq> LinearCode<F> {
    pub fn new(g: Matrix<F>) -> Self {
        let k = g.rows;
        let n = g.cols;
        Self { n, k, g }
    }

    pub fn encode(&self, u: &Vector<F>) -> Vector<F> {
        // u: 1 x k
        (u.clone() * &self.g).row(0).unwrap()
    }
}
