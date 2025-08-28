use std::f64::consts::PI;

use linalg::{matrix::numerical::CholeskyDecomposition, Matrix, Vector};
use special_functions::gamma::log_gamma;
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::distribution::{
    continuous::{core::Distribution, gamma::Gamma, normal::Normal},
    multivariate_continuous::core::MultivariateDistribution,
};

pub struct MultivariateT {
    nu: f64,
    mu: Vector<f64>,
    cholesky_l: Matrix<f64>, // Σの代わりにLを保存
    log_det_cov: f64,        // log(|Σ|)を保存
}

impl MultivariateT {
    pub fn new(nu: f64, mu: Vector<f64>, sigma: Matrix<f64>) -> Result<Self, String> {
        let dim = mu.len();
        if sigma.rows != dim || sigma.cols != dim {
            return Err(
                "Covariance matrix must be square and match the dimension of the mean vector."
                    .to_string(),
            );
        }
        if nu <= 0.0 {
            return Err("Degrees of freedom must be positive.".to_string());
        }
        let chol_attempt = catch_unwind(AssertUnwindSafe(|| sigma.cholesky()));
        match chol_attempt {
            Ok(Ok(l)) => {
                // 対角成分の積からlog(|Σ|)を計算
                let log_det_cov = 2.0 * (0..dim).map(|i| l[(i, i)].ln()).sum::<f64>();
                Ok(Self {
                    nu,
                    mu,
                    cholesky_l: l,
                    log_det_cov,
                })
            }
            Ok(Err(_)) | Err(_) => Err("Covariance matrix must be positive-definite.".to_string()),
        }
    }
}

impl MultivariateDistribution for MultivariateT {
    type Item = Vector<f64>;

    fn mean(&self) -> Vector<f64> {
        self.mu.clone()
    }

    fn covariance(&self) -> Matrix<f64> {
        if self.nu <= 2.0 {
            return Matrix::zeros(self.mu.len(), self.mu.len());
        }
        self.cholesky_l.clone() * self.cholesky_l.clone().transpose() * (self.nu / (self.nu - 2.0))
    }

    fn mode(&self) -> Option<Self::Item> {
        Some(self.mu.clone())
    }

    fn pdf(&self, x: &Self::Item) -> f64 {
        self.log_pdf(x).exp()
    }

    fn log_pdf(&self, x: &Self::Item) -> f64 {
        let nu = self.nu;
        let d = self.mu.dim() as f64;
        if x.len() != d as usize {
            return f64::NAN;
        }

        let coeff = log_gamma((nu + d) / 2.0)
            - log_gamma(nu / 2.0)
            - 0.5 * d * (nu * PI).ln()
            - 0.5 * self.log_det_cov;

        let diff = x - self.mu.clone();

        // Σ⁻¹ * diff を、L*z=diff と Lᵀ*y=z の連立方程式を解くことで高速に計算
        let z = self.cholesky_l.forward_substitution(&diff).unwrap();
        let y = self
            .cholesky_l
            .transpose()
            .backward_substitution(&z)
            .unwrap();

        coeff - (nu + d) / 2.0 * (1.0 + diff.dot(&y) / nu).ln()
    }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::Item {
        let mut gamma_dist = Gamma::new(self.nu / 2.0, 0.5).unwrap();
        let u = gamma_dist.sample(rng);

        let dim = self.mu.len();
        let mut std_normal = Normal::new(0.0, 1.0).unwrap();
        let zs = Vector::new(Vec::from_iter((0..dim).map(|_| std_normal.sample(rng))));

        // 3. y = L*z を計算
        let y = &self.cholesky_l * &zs;

        self.mu.clone() + y * (1.0 / (u / self.nu).sqrt())
    }
}
