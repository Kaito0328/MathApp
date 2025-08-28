use super::*;
use crate::matrix::Matrix;

fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

fn assert_matrix_approx_eq(a: &Matrix<f64>, b: &Matrix<f64>, tol: f64) {
    assert_eq!(a.rows, b.rows, "row mismatch");
    assert_eq!(a.cols, b.cols, "col mismatch");
    for i in 0..a.rows {
        for j in 0..a.cols {
            let da = a[(i, j)];
            let db = b[(i, j)];
            assert!(
                approx_eq(da, db, tol),
                "matrix mismatch at ({i}, {j}): {da} vs {db} (tol {tol})",
            );
        }
    }
}

#[test]
fn test_apply_householder_transform_left_right_full() {
    // A: 3x3
    let a = Matrix::new(
        3,
        3,
        vec![
            1.0, 2.0, 3.0, //
            4.0, 5.0, 6.0, //
            7.0, 8.0, 9.0,
        ],
    )
    .unwrap();

    // u = e1 (単位ベクトル) -> H = I - 2 u u^T = diag(-1, 1, 1)
    let u_vec = crate::Vector::new(vec![1.0, 0.0, 0.0]);

    // 左適用: H * A -> 第0行の符号が反転
    let mut left = a.clone();
    left.apply_householder_transform(&u_vec, Direction::Left, 0, 0);

    let expected_left = Matrix::new(
        3,
        3,
        vec![
            -1.0, -2.0, -3.0, //
            4.0, 5.0, 6.0, //
            7.0, 8.0, 9.0,
        ],
    )
    .unwrap();
    assert_matrix_approx_eq(&left, &expected_left, 1e-12);

    // 右適用: A * H -> 第0列の符号が反転
    let mut right = a.clone();
    right.apply_householder_transform(&u_vec, Direction::Right, 0, 0);

    let expected_right = Matrix::new(
        3,
        3,
        vec![
            -1.0, 2.0, 3.0, //
            -4.0, 5.0, 6.0, //
            -7.0, 8.0, 9.0,
        ],
    )
    .unwrap();
    assert_matrix_approx_eq(&right, &expected_right, 1e-12);
}

#[test]
fn test_apply_householder_transform_partial_region() {
    // 4x4 行列
    let a = Matrix::new(
        4,
        4,
        vec![
            1.0, 2.0, 3.0, 4.0, //
            5.0, 6.0, 7.0, 8.0, //
            9.0, 10.0, 11.0, 12.0, //
            13.0, 14.0, 15.0, 16.0,
        ],
    )
    .unwrap();

    // サブスペース用 u = e1 (サイズ3): rows [1..4) に対応
    // Left: start_row=1, start_col=2 -> 行1の列2以降のみ反転（行0/列<2は無変更）
    let u_sub = crate::Vector::new(vec![1.0, 0.0, 0.0]);

    // 左適用の部分更新
    let mut left_partial = a.clone();
    left_partial.apply_householder_transform(&u_sub, Direction::Left, 1, 2);

    // 期待値を手で組み立て
    let mut expected_left = a.clone();
    for j in 2..4 {
        expected_left[(1, j)] = -expected_left[(1, j)];
    }
    assert_matrix_approx_eq(&left_partial, &expected_left, 1e-12);

    // 右適用の部分更新: start_row=0, start_col=1 -> 列1の行全体に対して反映（列0は無変更）
    let mut right_partial = a.clone();
    right_partial.apply_householder_transform(&u_sub, Direction::Right, 0, 1);

    // 期待値: 列1のみ反転
    let mut expected_right = a.clone();
    for i in 0..4 {
        expected_right[(i, 1)] = -expected_right[(i, 1)];
    }
    assert_matrix_approx_eq(&right_partial, &expected_right, 1e-12);
}

#[test]
fn test_to_hessenberg_properties() {
    // 非対称な 5x5 行列
    let a = Matrix::new(
        5,
        5,
        vec![
            4.0, 1.0, -2.0, 2.0, 3.0, //
            1.0, 2.0, 0.0, 1.0, -1.0, //
            0.0, 3.0, -2.0, 1.0, 4.0, //
            2.0, -1.0, 2.0, 0.0, 1.0, //
            -3.0, 2.0, 1.0, -2.0, 5.0,
        ],
    )
    .unwrap();

    let (h, v) = a.to_hessenberg().expect("to_hessenberg failed");

    // 1) 上ヘッセンベルグ性: i > j + 1 の要素は ~0
    for i in 0..h.rows {
        for j in 0..h.cols {
            if i > j + 1 {
                assert!(
                    h[(i, j)].abs() < 1e-8,
                    "H({},{}) = {} should be ~0 (Hessenberg)",
                    i,
                    j,
                    h[(i, j)]
                );
            }
        }
    }

    // 2) 直交性: V^T V ≈ I
    let vt = v.transpose();
    let vt_v = &vt * &v;
    for i in 0..vt_v.rows {
        for j in 0..vt_v.cols {
            let expected = if i == j { 1.0 } else { 0.0 };
            assert!(
                (vt_v[(i, j)] - expected).abs() < 1e-8,
                "V^T V not orthogonal at ({}, {}): {}",
                i,
                j,
                vt_v[(i, j)]
            );
        }
    }

    // 3) 類似変換の確認: H ≈ V^T A V
    let vtav = &(&v.transpose() * &a) * &v;
    assert_matrix_approx_eq(&h, &vtav, 1e-8);
}

#[test]
fn test_to_hessenberg_trivial_small() {
    // 0x0, 1x1, 2x2 のトリビアルケース検証
    // 1x1
    let a1 = Matrix::new(1, 1, vec![42.0]).unwrap();
    let (h1, v1) = a1.to_hessenberg().expect("hessenberg 1x1");
    assert_matrix_approx_eq(&h1, &a1, 1e-12);
    let i1 = Matrix::identity(1);
    assert_matrix_approx_eq(&v1, &i1, 1e-12);

    // 2x2
    let a2 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    let (h2, v2) = a2.to_hessenberg().expect("hessenberg 2x2");
    // 2x2 は既にヘッセンベルグ
    assert_matrix_approx_eq(&h2, &(&v2.transpose() * &a2 * &v2), 1e-12);
}

// ...existing code...

use num_complex::Complex;

#[test]
fn test_to_complex_converts_values_and_shape() {
    let a = Matrix::new(2, 3, vec![1.0, -2.5, 0.0, 3.0, 4.5, -7.0]).unwrap();
    let ac = a.to_complex();
    assert_eq!(ac.rows, 2);
    assert_eq!(ac.cols, 3);
    for i in 0..2 {
        for j in 0..3 {
            let z = ac[(i, j)];
            assert_eq!(z, Complex::new(a[(i, j)], 0.0));
        }
    }
}

#[test]
fn test_nullspace_vector_known_rank_deficient() {
    // A のランク < n の行列（2行はスカラー倍）
    // x + 2y - z = 0 を満たす。例: [1, 0, 1] や [0, 1, 2] など。
    let a = Matrix::new(
        2,
        3,
        vec![
            1.0, 2.0, -1.0, //
            2.0, 4.0, -2.0,
        ],
    )
    .unwrap();

    let x = a.nullspace_vector(1e-12).expect("should have nullspace");
    // Ax ≈ 0
    let ax = &a * &x;
    for i in 0..ax.dim() {
        assert!(ax[i].abs() < 1e-9, "Ax not near zero at {}: {}", i, ax[i]);
    }
    // 正規化されている
    let nrm = x.norm();
    assert!((nrm - 1.0).abs() < 1e-9, "not normalized: {nrm}");

    // 最初の非零成分が正（符号安定化の確認）
    for i in 0..x.dim() {
        if x[i].abs() > 1e-10 {
            assert!(x[i] > 0.0, "first nonzero should be positive, got {}", x[i]);
            break;
        }
    }
}

#[test]
fn test_nullspace_vector_full_rank_returns_none() {
    let a = Matrix::new(
        3,
        3,
        vec![
            2.0, 1.0, 0.0, //
            -1.0, 3.0, 1.0, //
            0.0, 2.0, 1.0,
        ],
    )
    .unwrap();

    assert!(
        a.nullspace_vector(1e-12).is_none(),
        "full rank should have no nullspace"
    );
}
