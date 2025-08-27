use linalg::Vector;
use rand::{rngs::StdRng, SeedableRng};
use statistics::distribution::mutivariate_discrete::core::MultivariateDistribution;
use statistics::distribution::mutivariate_discrete::multinomial::Multinomial;

fn main() {
    let n = 12usize;
    let p = Vector::new(vec![0.2, 0.3, 0.5]);
    let m = Multinomial::new(n, p.clone()).expect("valid multinomial");
    println!("Multinomial (n={n})");
    println!("p = {:?}", p.as_slice());
    println!("mean = {:?}", m.mean().as_slice());
    let c = m.covariance();
    println!("cov  = [[{:.3}, {:.3}, {:.3}],\n        [{:.3}, {:.3}, {:.3}],\n        [{:.3}, {:.3}, {:.3}]]",
        c[(0,0)], c[(0,1)], c[(0,2)], c[(1,0)], c[(1,1)], c[(1,2)], c[(2,0)], c[(2,1)], c[(2,2)]);

    let x = vec![2u64, 4u64, 6u64];
    println!("pmf([2,4,6]) = {:.6}", m.pmf(&x));
    println!("log_pmf([2,4,6]) = {:.6}", m.log_pmf(&x));

    let mut rng = StdRng::seed_from_u64(77);
    for i in 0..5 {
        let s = m.sample(&mut rng);
        println!("sample {}: {:?}", i + 1, s);
    }
}
