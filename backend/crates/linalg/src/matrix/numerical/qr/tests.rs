use super::*;
use crate::matrix::Matrix;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

fn assert_matrix_approx_eq(a: &Matrix<f64>, b: &Matrix<f64>, tol: f64) {
    assert_eq!(a.rows, b.rows, "row mismatch");
    assert_eq!(a.cols, b.cols, "col mismatch");
    for i in 0..a.rows {
        for j in 0..a.cols {
            assert!(
                approx(a[(i, j)], b[(i, j)], tol),
                "mismatch at ({}, {}): {} vs {}, tol={}",
                i,
                j,
                a[(i, j)],
                b[(i, j)],
                tol
            );
        }
    }
}

fn assert_orthogonal(q: &Matrix<f64>, tol: f64) {
    assert_eq!(q.rows, q.cols, "Q must be square");
    let qtq = &q.transpose() * q;
    let i = Matrix::identity(q.rows);
    assert_matrix_approx_eq(&qtq, &i, tol);
}

fn assert_upper_trapezoidal(r: &Matrix<f64>, tol: f64) {
    // 上三角（台形）: i > j の成分がほぼ 0
    for i in 0..r.rows {
        for j in 0..r.cols {
            if i > j {
                assert!(
                    r[(i, j)].abs() <= tol,
                    "R must be upper trapezoidal at ({}, {}) = {}",
                    i,
                    j,
                    r[(i, j)]
                );
            }
        }
    }
}

#[test]
fn qr_square_matrix_reconstructs() {
    let a = Matrix::new(
        3,
        3,
        vec![
            12.0, -51.0, 4.0, //
            6.0, 167.0, -68.0, //
            -4.0, 24.0, -41.0,
        ],
    )
    .unwrap();

    let qr = a.qr_decomposition().expect("qr failed");
    assert_eq!(qr.q.rows, 3);
    assert_eq!(qr.q.cols, 3);
    assert_eq!(qr.r.rows, 3);
    assert_eq!(qr.r.cols, 3);

    assert_orthogonal(&qr.q, 1e-10);
    assert_upper_trapezoidal(&qr.r, 1e-10);

    // A ≈ Q R
    let qr_prod = &qr.q * &qr.r;
    assert_matrix_approx_eq(&qr_prod, &a, 1e-8);

    // diag(R) >= 0
    for i in 0..3 {
        assert!(qr.r[(i, i)] >= -1e-12, "R diagonal should be nonnegative");
    }
}

#[test]
fn qr_tall_matrix_reconstructs() {
    // 行 > 列 の場合
    let a = Matrix::new(
        5,
        3,
        vec![
            2.0, -1.0, 0.0, //
            4.0, 1.0, -2.0, //
            0.0, 3.5, 1.0, //
            -2.0, 0.0, 1.0, //
            1.0, 2.0, 0.0,
        ],
    )
    .unwrap();

    let qr = a.qr_decomposition().expect("qr failed");
    assert_eq!(qr.q.rows, 5);
    assert_eq!(qr.q.cols, 5);
    assert_eq!(qr.r.rows, 5);
    assert_eq!(qr.r.cols, 3);

    assert_orthogonal(&qr.q, 1e-10);
    assert_upper_trapezoidal(&qr.r, 1e-10);

    let qr_prod = &qr.q * &qr.r;
    assert_matrix_approx_eq(&qr_prod, &a, 1e-8);

    for i in 0..3.min(qr.r.rows) {
        assert!(qr.r[(i, i)] >= -1e-12, "R diag >= 0");
    }
}

#[test]
fn qr_wide_matrix_reconstructs() {
    // 行 < 列 の場合
    let a = Matrix::new(
        3,
        5,
        vec![
            1.0, 2.0, 3.0, 4.0, 5.0, //
            2.0, 3.0, 4.0, 5.0, 6.0, //
            -1.0, 0.0, 1.0, 0.0, -1.0,
        ],
    )
    .unwrap();

    let qr = a.qr_decomposition().expect("qr failed");
    assert_eq!(qr.q.rows, 3);
    assert_eq!(qr.q.cols, 3);
    assert_eq!(qr.r.rows, 3);
    assert_eq!(qr.r.cols, 5);

    assert_orthogonal(&qr.q, 1e-10);
    assert_upper_trapezoidal(&qr.r, 1e-10);

    let qr_prod = &qr.q * &qr.r;
    assert_matrix_approx_eq(&qr_prod, &a, 1e-8);

    for i in 0..qr.r.rows.min(qr.r.cols) {
        assert!(qr.r[(i, i)] >= -1e-12, "R diag >= 0");
    }
}

#[test]
fn qr_zero_matrix() {
    let a = Matrix::zeros(4, 4);
    let qr = a.qr_decomposition().expect("qr failed");
    assert_orthogonal(&qr.q, 1e-12);
    assert_upper_trapezoidal(&qr.r, 1e-12);

    let qr_prod = &qr.q * &qr.r;
    assert_matrix_approx_eq(&qr_prod, &a, 1e-12);

    for i in 0..4 {
        assert!(approx(qr.r[(i, i)], 0.0, 1e-12));
    }
}
