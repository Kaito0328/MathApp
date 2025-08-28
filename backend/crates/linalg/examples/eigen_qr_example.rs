use linalg::matrix::numerical::eigen::EigenDecomposition;
use linalg::matrix::numerical::qr::QrDecomposition;
use linalg::Matrix;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 固有値分解 / QR 分解のスモールデモ
    let a = Matrix::new(3, 3, vec![4.0, -2.0, 1.0, 0.0, 3.0, -1.0, 0.0, 0.0, 2.0])?;
    println!("A = {a}");

    // QR 分解
    let qr = a.qr_decomposition()?;
    println!(
        "Q = {}\nR = {}\nQ*R = {}",
        qr.q,
        qr.r,
        qr.q.clone() * qr.r.clone()
    );

    // 固有値分解
    match a.eigen_decomposition() {
        Ok(eig) => {
            println!("eigenvalues = {:?}", eig.eigen_values);
            println!("eigenvectors = {}", eig.eigen_vectors);
        }
        Err(e) => println!("eigendecomp error: {e}"),
    }
    Ok(())
}
