use linalg::{Matrix, Vector};
use rand::{rngs::StdRng, SeedableRng};
use statistics::distribution::multivariate_continuous::core::MultivariateDistribution as MVContinuous;
use statistics::distribution::multivariate_continuous::t::MultivariateT;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn mvt_pdf_logpdf_and_covariance() {
    let nu = 5.0;
    let mu = Vector::new(vec![0.0, 0.0]);
    let sigma = Matrix::new(2, 2, vec![1.0, 0.2, 0.2, 2.0]).unwrap();
    let mvt = MultivariateT::new(nu, mu.clone(), sigma.clone()).expect("valid mvt");

    // pdf/log_pdf consistency
    let p = mvt.pdf(&mu);
    let lp = mvt.log_pdf(&mu);
    assert!(approx(lp.exp(), p, 1e-12));

    // covariance should be (nu/(nu-2)) * Sigma
    let cov = mvt.covariance();
    for i in 0..2 {
        for j in 0..2 {
            assert!(approx(cov[(i, j)], sigma[(i, j)] * (nu / (nu - 2.0)), 1e-9));
        }
    }
}

#[test]
fn mvt_sampling_mean_close() {
    let nu = 7.0;
    let mu = Vector::new(vec![1.0, -1.0]);
    let sigma = Matrix::identity(2);
    let mvt = MultivariateT::new(nu, mu.clone(), sigma).unwrap();
    let mut rng = StdRng::seed_from_u64(999);
    let n = 25_000;
    let samples = mvt.sample_n(&mut rng, n);
    let mut acc = [0.0f64, 0.0f64];
    for x in samples {
        acc[0] += x[0];
        acc[1] += x[1];
    }
    let xbar = [acc[0] / n as f64, acc[1] / n as f64];
    // 学生化tの平均はmu（nu>1）
    assert!(approx(xbar[0], mu[0], 0.06));
    assert!(approx(xbar[1], mu[1], 0.06));
}

#[test]
fn mvt_invalid_params() {
    let mu = Vector::new(vec![0.0, 0.0]);
    let sigma = Matrix::identity(2);
    assert!(MultivariateT::new(0.0, mu.clone(), sigma.clone()).is_err());
    // wrong dimension
    let sigma3 = Matrix::identity(3);
    assert!(MultivariateT::new(3.0, mu, sigma3).is_err());
}
