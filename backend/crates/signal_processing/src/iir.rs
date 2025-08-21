use crate::signal::Signal;
use linalg::Vector;
use poly::Polynomial;

use crate::dft::conv_with_dft_for_f64;
use std::cmp::max;

/// IIRフィルタ（直接型I種）を信号に適用する。
///
/// # 引数
/// * `signal` - 入力信号。
/// * `b_coeffs` - FIR部の係数（分子係数）。
/// * `a_coeffs` - IIR部の係数（分母係数）。a₀は通常1。
///
/// # 戻り値
/// * フィルタリングされた後の信号。
pub fn iir_filter(signal: &[f64], b_coeffs: &[f64], a_coeffs: &[f64]) -> Vector<f64> {
    // 内部で、過去の入力(x)と出力(y)の値を保持するバッファを管理しながら
    // 上記の差分方程式をサンプルごとに計算していく。
    let mut y = vec![0.0; signal.len()];
    let a_0 = a_coeffs[0]; // a₀は通常1なので、除算を避けるために保持

    for n in 0..signal.len() {
        // 分子部分の計算
        let mut b_sum = 0.0;
        for (i, &b) in b_coeffs.iter().enumerate() {
            if n >= i {
                b_sum += b * signal[n - i];
            }
        }

        // 分母部分の計算
        let mut a_sum = 0.0;
        for (j, &a) in a_coeffs.iter().enumerate().skip(1) {
            if n >= j {
                // a₀は除外
                a_sum += a * y[n - j];
            }
        }

        // 出力の計算
        y[n] = (b_sum - a_sum) / a_0;
    }

    Vector::new(y)
}

/// IIR フィルタ型（分子 b と分母 a の多項式）。係数は低次→高次。
#[derive(Clone, Debug, PartialEq)]
pub struct IIRFilter {
    pub b: Polynomial<f64>,
    pub a: Polynomial<f64>,
}

impl IIRFilter {
    /// a[0] を 1 に正規化して保持（a[0] == 0 はパニック）。
    pub fn new(mut b: Polynomial<f64>, mut a: Polynomial<f64>) -> Self {
        let a0 = a.coeffs.first().copied().unwrap_or(1.0);
        assert!(a0 != 0.0, "a[0] must not be zero");
        if (a0 - 1.0).abs() > 0.0 {
            for c in b.coeffs.iter_mut() {
                *c /= a0;
            }
            for c in a.coeffs.iter_mut() {
                *c /= a0;
            }
        }
        Self { b, a }
    }

    /// 直接型Iで適用した結果を返す（内部は既存差分方程式）。
    pub fn apply(&self, x: &Signal) -> Signal {
        let y_vec = iir_filter(x.data(), &self.b.coeffs, &self.a.coeffs);
        Signal::new(y_vec.data, x.sample_rate())
    }
}

impl Signal {
    /// IIR フィルタを適用。
    pub fn apply_iir(&self, filt: &IIRFilter) -> Signal {
        filt.apply(self)
    }
}

/// 双一次変換を用いてアナログフィルタ係数をデジタルフィルタ係数に変換する。
///
/// # 引数
/// * `analog_b` - アナログ伝達関数の分子(B(s))の係数。
/// * `analog_a` - アナログ伝達関数の分母(A(s))の係数。
/// * `fs` - サンプリング周波数。周波数ワーピングの補正に必要。
///
/// # 戻り値
/// * (digital_b, digital_a) - デジタルIIRフィルタの係数のペア。
pub fn bilinear_transform(
    analog_b: &Vector<f64>,
    analog_a: &Vector<f64>,
    fs: f64,
) -> (Vector<f64>, Vector<f64>) {
    let mut cache = BinomialCoeffsCache::new();
    let b_len = analog_b.dim();
    let a_len = analog_a.dim();
    let n = max(b_len, a_len);

    let mut digital_b = Vector::new(vec![0.0; n]);
    let mut digital_a = Vector::new(vec![0.0; n]);

    let double_fs = 2.0 * fs;
    let mut power_fs = 1.0;

    for i in 0..n {
        let coeffs_plus = cache.get_x_plus_1(n - 1 - i);
        let coeffs_minus = cache.get_x_minus_1(i);
        let mut coeffs = conv_with_dft_for_f64(&coeffs_plus, &coeffs_minus);
        // Use (1 - x)^i instead of (x - 1)^i: multiply by (-1)^i
        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
        if sign < 0.0 {
            for k in 0..coeffs.dim() {
                coeffs[k] = -coeffs[k];
            }
        }

        if i < b_len {
            digital_b = digital_b + &coeffs * analog_b[i] * power_fs;
        }

        if i < a_len {
            digital_a = digital_a + &coeffs * analog_a[i] * power_fs;
        }

        power_fs *= double_fs;
    }

    (digital_b, digital_a)
}

/// Roots a_k から多項式 P(x) = Π(x - a_k) を展開し、
/// x^n の係数を返す。
///
/// # 引数
/// * `roots` - 根のリスト [a_0, a_1, ..., a_{N-1}]
/// * `n` - 求めたい係数の次数
/// # 戻り値
/// * x^n の係数
pub fn find_polynomial_coefficient(roots: &[f64]) -> Vector<f64> {
    let num_roots = roots.len();
    if num_roots == 0 {
        return Vector::new(vec![1.0]);
    }

    // `coeffs[i]` が x^i の係数を保持する
    // P₀(x) = (x - a₀) の係数で初期化 -> [ -roots[0], 1.0 ]
    let mut coeffs = vec![-roots[0], 1.0];

    // P₁(x) から P_{N-1}(x) まで順番に係数を更新していく
    for (k, &a_k) in roots.iter().enumerate().skip(1) {
        // 次数が1つ増えるので、最高次の係数のために要素を一つ追加
        coeffs.push(0.0);

        // 係数を更新：高次の項から計算しないと、更新前の値が上書きされてしまう
        for i in (1..=k).rev() {
            // new_coeffs[i] = coeffs[i-1] - a_k * coeffs[i]
            coeffs[i] = coeffs[i - 1] - a_k * coeffs[i];
        }
        // 定数項 (x^0) を更新
        coeffs[0] *= -a_k;

        // 最高次の係数 (x^{k+1}) は常に 1
        coeffs[k + 1] = 1.0;
    }

    Vector::new(coeffs)
}

pub fn find_polynomial_coefficient_fast(roots: &[f64]) -> Vector<f64> {
    let num_roots = roots.len();
    if num_roots == 0 {
        return Vector::new(vec![1.0]);
    }
    if num_roots == 1 {
        // P(x) = x - a₀
        return Vector::new(vec![-roots[0], 1.0]);
    }

    let mid = num_roots / 2;
    let roots_left = &roots[..mid];
    let roots_right = &roots[mid..];

    let left_coeff = &find_polynomial_coefficient_fast(roots_left);
    let right_coeff = &find_polynomial_coefficient_fast(roots_right);

    conv_with_dft_for_f64(left_coeff, right_coeff)
}

use std::collections::HashMap;

/// 二項係数の計算結果をキャッシュするための構造体
pub struct BinomialCoeffsCache {
    // (x+1)^k の結果を保存するキャッシュ
    plus_one_cache: HashMap<usize, Vec<f64>>,
    // (x-1)^k の結果を保存するキャッシュ
    minus_one_cache: HashMap<usize, Vec<f64>>,
}

impl Default for BinomialCoeffsCache {
    fn default() -> Self {
        Self::new()
    }
}

impl BinomialCoeffsCache {
    /// 新しい空のキャッシュを作成する
    pub fn new() -> Self {
        BinomialCoeffsCache {
            plus_one_cache: HashMap::new(),
            minus_one_cache: HashMap::new(),
        }
    }

    /// (x+1)^k の係数を取得する（メモ化対応）
    pub fn get_x_plus_1(&mut self, k: usize) -> Vector<f64> {
        // 1. まずキャッシュに結果があるか確認する
        if let Some(cached_coeffs) = self.plus_one_cache.get(&k) {
            // あれば、そのクローンを返して終了
            return Vector::new(cached_coeffs.clone());
        }

        // 2. キャッシュになければ、実際に計算する
        let coeffs = if k == 0 {
            vec![1.0]
        } else {
            let mut coeffs = vec![0.0; k + 1];
            coeffs[0] = 1.0;
            for i in 1..=k {
                for j in (1..=i).rev() {
                    coeffs[j] += coeffs[j - 1];
                }
            }
            coeffs
        };

        // 3. 計算結果をキャッシュに保存してから返す
        self.plus_one_cache.insert(k, coeffs.clone());
        Vector::new(coeffs)
    }

    /// (x-1)^k の係数を取得する（メモ化対応）
    pub fn get_x_minus_1(&mut self, k: usize) -> Vector<f64> {
        // (x-1)^k は (x+1)^k の結果を流用できるので、そちらを先にチェック
        if let Some(cached_coeffs) = self.minus_one_cache.get(&k) {
            return Vector::new(cached_coeffs.clone());
        }

        // (x+1)^k の係数を取得（こちらもメモ化されている）
        let mut coeffs = self.get_x_plus_1(k);

        // 符号を入れ替える
        for i in 0..=k {
            if (k - i) % 2 != 0 {
                coeffs[i] *= -1.0;
            }
        }

        // 計算結果をキャッシュに保存してから返す
        self.minus_one_cache.insert(k, coeffs.data.clone());
        coeffs
    }
}
