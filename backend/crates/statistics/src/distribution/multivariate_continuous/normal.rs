use std::f64::consts::PI;

use linalg::{matrix::numerical::CholeskyDecomposition, Matrix, Vector};
use rand::Rng;
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::distribution::{
    continuous::{core::Distribution, normal::Normal},
    multivariate_continuous::core::MultivariateDistribution,
};

pub struct MultivariateNormal {
    mean: Vector<f64>,
    cholesky_l: Matrix<f64>, // Σの代わりにLを保存
    log_det_cov: f64,        // log(|Σ|)を保存
}

impl MultivariateNormal {
    pub fn new(mean: Vector<f64>, covariance: Matrix<f64>) -> Result<Self, String> {
        let dim = mean.len();
        if covariance.rows != dim || covariance.cols != dim {
            return Err(
                "Covariance matrix must be square and match the dimension of the mean vector."
                    .to_string(),
            );
        }

        let chol_attempt = catch_unwind(AssertUnwindSafe(|| covariance.cholesky()));
        match chol_attempt {
            Ok(Ok(l)) => {
                // 対角成分の積からlog(|Σ|)を計算
                let log_det_cov = 2.0 * (0..dim).map(|i| l[(i, i)].ln()).sum::<f64>();
                Ok(Self {
                    mean,
                    cholesky_l: l,
                    log_det_cov,
                })
            }
            Ok(Err(_)) | Err(_) => Err("Covariance matrix must be positive-definite.".to_string()),
        }
    }
}

impl MultivariateDistribution for MultivariateNormal {
    type Item = Vector<f64>;

    fn mean(&self) -> Vector<f64> {
        self.mean.clone()
    }

    fn covariance(&self) -> Matrix<f64> {
        self.cholesky_l.clone() * self.cholesky_l.clone().transpose()
    }

    fn mode(&self) -> Option<Self::Item> {
        Some(self.mean.clone())
    }

    fn pdf(&self, x: &Self::Item) -> f64 {
        self.log_pdf(x).exp()
    }

    fn log_pdf(&self, x: &Self::Item) -> f64 {
        let diff = x - &self.mean;
        let d = self.mean.len() as f64;

        // Σ⁻¹ * diff を、L*z=diff と Lᵀ*y=z の連立方程式を解くことで高速に計算
        let z = self.cholesky_l.forward_substitution(&diff).unwrap();
        let y = self
            .cholesky_l
            .transpose()
            .backward_substitution(&z)
            .unwrap();

        let quad_form = diff.dot(&y); // diffᵀ * y = diffᵀ * Σ⁻¹ * diff

        -0.5 * (d * (2.0 * PI).ln() + self.log_det_cov + quad_form)
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::Item {
        let mut normal = Normal::new(0.0, 1.0).unwrap(); // .unwrap()は不要に
        let zs = Vector::new(normal.sample_n(self.mean.len(), rng));

        // 保存しておいたLを直接使う
        &self.mean + &(&self.cholesky_l * &zs)
    }

    fn sample_n<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Vec<Self::Item> {
        // Normalインスタンスを一度だけ生成
        let mut std_normal = Normal::new(0.0, 1.0).unwrap();
        let dim = self.mean.len();

        (0..n)
            .map(|_| {
                // sample_nを直接呼び出すのではなく、各要素を生成する
                let zs = Vector::new(Vec::from_iter((0..dim).map(|_| std_normal.sample(rng))));
                &self.mean + &(&self.cholesky_l * &zs)
            })
            .collect()
    }
}
