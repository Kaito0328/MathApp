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
                "mismatch at ({}, {}): {} vs {}",
                i,
                j,
                a[(i, j)],
                b[(i, j)]
            );
        }
    }
}

fn assert_lower_unit_triangular(l: &Matrix<f64>, tol: f64) {
    assert_eq!(l.rows, l.cols);
    let n = l.rows;
    for i in 0..n {
        for j in 0..n {
            if i < j {
                assert!(
                    l[(i, j)].abs() <= tol,
                    "L should be lower triangular at ({},{}) = {}",
                    i,
                    j,
                    l[(i, j)]
                );
            } else if i == j {
                assert!(
                    approx(l[(i, j)], 1.0, tol),
                    "diag(L) should be 1 at {} got {}",
                    i,
                    l[(i, j)]
                );
            }
        }
    }
}

fn assert_upper_triangular(u: &Matrix<f64>, tol: f64) {
    assert_eq!(u.rows, u.cols);
    let n = u.rows;
    for i in 0..n {
        for j in 0..n {
            if i > j {
                assert!(
                    u[(i, j)].abs() <= tol,
                    "U should be upper triangular at ({},{}) = {}",
                    i,
                    j,
                    u[(i, j)]
                );
            }
        }
    }
}

fn assert_permutation_matrix(p: &Matrix<f64>, tol: f64) {
    assert_eq!(p.rows, p.cols);
    let n = p.rows;
    // entries near 0 or 1
    for i in 0..n {
        for j in 0..n {
            let x = p[(i, j)];
            assert!(
                approx(x, 0.0, tol) || approx(x, 1.0, tol),
                "P entries must be 0/1 near, P({i}, {j}) = {x}",
            );
        }
    }
    // each row sums to 1
    for i in 0..n {
        let s: f64 = (0..n).map(|j| p[(i, j)]).sum();
        assert!(approx(s, 1.0, 1e-10), "row {i} sum = {s}");
    }
    // each column sums to 1
    for j in 0..n {
        let s: f64 = (0..n).map(|i| p[(i, j)]).sum();
        assert!(approx(s, 1.0, 1e-10), "col {j} sum = {s}");
    }
}

#[test]
fn lu_no_pivot_needed_reconstructs() {
    // 先頭列の最大値が先頭行にある -> 交換なし
    let a = Matrix::new(
        3,
        3,
        vec![
            10.0, 2.0, 3.0, //
            1.0, 0.0, 4.0, //
            2.0, 8.0, 2.0,
        ],
    )
    .unwrap();

    let lu = a.lu_decomposition().expect("LU failed");
    assert_lower_unit_triangular(&lu.l, 1e-12);
    assert_upper_triangular(&lu.u, 1e-12);
    assert_permutation_matrix(&lu.p, 1e-12);

    // P*A ≈ L*U
    let pa = &lu.p * &a;
    let lu_mat = &lu.l * &lu.u;
    assert_matrix_approx_eq(&pa, &lu_mat, 1e-10);
}

#[test]
fn lu_with_pivoting_swaps_rows_and_reconstructs() {
    // ピボットが必要 (先頭が0)
    // 期待されるPは [[0,1],[1,0]]
    let a = Matrix::new(2, 2, vec![0.0, 2.0, 1.0, 2.0]).unwrap();

    let lu = a.lu_decomposition().expect("LU failed");
    assert_lower_unit_triangular(&lu.l, 1e-12);
    assert_upper_triangular(&lu.u, 1e-12);
    assert_permutation_matrix(&lu.p, 1e-12);

    // P should be a swap
    let p_exp = Matrix::new(2, 2, vec![0.0, 1.0, 1.0, 0.0]).unwrap();
    assert_matrix_approx_eq(&lu.p, &p_exp, 1e-12);

    // Reconstruct
    let pa = &lu.p * &a;
    let lu_mat = &lu.l * &lu.u;
    assert_matrix_approx_eq(&pa, &lu_mat, 1e-12);
}

#[test]
fn lu_random_like_matrix_reconstructs() {
    let a = Matrix::new(
        5,
        5,
        vec![
            2.0, -1.0, 0.0, 3.0, 1.0, //
            4.0, 1.0, -2.0, 0.5, 2.0, //
            0.0, 3.5, 1.0, -1.0, 0.0, //
            -2.0, 0.0, 1.0, 4.0, -3.0, //
            1.0, 2.0, 0.0, -1.0, 2.5,
        ],
    )
    .unwrap();

    let lu = a.lu_decomposition().expect("LU failed");
    assert_lower_unit_triangular(&lu.l, 1e-10);
    assert_upper_triangular(&lu.u, 1e-10);
    assert_permutation_matrix(&lu.p, 1e-10);

    let pa = &lu.p * &a;
    let lu_mat = &lu.l * &lu.u;
    // 少し緩めの許容誤差
    assert_matrix_approx_eq(&pa, &lu_mat, 1e-8);
}

#[test]
fn lu_non_square_returns_none() {
    let a = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
    assert!(a.lu_decomposition().is_none(), "non-square should be None");
}

#[test]
fn lu_singular_returns_none() {
    // 全ゼロ -> ピボットが見つからず None
    let a = Matrix::zeros(3, 3);
    assert!(a.lu_decomposition().is_none(), "singular should be None");

    // もう1件: 2行目が1行目のスカラー倍でランク落ち
    let a2 = Matrix::new(
        3,
        3,
        vec![
            1.0, 2.0, 3.0, 2.0, 4.0, 6.0, // = 2 * row0
            0.0, 1.0, 1.0,
        ],
    )
    .unwrap();
    // 実装は途中でピボットが非常に小さい場合に None を返す
    assert!(
        a2.lu_decomposition().is_none(),
        "rank-deficient should be None"
    );
}
