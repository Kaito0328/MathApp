use linalg::{Matrix, Vector};
use statsmodels::linear_model::ols::solve_linear_system;

fn main() {
    // 目的: 疑似逆行列で最小二乗解を求める
    println!("[Goal] OLS: 与えられた A, b に対して最小二乗解 x̂ = A^+ b を計算します。");

    let a = Matrix::new(4, 2, vec![1.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0]).unwrap();
    // 真の係数 x_true から b = Ax_true を生成（ノイズなし）
    let x_true = Vector::new(vec![1.0, 0.5]);
    let b = &a * &x_true;
    println!("[Setup] A:\n{a}\n[Setup] x_true={x_true}, b=Ax_true={b}");

    let x_hat = solve_linear_system(&a, &b).expect("ols failed");
    println!("[Result] 推定係数 x̂ = {x_hat}");
    println!("[Compare] x_true と x̂: x_true={x_true}, x̂={x_hat}");
    let fit = &a * &x_hat;
    let residual = &b - &fit;
    println!("[Check] 予測 Ax̂ = {fit}");
    println!("[Check] 残差 r = b - Ax̂ = {residual} (小さいほど良い)");
}
