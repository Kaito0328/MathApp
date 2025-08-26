use std::f64::consts::PI;

/// Error function `erf(x)`.
///
/// Uses a hybrid strategy for best performance and accuracy:
/// - Taylor series for small `|x|`.
/// - Asymptotic expansion of `erfc` for large `|x|`.
/// - Abramowitz and Stegun's rational approximation for the intermediate range.
pub fn erf(x: f64) -> f64 {
    let sign = x.signum();
    let abs_x = x.abs();

    let result = if abs_x < 0.5 {
        erf_taylor_positive(abs_x)
    } else if abs_x > 4.0 {
        // For large x, calculate erfc(x) and use erf(x) = 1.0 - erfc(x)
        1.0 - erfc_asymptotic_positive(abs_x)
    } else {
        // This is the most robust and efficient for the main range.
        calc_abramowitz_and_stegun_positive(abs_x)
    };

    // Apply the original sign at the end, since erf(-x) = -erf(x)
    sign * result
}

pub fn erfc(x: f64) -> f64 {
    1.0 - erf(x)
}

/// Calculates erf(x) for x >= 0 using Taylor series expansion.
/// Most effective for small x.
fn erf_taylor_positive(x: f64) -> f64 {
    let x_squared = x * x;
    let mut term = x;
    let mut sum = x;

    const MAX_ITER: usize = 100;
    const THRESHOLD: f64 = 1e-15; // Increased precision for f64

    for n in 1..MAX_ITER {
        term *= -x_squared * (2.0 * n as f64 - 1.0) / (n as f64 * (2.0 * n as f64 + 1.0));

        let old_sum = sum;
        sum += term;

        // Break if the term is too small to make a difference
        if (sum - old_sum).abs() < THRESHOLD {
            break;
        }
    }

    sum * 2.0 / PI.sqrt()
}

/// Calculates complementary error function erfc(x) for x > 0 using asymptotic expansion.
/// Most effective for large x.
fn erfc_asymptotic_positive(x: f64) -> f64 {
    let x_squared = x * x;
    let mut term = 1.0;
    let mut sum = 1.0;

    const MAX_ITER: usize = 30; // Asymptotic series converges fast or diverges
    const THRESHOLD: f64 = 1e-15;

    for n in 1..MAX_ITER {
        // Update term using the ratio: term_n = term_{n-1} * -(2n-1)/(2x^2)
        term *= -(2.0 * n as f64 - 1.0) / (2.0 * x_squared);

        let old_sum = sum;
        sum += term;

        if (sum - old_sum).abs() < THRESHOLD {
            break;
        }
    }

    sum * (-x_squared).exp() / (x * PI.sqrt())
}

/// Calculates erf(x) for x >= 0 using the famous Abramowitz and Stegun approximation 7.1.26.
/// Provides excellent accuracy (error < 1.5e-7) across its effective range.
fn calc_abramowitz_and_stegun_positive(x: f64) -> f64 {
    // These are the "magic" constants for the approximation
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let t = 1.0 / (1.0 + p * x);

    // The core of the approximation
    let poly = ((((a5 * t + a4) * t + a3) * t + a2) * t + a1) * t;
    1.0 - poly * (-x * x).exp()
}

pub fn erf_inv(y: f64) -> f64 {
    let phi_inv = calc_quantile_acklam((y + 1.0) / 2.0);
    phi_inv / 2.0_f64.sqrt()
}

const P_LOW: f64 = 0.02425;
const C0: f64 = 2.515517;
const C1: f64 = 0.802853;
const C2: f64 = 0.010328;
const D1: f64 = 1.432788;
const D2: f64 = 0.189269;
const D3: f64 = 0.001308;
const A0: f64 = -39.69683028665376;
const A1: f64 = 220.9460984245205;
const A2: f64 = -275.9285104469687;
const A3: f64 = 138.357751867269;
const B1: f64 = -54.47609879822406;
const B2: f64 = 161.5858368580409;
const B3: f64 = -155.6989798598866;
const B4: f64 = 66.80131188771972;

pub fn calc_quantile_acklam(p: f64) -> f64 {
    if !(0.0..=1.0).contains(&p) {
        panic!("Input probability must be in [0, 1]");
    }
    if p == 0.0 {
        return f64::NEG_INFINITY;
    }
    if p == 1.0 {
        return f64::INFINITY;
    }
    if p == 0.5 {
        return 0.0;
    }

    if p > 0.5 {
        return -calc_quantile_acklam(1.0 - p);
    }

    if p < P_LOW {
        let q = (-2.0 * p.ln()).sqrt();

        let numerator = C0 + q * (C1 + q * C2);
        let denominator = 1.0 + q * (D1 + q * (D2 + q * D3));
        return -numerator / denominator;
    }

    let q = p - 0.5;
    let r = q * q;

    let numerator = A0 + r * (A1 + r * (A2 + r * A3));
    let denominator = 1.0 + r * (B1 + r * (B2 + r * (B3 + r * B4)));

    q * numerator / denominator
}
