use linalg::matrix::numerical::qr::QrDecomposition;
use linalg::Matrix;

fn almost_eq(a: &Matrix<f64>, b: &Matrix<f64>, tol: f64) -> bool {
    if a.rows != b.rows || a.cols != b.cols {
        return false;
    }
    let mut acc = 0.0;
    for i in 0..a.rows {
        for j in 0..a.cols {
            let d = a[(i, j)] - b[(i, j)];
            acc += d * d;
        }
    }
    acc.sqrt() < tol
}

#[test]
fn qr_smoke_identity() {
    let i = Matrix::<f64>::identity(3);
    let qr = i.qr_decomposition().expect("qr");
    // I = Q*R, ここで Q=I, R=I が自然
    let recon = qr.q * qr.r;
    assert!(almost_eq(&recon, &i, 1e-12));
}
