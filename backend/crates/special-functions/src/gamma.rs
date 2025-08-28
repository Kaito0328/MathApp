use std::f64::consts::PI;

// Lanczos approximation (g=7, coefficients from common references)
const G: f64 = 7.0;
#[allow(clippy::excessive_precision, clippy::inconsistent_digit_grouping)]
const C: [f64; 9] = [
    0.99999999999980993,
    676.5203681218851,
    -1259.1392167224028,
    771.32342877765313,
    -176.61502916214059,
    12.507343278686905,
    -0.13857109526572012,
    9.984369578019571e-6,
    1.5056327351493116e-7,
];

pub fn gamma(z: f64) -> f64 {
    if z.is_nan() {
        return f64::NAN;
    }
    if z < 0.5 {
        // Reflection formula: Γ(z) = π / (sin(πz) Γ(1-z))
        return PI / ((PI * z).sin() * gamma(1.0 - z));
    }
    lanczos_gamma(z)
}

fn lanczos_gamma(z: f64) -> f64 {
    // Use standard Lanczos form with shift by 1
    let x = z - 1.0;
    let mut a = C[0];
    for (i, &c) in C.iter().enumerate().skip(1) {
        a += c / (x + i as f64);
    }
    let t = x + G + 0.5;
    (2.0 * PI).sqrt() * t.powf(x + 0.5) * (-t).exp() * a
}

pub fn log_gamma(z: f64) -> f64 {
    if z < 0.5 {
        // log Γ(z) = log π - log sin(πz) - log Γ(1-z)
        return PI.ln() - (PI * z).sin().ln() - log_gamma(1.0 - z);
    }
    let x = z - 1.0;
    let mut a = C[0];
    for (i, &c) in C.iter().enumerate().skip(1) {
        a += c / (x + i as f64);
    }
    let t = x + G + 0.5;
    0.5 * (2.0 * PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
}

#[allow(dead_code)]
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

// 下側正規化ガンマ関数 P(s, x) = γ(s, x) / Γ(s)
// s > 0, x >= 0 を主対象とする。
pub fn regularized_gamma(s: f64, x: f64) -> f64 {
    if s <= 0.0 || x < 0.0 || s.is_nan() || x.is_nan() {
        return f64::NAN;
    }
    if x == 0.0 {
        return 0.0;
    }
    let lg = log_gamma(s);
    if x < s + 1.0 {
        // 級数展開（下側 γ）
        let mut sum = 1.0 / s;
        let mut term = sum;
        const EPS: f64 = 1e-15;
        for n in 1..=10_000 {
            term *= x / (s + n as f64);
            sum += term;
            if term.abs() < sum.abs() * EPS {
                break;
            }
        }
        (s * x.ln() - lg - x).exp() * sum
    } else {
        // 連分数展開（上側 Q）→ P = 1 - Q
        let q = upper_gamma(s, x, lg);
        1.0 - q
    }
}

// 上側正規化ガンマ Q(s,x) を連分数（Lentz法）で評価
fn upper_gamma(s: f64, x: f64, lg: f64) -> f64 {
    const EPS: f64 = 1e-15;
    const FPMIN: f64 = 1e-300;
    // Lentz's algorithm initialization
    let mut b = x + 1.0 - s;
    let mut c = 1.0 / FPMIN;
    let mut d = 1.0 / b.max(FPMIN);
    let mut h = d;
    for i in 1..=10_000 {
        // a_i = -i (s - i)
        let a_i = -(i as f64) * (s - i as f64);
        b += 2.0;
        // d = 1 / (b + a_i * d)
        d = 1.0 / (b + a_i * d).max(FPMIN);
        // c = b + a_i / c
        c = (b + a_i / c).max(FPMIN);
        let delta = c * d;
        h *= delta;
        if (delta - 1.0).abs() < EPS {
            break;
        }
    }
    (s * x.ln() - x - lg).exp() * h
}
