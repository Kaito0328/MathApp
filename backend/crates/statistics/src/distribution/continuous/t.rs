use std::f64::consts::PI;

use crate::{
    distribution::continuous::{chi_square::ChiSquare, core::Distribution, normal::Normal},
    error::{Result, StatisticsError},
};

use special_functions::{beta::regularized_beta, erf::calc_quantile_acklam, gamma::log_gamma};

pub struct T {
    nu: usize,
}

impl T {
    pub fn new(nu: usize) -> Result<Self> {
        if nu == 0 { return Err(StatisticsError::InvalidParameter { what: "T::nu", value: nu.to_string() }); }
        Ok(T { nu })
    }
}

impl Distribution for T {
    type Item = f64;
    fn mean(&self) -> f64 {
        if self.nu > 1 {
            0.0
        } else {
            f64::NAN
        }
    }

    fn variance(&self) -> f64 {
        if self.nu > 2 {
            self.nu as f64 / (self.nu - 2) as f64
        } else if self.nu == 2 {
            f64::INFINITY
        } else {
            f64::NAN
        }
    }

    fn mode(&self) -> Vec<f64> {
        vec![0.0]
    }

    fn pdf(&self, x: Self::Item) -> f64 {
        self.log_pdf(x).exp()
    }

    fn log_pdf(&self, x: Self::Item) -> f64 {
        let half_nu = self.nu as f64 / 2.0;
        let half_nu_plus_one = half_nu + 0.5;

        log_gamma(half_nu_plus_one)
            - log_gamma(half_nu)
            - 0.5 * (self.nu as f64 * PI).ln()
            - half_nu_plus_one * (1.0 + x * x / self.nu as f64).ln()
    }

    fn sample<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) -> Self::Item {
    let mut normal = Normal::new(0.0, 1.0).unwrap();
    let mut chi_square = ChiSquare::new(self.nu).unwrap();
        let z = normal.sample(rng);
        let v = chi_square.sample(rng);
        z / (v / self.nu as f64).sqrt()
    }

    fn skewness(&self) -> Option<f64> {
        if self.nu > 3 {
            Some(0.0)
        } else {
            None
        }
    }

    fn kurtosis(&self) -> Option<f64> {
        if self.nu > 4 {
            Some(6.0 / (self.nu - 4) as f64)
        } else {
            None
        }
    }

    fn cdf(&self, x: Self::Item) -> f64 {
        // Use relation with regularized incomplete beta.
        // For t >= 0: F(t) = 1 - 0.5 * I_{nu/(nu + t^2)}(nu/2, 1/2)
        // For t < 0:  F(t) = 0.5 * I_{nu/(nu + t^2)}(nu/2, 1/2)
        let nu = self.nu;
        let x2 = x * x;
        let a = nu as f64 / 2.0;
        let b = 0.5;
        if x >= 0.0 {
            let ib = regularized_beta(a, b, nu as f64 / (x2 + nu as f64));
            1.0 - 0.5 * ib
        } else {
            0.5 * regularized_beta(a, b, x2 / (x2 + nu as f64))
        }
    }

    fn quantile(&self, p: f64) -> Self::Item {
        if !(0.0..=1.0).contains(&p) || p.is_nan() {
            return f64::NAN;
        }

        if p > 0.5 {
            return -self.quantile(1.0 - p);
        }

        let z = calc_quantile_acklam(p);
        let z2 = z * z;
        let nu = self.nu as f64;

        let g1 = 0.25 * z * (z2 + 1.0);
        let g2 = (z * (3.0 + z2 * (16.0 + 5.0 * z2))) / 96.0;
        let g3 = (z * (-15.0 + z2 * (17.0 + z2 * (19.0 + z2 * 3.0)))) / 384.0;

        z + (g1 + (g2 + g3 / nu) / nu) / nu
    }
}
