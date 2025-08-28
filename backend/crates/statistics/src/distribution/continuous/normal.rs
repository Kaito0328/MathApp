use std::f64::consts::PI;

use rand::Rng;

use crate::{distribution::continuous::core::Distribution, error::{Result, StatisticsError}};
use special_functions::erf::{calc_quantile_acklam, erf};

pub struct Normal {
    mu: f64,
    sigma: f64,
    sample_cache: Option<f64>,
}

impl Normal {
    pub fn new(mu: f64, sigma: f64) -> Result<Self> {
        if !(mu.is_finite() && sigma.is_finite()) || sigma <= 0.0 {
            return Err(StatisticsError::InvalidParameter { what: "Normal::sigma", value: sigma.to_string() });
        }
        Ok(Self { mu, sigma, sample_cache: None })
    }
}

impl Distribution for Normal {
    type Item = f64;
    fn mean(&self) -> f64 {
        self.mu
    }

    fn variance(&self) -> f64 {
        self.sigma * self.sigma
    }

    fn mode(&self) -> Vec<Self::Item> {
        vec![self.mu]
    }

    fn pdf(&self, x: Self::Item) -> f64 {
        let coeff = 1.0 / (self.sigma * (2.0 * PI).sqrt());
        let exponent = -0.5 * ((x - self.mu) * (x - self.mu) / (self.sigma * self.sigma));
        coeff * exponent.exp()
    }

    fn cdf(&self, x: Self::Item) -> f64 {
        let z = (x - self.mu) / self.sigma;
        0.5 * (1.0 + erf(z / 2.0_f64.sqrt()))
    }

    fn quantile(&self, p: f64) -> Self::Item {
        calc_quantile_acklam(p) * self.sigma + self.mu
    }

    fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Self::Item {
        if let Some(z1) = self.sample_cache.take() {
            return z1 * self.sigma + self.mu;
        }

        let (z0, z1) = loop {
            // ✨ 修正点1: 正しい範囲 [-1, 1) から乱数を生成
            let v1: f64 = rng.gen_range(-1.0..1.0);
            let v2: f64 = rng.gen_range(-1.0..1.0);

            let s = v1 * v1 + v2 * v2;

            // ✨ 修正点4: loopを使う
            if s < 1.0 && s != 0.0 {
                // 棄却されなかった場合、z0を計算してループを抜ける
                let multiplier = (-2.0 * s.ln() / s).sqrt();
                break (v1 * multiplier, v2 * multiplier);
            }
        };

        self.sample_cache = Some(z1);

        // 標準正規乱数z0を、目的の分布に合わせて変換
        z0 * self.sigma + self.mu
    }

    fn log_pdf(&self, x: Self::Item) -> f64 {
        let coeff = -0.5 * ((2.0 * PI * self.sigma * self.sigma).ln());
        let exponent = -0.5 * ((x - self.mu) * (x - self.mu) / (self.sigma * self.sigma));
        coeff + exponent
    }

    fn skewness(&self) -> Option<f64> {
        Some(0.0)
    }

    fn kurtosis(&self) -> Option<f64> {
        Some(3.0)
    }
}
