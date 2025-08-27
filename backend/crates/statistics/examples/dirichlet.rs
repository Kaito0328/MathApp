use linalg::Vector;
use rand::rngs::StdRng;
use rand::SeedableRng;
use statistics::distribution::multivariate_continuous::core::MultivariateDistribution;
use statistics::distribution::multivariate_continuous::dirichlet::Dirichlet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Dirichlet(alpha=[2,3,4]) の基本
    let alpha = Vector::new(vec![2.0, 3.0, 4.0]);
    let d = Dirichlet::new(alpha.clone())?;
    println!("alpha = {:?}", alpha.as_slice());
    println!("mean = {:?}", d.mean().as_slice());
    println!("cov ({}x{})", d.covariance().rows, d.covariance().cols);

    // サンプリング
    let mut rng = StdRng::seed_from_u64(123);
    for i in 0..5 {
        let x = d.sample(&mut rng);
        println!(
            "sample {}: {:?} (sum={:.6})",
            i + 1,
            x.as_slice(),
            x.iter().sum::<f64>()
        );
    }

    // mode（全α>1のとき）
    let d2 = Dirichlet::new(Vector::new(vec![1.5, 2.5, 3.5]))?;
    println!(
        "mode(all>1) = {:?}",
        d2.mode().map(|m| m.as_slice().to_vec())
    );

    Ok(())
}
