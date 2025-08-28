use linalg::{matrix::numerical::Pseudoinverse, Matrix, Result, Vector}; // linalgから必要なものをインポート

/// 線形方程式 Ax=b の「最も良い」解を疑似逆行列を使って求める
pub fn solve_linear_system(a: &Matrix<f64>, b: &Vector<f64>) -> Result<Vector<f64>> {
    // 1. Aの疑似逆行列を計算する
    let a_pinv = a.pinv()?;

    // 2. 解 x = A⁺b を計算する
    let x = &a_pinv * b;

    Ok(x)
}
