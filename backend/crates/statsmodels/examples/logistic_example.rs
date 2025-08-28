use linalg::{Matrix, Vector};
use statsmodels::estimation::logistic::LogisticRegression;

fn main() {
    // 問題設定: 2特徴の2値分類 (教師あり学習)
    // - 説明変数X: 4サンプル × 2特徴
    // - 目的変数y: {0,1}
    // 真の分離超平面 β_true を仮定して y を生成すると意味が分かりやすい
    let x = Matrix::new(4, 2, vec![0.0, 0.0, 0.5, 0.2, 1.0, 1.5, 2.0, 2.5]).unwrap();
    let beta_true = Vector::new(vec![-4.0, 1.2, 5.8]); // [切片, x1, x2]
    let y = Vector::new(
        (0..x.rows)
            .map(|i| {
                let xi = x.row(i).unwrap();
                let z = beta_true[0] + beta_true[1] * xi[0] + beta_true[2] * xi[1];
                if z > 0.0 {
                    1.0
                } else {
                    0.0
                }
            })
            .collect(),
    );

    println!(
        "[Setup] X shape = ({}x{}), y len = {}",
        x.rows,
        x.cols,
        y.len()
    );
    println!(
        "[Goal] ロジスティック回帰: P(y=1|x)=σ(β₀ + β₁x₁ + β₂x₂)。閾値0.5でクラスを決定します。"
    );

    // 学習 (単純勾配上昇: alpha=学習率, max_iter=反復回数)
    let model = LogisticRegression::fit(&x, &y, 0.5, 200);

    println!("[Truth] 真のパラメータ β_true (先頭が切片): {beta_true}");
    println!(
        "[Result] 学習済みパラメータ β (index 0 が切片β₀): {}",
        model.coefficients()
    );
    println!("         係数の符号は、各特徴がy=1になる方向にどれだけ効くかを示します。");

    // 予測と解釈: 確率→クラス、正解との比較、精度
    let mut correct = 0;
    for i in 0..x.rows {
        let xi = x.row(i).unwrap();
        let proba = model.predict_proba(&xi);
        let pred = if proba > 0.5 { 1.0 } else { 0.0 };
        let yi = y[i];
        if (pred - yi).abs() < 1e-9 {
            correct += 1;
        }
        println!("[Pred] x[{i}]={xi} -> P(y=1|x)={proba:.3} => class={pred} (true={yi})");
    }

    let acc = correct as f64 / x.rows as f64;
    println!(
        "[Score] 学習データに対する精度(Accuracy) = {:.1}%",
        acc * 100.0
    );
    println!("[Compare] β_true と 学習β の方向が概ね一致していれば妥当です (スケールは学習率やデータで多少変動)。");
}
