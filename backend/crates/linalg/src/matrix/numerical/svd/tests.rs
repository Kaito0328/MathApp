use num_traits::Float;

use crate::matrix::{
    numerical::{svd::Svd, SvdDeComposition},
    Matrix,
};

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}
fn assert_orthogonal(a: &Matrix<f64>, tol: f64, msg: &str) {
    let identity = Matrix::identity(a.cols);
    let a_t_a = &a.transpose() * a;
    assert_matrix_approx_eq(&identity, &a_t_a, tol, msg);
}

/// 2つの行列が近似的に等しいかをチェックする
fn assert_matrix_approx_eq(a: &Matrix<f64>, b: &Matrix<f64>, tol: f64, msg: &str) {
    assert_eq!(a.rows, b.rows, "Matrix dimensions differ (rows)");
    assert_eq!(a.cols, b.cols, "Matrix dimensions differ (cols)");
    for i in 0..a.rows {
        for j in 0..a.cols {
            if (a[(i, j)] - b[(i, j)]).abs() > tol {
                panic!(
                    "{}: mismatch at ({}, {}): {} vs {} (tol={})",
                    msg,
                    i,
                    j,
                    a[(i, j)],
                    b[(i, j)],
                    tol
                );
            }
        }
    }
}

/// SVDの結果を検証するためのヘルパー関数
fn validate_svd(a: &Matrix<f64>, svd_result: &Svd) {
    let u = &svd_result.u;
    let sigma_vec = &svd_result.sigma;
    let v = &svd_result.v;

    // 1. UとVの直交性をチェック
    assert_orthogonal(u, 1e-10, "U is not orthogonal");
    assert_orthogonal(v, 1e-10, "V is not orthogonal");

    // 2. 特異値が非負で降順にソートされているかチェック
    for i in 0..sigma_vec.dim() - 1 {
        assert!(
            sigma_vec[i] >= sigma_vec[i + 1],
            "Sigma is not sorted descending"
        );
        assert!(sigma_vec[i + 1] >= -1e-12, "A singular value is negative");
    }
    if sigma_vec.dim() > 0 {
        assert!(
            sigma_vec[0] >= -1e-12,
            "The first singular value is negative"
        );
    }

    // 3. 元の行列Aを復元して比較
    let mut s_mat = Matrix::zeros(u.cols, v.cols);
    for i in 0..sigma_vec.dim() {
        s_mat[(i, i)] = sigma_vec[i];
    }

    let a_reconstructed = &(u * &s_mat) * &v.transpose();

    assert_matrix_approx_eq(a, &a_reconstructed, 1e-9, "A ≈ UΣV^T");
}

fn build_bidiagonal(d: &[f64], e: &[f64]) -> Matrix<f64> {
    let n = d.len();
    let mut b = Matrix::zeros(n, n);
    for i in 0..n {
        b[(i, i)] = d[i];
        if i + 1 < n {
            b[(i, i + 1)] = e[i];
        }
    }
    b
}

#[test]
fn test_calculate_givens_params_properties() {
    let cases = vec![
        (3.0, 4.0),
        (0.0, 5.0),
        (5.0, 0.0),
        (1e-12, 1e-9),
        (1e9, -1e8),
    ];
    for (a, b) in cases {
        let (c, s) = Matrix::<f64>::calculate_givens_params(a, b);
        let norm_cs = c * c + s * s;
        let r = c * a - s * b;
        let z = s * a + c * b;
        println!("a={a}, b={b}, c={c}, s={s}, r={r}, z={z}, c^2+s^2={norm_cs}");
        assert!(approx(norm_cs, 1.0, 1e-12));
        assert!(z.abs() < 1e-10, "second component not ~0");
        // r は ||(a,b)|| に一致（符号は定義に依存）
        assert!(approx(r.abs(), a.hypot(b), 1e-10));
    }
}

#[test]
fn test_apply_right_givens_rotation_columns() {
    let mut m = Matrix::new(
        3,
        3,
        vec![
            1.0, 2.0, 3.0, //
            4.0, 5.0, 6.0, //
            7.0, 8.0, 9.0,
        ],
    )
    .unwrap();
    let c = 0.6;
    let s = 0.8;

    let before_c1 = (0..m.rows).map(|i| m[(i, 1)]).collect::<Vec<_>>();
    let before_c2 = (0..m.rows).map(|i| m[(i, 2)]).collect::<Vec<_>>();

    m.apply_right_givens_rotation(1, 2, c, s);

    for i in 0..m.rows {
        let exp1 = c * before_c1[i] - s * before_c2[i];
        let exp2 = s * before_c1[i] + c * before_c2[i];
        println!(
            "row={i}, exp1={exp1}, got1={}, exp2={exp2}, got2={}",
            m[(i, 1)],
            m[(i, 2)]
        );
        assert!(approx(m[(i, 1)], exp1, 1e-12));
        assert!(approx(m[(i, 2)], exp2, 1e-12));
    }
}

#[test]
fn test_apply_left_givens_rotation_columns() {
    let mut u = Matrix::new(
        3,
        3,
        vec![
            3.0, 1.0, -1.0, //
            -2.0, 0.5, 4.0, //
            1.0, -3.0, 2.0,
        ],
    )
    .unwrap();
    let c = 0.8;
    let s = -0.6;

    let before_c0 = (0..u.rows).map(|i| u[(i, 0)]).collect::<Vec<_>>();
    let before_c2 = (0..u.rows).map(|i| u[(i, 2)]).collect::<Vec<_>>();

    u.apply_left_givens_rotation(0, 2, c, s);

    for i in 0..u.rows {
        let exp0 = c * before_c0[i] - s * before_c2[i];
        let exp2 = s * before_c0[i] + c * before_c2[i];
        println!(
            "row={i}, exp0={exp0}, got0={}, exp2={exp2}, got2={}",
            u[(i, 0)],
            u[(i, 2)]
        );
        assert!(approx(u[(i, 0)], exp0, 1e-12));
        assert!(approx(u[(i, 2)], exp2, 1e-12));
    }
}

// qr_step は削除したため、その専用テストも削除

#[test]
fn test_chase_zero_off_diagonal_preserves_relation_and_sets_zero() {
    let mut d = vec![2.0, -1.0, 0.5];
    let mut e = vec![0.25, 0.1];
    let b0 = build_bidiagonal(&d, &e);
    let mut v = Matrix::identity(3);

    Matrix::<f64>::chase_zero_off_diagonal(0, 2, &mut d, &mut e, &mut v).expect("chase failed");

    // e[0] は明示的に 0 に設定される
    println!("e after chase: {e:?}");
    assert!(approx(e[0], 0.0, 1e-15));

    println!("d after chase: {d:?}");
    println!("v after chase: {v:?}");

    // 直交性
    assert_orthogonal(&v, 1e-12, "V is not orthogonal");

    // 関係式: 右回転のみでは厳密な双対角を維持しないため、
    // B0*V の対角・超対角成分が b1 と一致することを確認する。
    let b1 = build_bidiagonal(&d, &e);
    let b0v = &b0 * &v;

    println!("B1 = {b1}");
    println!("B0 * V = {b0v}");
    for i in 0..b1.rows {
        // 対角
        assert!(
            approx(b1[(i, i)], b0v[(i, i)], 1e-8),
            "diag mismatch at {i}"
        );
        // 超対角
        if i + 1 < b1.cols {
            assert!(
                approx(b1[(i, i + 1)], b0v[(i, i + 1)], 1e-8),
                "superdiag mismatch at {i}"
            );
        }
    }
}

#[test]
fn test_bidiagonalize_gives_bidiagonal_and_relation() {
    // 縦長
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

    let (b, u, v) = a.bidiagonalize().expect("bidiagonalize failed");
    println!("B=\n{b}");
    assert_orthogonal(&u, 1e-10, "U is not orthogonal");
    assert_orthogonal(&v, 1e-10, "V is not orthogonal");

    // 双対角性（対角と超対角以外は ~0）
    for i in 0..b.rows {
        for j in 0..b.cols {
            if !(i == j || (i + 1 == j)) {
                assert!(b[(i, j)].abs() < 1e-8, "B({}, {}) = {}", i, j, b[(i, j)]);
            }
        }
    }

    // B ≈ U^T A V
    let utav = &(&u.transpose() * &a) * &v;
    assert_matrix_approx_eq(&b, &utav, 1e-8, "B ≈ U^T A V");
}

#[test]
fn test_solve_bidiagonal_svd_diagonalizes_and_reconstructs() {
    // 適当な A を双対角化 -> 双対角 SVD を解く
    let a = Matrix::new(
        4,
        3,
        vec![
            1.0, 2.0, 0.0, //
            0.5, -0.5, 3.0, //
            0.0, 1.0, 1.0, //
            2.0, 0.0, -1.0,
        ],
    )
    .unwrap();

    let (mut b, mut u, mut v) = a.bidiagonalize().expect("bidiagonalize failed");

    println!("B before SVD:\n{b}");
    println!("U before SVD:\n{u}");
    println!("V before SVD:\n{v}");
    Matrix::<f64>::solve_bidiagonal_svd(&mut b, &mut u, &mut v)
        .expect("solve_bidiagonal_svd failed");

    println!("B after SVD:\n{b}");
    println!("U after SVD:\n{u}");
    println!("V after SVD:\n{v}");

    // b は（ほぼ）対角で対角は非負
    let dsz = b.rows.min(b.cols);
    for i in 0..dsz {
        for j in 0..b.cols {
            if i != j {
                assert!(b[(i, j)].abs() < 1e-8, "off diag not ~0 at ({i},{j})");
            }
        }
        assert!(b[(i, i)] >= -1e-12, "diag should be >= 0 at {i}");
    }

    // 直交性
    assert_orthogonal(&u, 1e-10, "U is not orthogonal");
    assert_orthogonal(&v, 1e-10, "V is not orthogonal");

    // 復元誤差
    let mut s_mat = Matrix::zeros(u.cols, v.cols);
    for i in 0..dsz {
        s_mat[(i, i)] = b[(i, i)];
    }
    let a_hat = &(&u * &s_mat) * &v.transpose();

    let mut max_err = 0.0;
    for i in 0..a.rows {
        for j in 0..a.cols {
            max_err = max_err.max((a[(i, j)] - a_hat[(i, j)]).abs());
        }
    }
    println!("reconstruction max_err={max_err}");
    assert!(max_err < 1e-6);
}

#[test]
fn test_svd_for_various_matrix_shapes() {
    // --- ケース1: 正方行列 ---
    let a_square = Matrix::new(
        3,
        3,
        vec![
            4.0, 1.0, -2.0, //
            2.0, 5.0, 1.0, //
            -1.0, 0.0, 6.0,
        ],
    )
    .unwrap();
    println!("Testing SVD for a square matrix...");
    let svd_square = a_square.svd().expect("SVD failed for square matrix");
    validate_svd(&a_square, &svd_square);
    println!("Square matrix test PASSED.");

    // --- ケース2: 縦長の行列 (rows > cols) ---
    let a_tall = Matrix::new(
        4,
        3,
        vec![
            1.0, 2.0, 3.0, //
            4.0, 5.0, 6.0, //
            7.0, 8.0, 9.0, //
            10.0, 11.0, 12.0,
        ],
    )
    .unwrap();
    println!("\nTesting SVD for a tall matrix...");
    let svd_tall = a_tall.svd().expect("SVD failed for tall matrix");
    validate_svd(&a_tall, &svd_tall);
    println!("Tall matrix test PASSED.");

    // --- ケース3: 横長の行列 (rows < cols) ---
    let a_wide = Matrix::new(
        3,
        4,
        vec![
            1.0, 2.0, 3.0, 4.0, //
            5.0, 6.0, 7.0, 8.0, //
            9.0, 10.0, 11.0, 12.0,
        ],
    )
    .unwrap();
    println!("\nTesting SVD for a wide matrix...");
    let svd_wide = a_wide.svd().expect("SVD failed for wide matrix");
    validate_svd(&a_wide, &svd_wide);
    println!("Wide matrix test PASSED.");

    // --- ケース4: ランク落ちした行列 ---
    let a_rank_deficient = Matrix::new(
        3,
        3,
        vec![
            1.0, 2.0, 3.0, //
            2.0, 4.0, 6.0, // (row 1 * 2)
            3.0, 6.0, 9.0, // (row 1 * 3)
        ],
    )
    .unwrap();
    println!("\nTesting SVD for a rank-deficient matrix...");
    let svd_rank_deficient = a_rank_deficient
        .svd()
        .expect("SVD failed for rank-deficient matrix");
    validate_svd(&a_rank_deficient, &svd_rank_deficient);
    // ランクが1なので、2番目以降の特異値はほぼ0になるはず
    assert!(svd_rank_deficient.sigma[1].abs() < 1e-9);
    assert!(svd_rank_deficient.sigma[2].abs() < 1e-9);
    println!("Rank-deficient matrix test PASSED.");
}

#[test]
fn test_svd_zero_and_identity_matrices() {
    // 単位行列: Σ は全て 1、再構成は I
    let i4 = Matrix::identity(4);
    let svd_i4 = i4.svd().expect("SVD failed for identity");
    assert_orthogonal(&svd_i4.u, 1e-12, "U not orthogonal for I");
    assert_orthogonal(&svd_i4.v, 1e-12, "V not orthogonal for I");
    for k in 0..4 {
        assert!(approx(svd_i4.sigma[k], 1.0, 1e-12), "sigma[{k}] != 1 for I");
    }
    let mut s = Matrix::zeros(4, 4);
    for k in 0..4 {
        s[(k, k)] = svd_i4.sigma[k];
    }
    let recon = &(&svd_i4.u * &s) * &svd_i4.v.transpose();
    assert_matrix_approx_eq(&i4, &recon, 1e-12, "I ≈ UΣV^T");

    // ゼロ行列: Σ は全て 0、再構成は 0
    let zero = Matrix::zeros(3, 2);
    let svd_zero = zero.svd().expect("SVD failed for zero matrix");
    assert_orthogonal(&svd_zero.u, 1e-12, "U not orthogonal for zero");
    assert_orthogonal(&svd_zero.v, 1e-12, "V not orthogonal for zero");
    for k in 0..svd_zero.sigma.dim() {
        assert!(
            approx(svd_zero.sigma[k], 0.0, 1e-14),
            "sigma[{k}] != 0 for zero"
        );
    }
    let mut s0 = Matrix::zeros(svd_zero.u.cols, svd_zero.v.cols);
    for k in 0..svd_zero.sigma.dim() {
        s0[(k, k)] = svd_zero.sigma[k];
    }
    let recon0 = &(&svd_zero.u * &s0) * &svd_zero.v.transpose();
    assert_matrix_approx_eq(&zero, &recon0, 1e-12, "0 ≈ UΣV^T");
}

#[test]
fn test_svd_output_dimensions() {
    // 縦長 m>n -> U は m×m、V は n×n、sigma は n
    let a_tall = Matrix::new(
        5,
        3,
        vec![
            2.0, -1.0, 0.0, 4.0, 1.0, -2.0, 0.0, 3.5, 1.0, -2.0, 0.0, 1.0, 1.0, 2.0, 0.0,
        ],
    )
    .unwrap();
    let svd_tall = a_tall.svd().expect("SVD failed for tall");
    assert_eq!(svd_tall.u.rows, 5);
    assert_eq!(svd_tall.u.cols, 5);
    assert_eq!(svd_tall.v.rows, 3);
    assert_eq!(svd_tall.v.cols, 3);
    assert_eq!(svd_tall.sigma.dim(), 3);

    // 横長 m<n -> U は m×m、V は n×n、sigma は m
    let a_wide = Matrix::new(2, 4, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]).unwrap();
    let svd_wide = a_wide.svd().expect("SVD failed for wide");
    assert_eq!(svd_wide.u.rows, 2);
    assert_eq!(svd_wide.u.cols, 2);
    assert_eq!(svd_wide.v.rows, 4);
    assert_eq!(svd_wide.v.cols, 4);
    assert_eq!(svd_wide.sigma.dim(), 2);
}

#[test]
fn test_svd_sign_flip_invariance() {
    let a = Matrix::new(3, 2, vec![1.0, -2.0, 0.5, 3.0, -1.5, 0.0]).unwrap();
    let svd_a = a.svd().expect("SVD failed for A");
    let svd_neg_a = (&a * -1.0).svd().expect("SVD failed for -A");
    assert_eq!(svd_a.sigma.dim(), svd_neg_a.sigma.dim());
    for k in 0..svd_a.sigma.dim() {
        assert!(
            approx(svd_a.sigma[k], svd_neg_a.sigma[k], 1e-12),
            "sigma differs at {k}"
        );
    }
}

#[test]
fn test_svd_matches_simple_svd_singular_values() {
    // いくつかの形状で svd() と simple_svd() の特異値が一致すること
    let cases = vec![
        Matrix::new(3, 3, vec![4.0, 1.0, -2.0, 2.0, 5.0, 1.0, -1.0, 0.0, 6.0]).unwrap(),
        Matrix::new(4, 2, vec![1.0, 2.0, 0.0, -1.0, 3.0, 1.0, 2.0, 0.0]).unwrap(),
        Matrix::new(2, 4, vec![1.0, 2.0, 3.0, 4.0, 4.0, 3.0, 2.0, 1.0]).unwrap(),
    ];
    for (idx, a) in cases.into_iter().enumerate() {
        let svd1 = a.svd().expect("svd() failed");
        let svd2 = a.simple_svd().expect("simple_svd() failed");
        assert_eq!(
            svd1.sigma.dim(),
            svd2.sigma.dim(),
            "sigma dim mismatch at case {idx}"
        );
        for k in 0..svd1.sigma.dim() {
            assert!(
                approx(svd1.sigma[k], svd2.sigma[k], 1e-8),
                "sigma mismatch at case {idx}, k={k}"
            );
        }
        // 再構成の確認（svd のみ）
        let mut s = Matrix::zeros(svd1.u.cols, svd1.v.cols);
        for k in 0..svd1.sigma.dim() {
            s[(k, k)] = svd1.sigma[k];
        }
        let recon = &(&svd1.u * &s) * &svd1.v.transpose();
        assert_matrix_approx_eq(&a, &recon, 1e-8, &format!("recon failed at case {idx}"));
    }
}
