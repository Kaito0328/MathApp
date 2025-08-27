use linalg::{Matrix, Vector};
use rand::{rngs::StdRng, SeedableRng};
use statistics::distribution::multivariate_continuous::core::MultivariateDistribution;
use statistics::distribution::multivariate_continuous::t::MultivariateT;

fn main() {
    let nu = 6.0;
    let mu = Vector::new(vec![1.0, -1.0]);
    let sigma = Matrix::new(2, 2, vec![1.0, 0.3, 0.3, 1.5]).unwrap();
    let mvt = MultivariateT::new(nu, mu.clone(), sigma).expect("valid mvt");

    println!("Multivariate t (nu={nu})");
    println!("mean = {:?}", mvt.mean().as_slice());
    let c = mvt.covariance();
    println!(
        "cov  = [[{:.3}, {:.3}], [{:.3}, {:.3}]]",
        c[(0, 0)],
        c[(0, 1)],
        c[(1, 0)],
        c[(1, 1)]
    );

    let x = mu.clone();
    println!("pdf(mu) = {:.6}", mvt.pdf(&x));

    let mut rng = StdRng::seed_from_u64(7);
    for i in 0..5 {
        let s = mvt.sample(&mut rng);
        println!("sample {}: {:?}", i + 1, s.as_slice());
    }
}
