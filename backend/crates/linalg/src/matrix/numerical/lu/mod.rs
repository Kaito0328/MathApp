use crate::matrix::Matrix;

pub struct LU {
    pub p: Matrix<f64>,
    pub l: Matrix<f64>,
    pub u: Matrix<f64>,
}

pub trait LuDecomposition {
    /// LU分解を行う。成功した場合はLU構造体を返す。
    /// 行列が正方行列でない場合はNoneを返す。
    fn lu_decomposition(&self) -> Option<LU>;
}

impl LuDecomposition for Matrix<f64> {
    fn lu_decomposition(&self) -> Option<LU> {
        if !self.is_square() {
            return None;
        }
        let n = self.rows;
        // Lはゼロ行列から始め、UはAのコピーから始めるのがシンプル
        let mut l = Matrix::zeros(n, n);
        let mut u = self.clone();
        let mut p = Matrix::identity(n);

        for k in 0..n {
            // --- 1. ピボット選択 ---
            let mut max_val = 0.0;
            let mut pivot_row = k;
            for i in k..n {
                if u[(i, k)].abs() > max_val {
                    max_val = u[(i, k)].abs();
                    pivot_row = i;
                }
            }

            if max_val < 1e-10 {
                return None;
            }

            // --- 2. 行の交換 ---
            if pivot_row != k {
                let _ = p.swap_rows(k, pivot_row);
                let _ = u.swap_rows(k, pivot_row);
                // L行列の計算済み部分(k列目より前)だけを交換する
                for j in 0..k {
                    let temp = l[(k, j)];
                    l[(k, j)] = l[(pivot_row, j)];
                    l[(pivot_row, j)] = temp;
                }
            }

            // --- 3. LとUの計算 (ガウスの消去法) ---
            // Lの対角成分は1
            l[(k, k)] = 1.0;

            // Lのk列目の計算
            for i in k + 1..n {
                l[(i, k)] = u[(i, k)] / u[(k, k)];
            }

            // Uの更新 (前方消去)
            for i in k + 1..n {
                for j in k..n {
                    u[(i, j)] -= l[(i, k)] * u[(k, j)];
                }
            }
        }

        Some(LU { p, l, u })
    }
}

#[cfg(test)]
mod tests;
