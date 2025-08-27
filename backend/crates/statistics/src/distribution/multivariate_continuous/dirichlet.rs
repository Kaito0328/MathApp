use linalg::{Matrix, Vector};
use special_functions::gamma::log_gamma;

use crate::distribution::{
    continuous::{core::Distribution, gamma::Gamma},
    multivariate_continuous::core::MultivariateDistribution,
};
use crate::error::{Result, StatisticsError};

pub struct Dirichlet {
    alpha: Vector<f64>,
}

impl Dirichlet {
    pub fn new(alpha: Vector<f64>) -> Result<Self> {
        if alpha.iter().any(|&a| !a.is_finite() || a <= 0.0) {
            return Err(StatisticsError::InvalidParameter {
                what: "Dirichlet::alpha",
                value: format!("{:?}", alpha.as_slice()),
            });
        }
        if alpha.len() < 2 {
            return Err(StatisticsError::InvalidParameter {
                what: "Dirichlet::k",
                value: alpha.len().to_string(),
            });
        }
        Ok(Self { alpha })
    }
}

impl MultivariateDistribution for Dirichlet {
    type Item = Vector<f64>;
    fn mean(&self) -> Vector<f64> {
        let sum: f64 = self.alpha.iter().sum();
        self.alpha.clone() * (1.0 / sum)
    }

    fn covariance(&self) -> linalg::Matrix<f64> {
        let k = self.alpha.len();
        let mut cov = Vec::with_capacity(k * k);
        let sum = self.alpha.iter().sum::<f64>();
        for i in 0..k {
            for j in 0..k {
                if i == j {
                    cov.push(self.alpha[i] * (sum - self.alpha[i]) / (sum * sum * (sum + 1.0)));
                } else {
                    cov.push(-self.alpha[i] * self.alpha[j] / (sum * sum * (sum + 1.0)));
                }
            }
        }
        Matrix::new(k, k, cov).unwrap()
    }

    fn mode(&self) -> Option<Self::Item> {
        if self.alpha.iter().all(|&a| a > 1.0) {
            let sum: f64 = self.alpha.iter().sum();
            let mode = self.alpha.clone() - 1.0;
            Some(mode * (1.0 / (sum - self.alpha.len() as f64)))
        } else {
            None
        }
    }

    fn pdf(&self, x: &Self::Item) -> f64 {
        self.log_pdf(x).exp()
    }

    fn log_pdf(&self, x: &Self::Item) -> f64 {
        assert_eq!(x.len(), self.alpha.len(), "Dimension mismatch");
        assert!(
            x.iter().all(|&xi| (0.0..=1.0).contains(&xi)),
            "Elements of x must be in [0,1]"
        );
        assert!(
            (x.iter().sum::<f64>() - 1.0).abs() < 1e-9,
            "Elements of x must sum to 1"
        );

        let sum: f64 = self.alpha.iter().sum();
        let log_beta_alpha = self.alpha.iter().map(|&a| log_gamma(a)).sum::<f64>() - log_gamma(sum);

        // Σ (αᵢ-1)ln(xᵢ) の部分
        let sum_log_x = self
            .alpha
            .iter()
            .zip(x.iter())
            .map(|(&a, &xi)| (a - 1.0) * xi.ln())
            .sum::<f64>();

        // ✨ log_beta_alpha を引くのが正しい
        sum_log_x - log_beta_alpha
    }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::Item {
        let k = self.alpha.len();
        let mut gammas = Vec::with_capacity(k);

        for i in 0..k {
            let mut gamma_dist = Gamma::new(self.alpha[i], 1.0).unwrap();
            gammas.push(gamma_dist.sample(rng));
        }

        let sum: f64 = gammas.iter().sum();
        gammas.iter_mut().for_each(|g| *g /= sum);
        Vector::new(gammas)
    }
}
