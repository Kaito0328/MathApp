use num_complex::Complex;
use std::f64::consts::PI;
pub mod error;
pub mod prelude { pub use crate::error::{FftError, Result as FftResult}; }

/// Public API: compute DFT using an automatically chosen algorithm (FFT when possible).
pub fn dft(x: &[Complex<f64>]) -> crate::error::Result<Vec<Complex<f64>>> {
    let n = x.len();
    let y = if is_power_of_two(n) {
        fft_radix2_iterative(x)
    } else {
        mixed_radix_fft(x)
    };
    Ok(y)
}

/// Checked variant of DFT (kept for compatibility with earlier staging).
pub fn dft_checked(x: &[Complex<f64>]) -> crate::error::Result<Vec<Complex<f64>>> {
    dft(x)
}

/// Inverse transform.
pub fn ift(x: &[Complex<f64>]) -> crate::error::Result<Vec<Complex<f64>>> {
    let mut y = dft(x)?;
    if y.len() > 1 {
        y[1..].reverse();
    }
    let n = x.len() as f64;
    for v in &mut y {
        *v /= n;
    }
    Ok(y)
}

/// Checked variant of inverse transform that returns a Result.
pub fn ift_checked(x: &[Complex<f64>]) -> crate::error::Result<Vec<Complex<f64>>> {
    ift(x)
}

/// Naive O(n^2) DFT (for small sizes or prime factors) - kept internal.
fn dft_naive(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = x.len();
    let mut y = Vec::with_capacity(n);
    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for (j, &xj) in x.iter().enumerate() {
            let ang = -2.0 * PI * (k * j) as f64 / n as f64;
            let w = Complex::new(ang.cos(), ang.sin());
            sum += xj * w;
        }
        y.push(sum);
    }
    y
}

fn fft_radix2_iterative(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = x.len();
    if n <= 1 {
        return x.to_vec();
    }
    let mut a = x.to_vec();
    // bit reverse
    let mut j = 0usize;
    for i in 1..(n - 1) {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }
    let mut m = 2usize;
    while m <= n {
        let theta = -2.0 * PI / m as f64;
        let wm = Complex::new(theta.cos(), theta.sin());
        for k in (0..n).step_by(m) {
            let mut w = Complex::new(1.0, 0.0);
            for j in 0..(m / 2) {
                let t = w * a[k + j + m / 2];
                let u = a[k + j];
                a[k + j] = u + t;
                a[k + j + m / 2] = u - t;
                w *= wm;
            }
        }
        m <<= 1;
    }
    a
}

fn mixed_radix_fft(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = x.len();
    if n <= 16 {
        return dft_naive(x);
    }
    let p = smallest_prime_factor(n).unwrap_or(n);
    if p == n {
        return dft_naive(x);
    }
    let m = n / p;
    let mut a_r: Vec<Vec<Complex<f64>>> = Vec::with_capacity(p);
    for r in 0..p {
        let mut seq = Vec::with_capacity(m);
        for j2 in 0..m {
            seq.push(x[j2 * p + r]);
        }
        a_r.push(mixed_radix_fft(&seq));
    }
    let mut out = vec![Complex::new(0.0, 0.0); n];
    for k1 in 0..m {
        let mut t: Vec<Complex<f64>> = Vec::with_capacity(p);
        for (r, ar) in a_r.iter().enumerate() {
            let ang = -2.0 * PI * (r * k1) as f64 / n as f64;
            let w = Complex::new(ang.cos(), ang.sin());
            t.push(ar[k1] * w);
        }
        let dft_t = dft_naive(&t);
        for k2 in 0..p {
            out[k1 + m * k2] = dft_t[k2];
        }
    }
    out
}

fn smallest_prime_factor(n: usize) -> Option<usize> {
    if n % 2 == 0 {
        return Some(2);
    }
    let mut d = 3usize;
    while d * d <= n {
        if n % d == 0 {
            return Some(d);
        }
        d += 2;
    }
    None
}

fn is_power_of_two(n: usize) -> bool {
    n != 0 && (n & (n - 1) == 0)
}
