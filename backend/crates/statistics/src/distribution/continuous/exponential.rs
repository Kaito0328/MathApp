use rand::Rng;

use crate::{
    distribution::continuous::core::Distribution,
    error::{Result, StatisticsError},
};

pub struct Exponential {
    lambda: f64,
}

impl Exponential {
    pub fn new(lambda: f64) -> Result<Self> {
        if !(lambda.is_finite()) || lambda <= 0.0 {
            return Err(StatisticsError::InvalidParameter {
                what: "Exponential::lambda",
                value: lambda.to_string(),
            });
        }
        Ok(Self { lambda })
    }
}

impl Distribution for Exponential {
    type Item = f64;
    fn mean(&self) -> f64 {
        1.0 / self.lambda
    }

    fn variance(&self) -> f64 {
        1.0 / (self.lambda * self.lambda)
    }

    fn mode(&self) -> Vec<Self::Item> {
        vec![0.0]
    }

    fn pdf(&self, x: Self::Item) -> f64 {
        if x < 0.0 {
            0.0
        } else {
            self.lambda * (-self.lambda * x).exp()
        }
    }

    fn cdf(&self, x: Self::Item) -> f64 {
        if x < 0.0 {
            0.0
        } else {
            1.0 - (-self.lambda * x).exp()
        }
    }

    fn quantile(&self, p: f64) -> Self::Item {
        if !(0.0..=1.0).contains(&p) || p.is_nan() {
            return f64::NAN;
        }
        -(1.0 - p).ln() / self.lambda
    }

    fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Self::Item {
        let u: f64 = rng.gen_range(f64::EPSILON..=1.0);
        self.quantile(u)
    }

    fn skewness(&self) -> Option<f64> {
        Some(2.0)
    }

    fn kurtosis(&self) -> Option<f64> {
        Some(9.0)
    }

    fn log_pdf(&self, x: Self::Item) -> f64 {
        if x < 0.0 {
            f64::NEG_INFINITY
        } else {
            self.lambda.ln() - self.lambda * x
        }
    }
}
