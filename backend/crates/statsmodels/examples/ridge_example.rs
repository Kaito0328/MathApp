use linalg::{Matrix, Vector};
use statsmodels::linear_model::ridge::ridge_regression_optimized;

fn main() {
    // 目的: リッジ回帰で正則化付き最小二乗を解く
    println!("[Goal] リッジ回帰: min ||Ax-b||^2 + α^2||x||^2。αが大きいほど係数が小さくなりがち。");

    // 小さな設計行列とターゲット
    let a = Matrix::new(4, 2, vec![1.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0]).unwrap();
    // 真の係数 x_true を仮定して b = A x_true を生成（ノイズなし）
    let x_true = Vector::new(vec![1.0, 1.0]);
    let b = &a * &x_true;
    println!("[Setup] A:\n{a}\n[Setup] x_true={x_true}, b=Ax_true={b}");

    let alpha = 0.5;
    let x_hat = ridge_regression_optimized(&a, &b, alpha).expect("ridge failed");
    println!("[Result] 推定係数 x̂ (α={alpha}): {x_hat}");
    println!(
        "         これは (A^TA + α^2 I)⁻¹A^Tb に等価 (SVDで安定計算)。α>0 で係数がやや収縮します。"
    );
    println!("[Compare] x_true と x̂: x_true={x_true}, x̂={x_hat}");

    let fit = &a * &x_hat;
    let resid = &b - &fit;
    println!("[Check] 予測 Ax̂ = {fit}");
    println!("[Check] 残差 r = b - Ax̂ = {resid} (ノイズなしなら小さめ、αが大きいとやや増えます)");
}
