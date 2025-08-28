use linalg::matrix::numerical::qr::QrDecomposition; // bring the trait into scope
use linalg::Matrix;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = Matrix::new(
        3,
        3,
        vec![12.0, -51.0, 4.0, 6.0, 167.0, -68.0, -4.0, 24.0, -41.0],
    )?;
    let qr = a.qr_decomposition()?;
    println!(
        "Q = {}\nR = {}\nQ*R = {}",
        qr.q,
        qr.r,
        qr.q.clone() * qr.r.clone()
    );
    Ok(())
}
