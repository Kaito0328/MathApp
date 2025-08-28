use num_complex::Complex;
use poly::polynomial::special::legendre;

// Gauss–Legendre (n点) で \int_{-1}^{1} f(x) dx を近似
fn gauss_legendre<F>(n: usize, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    // P_n と その導関数
    let p = legendre::<f64>(n);
    let dp = p.differentiate();

    // 節点: P_n の根
    let mut nodes: Vec<f64> = p
        .find_roots()
        .into_iter()
        .map(|z: Complex<f64>| z.re)
        .collect();
    nodes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    // 重み: w_i = 2 / ((1 - x_i^2) [P'_n(x_i)]^2)
    let weights: Vec<f64> = nodes
        .iter()
        .map(|&x| {
            let dpi = dp.eval(x);
            2.0 / ((1.0 - x * x) * dpi * dpi)
        })
        .collect();

    // 近似積分値
    nodes
        .iter()
        .zip(weights.iter())
        .map(|(&x, &w)| w * f(x))
        .sum()
}

fn main() {
    // 例1: f(x) = e^x の積分（-1..1）
    let exact = std::f64::consts::E - 1.0 / std::f64::consts::E;
    for n in [2usize, 4, 8, 12] {
        let approx = gauss_legendre(n, |x| x.exp());
        println!(
            "n={:>2}: approx={:.12}, error={:.3e}",
            n,
            approx,
            (approx - exact).abs()
        );
    }

    // 例2: 多項式 f(x)=x^k は 2n-1 次まで厳密（数値誤差範囲）
    let n = 5; // 5点なら次数9まで厳密
    for k in 0..=9 {
        let approx = gauss_legendre(n, |x| x.powi(k));
        // 厳密値: k が奇数なら 0、偶数なら 2/(k+1)
        let exact = if k % 2 == 1 {
            0.0
        } else {
            2.0 / (k as f64 + 1.0)
        };
        println!(
            "k={}: approx={:.12}, error={:.3e}",
            k,
            approx,
            (approx - exact).abs()
        );
    }
}
