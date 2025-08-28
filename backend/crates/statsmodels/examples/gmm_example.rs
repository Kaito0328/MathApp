use linalg::Vector;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use statistics::distribution::multivariate_continuous::{
    core::MultivariateDistribution, normal::MultivariateNormal,
};
use statsmodels::estimation::gmm::GaussianMixtureModel;

fn main() {
    // 目的: 真の混合正規分布からサンプルを生成し、その真値をEMで推定
    let k = 2;
    println!("[Goal] 真のGMM(混合正規)からデータを生成し、EMで {k} 成分を推定します。");

    // 真のパラメータ（2成分）
    let true_weights = vec![0.4, 0.6];
    let true_means = [Vector::new(vec![1.0, 1.0]), Vector::new(vec![-1.0, -1.0])];
    let true_covs = [
        // 共分散は正定値に（対角に小さい分散）
        linalg::Matrix::new(2, 2, vec![0.05, 0.0, 0.0, 0.05]).unwrap(),
        linalg::Matrix::new(2, 2, vec![0.08, 0.0, 0.0, 0.08]).unwrap(),
    ];

    // 正規分布オブジェクト
    let comps: Vec<MultivariateNormal> = true_means
        .iter()
        .zip(true_covs.iter())
        .map(|(m, s)| MultivariateNormal::new(m.clone(), s.clone()).expect("PD cov"))
        .collect();

    println!("[Truth] weights={true_weights:?}");
    for (j, (m, s)) in true_means.iter().zip(true_covs.iter()).enumerate() {
        println!("  comp {j}: mean={m}, cov=\n{s}");
    }

    // データ生成: 混合分布からNサンプル
    let mut rng = StdRng::seed_from_u64(42);
    let n = 120;
    let mut data: Vec<Vector<f64>> = Vec::with_capacity(n);
    let mut labels: Vec<usize> = Vec::with_capacity(n); // 真の成分ラベル
    for _ in 0..n {
        // 2成分なので乱数と閾値で割当
        let u: f64 = rng.gen();
        let z = if u < true_weights[0] { 0 } else { 1 };
        let x = comps[z].sample(&mut rng);
        data.push(x);
        labels.push(z);
    }
    println!("[Data] サンプル数={n}, 次元=2 (真のラベルも保持)");

    let model = GaussianMixtureModel::fit(&data, k, 200, 1e-6).expect("fit failed");
    println!("[Result] 学習済み混合重み π: {:?}", model.weights());
    for (j, dist) in model.distributions().iter().enumerate() {
        println!(
            "  component {j}: mean={}, cov=\n{}",
            dist.mean(),
            dist.covariance()
        );
    }

    // 予測の意味: 各点に対する所属確率(resp)と最尤クラスタ
    let test = Vector::new(vec![0.9, 1.1]);
    let resp = model.predict_proba(&test);
    let cls = model.predict(&test);
    println!("[Pred] x={test} -> responsibilities={resp:?} => most likely cluster={cls}");

    // 真のラベルに対する精度（ラベル交換不定性に配慮してmaxを採用）
    let preds: Vec<usize> = data.iter().map(|x| model.predict(x)).collect();
    let acc_id = |flip: bool| -> f64 {
        let mut correct = 0usize;
        for (p, &z) in preds.iter().zip(labels.iter()) {
            let pp = if flip { 1 - *p } else { *p };
            if pp == z {
                correct += 1;
            }
        }
        correct as f64 / n as f64
    };
    let acc = acc_id(false).max(acc_id(true));
    println!(
        "[Score] ラベル不定性を考慮したクラスタリング精度 ≈ {:.1}%",
        acc * 100.0
    );
}
