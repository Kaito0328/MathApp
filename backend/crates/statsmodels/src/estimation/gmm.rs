use std::vec;

use crate::error::{Result, StatsModelsError};
use linalg::{Matrix, Vector};
use rand::seq::index;
use statistics::distribution::multivariate_continuous::{
    core::MultivariateDistribution, normal::MultivariateNormal,
};

pub struct GaussianMixtureModel {
    weights: Vec<f64>,
    distributions: Vec<MultivariateNormal>,
}

impl GaussianMixtureModel {
    pub fn new(
        weights: Vec<f64>,
        means: Vec<Vector<f64>>,
        covariances: Vec<Matrix<f64>>,
    ) -> Result<Self> {
        if weights.is_empty() || means.is_empty() || covariances.is_empty() {
            return Err(StatsModelsError::EmptyInput);
        }
        if (weights.iter().sum::<f64>() - 1.0).abs() > 1e-12 {
            return Err(StatsModelsError::InvalidParameter {
                what: "GaussianMixtureModel::weights",
                details: "weights must sum to 1".to_string(),
            });
        }
        if means.len() != covariances.len() || means.len() != weights.len() {
            return Err(StatsModelsError::DimensionMismatch {
                expected: format!("{0} means & covariances", weights.len()),
                found: format!("means: {}, covs: {}", means.len(), covariances.len()),
            });
        }

        let distributions = means
            .iter()
            .zip(covariances.iter())
            .map(|(mean, cov)| MultivariateNormal::new(mean.clone(), cov.clone()))
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| StatsModelsError::InvalidParameter {
                what: "GaussianMixtureModel::covariances",
                details: e,
            })?;

        Ok(Self {
            weights,
            distributions,
        })
    }

    pub fn weights(&self) -> &Vec<f64> {
        &self.weights
    }

    pub fn distributions(&self) -> &Vec<MultivariateNormal> {
        &self.distributions
    }

    pub fn pdf(&self, x: &Vector<f64>) -> f64 {
        self.log_pdf(x).exp()
    }

    pub fn log_pdf(&self, x: &Vector<f64>) -> f64 {
        let log_probs: Vec<f64> = self
            .weights
            .iter()
            .zip(self.distributions.iter())
            .map(|(w, dist)| w.ln() + dist.log_pdf(x))
            .collect();

        // LogSumExpトリック
        let max_log_prob = log_probs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        max_log_prob
            + log_probs
                .iter()
                .map(|lp| (lp - max_log_prob).exp())
                .sum::<f64>()
                .ln()
    }

    pub fn predict_proba(&self, x: &Vector<f64>) -> Vec<f64> {
        // 1. 各クラスタからの尤度を計算
        let likelihoods: Vec<f64> = self
            .weights
            .iter()
            .zip(self.distributions.iter())
            .map(|(w, dist)| w * dist.pdf(x))
            .collect();

        // 2. 尤度の合計で正規化して、負担率（確率）を計算
        let total_likelihood: f64 = likelihoods.iter().sum();

        likelihoods
            .into_iter()
            .map(|l| l / total_likelihood)
            .collect()
    }

    pub fn predict(&self, x: &Vector<f64>) -> usize {
        self.predict_proba(x)
            .into_iter() // cloned()は不要
            .enumerate()
            // `max_by_key` の代わりに `max_by` を使用
            .max_by(|(_, a), (_, b)| {
                // partial_cmpでf64を比較
                a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(index, _)| index)
            .unwrap() // 確率ベクトルが空でないことは保証されている
    }

    pub fn fit(data: &[Vector<f64>], k: usize, max_iter: usize, tol: f64) -> Result<Self> {
        let mut gmm = Self::init_params(data, k)?;

        let mut last_log_likelihood = f64::NEG_INFINITY;

        for _ in 0..max_iter {
            let responsibilities = gmm.expectation(data);

            gmm.maximization(&responsibilities, data)?;

            let log_likelihood = gmm.log_likelihood(data);
            if (log_likelihood - last_log_likelihood).abs() < tol {
                break;
            }
            last_log_likelihood = log_likelihood;
        }

        Ok(gmm)
    }

    pub fn log_likelihood(&self, data: &[Vector<f64>]) -> f64 {
        data.iter().map(|x| self.log_pdf(x)).sum()
    }
    fn init_params(data: &[Vector<f64>], k: usize) -> Result<Self> {
        let n = data.len();
        if k == 0 || n < k {
            return Err(StatsModelsError::InvalidParameter {
                what: "GaussianMixtureModel::k",
                details: "k must be in 1..=n".to_string(),
            });
        }

        let mut rng = rand::thread_rng();

        // 1. 重み(weights)の初期化 (これは完璧です)
        let weights = vec![1.0 / k as f64; k];

        // 2. 平均(means)の初期化
        // データの中からランダムにk個のインデックスを選ぶ
        let random_indices = index::sample(&mut rng, n, k).into_vec();
        // 選ばれたインデックスに対応するデータ点を初期平均とする
        let means: Vec<Vector<f64>> = random_indices
            .into_iter()
            .map(|i| data[i].clone())
            .collect();

        // 3. 共分散(covariances)の初期化 (これも完璧です)
        let dim = data[0].len();
        let covariances = vec![Matrix::identity(dim); k];

        // 4. newを呼び出す
        Self::new(weights, means, covariances)
    }

    fn expectation(&self, data: &[Vector<f64>]) -> Vec<Vector<f64>> {
        data.iter()
            .map(|x| {
                // 各クラスタkに対して、log(π_k * N(x|μ_k, Σ_k)) を計算
                let log_probs: Vec<f64> = self
                    .weights
                    .iter()
                    .zip(self.distributions.iter())
                    .map(|(w, dist)| w.ln() + dist.log_pdf(x))
                    .collect();

                // LogSumExpトリックで、log(Σ_j π_j N(x|μ_j, Σ_j)) を計算
                let max_log_prob = log_probs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let log_total = max_log_prob
                    + log_probs
                        .iter()
                        .map(|lp| (lp - max_log_prob).exp())
                        .sum::<f64>()
                        .ln();

                // 負担率 log(γ_ik) = log(π_k) + log_pdf_k(x) - log_total を計算し、exp()で戻す
                let responsibility: Vec<f64> =
                    log_probs.iter().map(|lp| (lp - log_total).exp()).collect();

                Vector::new(responsibility)
            })
            .collect()
    }

    fn maximization(
        &mut self,
        responsibilities: &[Vector<f64>],
        data: &[Vector<f64>],
    ) -> Result<()> {
        let n = data.len();
        let k = self.weights.len();
        let dim = data[0].len();

        let mut new_weights = Vec::with_capacity(k);
        let mut new_distributions = Vec::with_capacity(k);

        // 各クラスタ j (0..k) についてパラメータを更新
        for j in 0..k {
            // N_j: クラスタjの負担率の合計
            let resp_j_sum = responsibilities.iter().map(|resp_i| resp_i[j]).sum::<f64>();

            // 1. 重みπ_j を更新
            new_weights.push(resp_j_sum / n as f64);

            // 2. 平均μ_j を更新
            let new_mean_j = data.iter().zip(responsibilities.iter()).fold(
                Vector::zeros(dim),
                |acc, (x_i, resp_i)| {
                    acc + (x_i * resp_i[j]) // x_iを、i番目のデータのj番目の負担率で重み付け
                },
            ) * (1.0 / resp_j_sum);

            // 3. 共分散行列Σ_j を更新
            let new_sigma_j = data.iter().zip(responsibilities.iter()).fold(
                Matrix::zeros(dim, dim),
                |acc, (x_i, resp_i)| {
                    let diff = x_i - &new_mean_j;
                    acc + (&(&diff * &diff.transpose()) * resp_i[j])
                },
            ) * (1.0 / resp_j_sum);

            let dist = MultivariateNormal::new(new_mean_j, new_sigma_j).map_err(|e| {
                StatsModelsError::InvalidParameter {
                    what: "GaussianMixtureModel::covariances",
                    details: e,
                }
            })?;
            new_distributions.push(dist);
        }

        self.weights = new_weights;
        self.distributions = new_distributions;
        Ok(())
    }
}
