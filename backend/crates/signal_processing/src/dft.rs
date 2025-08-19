use linalg::Vector;
use num_complex::Complex;
use std::f64::consts::PI;

pub fn dft_simple(x: &Vector<Complex<f64>>) -> Vector<Complex<f64>> {
    let n = x.dim();
    let mut dft_result = Vec::with_capacity(n);
    for i in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for j in 0..n {
            let angle = -2.0 * std::f64::consts::PI * (i as f64 * j as f64) / (n as f64);
            let w = Complex::new(angle.cos(), angle.sin());
            sum += x.iter().nth(j).unwrap() * w;
        }
        dft_result.push(sum);
    }
    Vector::new(dft_result)
}

fn base_ift(dft_result: &mut [Complex<f64>], n: usize) {
    if dft_result.len() > 1 {
        dft_result[1..].reverse();
    }

    let n_float = n as f64;
    for i in dft_result.iter_mut() {
        *i /= n_float;
    }
}

pub fn ift_simple(x: &Vector<Complex<f64>>) -> Vector<Complex<f64>> {
    let mut dft_result = dft_simple(x).data;
    base_ift(&mut dft_result, x.dim());
    Vector::new(dft_result)
}

// 公開API: 入力長が2のべき乗ならCooley–Tukey, それ以外は混合基数FFT
pub fn dft(x: &Vector<Complex<f64>>) -> Vector<Complex<f64>> {
    let n = x.dim();
    if is_power_of_two(n) {
        dft_cooley_tukey(x)
    } else {
        mixed_radix_fft(x)
    }
}

pub fn ift(x: &Vector<Complex<f64>>) -> Vector<Complex<f64>> {
    let mut dft_result = dft(x).data;
    base_ift(&mut dft_result, x.dim());
    Vector::new(dft_result)
}

pub fn conv_with_dft(x: &Vector<Complex<f64>>, h: &Vector<Complex<f64>>) -> Vector<Complex<f64>> {
    let n = x.dim() + h.dim() - 1;
    let mut x_padded = x.data.clone();
    x_padded.resize(n, Complex::new(0.0, 0.0));
    let mut h_padded = h.data.clone();
    h_padded.resize(n, Complex::new(0.0, 0.0));

    let dft_x = dft(&Vector::new(x_padded));
    let dft_h = dft(&Vector::new(h_padded));

    let dft_result = dft_x * dft_h;

    ift(&dft_result)
}

pub fn conv_with_dft_for_f64(x: &Vector<f64>, h: &Vector<f64>) -> Vector<f64> {
    let complex_x: Vec<Complex<f64>> = x.iter().map(|&v| Complex::new(v, 0.0)).collect();
    let complex_h: Vec<Complex<f64>> = h.iter().map(|&v| Complex::new(v, 0.0)).collect();
    let result = conv_with_dft(&Vector::new(complex_x), &Vector::new(complex_h));
    Vector::new(result.data.iter().map(|c| c.re).collect())
}

fn dft_cooley_tukey(x: &Vector<Complex<f64>>) -> Vector<Complex<f64>> {
    let n = x.dim();
    if n == 0 {
        return Vector::new(vec![]);
    }
    if n == 1 {
        return x.clone();
    }
    // 反復型ラディックス2 FFT（n は 2 のべき乗を仮定）
    let mut a: Vec<Complex<f64>> = x.iter().cloned().collect();
    // ビット反転置換
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
    // 段階ごとの蝶演算
    let mut m = 2usize;
    while m <= n {
        let theta = -2.0 * PI / (m as f64);
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
    Vector::new(a)
}

fn mixed_radix_fft(x: &Vector<Complex<f64>>) -> Vector<Complex<f64>> {
    // Convert Vector -> Vec for internal recursion
    let data: Vec<Complex<f64>> = x.iter().cloned().collect();
    let y = fft_mixed_radix_recursive(&data);
    Vector::new(y)
}

fn fft_mixed_radix_recursive(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = x.len();
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![x[0]];
    }
    // For small n or prime n, fall back to O(n^2) DFT for simplicity/stability
    if n <= 16 {
        return dft_naive_vec(x);
    }
    // find a small prime factor p of n
    let p = smallest_prime_factor(n).unwrap_or(n);
    if p == n {
        // n is prime, use naive DFT
        return dft_naive_vec(x);
    }
    let m = n / p;
    // Stage 1: compute m-point FFTs of p stride sequences
    let mut a_r: Vec<Vec<Complex<f64>>> = Vec::with_capacity(p);
    for r in 0..p {
        let mut seq = Vec::with_capacity(m);
        for j2 in 0..m {
            seq.push(x[j2 * p + r]);
        }
        a_r.push(fft_mixed_radix_recursive(&seq));
    }
    // Stage 2: combine with twiddle and p-point DFT across r
    let mut out = vec![Complex::new(0.0, 0.0); n];
    for k1 in 0..m {
        // precompute twiddle for W_n^{r*k1}
        let mut t: Vec<Complex<f64>> = Vec::with_capacity(p);
        for (r, a_r_r) in a_r.iter().enumerate() {
            let angle = -2.0 * PI * (r * k1) as f64 / n as f64;
            let w = Complex::new(angle.cos(), angle.sin());
            t.push(a_r_r[k1] * w);
        }
        // compute p-point DFT of t to fill bins for k2=0..p-1
        let dft_t = dft_naive_vec(&t);
        for k2 in 0..p {
            out[k1 + m * k2] = dft_t[k2];
        }
    }
    out
}

fn dft_naive_vec(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = x.len();
    let mut y = Vec::with_capacity(n);
    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for (j, x_j) in x.iter().enumerate() {
            let angle = -2.0 * PI * (k * j) as f64 / n as f64;
            let w = Complex::new(angle.cos(), angle.sin());
            sum += x_j * w;
        }
        y.push(sum);
    }
    y
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
