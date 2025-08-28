use crate::{Matrix, Result}; // Result型はエラーを返すためにあると仮定

pub trait CholeskyDecomposition {
    fn cholesky(&self) -> Result<Matrix<f64>>;
}

impl CholeskyDecomposition for Matrix<f64> {
    fn cholesky(&self) -> Result<Matrix<f64>> {
        let n = self.rows;
        // 正方行列であるかのチェック
        if n != self.cols {
            // エラーを返すのが望ましい
            // return Err(...);
            panic!("Matrix must be square.");
        }

        // 結果を格納するL行列（下三角行列）
        let mut l_data = vec![0.0; n * n];

        // i列目を計算する
        for i in 0..n {
            // j行目を計算する (j >= i)
            for j in i..n {
                let mut sum = 0.0;
                // k列目までの内積を計算
                for k in 0..i {
                    // 正しいインデックスは i*cols + k
                    sum += l_data[j * n + k] * l_data[i * n + k];
                }

                if i == j {
                    // 対角要素の場合: l_ii = sqrt(a_ii - Σ l_ik^2)
                    let val = self[(i, i)] - sum;
                    if val <= 0.0 {
                        // 行列が正定値ではない
                        // return Err(...);
                        panic!("Matrix is not positive-definite.");
                    }
                    l_data[i * n + i] = val.sqrt();
                } else {
                    // 非対角要素の場合: l_ji = (1/l_ii) * (a_ji - Σ l_jk*l_ik)
                    // l_data[i*n+i] (l_ii) はこのループの最初で計算済み
                    l_data[j * n + i] = (self[(j, i)] - sum) / l_data[i * n + i];
                }
            }
        }

        // Matrix::new が Result<Matrix, ...> を返すと仮定
        Matrix::new(n, n, l_data)
    }
}
