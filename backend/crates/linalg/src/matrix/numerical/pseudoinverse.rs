use crate::{matrix::numerical::SvdDeComposition, Matrix, Result};

pub trait Pseudoinverse {
    fn pinv(&self) -> Result<Matrix<f64>>;
}

impl Pseudoinverse for Matrix<f64> {
    fn pinv(&self) -> Result<Matrix<f64>> {
        let svd = self.svd()?;
        let sigma = svd.sigma;
        let u = svd.u;
        let v = svd.v;
        let mut sigma_pinv = Matrix::zeros(v.cols, u.cols);

        const THRESHOLD: f64 = 1e-10;

        for (i, s) in sigma.iter().enumerate() {
            if *s > THRESHOLD {
                sigma_pinv[(i, i)] = 1.0 / *s;
            }
        }
        Ok(v * sigma_pinv * u.transpose())
    }
}
