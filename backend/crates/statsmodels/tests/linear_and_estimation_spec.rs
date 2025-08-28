use linalg::{Matrix, Vector};
use statsmodels::estimation::{gmm::GaussianMixtureModel, logistic::LogisticRegression};
use statsmodels::linear_model::{
    lasso::lasso_regression, ols::solve_linear_system, ridge::ridge_regression_optimized,
};

#[test]
fn ols_pinv_solves_small_system() {
    let a = Matrix::new(4, 2, vec![1.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0]).unwrap();
    let b = Vector::new(vec![1.0, 2.0, 2.5, 3.0]);
    let x = solve_linear_system(&a, &b).expect("ols failed");

    // 予測が観測にそこそこ近いこと
    let fit = &a * &x;
    let resid = (&b - &fit).norm();
    assert!(resid < 0.6, "residual too large: {resid}");
}

#[test]
fn ridge_returns_reasonable_solution() {
    let a = Matrix::new(4, 2, vec![1.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0]).unwrap();
    let b = Vector::new(vec![1.0, 2.0, 2.5, 3.0]);
    let x = ridge_regression_optimized(&a, &b, 0.5).expect("ridge failed");

    // リッジはOLSに近いがやや収縮
    let fit = &a * &x;
    let resid = (&b - &fit).norm();
    assert!(resid < 0.7);
}

#[test]
fn lasso_encourages_sparsity() {
    // 2番目の特徴のみが真の寄与
    let a = Matrix::new(
        6,
        3,
        vec![
            1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 2.0, 0.0, 1.0, 3.0, 0.0, 1.0, 4.0, 0.0, 1.0, 5.0,
            0.0,
        ],
    )
    .unwrap();
    let b = Vector::new(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
    let x = lasso_regression(&a, &b, 0.5, 500, 1e-8).expect("lasso failed");

    // スパース性: 3列目は0付近、2列目は1付近、1列目は小さい
    assert!(x[2].abs() < 1e-3, "third coef not near 0: {}", x[2]);
}

#[test]
fn logistic_fits_simple_separable_data() {
    // 多少スケールを上げて分離しやすくする
    let x = Matrix::new(4, 2, vec![0.0, 0.0, 1.0, 0.4, 2.0, 3.0, 4.0, 5.0]).unwrap();
    let y = Vector::new(vec![0.0, 0.0, 1.0, 1.0]);

    let model = LogisticRegression::fit(&x, &y, 0.5, 500);
    // 学習データでの分類精度が高いこと
    let mut correct = 0;
    for i in 0..x.rows {
        let xi = x.row(i).unwrap();
        let pred = model.predict(&xi);
        if (pred - y[i]).abs() < 1e-9 {
            correct += 1;
        }
    }
    assert!(correct >= 3, "accuracy too low: {}/{}", correct, x.rows);
}

#[test]
fn gmm_separates_two_clusters() {
    // 二つのクラスタ [1,1] と [-1,-1] に小さな球状ノイズ
    let mut data: Vec<Vector<f64>> = Vec::new();
    for t in [-1.0, 1.0] {
        for i in 0..20 {
            let angle = (i as f64) * 0.3;
            let dx = 0.1 * angle.cos();
            let dy = 0.1 * angle.sin();
            data.push(Vector::new(vec![t + dx, t + dy]));
        }
    }

    let gmm = GaussianMixtureModel::fit(&data, 2, 100, 1e-6).expect("fit failed");

    // 中心に近い点の予測が「それっぽい」クラスタになること
    let near_plus = Vector::new(vec![0.9, 1.1]);
    let near_minus = Vector::new(vec![-1.1, -0.9]);
    let c1 = gmm.predict(&near_plus);
    let c2 = gmm.predict(&near_minus);
    assert_ne!(c1, c2);
}
