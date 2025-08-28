use std::f64::consts::E;

use crate::distribution::continuous::{
    core::Distribution, normal::Normal, utils::calc_quantile_newton,
};

use special_functions::{
    erf::calc_quantile_acklam,
    gamma::{log_gamma, regularized_gamma},
};
use crate::error::{Result, StatisticsError};

pub struct Gamma {
    shape: f64,
    rate: f64,
}

impl Gamma {
    pub fn new(shape: f64, rate: f64) -> Result<Self> {
        if !(shape.is_finite() && rate.is_finite()) || shape <= 0.0 || rate <= 0.0 {
            return Err(StatisticsError::InvalidParameter { what: "Gamma::(shape,rate)", value: format!("({shape},{rate})") });
        }
        Ok(Gamma { shape, rate })
    }
}

impl Distribution for Gamma {
    type Item = f64;
    fn mean(&self) -> f64 {
        self.shape / self.rate
    }

    fn variance(&self) -> f64 {
        self.shape / (self.rate * self.rate)
    }

    fn mode(&self) -> Vec<Self::Item> {
        if self.shape > 1.0 {
            vec![(self.shape - 1.0) / self.rate]
        } else {
            vec![0.0]
        }
    }

    fn pdf(&self, x: Self::Item) -> f64 {
        self.log_pdf(x).exp()
    }

    fn log_pdf(&self, x: Self::Item) -> f64 {
        if x < 0.0 {
            return f64::NEG_INFINITY;
        }
        self.shape * self.rate.ln() - log_gamma(self.shape) + (self.shape - 1.0) * x.ln()
            - self.rate * x
    }

    fn cdf(&self, x: Self::Item) -> f64 {
        regularized_gamma(self.shape, self.rate * x)
    }

    fn quantile(&self, p: f64) -> Self::Item {
        let z = calc_quantile_acklam(p);
        let x_guess = self.shape / self.rate
            * (1.0 - 1.0 / (9.0 * self.shape) + z * (1.0 / (9.0 * self.shape)).sqrt()).powi(3);

        calc_quantile_newton(x_guess, p, self)
    }

    fn sample<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) -> Self::Item {
        if self.shape > 1.0 {
            let one_third = 1.0 / 3.0;
            let d = self.shape - one_third;
            let c = one_third / d.sqrt();
            let mut normal = Normal::new(0.0, 1.0).unwrap();

            let v = loop {
                let z = normal.sample(rng);
                let v = (1.0 + c * z).powi(3);

                if v < 0.0 {
                    continue;
                }

                let u: f64 = rng.gen();

                if u.ln() < 0.5 * z * z + d - d * v + d * v.ln() {
                    break v;
                }
            };

            d * v / self.rate
        } else {
            let b = 1.0 + self.shape / E;
            let x = loop {
                let u1: f64 = rng.gen();
                let p = b * u1;

                if p <= 1.0 {
                    let x = p.powf(1.0 / self.shape);
                    let u2: f64 = rng.gen();
                    if u2 <= (-x).exp() {
                        break x;
                    }
                } else {
                    let x = -(p - 1.0).ln();
                    let u2: f64 = rng.gen();

                    if u2 <= x.powf(self.shape - 1.0) {
                        break x;
                    }
                }
            };

            x / self.rate
        }
    }

    fn skewness(&self) -> Option<f64> {
        Some(2.0 / self.shape.sqrt())
    }

    fn kurtosis(&self) -> Option<f64> {
        Some(6.0 / self.shape)
    }
}
