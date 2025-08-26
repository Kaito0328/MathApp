use crate::gamma::log_gamma;

pub fn beta(a: f64, b: f64) -> f64 {
    if !(a.is_finite() && b.is_finite()) {
        return f64::NAN;
    }
    if a <= 0.0 || b <= 0.0 {
        return f64::NAN; // Beta is defined for a>0, b>0 in statistics context
    }
    log_beta(a, b).exp()
}

pub fn log_beta(a: f64, b: f64) -> f64 {
    if !(a.is_finite() && b.is_finite()) {
        return f64::NAN;
    }
    if a <= 0.0 || b <= 0.0 {
        return f64::NAN; // restrict to positive shapes
    }
    log_gamma(a) + log_gamma(b) - log_gamma(a + b)
}

/// Regularized Incomplete Beta function I_x(a, b)
pub fn regularized_beta(a: f64, b: f64, x: f64) -> f64 {
    if !(a.is_finite() && b.is_finite() && x.is_finite()) {
        return f64::NAN;
    }
    if a <= 0.0 || b <= 0.0 || !(0.0..=1.0).contains(&x) {
        return f64::NAN;
    }
    if x == 0.0 {
        return 0.0;
    }
    if x == 1.0 {
        return 1.0;
    }

    // Use symmetry for faster convergence and better numerical stability
    // I_x(a,b) = 1 - I_{1-x}(b,a)
    // Common heuristic: branch if x > a/(a+b)
    if x > a / (a + b) {
        return 1.0 - regularized_beta(b, a, 1.0 - x);
    }

    // log(B(a,b)) for the normalization factor
    let log_beta_ab = log_gamma(a) + log_gamma(b) - log_gamma(a + b);

    // Normalization factor part
    let factor = (a * x.ln() + b * (1.0 - x).ln() - log_beta_ab).exp();

    // Lentz's method for the continued fraction
    const MAX_ITER: usize = 200;
    const EPS: f64 = 1e-15;
    const FPMIN: f64 = 1e-300;

    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;

    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < FPMIN {
        d = FPMIN;
    }
    d = 1.0 / d;
    let mut h = d;

    for m in 1..=MAX_ITER {
        let m_f64 = m as f64;

        // Even term
        let term_even = m_f64 * (b - m_f64) * x / ((qam + 2.0 * m_f64) * (a + 2.0 * m_f64));
        d = 1.0 + term_even * d;
        if d.abs() < FPMIN {
            d = FPMIN;
        }
        c = 1.0 + term_even / c;
        if c.abs() < FPMIN {
            c = FPMIN;
        }
        d = 1.0 / d;
        h *= d * c;

        // Odd term
        let term_odd = -(a + m_f64) * (qab + m_f64) * x / ((a + 2.0 * m_f64) * (qap + 2.0 * m_f64));
        d = 1.0 + term_odd * d;
        if d.abs() < FPMIN {
            d = FPMIN;
        }
        c = 1.0 + term_odd / c;
        if c.abs() < FPMIN {
            c = FPMIN;
        }
        d = 1.0 / d;
        let delta = d * c;
        h *= delta;

        if (delta - 1.0).abs() < EPS {
            break;
        }
    }

    factor * h / a
}
