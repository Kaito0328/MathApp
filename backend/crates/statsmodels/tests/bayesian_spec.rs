use linalg::{Matrix, Vector};
use statistics::distribution::multivariate_continuous::core::MultivariateDistribution;
use statsmodels::estimation::bayesian::{bayesian_estimation, bayesian_estimation_with_precision};

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn bayesian_estimation_simple_case() {
    // y = H x + e, 1次元×2観測
    let y = Vector::new(vec![1.0, 2.0]);
    let h = Matrix::new(2, 1, vec![1.0, 1.0]).unwrap();
    let prior_mean = Vector::new(vec![0.0]);
    let prior_cov = Matrix::new(1, 1, vec![1.0]).unwrap();
    let noise_cov = Matrix::identity(2) * 0.5; // R = 0.5 I

    let mvn = bayesian_estimation(&y, &h, &prior_mean, &prior_cov, &noise_cov).unwrap();
    let post_mean = mvn.mean();
    // 粗いチェック: posterior mean が観測平均に近い方向（~1.5）
    assert!(approx(post_mean[0], 1.0, 0.6));
}

#[test]
fn bayesian_estimation_with_precision_consistency() {
    // 2次元、整合性チェック
    let y = Vector::new(vec![1.0, 0.0]);
    let h = Matrix::new(2, 2, vec![1.0, 0.0, 0.0, 1.0]).unwrap(); // I
    let prior_mean = Vector::new(vec![0.0, 0.0]);
    let prior_cov = Matrix::identity(2);
    let prior_precision = prior_cov.inverse().unwrap();
    let noise_cov = Matrix::identity(2) * 0.1;

    let (m, p) =
        bayesian_estimation_with_precision(&y, &h, &prior_mean, &prior_precision, &noise_cov)
            .unwrap();
    // posterior_precision は H^T R^{-1} H + prior_precision なので、正定値
    assert_eq!(p.rows, 2);
    assert_eq!(p.cols, 2);
    // 事後平均の次元チェック
    assert_eq!(m.dim(), 2);
}
