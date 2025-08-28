pub mod cholesky;
pub mod eigen;
pub mod exp;
pub mod pseudoinverse;
pub mod qr;
pub mod svd;

mod helpers;

// --- トレイトを短いパスで使えるように再エクスポートする ---
// これを書いておくと `use crate::matrix::numerical::Svd;` のように書ける
pub use cholesky::CholeskyDecomposition;
pub use eigen::EigenDecomposition;
pub use exp::MatrixExponential;
pub use pseudoinverse::Pseudoinverse;
pub use qr::QrDecomposition;
pub use svd::SvdDeComposition;

use crate::Matrix;
use num_complex::Complex;

impl Matrix<f64> {
    /// Frobenius ノルム ||A||_F = sqrt(sum_{i,j} a_{ij}^2)
    pub fn frobenius_norm(&self) -> f64 {
        let mut s = 0.0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                let v = self[(i, j)];
                s += v * v;
            }
        }
        s.sqrt()
    }
}

impl Matrix<Complex<f64>> {
    /// Frobenius ノルム（複素）: sqrt(sum |a_ij|^2)
    pub fn frobenius_norm(&self) -> f64 {
        let mut s = 0.0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                let v = self[(i, j)];
                s += v.norm_sqr();
            }
        }
        s.sqrt()
    }
}
