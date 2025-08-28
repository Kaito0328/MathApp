use linalg::{Matrix, Vector};
use statsmodels::linear_model::lasso::lasso_regression;

fn main() {
    // 目的: LASSOでスパースな係数推定を体験
    println!("[Goal] LASSO: min 1/2||Ax-b||^2 + α||x||_1。αでスパース性(ゼロ係数)が促進されます。");

    // ダミーデータ: 3特徴のうち実は2つ目だけが効く
    let a = Matrix::new(
        5,
        3,
        vec![
            1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 2.0, 0.0, 1.0, 3.0, 0.0, 1.0, 4.0, 0.0,
        ],
    )
    .unwrap();
    // 真の係数（スパース）: x_true = [0, 1, 0]
    let x_true = Vector::new(vec![0.0, 1.0, 0.0]);
    let b = &a * &x_true; // ノイズなしで生成（必要なら小さなノイズを加えてもOK）
    println!(
        "[Setup] A shape=({}x{}), b len={}, x_true={}",
        a.rows,
        a.cols,
        b.len(),
        x_true
    );

    let alpha = 0.5;
    let x_hat = lasso_regression(&a, &b, alpha, 200, 1e-6).expect("lasso failed");
    println!("[Result] 推定係数 x̂ (α={alpha}): {x_hat}");
    println!("         0に近い係数は特徴が不要である可能性を示唆します (L1でソフトしきい値)。");
    println!("[Compare] 真の係数 x_true と x̂ の比較: x_true={x_true}, x̂={x_hat}");

    let fit = &a * &x_hat;
    let resid = &b - &fit;
    println!("[Check] 予測 Ax̂ = {fit}");
    println!("[Check] 残差 r = b - Ax̂ = {resid} (ノイズなしならほぼ0に近い)");
}
