use crate::{distribution::discrete::core::Distribution, error::{Result, StatisticsError}};

pub struct Bernoulli {
    p: f64,
}

impl Bernoulli {
    pub fn new(p: f64) -> Result<Self> {
        if !(0.0..=1.0).contains(&p) || p.is_nan() { return Err(StatisticsError::InvalidParameter { what: "Bernoulli::p", value: p.to_string() }); }
        Ok(Self { p })
    }
}

impl Distribution for Bernoulli {
    type Item = u64;
    fn mean(&self) -> f64 {
        self.p
    }

    fn variance(&self) -> f64 {
        self.p * (1.0 - self.p)
    }

    fn mode(&self) -> Vec<Self::Item> {
        if self.p < 0.5 {
            vec![0]
        } else if self.p > 0.5 {
            vec![1]
        } else {
            vec![0, 1]
        }
    }

    fn pmf(&self, k: Self::Item) -> f64 {
        match k {
            0 => 1.0 - self.p,
            1 => self.p,
            _ => 0.0,
        }
    }

    fn log_pmf(&self, k: Self::Item) -> f64 {
        self.pmf(k).ln()
    }

    fn cdf(&self, k: Self::Item) -> f64 {
        match k {
            0 => 1.0 - self.p,
            _ => 1.0,
        }
    }

    fn quantile(&self, p: f64) -> Self::Item {
        if p <= 1.0 - self.p {
            0
        } else {
            1
        }
    }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::Item {
        let u: f64 = rng.gen();

        match u {
            x if x < self.p => 1,
            _ => 0,
        }
    }

    fn skewness(&self) -> Option<f64> {
        let var = self.p * (1.0 - self.p);
        if var == 0.0 {
            return None;
        } // ゼロ除算を避ける
        Some((1.0 - 2.0 * self.p) / var.sqrt())
    }

    fn kurtosis(&self) -> Option<f64> {
        let var = self.p * (1.0 - self.p);
        if var == 0.0 {
            return None;
        } // ゼロ除算を避ける
        Some((1.0 - 6.0 * var) / var)
    }
}
