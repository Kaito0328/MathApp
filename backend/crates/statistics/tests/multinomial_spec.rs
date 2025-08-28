use linalg::Vector;
use rand::{rngs::StdRng, SeedableRng};
use statistics::distribution::mutivariate_discrete::core::MultivariateDistribution as MVDiscrete;
use statistics::distribution::mutivariate_discrete::multinomial::Multinomial;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn multinomial_basic_properties() {
    let n = 10usize;
    let p = Vector::new(vec![0.2, 0.3, 0.5]);
    let m = Multinomial::new(n, p.clone()).expect("valid");

    // mean = n p
    let mean = m.mean();
    for i in 0..p.len() {
        assert!(approx(mean[i], n as f64 * p[i], 1e-12));
    }

    // covariance shape and some entries
    let cov = m.covariance();
    assert_eq!(cov.rows, p.len());
    assert_eq!(cov.cols, p.len());
    for i in 0..p.len() {
        assert!(approx(cov[(i, i)], n as f64 * p[i] * (1.0 - p[i]), 1e-12));
    }
    for i in 0..p.len() {
        for j in 0..p.len() {
            if i != j {
                assert!(approx(cov[(i, j)], -(n as f64) * p[i] * p[j], 1e-12));
            }
        }
    }
}

#[test]
fn multinomial_pmf_logpmf_consistency() {
    let n = 6usize;
    let p = Vector::new(vec![0.25, 0.25, 0.5]);
    let m = Multinomial::new(n, p).unwrap();
    let x = vec![1u64, 2u64, 3u64];
    let lp = m.log_pmf(&x);
    let p = m.pmf(&x);
    assert!(approx(lp.exp(), p, 1e-12));
}

#[test]
fn multinomial_sampling_counts_sum() {
    let n = 100usize;
    let p = Vector::new(vec![0.1, 0.2, 0.7]);
    let m = Multinomial::new(n, p).unwrap();
    let mut rng = StdRng::seed_from_u64(2024);
    let x = m.sample(&mut rng);
    assert_eq!(x.iter().sum::<u64>(), n as u64);
}

#[test]
fn multinomial_mode_sums_to_n() {
    let n = 17usize;
    let p = Vector::new(vec![0.4, 0.35, 0.25]);
    let m = Multinomial::new(n, p).unwrap();
    let mode = m.mode().unwrap();
    assert_eq!(mode.iter().sum::<u64>(), n as u64);
}

#[test]
fn multinomial_invalid_params() {
    // probs not summing to 1
    let p = Vector::new(vec![0.2, 0.2, 0.2]);
    assert!(Multinomial::new(10, p).is_err());
}
