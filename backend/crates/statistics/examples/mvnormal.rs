use linalg::{Matrix, Vector};
use rand::{rngs::StdRng, SeedableRng};
use statistics::distribution::multivariate_continuous::core::MultivariateDistribution;
use statistics::distribution::multivariate_continuous::normal::MultivariateNormal;

fn main() {
    let mean = Vector::new(vec![0.0, 0.0]);
    let cov = Matrix::new(2, 2, vec![1.0, 0.4, 0.4, 2.0]).unwrap();
    let mvn = MultivariateNormal::new(mean.clone(), cov).expect("valid mvn");

    println!("Multivariate Normal");
    println!("mean = {:?}", mvn.mean().as_slice());
    let c = mvn.covariance();
    println!(
        "cov  = [[{:.3}, {:.3}], [{:.3}, {:.3}]]",
        c[(0, 0)],
        c[(0, 1)],
        c[(1, 0)],
        c[(1, 1)]
    );

    let mut rng = StdRng::seed_from_u64(42);
    for i in 0..5 {
        let x = mvn.sample(&mut rng);
        println!("sample {}: {:?}", i + 1, x.as_slice());
    }
}
