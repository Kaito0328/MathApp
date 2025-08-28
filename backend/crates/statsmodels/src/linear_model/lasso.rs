use linalg::{Matrix, Result, Vector};

pub fn lasso_regression(
    a: &Matrix<f64>,
    b: &Vector<f64>,
    alpha: f64,
    max_iter: usize,
    tol: f64,
) -> Result<Vector<f64>> {
    // 事前処理：Aの各列のL2ノルムの2乗を計算しておく
    let d = a.cols;
    let mut a_col_norm_sq = Vec::with_capacity(d);
    for j in 0..d {
        a_col_norm_sq.push(a.col(j)?.dot(&a.col(j)?));
    }

    let mut x = Vector::zeros(d);

    // ✨ 修正点1: 反復ループを追加
    for _ in 0..max_iter {
        let mut max_delta = 0.0;

        for j in 0..d {
            let old_xj = x[j];

            // 現在の係数xjの影響を除いた残差を計算
            let mut r = b.clone();
            for k in 0..d {
                if k != j {
                    r = r - &(a.col(k)? * x[k]);
                }
            }

            let rho = a.col(j)?.dot(&r);

            // ✨ 修正点2: 分母をノルムの2乗にする
            if a_col_norm_sq[j] > 1e-9 {
                // ゼロ割を避ける
                x[j] = soft_thresholding(rho, alpha) / a_col_norm_sq[j];
            } else {
                x[j] = 0.0;
            }

            // 更新量を記録
            let delta = (x[j] - old_xj).abs();
            if delta > max_delta {
                max_delta = delta;
            }
        }

        // 収束判定
        if max_delta < tol {
            break;
        }
    }

    Ok(x)
}

pub fn soft_thresholding(z: f64, gamma: f64) -> f64 {
    if z > gamma {
        z - gamma
    } else if z < -gamma {
        z + gamma
    } else {
        0.0
    }
}
