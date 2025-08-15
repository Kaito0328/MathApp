use linalg::matrix::numerical::eigen::EigenDecomposition;
use linalg::matrix::numerical::qr::QrDecomposition;
use linalg::Matrix;

fn main() {
    // 固有値分解 / QR 分解のスモールデモ
    let a = Matrix::new(3, 3, vec![4.0, -2.0, 1.0, 0.0, 3.0, -1.0, 0.0, 0.0, 2.0]).unwrap();
    println!("A = {a}");

    // QR 分解
    let qr = a.qr_decomposition().expect("qr");
    println!(
        "Q = {}\nR = {}\nQ*R = {}",
        qr.q,
        qr.r,
        qr.q.clone() * qr.r.clone()
    );

    // 固有値分解
    if let Some(eig) = a.eigen_decomposition() {
        println!("eigenvalues = {:?}", eig.eigen_values);
        println!("eigenvectors = {}", eig.eigen_vectors);
    }
}
