use std::vec;

use linalg::{Matrix, Vector};
use special_functions::gamma::log_gamma;

use crate::distribution::{
    discrete::{categorical::Categorical, core::Distribution},
    mutivariate_discrete::core::MultivariateDistribution,
};

pub struct Multinomial {
    trials: usize,
    probabilities: Vector<f64>,
}

impl Multinomial {
    pub fn new(trials: usize, probabilities: Vector<f64>) -> Result<Self, String> {
        if probabilities.is_empty() {
            return Err("Probabilities vector is empty.".to_string());
        }
        let sum: f64 = probabilities.iter().sum();
        if (sum - 1.0).abs() > f64::EPSILON {
            return Err("Probabilities must sum to 1.".to_string());
        }
        Ok(Self {
            trials,
            probabilities,
        })
    }
}

impl MultivariateDistribution for Multinomial {
    type Item = Vec<u64>;
    fn mean(&self) -> Vector<f64> {
        &self.probabilities * self.trials as f64
    }

    fn covariance(&self) -> Matrix<f64> {
        let k = self.probabilities.len(); // K: カテゴリ数
        let n = self.trials as f64;
        let mut cov_data = Vec::with_capacity(k * k);

        for i in 0..k {
            for j in 0..k {
                if i == j {
                    // 分散: n * p_i * (1 - p_i)
                    cov_data.push(n * self.probabilities[i] * (1.0 - self.probabilities[i]));
                } else {
                    // 共分散: -n * p_i * p_j
                    cov_data.push(-n * self.probabilities[i] * self.probabilities[j]);
                }
            }
        }
        // Matrix::new は Result を返さないと仮定
        Matrix::new(k, k, cov_data).unwrap()
    }

    fn mode(&self) -> Option<Self::Item> {
        let mean = self.mean();

        let mut mean_round: Vec<u64> = mean.iter().map(|&m| m.round() as u64).collect();

        let sum_round: u64 = mean_round.iter().sum();

        if sum_round < self.trials as u64 {
            for _ in 0..(self.trials as u64 - sum_round) as usize {
                if let Some((max_idx, _)) = self.probabilities.iter().enumerate().max_by(|a, b| {
                    (a.1 / (mean_round[a.0] as f64 + 1.0))
                        .partial_cmp(&(b.1 / (mean_round[b.0] as f64 + 1.0)))
                        .unwrap()
                }) {
                    mean_round[max_idx] += 1;
                }
            }
        } else if sum_round > self.trials as u64 {
            for _ in 0..(sum_round - self.trials as u64) as usize {
                if let Some((max_idx, _)) = self.probabilities.iter().enumerate().max_by(|a, b| {
                    (mean_round[a.0] as f64 / a.1)
                        .partial_cmp(&(mean_round[b.0] as f64 / b.1))
                        .unwrap()
                }) {
                    mean_round[max_idx] -= 1;
                }
            }
        }

        Some(mean_round)
    }

    fn pmf(&self, x: &Self::Item) -> f64 {
        self.log_pmf(x).exp()
    }

    fn log_pmf(&self, x: &Self::Item) -> f64 {
        assert_eq!(
            x.len(),
            self.probabilities.len(),
            "Input vector has wrong dimension."
        );
        assert_eq!(
            x.iter().sum::<u64>(),
            self.trials as u64,
            "Sum of counts must equal number of trials."
        );
        let mut log_pmf = log_gamma(self.trials as f64 + 1.0);

        for (k, p) in self.probabilities.iter().enumerate() {
            log_pmf += x[k] as f64 * p.ln() - log_gamma(x[k] as f64 + 1.0);
        }
        log_pmf
    }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::Item {
        // 1. カテゴリカル分布のインスタンスを生成
        // self.probabilities.data.clone() はあなたのVector型に合わせてください
        let categorical = Categorical::new(self.probabilities.data.clone()).unwrap();

        // 2. カテゴリ数 K の長さを持つカウント用ベクトルを用意
        let mut counts = vec![0; self.probabilities.len()];

        // 3. n回試行する
        for _ in 0..self.trials {
            // 4. カテゴリカル分布から1つサンプル
            let outcome = categorical.sample(rng) as usize;
            // 5. 対応するカウントを増やす
            counts[outcome] += 1;
        }

        counts
    }
}
