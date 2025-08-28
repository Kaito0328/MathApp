use linalg::{Matrix, Vector};
use rand::{rngs::StdRng, SeedableRng};
use statistics::distribution::continuous::core::Distribution;
use statistics::distribution::continuous::normal::Normal;
use statsmodels::estimation::kalman::KalmanFilter;

// 2次元の一定速度モデル（状態=[位置, 速度]）を確率的に生成し、カルマン推定と比較
// 状態方程式: x_{t+1} = F x_t + w_t,  w_t ~ N(0, Q)
// 観測方程式: z_t     = H x_t + v_t,  v_t ~ N(0, R)
fn main() {
    let dt = 1.0;
    let n = 2usize; // 状態: [pos, vel]
    let m = 1usize; // 観測: 位置のみ

    // F = [[1, dt],[0,1]]
    let f = Matrix::new(n, n, vec![1.0, dt, 0.0, 1.0]).unwrap();
    // H = [1, 0]
    let h = Matrix::new(m, n, vec![1.0, 0.0]).unwrap();
    // 対角Q, R（簡単化）
    let q_pos = 1e-3; // 位置雑音の分散
    let q_vel = 1e-3; // 速度雑音の分散
    let r_meas = 1e-2; // 観測雑音の分散
    let q = Matrix::new(n, n, vec![q_pos, 0.0, 0.0, q_vel]).unwrap();
    let r = Matrix::new(m, m, vec![r_meas]).unwrap();

    // 真の初期状態（位置0, 速度1）
    let mut x_true = Vector::new(vec![0.0, 1.0]);
    // フィルタ初期推定
    let x0 = Vector::new(vec![0.0, 0.0]);
    let p0 = Matrix::new(n, n, vec![1.0, 0.0, 0.0, 1.0]).unwrap();

    let mut kf =
        KalmanFilter::new(x0, p0, f.clone(), h.clone(), q.clone(), r.clone()).expect("init");

    // 乱数準備（再現用シード固定）
    let mut rng = StdRng::seed_from_u64(42);
    let mut std_norm = Normal::new(0.0, 1.0).unwrap();

    // 時系列を生成
    let t_max = 50;
    let mut zs: Vec<Vector<f64>> = Vec::with_capacity(t_max);
    let mut xs_true: Vec<Vector<f64>> = Vec::with_capacity(t_max);
    let mut xs_hat: Vec<Vector<f64>> = Vec::with_capacity(t_max);

    println!("[Model] F=\n{f}\nH=\n{h}\nQ=\n{q}\nR=\n{r}");
    println!("[Sim] steps={t_max}, q_pos={q_pos}, q_vel={q_vel}, r={r_meas}");
    println!(
        "[Start] x_true0={x_true}, x̂0={}, P0=\n{}",
        kf.state(),
        kf.covariance()
    );

    for t in 0..t_max {
        // 真の遷移: x_true = F x_true + w, w ~ N(0,Q)（対角Qなので各成分独立）
        let w_pos = std_norm.sample(&mut rng) * q_pos.sqrt();
        let w_vel = std_norm.sample(&mut rng) * q_vel.sqrt();
        x_true = &f * &x_true + Vector::new(vec![w_pos, w_vel]);

        // 観測: z = H x_true + v, v ~ N(0,R)
        let v = std_norm.sample(&mut rng) * r_meas.sqrt();
        let z_val = (&h * &x_true)[0] + v;
        let z = Vector::new(vec![z_val]);

        // カルマン予測・更新
        kf.predict().expect("predict");
        kf.update(&z).expect("update");

        // 記録
        xs_true.push(x_true.clone());
        zs.push(z.clone());
        xs_hat.push(kf.state().clone());

        // 最初の数ステップは詳細表示
        if t < 5 {
            println!(
                "t={t:02} z={z} | true=[pos={:.3}, vel={:.3}] est=[pos={:.3}, vel={:.3}]",
                x_true[0],
                x_true[1],
                kf.state()[0],
                kf.state()[1]
            );
        }
    }

    // RMSEを計算（位置と速度）
    let rmse = |idx: usize| -> f64 {
        let mut s = 0.0;
        for t in 0..t_max {
            let e = xs_hat[t][idx] - xs_true[t][idx];
            s += e * e;
        }
        (s / t_max as f64).sqrt()
    };
    let rmse_pos = rmse(0);
    let rmse_vel = rmse(1);

    // 最終ステップの比較
    let last_t = t_max - 1;
    println!(
        "[End] true_last=[pos={:.3}, vel={:.3}] est_last=[pos={:.3}, vel={:.3}]",
        xs_true[last_t][0], xs_true[last_t][1], xs_hat[last_t][0], xs_hat[last_t][1]
    );
    println!("[Score] RMSE: pos={rmse_pos:.4}, vel={rmse_vel:.4}");
}
