use linalg::{Matrix, Vector};

pub struct LogisticRegression {
    // 外部から直接アクセスできないようにprivateにするのが一般的
    coefficients: Vector<f64>,
}

impl LogisticRegression {
    // newは外部から直接インスタンスを作るためには不要になることが多い
    // pub fn new(coefficients: Vector<f64>) -> Self { ... }

    // 学習済みの係数を取得するためのgetterを追加
    pub fn coefficients(&self) -> &Vector<f64> {
        &self.coefficients
    }

    // fitは学習を行い、新しいモデルインスタンスを返す
    pub fn fit(x_input: &Matrix<f64>, y: &Vector<f64>, alpha: f64, max_iter: usize) -> Self {
        // xの先頭に1の列を追加して、切片（バイアス）項に対応
        let ones = Matrix::new(x_input.rows, 1, vec![1.0; x_input.rows]).unwrap();
        // 切片を先頭列に置く: 係数[0]が切片というAPI契約と整合
        let x = ones.hstack(x_input).unwrap();

        let mut beta = Vector::zeros(x.cols);

        for _ in 0..max_iter {
            // mapの中でbetaをmoveしないように、参照で渡す
            let p = Vector::new(
                (0..x.rows)
                    .map(|i| Self::predict_proba_internal(&beta, &x.row(i).unwrap()))
                    .collect(),
            );

            let error = y - &p;
            let gradient = &x.transpose() * &error;

            beta = beta + &(&gradient * alpha);
        }

        Self { coefficients: beta }
    }

    // &self を取るメソッドに変更
    pub fn predict_proba(&self, x: &Vector<f64>) -> f64 {
        let intercept = self.coefficients[0];
        // 係数のβ₁以降と、入力xの内積を計算
        let linear_term = (0..self.coefficients.dim() - 1)
            .map(|j| self.coefficients[j + 1] * x[j])
            .sum::<f64>();
        let z = intercept + linear_term;
        Self::sigmoid(z)
    }

    // &self を取るメソッドに変更
    pub fn predict(&self, x: &Vector<f64>) -> f64 {
        const THRESHOLD: f64 = 0.5;
        if self.predict_proba(x) > THRESHOLD {
            1.0
        } else {
            0.0
        }
    }

    // 内部計算用のヘルパー関数（これはこのままでOK）
    fn predict_proba_internal(beta: &Vector<f64>, x_row: &Vector<f64>) -> f64 {
        let z = beta.dot(x_row);
        Self::sigmoid(z)
    }

    fn sigmoid(z: f64) -> f64 {
        1.0 / (1.0 + (-z).exp())
    }
}
