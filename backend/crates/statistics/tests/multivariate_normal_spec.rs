use linalg::{Matrix, Vector};
use rand::{rngs::StdRng, SeedableRng};
use statistics::distribution::multivariate_continuous::core::MultivariateDistribution as MVContinuous;
use statistics::distribution::multivariate_continuous::normal::MultivariateNormal;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn mvn_constructor_and_basic() {
    let mean = Vector::new(vec![0.0, 0.0]);
    let cov = Matrix::identity(2);
    let mvn = MultivariateNormal::new(mean.clone(), cov.clone()).expect("valid mvn");

    // mean
    assert_eq!(mvn.mean(), mean);
    // covariance shape and approximate equality to input
    let got = mvn.covariance();
    assert_eq!(got.rows, 2);
    assert_eq!(got.cols, 2);
    for i in 0..2 {
        for j in 0..2 {
            assert!(approx(got[(i, j)], cov[(i, j)], 1e-12));
        }
    }

    // pdf/log_pdf consistency at mean
    let logp0 = mvn.log_pdf(&mean);
    let p0 = mvn.pdf(&mean);
    assert!(approx(logp0.exp(), p0, 1e-12));
    // analytical value for 2D standard normal at mean
    assert!(approx(p0, 1.0 / (2.0 * std::f64::consts::PI), 1e-12));
}

#[test]
fn mvn_sampling_mean_close() {
    let mean = Vector::new(vec![1.0, -2.0]);
    let cov = Matrix::identity(2);
    let mvn = MultivariateNormal::new(mean.clone(), cov).unwrap();
    let mut rng = StdRng::seed_from_u64(123);
    let n = 20_000;
    let samples = mvn.sample_n(&mut rng, n);
    let mut acc = [0.0f64, 0.0f64];
    for x in samples {
        acc[0] += x[0];
        acc[1] += x[1];
    }
    let xbar = [acc[0] / n as f64, acc[1] / n as f64];
    assert!(approx(xbar[0], mean[0], 0.05));
    assert!(approx(xbar[1], mean[1], 0.05));
}

#[test]
fn mvn_invalid_cov_dimension() {
    let mean = Vector::new(vec![0.0, 0.0]);
    let cov = Matrix::identity(3); // wrong size
    let res = MultivariateNormal::new(mean, cov);
    assert!(res.is_err());
}

#[test]
fn mvn_non_pd_covariance() {
    // [[1,2],[2,1]] は非正定値（固有値に負あり）
    let mean = Vector::new(vec![0.0, 0.0]);
    let cov = Matrix::new(2, 2, vec![1.0, 2.0, 2.0, 1.0]).unwrap();
    let res = MultivariateNormal::new(mean, cov);
    assert!(res.is_err());
}
