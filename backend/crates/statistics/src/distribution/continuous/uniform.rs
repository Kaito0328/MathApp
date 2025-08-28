use rand::Rng;

use crate::{
    distribution::continuous::core::Distribution,
    error::{Result, StatisticsError},
};
pub struct Uniform {
    min: f64,
    max: f64,
}

impl Uniform {
    pub fn new(min: f64, max: f64) -> Result<Self> {
        if !(min.is_finite() && max.is_finite() && (min < max)) {
            return Err(StatisticsError::InvalidParameter {
                what: "Uniform::(min,max)",
                value: format!("({min},{max})"),
            });
        }
        Ok(Self { min, max })
    }
}

impl Distribution for Uniform {
    type Item = f64;
    fn mean(&self) -> f64 {
        (self.min + self.max) / 2.0
    }

    fn variance(&self) -> f64 {
        let range = self.max - self.min;
        range * range / 12.0
    }

    fn mode(&self) -> Vec<f64> {
        vec![]
    }

    fn pdf(&self, x: Self::Item) -> f64 {
        if x < self.min || x > self.max {
            0.0
        } else {
            1.0 / (self.max - self.min)
        }
    }

    fn cdf(&self, x: Self::Item) -> f64 {
        if x < self.min {
            0.0
        } else if x > self.max {
            1.0
        } else {
            (x - self.min) / (self.max - self.min)
        }
    }

    fn quantile(&self, p: f64) -> Self::Item {
        self.min + p * (self.max - self.min)
    }

    fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Self::Item {
        rng.gen_range(0.0..1.0) * (self.max - self.min) + self.min
    }

    fn log_pdf(&self, x: Self::Item) -> f64 {
        self.pdf(x).ln()
    }

    fn skewness(&self) -> Option<f64> {
        Some(0.0)
    }

    fn kurtosis(&self) -> Option<f64> {
        Some(1.8)
    }
}
