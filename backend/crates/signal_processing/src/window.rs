use linalg::Vector;
use std::f64::consts::PI;

/// 使用する窓関数の種類を定義する列挙型。
#[derive(Clone, Copy, Debug)]
pub enum WindowType {
    Hann,
    Hamming,
    /// カイザー窓。性能を調整するためのbetaパラメータを持つ。
    Kaiser {
        beta: f64,
    },
    Blackman,
    Rectangular,
}

/// 指定された種類と長さの窓関数を生成する。
///
/// # 引数
/// * `size` - 生成する窓の長さ（サンプル数）。
/// * `window_type` - `WindowType` enum で指定する窓の種類。
pub fn generate_window(size: usize, window_type: WindowType) -> Vector<f64> {
    // エッジケース: 長さが0または1の場合
    if size == 0 {
        return Vector::new(vec![]);
    }
    if size == 1 {
        return Vector::new(vec![1.0]);
    }

    let mut window = Vec::with_capacity(size);
    let size_minus_1 = (size - 1) as f64;

    // カイザー窓の場合、分母となるI₀(β)は定数なのでループの前に一度だけ計算する。
    let kaiser_denominator = if let WindowType::Kaiser { beta } = window_type {
        // 前提条件: betaは非負の値（>= 0.0）であること。
        Some(calc_bessel_i0(beta))
    } else {
        None
    };

    for i in 0..size {
        // インデックス`i`を [0.0, 1.0] の範囲に正規化する。
        let x = i as f64 / size_minus_1;
        let value = match window_type {
            WindowType::Hann => 0.5 * (1.0 - (2.0 * PI * x).cos()),
            WindowType::Hamming => 0.54 - 0.46 * (2.0 * PI * x).cos(),
            WindowType::Kaiser { beta } => {
                // xを [-1.0, 1.0] の範囲に変換し、カイザー窓の公式に適用する。
                let t = 2.0 * x - 1.0;
                calc_bessel_i0(beta * (1.0 - t * t).sqrt()) / kaiser_denominator.unwrap()
            }
            WindowType::Blackman => 0.42 - 0.5 * (2.0 * PI * x).cos() + 0.08 * (4.0 * PI * x).cos(),
            WindowType::Rectangular => 1.0,
        };
        window.push(value);
    }

    Vector::new(window)
}

/// 阻止域減衰量からカイザー窓に必要なbetaパラメータを計算する。
///
/// # 引数
/// * `attenuation` - 目標とする阻止域減衰量。
/// # 前提条件: 単位はデシベル(dB)で、正の値であること。例: 60.0
pub fn calc_beta(attenuation: f64) -> f64 {
    match attenuation {
        a if a > 50.0 => 0.1102 * (a - 8.7),
        a if (21.0..=50.0).contains(&a) => 0.5842 * (a - 21.0).powf(0.4) + 0.07886 * (a - 21.0),
        _ => 0.0,
    }
}

/// 遷移帯域幅と阻止域減衰量から、カイザー窓に必要な窓の長さ(N)を計算する。
///
/// # 引数
/// * `bandwidth` - 目標とする遷移帯域幅。
/// # 前提条件: 正規化された周波数で、単位はラジアン/サンプルであること。
/// - (0.0 < bandwidth < PI)
/// - 例: サンプリング周波数の10%の幅なら、0.1 * PI
/// * `attenuation` - 目標とする阻止域減衰量(dB)。
pub fn calc_window_size(bandwidth: f64, attenuation: f64) -> usize {
    // カイザーの経験式を用いて必要な窓の長さを計算する。
    let size = (attenuation - 8.0) / (2.285 * bandwidth) + 1.0;

    // 計算結果が1未満の場合は最低でも1を返す。
    if size < 1.0 {
        return 1;
    }

    // usizeへのキャストは切り捨てとなる。
    // 仕様を確実に満たすためには、切り上げ(ceil)が望ましい場合もある。
    size as usize
}
pub fn calc_bessel_i0(x: f64) -> f64 {
    if x.abs() <= 3.75 {
        return calc_bessel_i0_rational(x);
    }
    calc_bessel_i0_asymptotic(x.abs())
}
pub fn calc_bessel_i0_base(x: f64) -> f64 {
    // x/2 の2乗はループ内で不変なので、最初に計算しておく
    let x_squared_over_4 = (x / 2.0).powi(2);

    let mut k = 1;
    let mut sum = 1.0; // k=0 の項 (値は 1.0)
    let mut term = 1.0; // k=0 の項の値

    const THRESHOLD: f64 = 1e-15;
    const MAX_ITERATIONS: usize = 1000;

    while k < MAX_ITERATIONS {
        // 正しい漸化式で次の項を計算
        // term_k = term_{k-1} * (x/2)^2 / k^2
        term *= x_squared_over_4 / (k * k) as f64;
        // 新しい項を合計に加える
        sum += term;

        // 収束判定
        if term < THRESHOLD {
            break;
        }
        k += 1;
    }

    sum
}

fn calc_bessel_i0_rational(x: f64) -> f64 {
    // 事前条件: xの絶対値が3.75以下であること
    let t = x / 3.75;
    let t_squared = t * t;

    // P = 1.0 + t²(3.515...) + t⁴(3.089...) + ...
    1.0 + t_squared
        * (3.5156229
            + t_squared
                * (3.0899424
                    + t_squared
                        * (1.2067492
                            + t_squared
                                * (0.2659732 + t_squared * (0.0360768 + t_squared * 0.0045813)))))
}

fn calc_bessel_i0_asymptotic(x: f64) -> f64 {
    // 事前条件: x は正の数であること (呼び出し側で abs() を取る)

    // 逆数 1/x を先に計算しておくと効率が良い
    let inv_x = 1.0 / x;

    let series = 1.0
        + inv_x
            * (0.125
                + inv_x
                    * (
                        // 1/8 = 0.125
                        0.0703125
                            + inv_x
                                * (
                                    // 9/128 = 0.0703125
                                    0.0732421875
                                    // 225/3072 = 0.0732421875
                                )
                    ));

    // e^x / sqrt(2*pi*x) の部分を計算
    let main_factor = x.exp() / (2.0 * PI * x).sqrt();

    // 最終的な結果を返す
    main_factor * series
}
