use num_complex::Complex;

use crate::{
    matrix::numerical::{EigenDecomposition, QrDecomposition},
    Matrix, Vector,
};

fn capprox(a: Complex<f64>, b: Complex<f64>, tol: f64) -> bool {
    (a - b).norm() <= tol
}

fn vnorm(v: &Vector<Complex<f64>>) -> f64 {
    v.data.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt()
}

fn make_lambda(eigs: &[Complex<f64>]) -> Matrix<Complex<f64>> {
    let n = eigs.len();
    let mut d = Matrix::zeros(n, n);
    for i in 0..n {
        d[(i, i)] = eigs[i];
    }
    d
}

#[test]
fn test_eigen_decomposition_complex_rotation_2x2() {
    // 回転行列 [[0, -1],[1, 0]] -> 固有値は ±i
    let a = Matrix::new(2, 2, vec![0.0, -1.0, 1.0, 0.0]).unwrap();
    let res = a.eigen_decomposition_complex().expect("complex eig failed");

    // 固有値のチェック（順序は未定なのでソート）
    let mut got = res.eigen_values.clone();
    got.sort_by(|x, y| {
        x.re.partial_cmp(&y.re)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(x.im.partial_cmp(&y.im).unwrap_or(std::cmp::Ordering::Equal))
    });
    assert!(capprox(got[0], Complex::new(0.0, -1.0), 1e-8));
    assert!(capprox(got[1], Complex::new(0.0, 1.0), 1e-8));

    // A v ≈ λ v を各列で検証
    let a_c = a.to_complex();
    for (j, eigenvalue) in got.iter().enumerate() {
        let vj = res.eigen_vectors.col(j).unwrap();
        let av = &a_c * &vj;
        let lv = &vj * eigenvalue; // got[j] の代わりに enumerate で得た値を使用
        let err = vnorm(&(&av - &lv));
        assert!(err < 1e-6, "residual too large: {err}");
    }
}

#[test]
fn test_eigen_decomposition_complex_block_diag_rot_plus_scalar() {
    // A = blockdiag([[0,-1],[1,0]], [2]) -> 固有値 {±i, 2}
    let a = Matrix::new(
        3,
        3,
        vec![
            0.0, -1.0, 0.0, //
            1.0, 0.0, 0.0, //
            0.0, 0.0, 2.0,
        ],
    )
    .unwrap();

    let res = a.eigen_decomposition_complex().expect("complex eig failed");

    // 固有値集合の一致を確認
    let mut got = res.eigen_values.clone();
    got.sort_by(|x, y| {
        x.re.partial_cmp(&y.re)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(x.im.partial_cmp(&y.im).unwrap_or(std::cmp::Ordering::Equal))
    });
    let mut exp = [
        Complex::new(0.0, -1.0),
        Complex::new(0.0, 1.0),
        Complex::new(2.0, 0.0),
    ];
    exp.sort_by(|x, y| {
        x.re.partial_cmp(&y.re)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(x.im.partial_cmp(&y.im).unwrap_or(std::cmp::Ordering::Equal))
    });
    for (g, e) in got.iter().zip(exp.iter()) {
        assert!(capprox(*g, *e, 1e-8), "expected {e}, got {g}");
    }

    // AV ≈ VΛ の残差チェック
    let a_c = a.to_complex();
    let v = res.eigen_vectors;
    let lambda = make_lambda(&got);
    let lhs = &a_c * &v;
    let rhs = &v * &lambda;
    for i in 0..3 {
        for j in 0..3 {
            assert!(
                (lhs[(i, j)] - rhs[(i, j)]).norm() < 1e-6,
                "AV ≈ VΛ mismatch at ({i},{j})",
            );
        }
    }
}

#[test]
fn test_extract_eigenvalues_from_schur() {
    // 準上三角 (実シュア形式) を手で構成：上左に 2x2 回転ブロック、右下に 1x1
    let t = Matrix::new(
        3,
        3,
        vec![
            0.0, -1.0, 0.0, //
            1.0, 0.0, 0.0, //
            0.0, 0.0, 2.0,
        ],
    )
    .unwrap();

    let eigs = Matrix::<f64>::extract_eigenvalues_from_schur(&t, 1e-12);
    let mut got = eigs.clone();
    got.sort_by(|x, y| {
        x.re.partial_cmp(&y.re)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(x.im.partial_cmp(&y.im).unwrap_or(std::cmp::Ordering::Equal))
    });

    let mut exp = [
        Complex::new(0.0, -1.0),
        Complex::new(0.0, 1.0),
        Complex::new(2.0, 0.0),
    ];
    exp.sort_by(|x, y| {
        x.re.partial_cmp(&y.re)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(x.im.partial_cmp(&y.im).unwrap_or(std::cmp::Ordering::Equal))
    });

    for (g, e) in got.iter().zip(exp.iter()) {
        assert!(capprox(*g, *e, 1e-10));
    }
}

#[test]
fn test_qr_iteration_to_schur_keeps_quasi_upper_triangular() {
    // 既に準上三角の行列（2x2 回転ブロック + 1x1）
    let mut h = Matrix::new(
        3,
        3,
        vec![
            0.0, -1.0, 0.0, //
            1.0, 0.0, 0.0, //
            0.0, 0.0, 2.0,
        ],
    )
    .unwrap();
    let mut q = Matrix::identity(3);

    let ok = Matrix::<f64>::qr_iteration_to_schur(&mut h, &mut q, 1e-12);
    assert!(ok, "QR iteration did not converge");

    // 準上三角性: 対角より2つ以上下は ~0
    for i in 0..h.rows {
        for j in 0..h.cols {
            if i > j + 1 {
                assert!(h[(i, j)].abs() < 1e-10, "H({i},{j}) not ~0");
            }
        }
    }
}

#[test]
fn test_compute_schur_eigenvectors_satisfy_relation() {
    // T は実シュア形式（2x2 複素対ブロック + 1x1 実固有値）
    let t = Matrix::new(
        3,
        3,
        vec![
            0.0, -1.0, 0.0, //
            1.0, 0.0, 0.0, //
            0.0, 0.0, 2.0,
        ],
    )
    .unwrap();
    let eigs = Matrix::<f64>::extract_eigenvalues_from_schur(&t, 1e-12);
    let y = Matrix::<f64>::compute_schur_eigenvectors(&t, &eigs).expect("Y compute failed");

    // T_c Y ≈ Y Λ を確認
    let t_c = t.to_complex();
    let lambda = make_lambda(&eigs);
    let lhs = &t_c * &y;
    let rhs = &y * &lambda;

    for i in 0..3 {
        for j in 0..3 {
            assert!(
                (lhs[(i, j)] - rhs[(i, j)]).norm() < 1e-6,
                "T Y ≈ Y Λ mismatch at ({i},{j})",
            );
        }
    }
}

#[test]
fn test_eigen_decomposition_dense_4x4_mixed_eigenvalues() {
    // このテストケースは、意図的に{1±2i, 3, -4}という固有値を持つように
    // 相似変換を用いて作成された、4x4の密な非対称行列を使用します。
    // これにより、ヘッセンベルグ化、QR反復、後退代入のすべてのステップが
    // より複雑な状況で正しく連携して動作するかを検証します。
    // 期待固有値 {1±2i, 3, -4} を持つブロック対角 D を作り、
    // 直交行列 Q による相似変換 A = Q D Q^T で密行列を作る
    let mut d = Matrix::zeros(4, 4);
    // 2x2 複素対ブロック (1±2i) に対応する実ブロック [[1,-2],[2,1]]
    d[(0, 0)] = 1.0;
    d[(0, 1)] = -2.0;
    d[(1, 0)] = 2.0;
    d[(1, 1)] = 1.0;
    d[(2, 2)] = 3.0;
    d[(3, 3)] = -4.0;
    // 適当なフルランク行列からQRで直交行列Qを得る
    let r_for_q = Matrix::new(
        4,
        4,
        vec![
            0.0, 0.0, 3.0, -4.0, -2.0, 5.0, -2.0, 2.0, -6.0, 9.0, -1.0, 2.0, 1.0, 2.0, 0.0, -1.0,
        ],
    )
    .unwrap();
    let q = r_for_q.qr_decomposition().unwrap().q;
    let a = &(&q * &d) * &q.transpose();

    let res = a
        .eigen_decomposition_complex()
        .expect("complex eig failed on 4x4 dense matrix");

    // --- 固有値のチェック ---
    // 実装はソート済みの固有値を返すため、期待値もソートして比較します。
    let got_eigs = &res.eigen_values;
    let mut exp_eigs = [
        Complex::new(1.0, 2.0),
        Complex::new(1.0, -2.0),
        Complex::new(3.0, 0.0),
        Complex::new(-4.0, 0.0),
    ];

    // 実装と同じ順序（実部→虚部）で期待値をソート
    exp_eigs.sort_by(|a, b| {
        a.re.partial_cmp(&b.re)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(a.im.partial_cmp(&b.im).unwrap_or(std::cmp::Ordering::Equal))
    });

    assert_eq!(
        got_eigs.len(),
        4,
        "Incorrect number of eigenvalues returned"
    );
    for (g, e) in got_eigs.iter().zip(exp_eigs.iter()) {
        assert!(
            capprox(*g, *e, 1e-8),
            "Eigenvalue mismatch: expected {e}, got {g}"
        );
    }

    // --- 固有システム全体の残差チェック (AV = VΛ) ---
    // このチェックは、固有値と固有ベクトルの関係が全体として
    // 正しく満たされているかを確認する、最も包括的なテストです。
    let a_c = a.to_complex();
    let v = &res.eigen_vectors;
    // `make_lambda`はテストスイート内のヘルパー関数を想定
    let lambda = make_lambda(got_eigs);

    let lhs = &a_c * v;
    let rhs = v * &lambda;
    let residual_matrix = &lhs - &rhs;

    // 残差行列のフロベニウスノルムを計算
    let residual_norm = residual_matrix
        .data
        .iter()
        .map(|z| z.norm_sqr())
        .sum::<f64>()
        .sqrt();

    assert!(
        residual_norm < 1e-6,
        "High residual for the full eigensystem: ||AV - VΛ||_F = {residual_norm}",
    );
}

#[test]
fn test_characteristic_polynomial_coeffs_for_4x4_case() {
    // 期待固有値: {1±2i, 3, -4} のとき特性多項式は
    // λ^4 - λ^3 - 9 λ^2 + 29 λ - 60
    // 上のテストと同じ生成方法でAを作る
    let mut d = Matrix::zeros(4, 4);
    d[(0, 0)] = 1.0;
    d[(0, 1)] = -2.0;
    d[(1, 0)] = 2.0;
    d[(1, 1)] = 1.0;
    d[(2, 2)] = 3.0;
    d[(3, 3)] = -4.0;
    let r_for_q = Matrix::new(
        4,
        4,
        vec![
            0.0, 0.0, 3.0, -4.0, -2.0, 5.0, -2.0, 2.0, -6.0, 9.0, -1.0, 2.0, 1.0, 2.0, 0.0, -1.0,
        ],
    )
    .unwrap();
    let q = r_for_q.qr_decomposition().unwrap().q;
    let a = &(&q * &d) * &q.transpose();
    let coeffs = Matrix::<f64>::characteristic_polynomial_coeffs(&a);
    assert_eq!(coeffs.len(), 5);
    let exp = [1.0, -1.0, -9.0, 29.0, -60.0];
    for (i, (&c, &e)) in coeffs.iter().zip(exp.iter()).enumerate() {
        assert!(
            (c - e).abs() < 1e-6,
            "coeff[{i}] mismatch: got {c}, exp {e}"
        );
    }
}

#[test]
fn test_eigen_decomposition_complex_5x5_two_pairs_plus_real() {
    // 期待固有値: { (1±2i), (-3±1i), 4 }
    let mut d = Matrix::zeros(5, 5);
    // 実2x2ブロック for 1±2i
    d[(0, 0)] = 1.0;
    d[(0, 1)] = -2.0;
    d[(1, 0)] = 2.0;
    d[(1, 1)] = 1.0;
    // 実2x2ブロック for -3±1i -> [[-3, -1],[1, -3]]
    d[(2, 2)] = -3.0;
    d[(2, 3)] = -1.0;
    d[(3, 2)] = 1.0;
    d[(3, 3)] = -3.0;
    // 実固有値 4
    d[(4, 4)] = 4.0;

    // 適当なフルランク行列からQRで Q を作る
    let base = Matrix::new(
        5,
        5,
        vec![
            2.0, -1.0, 0.0, 3.0, -2.0, 0.5, 4.0, -2.0, 1.0, 0.0, -1.0, 2.0, 5.0, -3.0, 1.0, 3.0,
            0.0, -1.0, 2.0, -4.0, -2.0, 1.0, 0.0, -1.0, 3.0,
        ],
    )
    .unwrap();
    let q = base.qr_decomposition().unwrap().q;
    let a = &(&q * &d) * &q.transpose();

    let res = a
        .eigen_decomposition_complex()
        .expect("complex eig failed on 5x5");

    // 固有値チェック（実装のソート規則に合わせる）
    let got = &res.eigen_values;
    let mut exp = [
        Complex::new(1.0, 2.0),
        Complex::new(1.0, -2.0),
        Complex::new(-3.0, 1.0),
        Complex::new(-3.0, -1.0),
        Complex::new(4.0, 0.0),
    ];
    exp.sort_by(|a, b| {
        a.re.partial_cmp(&b.re)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(a.im.partial_cmp(&b.im).unwrap_or(std::cmp::Ordering::Equal))
    });
    assert_eq!(got.len(), 5);
    for (g, e) in got.iter().zip(exp.iter()) {
        assert!(
            capprox(*g, *e, 1e-6),
            "eigenvalue mismatch: expected {e}, got {g}"
        );
    }

    // AV ≈ VΛ の残差
    let a_c = a.to_complex();
    let v = &res.eigen_vectors;
    let lambda = make_lambda(got);
    let resid = &(&a_c * v) - &(v * &lambda);
    let frob = resid.data.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
    assert!(frob < 1e-6, "residual too large: {frob}");
}

#[test]
fn test_eigen_decomposition_complex_7x7_multiple_pairs_and_reals() {
    // 期待固有値: {(2±3i), (-1±2i), (0±1i), 5}
    let mut d = Matrix::zeros(7, 7);
    // 2±3i -> [[2, -3],[3, 2]]
    d[(0, 0)] = 2.0;
    d[(0, 1)] = -3.0;
    d[(1, 0)] = 3.0;
    d[(1, 1)] = 2.0;
    // -1±2i -> [[-1, -2],[2, -1]]
    d[(2, 2)] = -1.0;
    d[(2, 3)] = -2.0;
    d[(3, 2)] = 2.0;
    d[(3, 3)] = -1.0;
    // 0±1i -> [[0, -1],[1, 0]]
    d[(4, 4)] = 0.0;
    d[(4, 5)] = -1.0;
    d[(5, 4)] = 1.0;
    d[(5, 5)] = 0.0;
    // 実固有値 5
    d[(6, 6)] = 5.0;

    let base = Matrix::new(
        7,
        7,
        vec![
            1.0, -2.0, 3.0, 0.0, -1.0, 2.0, -3.0, 0.0, 4.0, -1.0, 2.0, 3.0, -2.0, 1.0, -2.0, 1.0,
            0.0, -3.0, 2.0, 1.0, -1.0, 3.0, 0.0, -2.0, 1.0, -4.0, 0.5, 2.0, -1.0, 2.0, 1.0, -2.0,
            0.0, 3.0, -2.0, 2.0, -1.0, 0.5, 1.0, -2.0, 4.0, 0.0, -3.0, 1.0, -1.0, 2.0, -2.0, 0.0,
            5.0,
        ],
    )
    .unwrap();
    let q = base.qr_decomposition().unwrap().q;
    let a = &(&q * &d) * &q.transpose();

    let res = a
        .eigen_decomposition_complex()
        .expect("complex eig failed on 7x7");

    let got = &res.eigen_values;
    let mut exp = [
        Complex::new(2.0, 3.0),
        Complex::new(2.0, -3.0),
        Complex::new(-1.0, 2.0),
        Complex::new(-1.0, -2.0),
        Complex::new(0.0, 1.0),
        Complex::new(0.0, -1.0),
        Complex::new(5.0, 0.0),
    ];
    exp.sort_by(|a, b| {
        a.re.partial_cmp(&b.re)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(a.im.partial_cmp(&b.im).unwrap_or(std::cmp::Ordering::Equal))
    });
    assert_eq!(got.len(), 7);
    for (g, e) in got.iter().zip(exp.iter()) {
        assert!(
            capprox(*g, *e, 5e-6),
            "eigenvalue mismatch: expected {e}, got {g}"
        );
    }

    let a_c = a.to_complex();
    let v = &res.eigen_vectors;
    let lambda = make_lambda(got);
    let resid = &(&a_c * v) - &(v * &lambda);
    let frob = resid.data.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
    assert!(frob < 5e-6, "residual too large: {frob}");
}
