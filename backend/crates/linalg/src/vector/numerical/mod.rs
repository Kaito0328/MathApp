use super::*;
use num_complex::Complex;

#[cfg(test)]
mod tests;
impl Vector<f64> {
    /// 指定した範囲の値で初期化されたベクトルを生成する
    pub fn linspace(start: f64, end: f64, num: usize) -> Result<Self> {
        if num <= 1 || start >= end {
            return Err(LinalgError::InvalidArgument {
                text: "num must be greater than 1 and start must be less than end".to_string(),
            });
        }
        let step = (end - start) / (num as f64 - 1.0);
        let data = (0..num)
            .map(|i| start + (i as f64 * step))
            .collect::<Vec<f64>>();
        Ok(Vector::new(data))
    }

    /// ベクトルのL2ノルム（大きさ）を計算する
    pub fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }

    /// ベクトルを正規化する（単位ベクトル化）
    pub fn normalize(&self) -> Vector<f64> {
        let norm = self.norm();
        if norm == 0.0 {
            return Vector::zeros(self.dim());
        }
        let data = self.data.iter().map(|x| x / norm).collect::<Vec<f64>>();
        Vector::new(data)
    }

    /// 他のベクトルとのコサイン類似度を計算する
    pub fn cosine_similarity(&self, other: &Self) -> f64 {
        let dot_product = self.dot(other);
        let norm_self = self.norm();
        let norm_other = other.norm();
        if norm_self == 0.0 || norm_other == 0.0 {
            return 0.0; // ゼロベクトルとの類似度は定義しない
        }
        dot_product / (norm_self * norm_other)
    }
    /// ベクトルの平均値を計算する
    pub fn mean(&self) -> Option<f64> {
        if self.data.is_empty() {
            return None;
        }
        let sum: f64 = self.data.iter().sum();
        Some(sum / self.data.len() as f64)
    }

    /// ベクトルの標準偏差を計算する
    pub fn std(&self) -> f64 {
        if self.data.len() < 2 {
            return 0.0;
        }
        let mean = self.mean().unwrap();
        let squared_diffs: Vec<f64> = self
            .data
            .iter()
            .map(|x| {
                let diff = *x - mean;
                diff * diff
            })
            .collect();
        let variance = squared_diffs.iter().sum::<f64>() / self.data.len() as f64;
        variance.sqrt()
    }

    pub fn householder_vector(&self) -> Option<Self> {
        if self.dim() == 0 {
            return None;
        }
        let mut v = self.data.clone();
        let norm = self.norm();
        if norm == 0.0 {
            return Some(Self::new(v)); // ゼロベクトルの場合はそのまま返す
        }
        v[0] += if v[0] >= 0.0 { norm } else { -norm };
        let scale = v.iter().map(|x| x * x).sum::<f64>().sqrt();
        v.iter_mut().for_each(|x| *x /= scale);
        Some(Self::new(v))
    }
}

impl Vector<Complex<f64>> {
    pub fn norm(&self) -> f64 {
        self.data
            .iter()
            .map(|c| c.norm_sqr()) // 各複素数要素のノルムの2乗 (a^2 + b^2) を計算
            .sum::<f64>() // 全てのノルムの2乗を合計する
            .sqrt() // 合計の平方根を取る
    }
}
