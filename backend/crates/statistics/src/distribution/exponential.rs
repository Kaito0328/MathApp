use rand::Rng;

use crate::distribution::core::Distribution;

pub struct Exponential {
    lambda: f64,
}

impl Exponential {
    pub fn new(lambda: f64) -> Self {
        assert!(lambda > 0.0, "Invalid lambda parameter");
        Self { lambda }
    }
}

impl Distribution for Exponential {
    fn mean(&self) -> f64 {
        1.0 / self.lambda
    }

    fn variance(&self) -> f64 {
        1.0 / (self.lambda * self.lambda)
    }

    fn mode(&self) -> Option<f64> {
        Some(0.0)
    }

    fn pdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            0.0
        } else {
            self.lambda * (-self.lambda * x).exp()
        }
    }

    fn cdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            0.0
        } else {
            1.0 - (-self.lambda * x).exp()
        }
    }

    fn quantile(&self, p: f64) -> f64 {
        assert!((0.0..=1.0).contains(&p), "Invalid quantile");
        -(1.0 - p).ln() / self.lambda
    }

    fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) -> f64 {
        let u: f64 = rng.gen_range(f64::EPSILON..=1.0);
        self.quantile(u)
    }

    fn skewness(&self) -> Option<f64> {
        Some(2.0)
    }

    fn kurtosis(&self) -> Option<f64> {
        Some(9.0)
    }

    fn log_pdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            f64::NEG_INFINITY
        } else {
            self.lambda.ln() - self.lambda * x
        }
    }
}
