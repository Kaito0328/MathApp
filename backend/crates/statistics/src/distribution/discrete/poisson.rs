use special_functions::gamma::{log_gamma, regularized_gamma};

use crate::distribution::{
    continuous::{core::Distribution as ConcreteDistribution, normal::Normal},
    discrete::{core::Distribution, utils::find_quantile_bs},
};
use crate::error::{Result, StatisticsError};

pub struct Poisson {
    lambda: f64,
}

impl Poisson {
    pub fn new(lambda: f64) -> Result<Self> {
        if !(lambda.is_finite()) || lambda <= 0.0 { return Err(StatisticsError::InvalidParameter { what: "Poisson::lambda", value: lambda.to_string() }); }
        Ok(Self { lambda })
    }
}

impl Distribution for Poisson {
    type Item = u64;
    fn mean(&self) -> f64 {
        self.lambda
    }

    fn variance(&self) -> f64 {
        self.lambda
    }

    fn mode(&self) -> Vec<Self::Item> {
        let lambda = self.lambda;
        // λが0以外の整数で、かつ小数点以下がほぼ0の場合
        if lambda > 0.0 && (lambda - lambda.floor()).abs() < 1e-9 {
            vec![(lambda - 1.0) as Self::Item, lambda as Self::Item]
        } else {
            vec![lambda.floor() as Self::Item]
        }
    }

    fn pmf(&self, k: Self::Item) -> f64 {
        self.log_pmf(k).exp()
    }

    fn log_pmf(&self, k: Self::Item) -> f64 {
        if k == 0 {
            -self.lambda
        } else {
            k as f64 * self.lambda.ln() - self.lambda - log_gamma(k as f64 + 1.0)
        }
    }

    fn cdf(&self, k: Self::Item) -> f64 {
        1.0 - regularized_gamma((k + 1) as f64, self.lambda)
    }

    fn quantile(&self, p: f64) -> Self::Item {
        if !(0.0..=1.0).contains(&p) {
            return 0;
        } // or panic
        if p == 0.0 {
            return 0;
        }
        if p == 1.0 {
            return Self::Item::MAX;
        }

        let mut high = (self.lambda + 10.0 * self.lambda.sqrt()).ceil() as u64;

        while self.cdf(high) < p {
            high *= 2;
        }

        find_quantile_bs(p, self, high / 2, high)
    }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::Item {
        const THRESHOLD: f64 = 30.0;

        if self.lambda < THRESHOLD {
            let l = (-self.lambda).exp();
            let mut k = 0;
            let mut p = 1.0;
            while p > l {
                let u = rng.gen::<f64>();
                p *= u;
                k += 1;
            }
            k - 1
        } else {
            let sqrt_lambda = self.lambda.sqrt();
            let b = 0.931 + 2.53 * sqrt_lambda;
            let vr = 0.9277 - 3.6224 / b;
            let vst = 1.0 / sqrt_lambda;

            let normal = Normal::new(self.lambda, sqrt_lambda).unwrap();

            loop {
                let u1: f64 = rng.gen();
                let v = (u1 - 0.5) / 0.456;
                let k_float = (self.lambda + v * sqrt_lambda).floor();

                if k_float < 0.0 {
                    continue;
                }

                let k = k_float as Self::Item;

                let u2: f64 = rng.gen();

                if u2 < 0.9277 - 3.6224 / (b - v * vst) {
                    return k;
                }

                if u2 > vr - v * vst {
                    continue;
                }

                if u2.ln() < self.log_pmf(k) - normal.log_pdf(k as f64) {
                    return k;
                }
            }
        }
    }

    fn skewness(&self) -> Option<f64> {
        Some(1.0 / self.lambda.sqrt())
    }

    fn kurtosis(&self) -> Option<f64> {
        Some(1.0 / self.lambda)
    }
}
