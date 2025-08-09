use num_complex::Complex;

use crate::matrix::{Direction, Matrix};
use crate::Vector;

// このimplブロックに共通ヘルパー関数を集約する
impl Matrix<f64> {
    pub(super) fn to_hessenberg(&self) -> Option<(Matrix<f64>, Matrix<f64>)> {
        if !self.is_square() {
            return None;
        }
        let mut h = self.clone();
        let mut v = Matrix::identity(self.rows);

        // usizeの引き算でオーバーフローしないように saturating_sub を使うとより安全です
        for k in 0..self.rows.saturating_sub(2) {
            // k列目のk+1行目以降を対象にベクトルを抽出
            let x = h.partial_col(k, k + 1, self.rows).ok()?;

            // 1. ハウスホルダー "ベクトル" を計算 (行列は作らない)
            // Vectorに実装されたhouseholder_vectorメソッドを使います
            let Some(h_vec) = x.householder_vector() else {
                continue;
            };

            // 2. 左から変換を適用: h = H_k * h
            // 影響範囲: h の k+1行目以降, k列目以降
            h.apply_householder_transform(&h_vec, Direction::Left, k + 1, k)?;

            // 3. 右から変換を適用: h = h * H_k
            // 影響範囲: h の全行, k+1列目以降
            h.apply_householder_transform(&h_vec, Direction::Right, 0, k + 1)?;

            // 4. 固有ベクトル用の変換行列も更新: v = v * H_k
            v.apply_householder_transform(&h_vec, Direction::Right, 0, k + 1)?;
        }
        Some((h, v))
    }

    pub(super) fn apply_householder_transform(
        &mut self,
        householder_vector: &Vector<f64>,
        direction: Direction,
        start_row: usize,
        start_col: usize,
    ) -> Option<()> {
        match direction {
            Direction::Left => {
                // H * A の計算
                for j in start_col..self.cols {
                    let mut dot_product = 0.0;
                    for i in 0..householder_vector.dim() {
                        if start_row + i < self.rows {
                            dot_product += householder_vector[i] * self[(start_row + i, j)];
                        }
                    }

                    for i in 0..householder_vector.dim() {
                        if start_row + i < self.rows {
                            self[(start_row + i, j)] -= 2.0 * householder_vector[i] * dot_product;
                        }
                    }
                }
            }
            Direction::Right => {
                // A * H の計算
                for i in start_row..self.rows {
                    let mut dot_product = 0.0;
                    for j in 0..householder_vector.dim() {
                        if start_col + j < self.cols {
                            dot_product += self[(i, start_col + j)] * householder_vector[j];
                        }
                    }

                    for j in 0..householder_vector.dim() {
                        if start_col + j < self.cols {
                            self[(i, start_col + j)] -= 2.0 * householder_vector[j] * dot_product;
                        }
                    }
                }
            }
        }
        Some(())
    }

    pub(super) fn givens_rotation(a: f64, b: f64) -> (f64, f64) {
        if b.abs() < f64::EPSILON {
            (1.0, 0.0)
        } else {
            let r = a.hypot(b); // sqrt(a^2 + b^2) を計算
            let c = a / r;
            let s = b / r;
            (c, s)
        }
    }

    /// 実数行列を、虚数部がゼロの複素数行列に変換する。
    /// 最終段階の固有ベクトル計算 (X = QY) で必要になる。
    pub(super) fn to_complex(&self) -> Matrix<Complex<f64>> {
        let complex_data: Vec<Complex<f64>> = self
            .data
            .iter()
            .map(|&val| Complex::new(val, 0.0))
            .collect();
        Matrix::new(self.rows, self.cols, complex_data).unwrap()
    }

    pub(super) fn nullspace_vector(&self, tol: f64) -> Option<crate::Vector<f64>> {
        let m = self.rows;
        let n = self.cols;
        let mut a = self.clone(); // 作業用行列
        let mut pivot_cols: Vec<Option<usize>> = vec![None; m]; // 各行のピボット列
        let mut row = 0usize;

        for col in 0..n {
            if row >= m {
                break;
            }
            // ピボットとなる行を探す（部分ピボット）
            let mut max_row = row;
            let mut max_val = a[(row, col)].abs();
            for r in (row + 1)..m {
                let val = a[(r, col)].abs();
                if val > max_val {
                    max_val = val;
                    max_row = r;
                }
            }
            if max_val <= tol {
                // この列はピボットにならない（ゼロ列に近い）
                continue;
            }
            // 行交換
            if max_row != row {
                for c in col..n {
                    let tmp = a[(row, c)];
                    a[(row, c)] = a[(max_row, c)];
                    a[(max_row, c)] = tmp;
                }
            }
            // 正規化してピボットを1にする
            let pivot = a[(row, col)];
            for c in col..n {
                a[(row, c)] /= pivot;
            }
            // 他行の col を消す（RREF：上にも下にも消す）
            for r in 0..m {
                if r == row {
                    continue;
                }
                let factor = a[(r, col)];
                if factor.abs() <= tol {
                    continue;
                }
                for c in col..n {
                    a[(r, c)] -= factor * a[(row, c)];
                }
            }

            pivot_cols[row] = Some(col);
            row += 1;
            if row >= m {
                break;
            }
        }

        // ランクを求める（ピボットが存在する行数）
        let rank = pivot_cols.iter().filter(|p| p.is_some()).count();

        if rank == n {
            // 零空間は自明
            return None;
        }

        // 自由変数（pivot でない列）を1つ選ぶ（ここでは最後の自由列）
        let mut is_pivot_col = vec![false; n];
        for &pc in pivot_cols.iter().flatten() {
            is_pivot_col[pc] = true;
        }
        let mut free_col = None;
        for j in (0..n).rev() {
            if !is_pivot_col[j] {
                free_col = Some(j);
                break;
            }
        }
        let free_col = free_col?;

        // 解ベクトル x を構築：自由変数 free_col = 1, その他は RREF の行から決める
        let mut x = vec![0.0f64; n];
        x[free_col] = 1.0;
        // 各ピボット行について、そのピボット列の変数を決定
        for r in (0..m).rev() {
            if let Some(pc) = pivot_cols[r] {
                // x[pc] + sum_{j>pc} a[r,j] * x[j] = 0  (RREFでは a[r,pc]==1)
                let mut s = 0.0f64;
                for j in (pc + 1)..n {
                    s += a[(r, j)] * x[j];
                }
                x[pc] = -s;
            }
        }

        // 正規化
        let norm = x.iter().map(|v| v * v).sum::<f64>().sqrt();
        if norm <= tol {
            return None;
        }
        for xi in &mut x {
            *xi /= norm;
        }

        // 符号安定化: 最初の非零成分を正にする
        for &xi in &x {
            if xi.abs() > tol {
                if xi < 0.0 {
                    for v in &mut x {
                        *v = -*v;
                    }
                }
                break;
            }
        }

        Some(crate::Vector::new(x))
    }
}

#[cfg(test)]
mod tests;
