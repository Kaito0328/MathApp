use crate::{matrix::numerical::EigenDecomposition, Matrix};

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}
#[test]
fn test_eigen_decomposition_diagonal_matrix() {
    // 対角行列: 固有値はそのまま対角成分
    let a = Matrix::new(3, 3, vec![1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0]).unwrap();
    let eig = a.eigen_decomposition().expect("eigendecomp failed");

    // 固有値（順序は保証されないためソートして比較）
    let mut got = eig.eigen_values.clone();
    got.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut exp = [1.0, 2.0, 3.0];
    exp.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for (g, e) in got.iter().zip(exp.iter()) {
        assert!(approx(*g, *e, 1e-10));
    }

    // A V ≈ V Λ を検証
    let v = eig.eigen_vectors;
    let lambda = {
        let mut d = Matrix::zeros(3, 3);
        for i in 0..3 {
            d[(i, i)] = got[i]; //.re は不要
        }
        d
    };

    let lhs = &a * &v;
    let rhs = &v * &lambda;
    for i in 0..3 {
        for j in 0..3 {
            assert!(
                approx(lhs[(i, j)], rhs[(i, j)], 1e-8),
                "AV ≈ VΛ mismatch at ({i},{j})",
            );
        }
    }
}

#[test]
fn test_eigen_decomposition_symmetric_tridiagonal() {
    // A = tridiag with 2 on diagonal and 1 on off-diagonals
    // n=3 の固有値は 2 + 2 cos(kπ/(n+1)), k=1..3
    let a = Matrix::new(3, 3, vec![2.0, 1.0, 0.0, 1.0, 2.0, 1.0, 0.0, 1.0, 2.0]).unwrap();

    let eig = a.eigen_decomposition().expect("eigendecomp failed");
    let mut got = eig.eigen_values.clone(); //.map(...).collect() は不要
    got.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let n = 3.0;
    let mut exp = vec![];
    for k in 1..=3 {
        exp.push(2.0 + 2.0 * (std::f64::consts::PI * (k as f64) / (n + 1.0)).cos());
    }
    exp.sort_by(|x, y| x.partial_cmp(y).unwrap());

    for (g, e) in got.iter().zip(exp.iter()) {
        assert!(approx(*g, *e, 1e-6), "expected {e}, got {g}");
    }
}

#[test]
fn test_eigen_decomposition_nonsymmetric_small() {
    // 非対称 2x2: A = [, [-2, -3]] -> λ = -1, -2
    let a = Matrix::new(2, 2, vec![0.0, 1.0, -2.0, -3.0]).unwrap();
    let eig = a.eigen_decomposition().expect("eigendecomp failed");

    let mut got = eig.eigen_values.clone(); //.map(...).collect() は不要
    got.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let mut exp = [-2.0, -1.0];
    exp.sort_by(|x, y| x.partial_cmp(y).unwrap());

    for (g, e) in got.iter().zip(exp.iter()) {
        assert!(approx(*g, *e, 1e-8), "expected {e}, got {g}");
    }

    println!("Eigenvalues: {got:?}");
    println!("Eigenvectors: {}", eig.eigen_vectors);

    // 近似的に AV ≈ VΛ を確認
    let v = eig.eigen_vectors;
    let lambda = {
        let mut d = Matrix::zeros(2, 2);
        for i in 0..2 {
            d[(i, i)] = got[i];
        }
        d
    };

    let lhs = &a * &v;
    let rhs = &v * &lambda;
    for i in 0..2 {
        for j in 0..2 {
            assert!(approx(lhs[(i, j)], rhs[(i, j)], 1e-6));
        }
    }
}

#[test]
fn test_eigen_decomposition_edge_cases() {
    // 0x0
    let a0 = Matrix::new(0, 0, vec![]).unwrap();
    let e0 = a0.eigen_decomposition().expect("empty OK");
    assert_eq!(e0.eigen_values.len(), 0);
    assert_eq!(e0.eigen_vectors.rows, 0);
    assert_eq!(e0.eigen_vectors.cols, 0);

    // 1x1
    let a1 = Matrix::new(1, 1, vec![7.0]).unwrap();
    let e1 = a1.eigen_decomposition().expect("1x1 OK");
    assert_eq!(e1.eigen_values.len(), 1);
    assert!(approx(e1.eigen_values[0], 7.0, 1e-12)); //.re は不要
    assert_eq!(e1.eigen_vectors.rows, 1);
    assert_eq!(e1.eigen_vectors.cols, 1);
    assert!(approx(e1.eigen_vectors[(0, 0)], 1.0, 1e-12));
}
