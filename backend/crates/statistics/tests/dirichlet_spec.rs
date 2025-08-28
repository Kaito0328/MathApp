use linalg::Vector;
use rand::rngs::StdRng;
use rand::SeedableRng;
use statistics::distribution::multivariate_continuous::core::MultivariateDistribution;
use statistics::distribution::multivariate_continuous::dirichlet::Dirichlet;

#[test]
fn dirichlet_basic_properties() {
    let alpha = Vector::new(vec![1.0, 2.0, 3.0]);
    let d = Dirichlet::new(alpha.clone()).expect("valid alpha");

    // mean should be alpha / sum(alpha)
    let mean = d.mean();
    let s: f64 = alpha.iter().sum();
    for i in 0..alpha.len() {
        assert!((mean[i] - alpha[i] / s).abs() < 1e-12);
    }

    // covariance is positive semidefinite on the simplex; just check shape and diagonals > 0
    let cov = d.covariance();
    assert_eq!(cov.rows, alpha.len());
    assert_eq!(cov.cols, alpha.len());
    for i in 0..alpha.len() {
        assert!(cov[(i, i)] > 0.0);
    }

    // mode exists when all alpha_i > 1
    assert!(d.mode().is_none());
    let d2 = Dirichlet::new(Vector::new(vec![2.0, 2.5, 3.0])).unwrap();
    assert!(d2.mode().is_some());
}

#[test]
fn dirichlet_pdf_logpdf_consistency() {
    let d = Dirichlet::new(Vector::new(vec![2.0, 3.0, 4.0])).unwrap();
    let x = Vector::new(vec![0.2, 0.3, 0.5]);
    let logp = d.log_pdf(&x);
    let p = d.pdf(&x);
    assert!((logp.exp() - p).abs() < 1e-12);
}

#[test]
fn dirichlet_sampling_on_simplex() {
    let d = Dirichlet::new(Vector::new(vec![1.0, 1.0, 1.0])).unwrap();
    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..1000 {
        let x = d.sample(&mut rng);
        // in [0,1]
        assert!(x.iter().all(|&xi| (0.0..=1.0).contains(&xi)));
        // sums to ~1
        let s: f64 = x.iter().sum();
        assert!((s - 1.0).abs() < 1e-12);
    }
}

#[test]
fn dirichlet_invalid_params() {
    assert!(Dirichlet::new(Vector::new(vec![1.0])).is_err()); // k<2
    assert!(Dirichlet::new(Vector::new(vec![1.0, -0.1])).is_err()); // non-positive
}
