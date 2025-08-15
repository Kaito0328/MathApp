use linalg::{
    matrix::numerical::{svd::SvdDeComposition, EigenDecomposition},
    Matrix,
};

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn svd_vs_eigen_on_ata() {
    // A^T A の固有値は Σ^2 と一致
    let a = Matrix::new(
        4,
        3,
        vec![1.0, 2.0, 0.0, 0.5, -0.5, 3.0, 0.0, 1.0, 1.0, 2.0, 0.0, -1.0],
    )
    .unwrap();
    let svd = a.svd().expect("svd failed");
    let ata = &a.transpose() * &a;
    let eig = ata.eigen_decomposition().expect("eig failed");
    // ソートは実装任せだが、集合として比較（許容誤差内で）
    let mut sig2: Vec<f64> = (0..svd.sigma.dim())
        .map(|i| svd.sigma[i] * svd.sigma[i])
        .collect();
    sig2.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut evals = eig.eigen_values.clone();
    evals.sort_by(|x, y| x.partial_cmp(y).unwrap());
    assert_eq!(sig2.len(), evals.len());
    for (s2, e) in sig2.iter().zip(evals.iter()) {
        assert!(approx(*s2, *e, 1e-6));
    }
}
