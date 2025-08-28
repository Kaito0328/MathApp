use linalg::matrix::numerical::svd::SvdDeComposition;
use linalg::Matrix;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = Matrix::new(3, 2, vec![1.0, 0.0, 0.0, 1.0, 1.0, 1.0])?;
    match a.svd() {
        Ok(svd) => println!("U = {}\nSigma = {:?}\nV = {}", svd.u, svd.sigma, svd.v),
        Err(e) => eprintln!("SVD failed: {e}"),
    }
    Ok(())
}
