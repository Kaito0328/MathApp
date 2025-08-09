use crate::matrix::Matrix;
use crate::Direction;
use std::cmp::min;

pub struct QR {
    pub q: Matrix<f64>,
    pub r: Matrix<f64>,
}

pub trait QrDecomposition {
    /// LU分解を行う。成功した場合はLU構造体を返す。
    /// 行列が正方行列でない場合はNoneを返す。
    fn qr_decomposition(&self) -> Option<QR>;
}

impl QrDecomposition for Matrix<f64> {
    fn qr_decomposition(&self) -> Option<QR> {
        let (rows, cols) = (self.rows, self.cols);
        let mut r = self.clone();
        let mut q = Matrix::identity(rows);

        for k in 0..min(rows, cols) {
            // 1. 部分ベクトルを抽出
            let x = r.partial_col(k, k, rows).ok()?;

            // 2. ハウスホルダー"ベクトル"を計算
            if let Some(h_vec) = x.householder_vector() {
                // 3. r に左から変換を適用: r = H * r
                r.apply_householder_transform(&h_vec, Direction::Left, k, k);

                // 4. q に右から変換を適用: q = q * H
                q.apply_householder_transform(&h_vec, Direction::Right, 0, k);
            }
        }

        // 対角成分の符号を正に揃える後処理 (この部分は変更なしでOK)
        for k in 0..min(rows, cols) {
            if r[(k, k)] < 0.0 {
                r.scale_row(k, -1.0).ok()?;
                q.scale_col(k, -1.0).ok()?;
            }
        }

        Some(QR { q, r })
    }
}

#[cfg(test)]
mod tests;
