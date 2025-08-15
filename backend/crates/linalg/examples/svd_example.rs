use linalg::matrix::numerical::svd::SvdDeComposition;
use linalg::Matrix;

fn main() {
    let a = Matrix::new(3, 2, vec![1.0, 0.0, 0.0, 1.0, 1.0, 1.0]).unwrap();
    if let Some(svd) = a.svd() {
        println!("U = {}\nSigma = {:?}\nV = {}", svd.u, svd.sigma, svd.v);
    } else {
        eprintln!("SVD failed");
    }
}
