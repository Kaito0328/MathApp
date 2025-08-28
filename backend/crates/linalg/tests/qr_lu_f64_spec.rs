use linalg::matrix::numerical::QrDecomposition;
use linalg::Matrix;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn qr_reconstruction() {
    let a = Matrix::new(
        5,
        3,
        vec![
            2.0, -1.0, 0.0, 4.0, 1.0, -2.0, 0.0, 3.5, 1.0, -2.0, 0.0, 1.0, 1.0, 2.0, 0.0,
        ],
    )
    .expect("Matrix::new");
    let qr = a.qr_decomposition().expect("qr failed");
    // Q は直交、R は上三角
    let qtq = &qr.q.transpose() * &qr.q;
    for i in 0..qtq.rows {
        for j in 0..qtq.cols {
            assert!(approx(qtq[(i, j)], if i == j { 1.0 } else { 0.0 }, 1e-10));
        }
    }
    for i in 0..qr.r.rows {
        for j in 0..qr.r.cols {
            if i > j {
                assert!(qr.r[(i, j)].abs() < 1e-12);
            }
        }
    }
    // 再構成
    let recon = &qr.q * &qr.r;
    for i in 0..a.rows {
        for j in 0..a.cols {
            assert!(approx(a[(i, j)], recon[(i, j)], 1e-8));
        }
    }
}

#[test]
fn lu_reconstruction() {
    // 正方行列に対するLU（部分ピボット）
    let a = Matrix::new(
        4,
        4,
        vec![
            0.0, 2.0, 1.0, -1.0, 2.0, 1.0, -2.0, 0.0, -1.0, 3.0, 0.0, 1.0, 1.0, -1.0, 4.0, 2.0,
        ],
    )
    .expect("Matrix::new");
    let lu = a.lu_decompose().expect("lu failed");
    // LU 実装は P*A = L*U を満たす想定
    let lhs = &lu.p * &a;
    let rhs = &lu.l * &lu.u;
    for i in 0..a.rows {
        for j in 0..a.cols {
            assert!(
                approx(lhs[(i, j)], rhs[(i, j)], 1e-8),
                "mismatch at ({i},{j})",
            );
        }
    }
}
