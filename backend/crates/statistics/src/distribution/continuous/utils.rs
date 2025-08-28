use crate::distribution::continuous::core::Distribution;
use num_traits::Float; // 1. Floatトレイトをインポート

// 2. ジェネリックな型パラメータ F と D を定義
pub fn calc_quantile_newton<F, D>(
    x_guess: F, // 3. f64 -> F
    p: f64,     // 確率はf64のままでOK
    distribution: &D,
) -> F
// 4. 返り値も f64 -> F
where
    F: Float,                  // FはFloatトレイトを実装する型
    D: Distribution<Item = F>, // DはItem=FとなるDistribution
{
    const MAX_ITER: usize = 100;
    // 5. TOLなどの定数もF型に変換
    let tol = F::from(1e-12).unwrap();

    let mut x = x_guess;

    for _ in 0..MAX_ITER {
        // distribution.cdf(x)はf64を返すので、fxはf64
        let fx = distribution.cdf(x) - p;
        if fx.abs() < 1e-12 {
            // f64リテラルとの比較でOK
            break;
        }

        // distribution.pdf(x)もf64を返すので、dfxはf64
        let dfx = distribution.pdf(x);
        if dfx.abs() < 1e-100 {
            break;
        }

        let step_f64 = fx / dfx; // f64 / f64 = f64

        // 6. f64で計算したstepを、F型に変換してから減算する
        let step = F::from(step_f64).unwrap();
        x = x - step;

        if step.abs() < tol {
            break;
        }
    }

    x
}
