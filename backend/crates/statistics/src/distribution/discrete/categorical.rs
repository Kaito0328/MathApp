use crate::{
    distribution::discrete::core::Distribution,
    error::{Result, StatisticsError},
};

pub struct Categorical {
    probs: Vec<f64>,
}

impl Categorical {
    pub fn new(mut probs: Vec<f64>) -> Result<Self> {
        if probs.is_empty() {
            return Err(StatisticsError::EmptyInput);
        }
        if !probs.iter().all(|&p| p.is_finite() && p >= 0.0) {
            return Err(StatisticsError::InvalidParameter {
                what: "Categorical::probs",
                value: format!("{probs:?}"),
            });
        }
        let sum: f64 = probs.iter().sum();
        if sum <= 0.0 {
            return Err(StatisticsError::DomainError {
                what: "Categorical::probs",
                details: "sum must be > 0",
            });
        }
        for p in &mut probs {
            *p /= sum;
        }
        Ok(Self { probs })
    }
}

impl Distribution for Categorical {
    type Item = u64;
    fn mean(&self) -> f64 {
        f64::NAN
    }

    fn variance(&self) -> f64 {
        f64::NAN
    }

    fn mode(&self) -> Vec<Self::Item> {
        let max_prob = self.probs.iter().cloned().fold(f64::MIN, f64::max);
        self.probs
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| {
                if (p - max_prob).abs() < 1e-9 {
                    Some(i as Self::Item)
                } else {
                    None
                }
            })
            .collect()
    }

    fn pmf(&self, k: Self::Item) -> f64 {
        if k < self.probs.len() as Self::Item {
            self.probs[k as usize]
        } else {
            0.0
        }
    }

    fn log_pmf(&self, k: Self::Item) -> f64 {
        self.probs
            .get(k as usize) // .get()で範囲外ならNoneを返す
            .map_or(f64::NEG_INFINITY, |&p| p.ln()) // pが0なら-infになる
    }

    fn cdf(&self, k: Self::Item) -> f64 {
        self.probs.iter().take(k as usize + 1).sum()
    }

    fn quantile(&self, p: f64) -> Self::Item {
        let mut cumulative = 0.0;
        for (i, &prob) in self.probs.iter().enumerate() {
            cumulative += prob;
            if cumulative >= p {
                return i as Self::Item;
            }
        }
        (self.probs.len() - 1) as Self::Item
    }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::Item {
        let u: f64 = rng.gen();

        self.quantile(u)
    }

    fn skewness(&self) -> Option<f64> {
        None
    }

    fn kurtosis(&self) -> Option<f64> {
        None
    }
}
