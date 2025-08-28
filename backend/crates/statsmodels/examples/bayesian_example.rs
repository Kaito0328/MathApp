use linalg::{Matrix, Vector};
use rand::thread_rng;
use statistics::distribution::multivariate_continuous::core::MultivariateDistribution;
use statistics::distribution::multivariate_continuous::normal::MultivariateNormal;
use statsmodels::estimation::bayesian::{bayesian_estimation, fit_em};

fn main() {
    // 真のパラメータ x_true ~ N([1.0], [0.2])
    let prior_true = MultivariateNormal::new(
        Vector::new(vec![1.0]),
        Matrix::new(1, 1, vec![0.2]).unwrap(),
    )
    .unwrap();
    let mut rng = thread_rng();
    let x_true = prior_true.sample(&mut rng);

    // 観測モデル y = H x + e, ここでは H=[1;1;1], e ~ N(0, 0.5 I)
    let h = Matrix::new(3, 1, vec![1.0, 1.0, 1.0]).unwrap();
    let noise = MultivariateNormal::new(Vector::zeros(3), Matrix::identity(3) * 0.5).unwrap();
    let e = noise.sample(&mut rng);
    let y = &(&h * &x_true) + &e;

    println!("[Data] true x: {x_true}");
    println!("[Data] y: {y}");

    // 事前（推定側）は弱情報（分散大）から開始
    let prior_mean = Vector::new(vec![0.0]);
    let prior_cov = Matrix::identity(1) * 10.0;
    let noise_cov = Matrix::identity(3) * 0.5;

    println!("\n[Bayes] single-shot posterior (analytic)");
    let mvn = bayesian_estimation(&y, &h, &prior_mean, &prior_cov, &noise_cov).unwrap();
    let mmse = mvn.mean(); // ガウスでは MMSE = MAP = 事後平均
    let err = (&mmse - &x_true).norm();
    println!("posterior mean (MMSE estimate): {mmse}");
    println!("error ||x_hat - x_true||: {err}");
    println!("posterior cov:\n{}", mvn.covariance());

    println!("\n[EM] start from weak prior, stopping by mean/precision/noise deltas");
    let (m_hat, cov_hat, r_hat) = fit_em(&y, &h, 100, 1e-4).unwrap();
    let em_err = (&m_hat - &x_true).norm();
    println!("EM posterior mean (MMSE): {m_hat}");
    println!("EM error ||x_hat - x_true||: {em_err}");
    println!("EM posterior cov:\n{cov_hat}");
    println!("EM noise cov:\n{r_hat}");
}
