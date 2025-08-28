use crate::{
    distribution::continuous::{core::Distribution, normal::Normal, utils::calc_quantile_newton},
    error::{Result, StatisticsError},
};
use special_functions::{
    erf::calc_quantile_acklam,
    gamma::{log_gamma, regularized_gamma},
};

pub struct ChiSquare {
    k: usize,
}

impl ChiSquare {
    pub fn new(k: usize) -> Result<Self> {
        if k == 0 { return Err(StatisticsError::InvalidParameter { what: "ChiSquare::k", value: k.to_string() }); }
        Ok(Self { k })
    }
}

impl Distribution for ChiSquare {
    type Item = f64;
    fn mean(&self) -> f64 {
        self.k as f64
    }

    fn variance(&self) -> f64 {
        2.0 * self.k as f64
    }

    fn mode(&self) -> Vec<Self::Item> {
        if self.k > 2 {
            vec![(self.k - 2) as f64]
        } else {
            vec![0.0]
        }
    }

    fn pdf(&self, x: Self::Item) -> f64 {
        self.log_pdf(x).exp()
    }

    fn cdf(&self, x: Self::Item) -> f64 {
        regularized_gamma(self.k as f64 / 2.0, x / 2.0)
    }

    fn quantile(&self, p: f64) -> Self::Item {
        let z = calc_quantile_acklam(p);
        let x_guess = self.k as f64
            * (1.0 - 2.0 / (9.0 * self.k as f64) + z * (2.0 / (9.0 * self.k as f64)).sqrt())
                .powi(3);
        calc_quantile_newton(x_guess, p, self)
    }

    fn log_pdf(&self, x: Self::Item) -> f64 {
        if x < 0.0 {
            f64::NEG_INFINITY
        } else {
            let half_k = self.k as f64 / 2.0;
            (half_k - 1.0) * x.ln() - (x / 2.0) - half_k * 2.0_f64.ln() - log_gamma(half_k)
        }
    }

    fn sample<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) -> Self::Item {
    let mut normal = Normal::new(0.0, 1.0).unwrap();

        (0..self.k).map(|_| normal.sample(rng).powi(2)).sum()
    }

    fn skewness(&self) -> Option<f64> {
        Some((8.0 / self.k as f64).sqrt())
    }

    fn kurtosis(&self) -> Option<f64> {
        Some(12.0 / self.k as f64)
    }
}
