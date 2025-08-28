use linalg::{Matrix, Vector};
use statistics::distribution::multivariate_continuous::normal::MultivariateNormal;

use crate::error::{Result, StatsModelsError};

pub fn bayesian_estimation(
    y: &Vector<f64>,
    h: &Matrix<f64>,
    prior_mean: &Vector<f64>,
    prior_cov: &Matrix<f64>,
    noise_cov: &Matrix<f64>,
) -> Result<MultivariateNormal> {
    let prior_precision = prior_cov
        .inverse()
        .ok_or(StatsModelsError::InvalidParameter {
            what: "bayesian_estimation",
            details: "prior_cov is not invertible".to_string(),
        })?;
    let (posterior_mean, posterior_precision) =
        bayesian_estimation_with_precision(y, h, prior_mean, &prior_precision, noise_cov)?;
    let posterior_cov = posterior_precision
        .inverse()
        .ok_or(StatsModelsError::Linalg(
            linalg::LinalgError::SingularMatrix,
        ))?;
    let mvn = MultivariateNormal::new(posterior_mean, posterior_cov).map_err(|e| {
        StatsModelsError::InvalidParameter {
            what: "MultivariateNormal::new",
            details: e,
        }
    })?;
    Ok(mvn)
}

pub fn bayesian_estimation_with_precision(
    y: &Vector<f64>,
    h: &Matrix<f64>,
    prior_mean: &Vector<f64>,
    prior_precision: &Matrix<f64>,
    noise_cov: &Matrix<f64>,
) -> Result<(Vector<f64>, Matrix<f64>)> {
    let ht = h.transpose();
    let noise_cov_lu = match noise_cov.lu_decompose() {
        Ok(lu) => lu,
        Err(linalg::LinalgError::SingularMatrix) => {
            // ほんの少し対角成分を持ち上げて再試行
            let jitter = Matrix::identity(noise_cov.rows) * 1e-8;
            (noise_cov + &jitter).lu_decompose()?
        }
        Err(e) => return Err(e.into()),
    };

    let z = Matrix::solve_with_lu(&noise_cov_lu, y)?;
    let w = Matrix::solve_matrix_with_lu(&noise_cov_lu, h)?;

    let q = prior_precision * prior_mean;

    let posterior_precision = &ht * &w + prior_precision;
    let ht_z: Vector<f64> = &ht * &z;
    let rhs: Vector<f64> = &ht_z + &q;
    let posterior_mean = posterior_precision.solve(&rhs)?;

    Ok((posterior_mean, posterior_precision))
}

pub fn fit_em(
    y: &Vector<f64>,
    h: &Matrix<f64>,
    max_iter: usize,
    tol: f64,
) -> Result<(Vector<f64>, Matrix<f64>, Matrix<f64>)> {
    let mut prior_mean = Vector::zeros(h.cols);
    let mut prior_precision = Matrix::identity(h.cols) * 0.01;
    let mut noise_cov = Matrix::identity(h.rows) * 0.01;

    // ヘルパ: Frobenius ノルム（簡易）
    fn frob(m: &Matrix<f64>) -> f64 {
        let mut s = 0.0;
        for i in 0..m.rows {
            for j in 0..m.cols {
                let v = m[(i, j)];
                s += v * v;
            }
        }
        s.sqrt()
    }

    // ヘルパ: 精度行列の安定化（対角を下限でクリップ）
    fn stabilize_precision(p: &Matrix<f64>, min_diag: f64) -> Matrix<f64> {
        let mut out = p.clone();
        let n = std::cmp::min(out.rows, out.cols);
        for i in 0..n {
            if out[(i, i)] < min_diag {
                out[(i, i)] = min_diag;
            }
        }
        out
    }

    // ヘルパ: 共分散行列の安定化（対称化 + 対角を下限でクリップ）
    fn stabilize_covariance(c: &Matrix<f64>, min_diag: f64) -> Matrix<f64> {
        let sym = (c + &c.transpose()) * 0.5; // 対称化
        let mut out = sym.clone();
        let n = std::cmp::min(out.rows, out.cols);
        for i in 0..n {
            if out[(i, i)] < min_diag {
                out[(i, i)] = min_diag;
            }
        }
        out
    }

    // EM 反復
    for _ in 0..max_iter {
        // 事前がゼロ情報に近い場合の安定化
        let stabilized_prior_precision = stabilize_precision(&prior_precision, 1e-8);

        let (posterior_mean, posterior_precision) = bayesian_estimation_with_precision(
            y,
            h,
            &prior_mean,
            &stabilized_prior_precision,
            &noise_cov,
        )?;

        let new_prior_mean = &posterior_mean.clone();
        let new_prior_precision = &posterior_precision.clone();
        let new_noise_cov = {
            let diff = &(y - h * &posterior_mean);
            let ht = h.transpose();
            let pn_ht = &posterior_precision.solve_matrix(&ht)?;
            let raw = &((diff * diff.transpose() + h * pn_ht) * (1.0 / y.dim() as f64));
            // 数値安定化（半正定に落ちるのを防ぐ）
            &stabilize_covariance(raw, 1e-8)
        };

        // 収束判定: 平均, 事後精度, ノイズ共分散の変化
        let delta_mean = (new_prior_mean - &prior_mean).norm();
        let delta_prec = frob(&(new_prior_precision - &prior_precision));
        let delta_noise = frob(&(new_noise_cov - &noise_cov));
        if delta_mean < tol && delta_prec < tol && delta_noise < tol {
            prior_mean = new_prior_mean.clone();
            prior_precision = new_prior_precision.clone();
            noise_cov = new_noise_cov.clone();
            break;
        }

        // 次反復へ
        prior_mean = new_prior_mean.clone();
        prior_precision = new_prior_precision.clone();
        noise_cov = new_noise_cov.clone();
    }

    let prior_cov = prior_precision.inverse().ok_or(StatsModelsError::Linalg(
        linalg::LinalgError::SingularMatrix,
    ))?;
    Ok((prior_mean, prior_cov, noise_cov))
}
