use crate::error::{Result, StatsModelsError};
use linalg::{Matrix, Vector};

pub struct KalmanFilter {
    // ---- 状態 ----
    /// 状態ベクトルの現在の推定値 (x̂)
    x: Vector<f64>,
    /// 誤差共分散行列の現在の推定値 (P)
    p: Matrix<f64>,

    // ---- システムモデル ----
    /// 状態遷移行列 (F)
    f: Matrix<f64>,
    /// 観測行列 (H)
    h: Matrix<f64>,
    /// プロセスノイズの共分散行列 (Q)
    q: Matrix<f64>,
    /// 観測ノイズの共分散行列 (R)
    r: Matrix<f64>,
}

impl KalmanFilter {
    /// 初期状態とシステムモデルからフィルタを生成する
    pub fn new(
        initial_x: Vector<f64>,
        initial_p: Matrix<f64>,
        f: Matrix<f64>,
        h: Matrix<f64>,
        q: Matrix<f64>,
        r: Matrix<f64>,
    ) -> Result<Self> {
        // 次元チェック
        let n = initial_x.len();
        if initial_p.rows != n || initial_p.cols != n {
            return Err(StatsModelsError::DimensionMismatch {
                expected: format!("{n}x{n}"),
                found: format!("{}x{}", initial_p.rows, initial_p.cols),
            });
        }
        if f.rows != n || f.cols != n {
            return Err(StatsModelsError::DimensionMismatch {
                expected: format!("F {n}x{n}"),
                found: format!("F {}x{}", f.rows, f.cols),
            });
        }
        if q.rows != n || q.cols != n {
            return Err(StatsModelsError::DimensionMismatch {
                expected: format!("Q {n}x{n}"),
                found: format!("Q {}x{}", q.rows, q.cols),
            });
        }
        if h.cols != n {
            return Err(StatsModelsError::DimensionMismatch {
                expected: format!("H cols {n}"),
                found: format!("H cols {}", h.cols),
            });
        }
        let m = h.rows;
        if r.rows != m || r.cols != m {
            return Err(StatsModelsError::DimensionMismatch {
                expected: format!("R {m}x{m}"),
                found: format!("R {}x{}", r.rows, r.cols),
            });
        }

        Ok(Self {
            x: initial_x,
            p: initial_p,
            f,
            h,
            q,
            r,
        })
    }

    /// 予測ステップを実行する
    pub fn predict(&mut self) -> Result<()> {
        // 状態の予測
        self.x = &self.f * &self.x;
        // 誤差共分散の予測
        self.p = &self.f * &self.p * &self.f.transpose() + &self.q;
        Ok(())
    }

    /// 観測データを使って更新ステップを実行する
    pub fn update(&mut self, z: &Vector<f64>) -> Result<()> {
        // 入力ベクトルの次元チェック
        if z.len() != self.h.rows {
            return Err(StatsModelsError::DimensionMismatch {
                expected: format!("z len {}", self.h.rows),
                found: format!("z len {}", z.len()),
            });
        }
        // イノベーション共分散 S = H P H^T + R (m×m)
        let s = &self.h * &self.p * &self.h.transpose() + &self.r;
        // P H^T (n×m)
        let pht = &self.p * &self.h.transpose();

        // カルマンゲイン K = P H^T S^{-1}
        // solve は A X = B を解く（左からの解）。形を合わせるために転置系を用いる:
        //   S^T Y = (P H^T)^T を解き、K = Y^T とする。
        let y_res = s.transpose().solve_matrix(&pht.transpose());
        let y = match y_res {
            Ok(mat) => mat,
            Err(e) => {
                // 数値不安定対策: 小さなダンピングを追加して再試行
                let eps = 1e-8;
                let damped = &s + &(&Matrix::identity(s.rows) * eps);
                damped
                    .transpose()
                    .solve_matrix(&pht.transpose())
                    .map_err(|_| StatsModelsError::Linalg(e))?
            }
        }; // Y: m×n
        let k = y.transpose(); // K: n×m
                               // 状態の更新
        self.x = &self.x + &k * (z - &self.h * &self.x);
        let i_kh = &Matrix::identity(self.p.rows) - &(&k * &self.h);
        let p_new = &(&i_kh * &self.p) * &i_kh.transpose() + &(&k * &self.r) * &k.transpose();
        self.p = p_new;
        Ok(())
    }

    /// 推定された状態ベクトル x̂ への参照
    pub fn state(&self) -> &Vector<f64> {
        &self.x
    }
    /// 推定された誤差共分散 P への参照
    pub fn covariance(&self) -> &Matrix<f64> {
        &self.p
    }
}
