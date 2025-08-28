use linalg::{matrix::numerical::SvdDeComposition, Matrix, Result, Vector};

pub fn ridge_regression_optimized(
    a: &Matrix<f64>,
    b: &Vector<f64>,
    alpha: f64,
) -> Result<Vector<f64>> {
    let svd = a.svd()?;

    // 1. U^T * b を計算 (結果はベクトル)
    let ut_b = svd.u.transpose() * b;

    // 2. 対角行列Dを適用 (ベクトルに対する要素ごとの操作)
    let mut d_ut_b = Vector::zeros(svd.sigma.len());
    let alpha_sq = alpha * alpha;
    for i in 0..svd.sigma.len() {
        let s = svd.sigma[i];
        d_ut_b[i] = (s / (s * s + alpha_sq)) * ut_b[i];
    }

    // 3. V * (結果のベクトル) を計算
    Ok(&svd.v * &d_ut_b)
}
