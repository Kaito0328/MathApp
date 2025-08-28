use super::core::Image;
use num_complex::Complex;
use std::borrow::Cow;

/// 1D DFT for complex input (given as separate real/imag slices), length N.
/// Returns (real, imag) of length N.
fn dft1d_complex(
    real_in: &[f32],
    imag_in_opt: Option<&[f32]>,
    inverse: bool,
) -> (Vec<f32>, Vec<f32>) {
    let n = real_in.len();
    // Keep zero buffer alive if needed using Cow to avoid temporary borrow issues
    let imag_cow: Cow<[f32]> = match imag_in_opt {
        Some(s) => Cow::Borrowed(s),
        None => Cow::Owned(vec![0.0; n]),
    };
    let imag_in: &[f32] = &imag_cow;

    // 入力をComplex<f64>に変換
    let mut x: Vec<Complex<f64>> = Vec::with_capacity(n);
    for i in 0..n {
        x.push(Complex::new(real_in[i] as f64, imag_in[i] as f64));
    }
    // 既存のFFT/DFTを使用
    let y: Vec<Complex<f64>> = if inverse {
        crate::dft::ift(&x)
    } else {
        crate::dft::dft(&x)
    };

    // f32へ戻す
    let mut real_out = vec![0.0f32; n];
    let mut imag_out = vec![0.0f32; n];
    for (i, c) in y.into_iter().enumerate() {
        real_out[i] = c.re as f32;
        imag_out[i] = c.im as f32;
    }
    (real_out, imag_out)
}

/// 2D DFT: input is real image (f32). Output is a pair (Real, Imag) as images.
pub fn dft2d(src: &Image<f32>) -> (Image<f32>, Image<f32>) {
    let w = src.width();
    let h = src.height();
    // Row-wise DFT
    let mut row_r = vec![0.0f32; w * h];
    let mut row_i = vec![0.0f32; w * h];
    for y in 0..h {
        let row = &src.as_slice()[y * w..(y + 1) * w];
        let (rr, ri) = dft1d_complex(row, None, false);
        row_r[y * w..(y + 1) * w].copy_from_slice(&rr);
        row_i[y * w..(y + 1) * w].copy_from_slice(&ri);
    }
    // Column-wise DFT for each u (frequency along x)
    let mut out_r = Image::<f32>::new(w, h);
    let mut out_i = Image::<f32>::new(w, h);
    let mut col_r = vec![0.0f32; h];
    let mut col_i = vec![0.0f32; h];
    for u in 0..w {
        for y in 0..h {
            col_r[y] = row_r[y * w + u];
            col_i[y] = row_i[y * w + u];
        }
        let (cr, ci) = dft1d_complex(&col_r, Some(&col_i), false);
        for v in 0..h {
            out_r.as_mut_slice()[v * w + u] = cr[v];
            out_i.as_mut_slice()[v * w + u] = ci[v];
        }
    }
    (out_r, out_i)
}

/// 2D inverse DFT: input real/imag images, output real image (f32).
pub fn idft2d(real: &Image<f32>, imag: &Image<f32>) -> Image<f32> {
    let w = real.width();
    let h = real.height();
    assert_eq!(imag.width(), w);
    assert_eq!(imag.height(), h);
    // Column-wise inverse DFT first (for each u)
    let mut row_r = vec![0.0f32; w * h];
    let mut row_i = vec![0.0f32; w * h];
    let mut col_r = vec![0.0f32; h];
    let mut col_i = vec![0.0f32; h];
    for u in 0..w {
        for v in 0..h {
            col_r[v] = real.as_slice()[v * w + u];
            col_i[v] = imag.as_slice()[v * w + u];
        }
        let (tr, ti) = dft1d_complex(&col_r, Some(&col_i), true);
        for y in 0..h {
            row_r[y * w + u] = tr[y];
            row_i[y * w + u] = ti[y];
        }
    }
    // Row-wise inverse DFT to get spatial domain
    let mut out = Image::<f32>::new(w, h);
    let mut row_cr = vec![0.0f32; w];
    let mut row_ci = vec![0.0f32; w];
    for y in 0..h {
        for u in 0..w {
            row_cr[u] = row_r[y * w + u];
            row_ci[u] = row_i[y * w + u];
        }
        let (sr, _si) = dft1d_complex(&row_cr, Some(&row_ci), true);
        out.as_mut_slice()[y * w..(y + 1) * w].copy_from_slice(&sr);
    }
    out
}

/// In-place FFT shift (swap quadrants so that DC is centered)
pub fn fftshift(real: &mut Image<f32>, imag: &mut Image<f32>) {
    let w = real.width();
    let h = real.height();
    assert_eq!(imag.width(), w);
    assert_eq!(imag.height(), h);
    let hw = w / 2;
    let hh = h / 2;
    // swap (Q1,Q3) and (Q2,Q4)
    for y in 0..hh {
        for x in 0..hw {
            let a = y * w + x; // top-left
            let b = (y + hh) * w + (x + hw); // bottom-right
            real.as_mut_slice().swap(a, b);
            imag.as_mut_slice().swap(a, b);
            let c = y * w + (x + hw); // top-right
            let d = (y + hh) * w + x; // bottom-left
            real.as_mut_slice().swap(c, d);
            imag.as_mut_slice().swap(c, d);
        }
    }
}

/// Magnitude spectrum (sqrt(R^2+I^2))
pub fn magnitude(real: &Image<f32>, imag: &Image<f32>) -> Image<f32> {
    let w = real.width();
    let h = real.height();
    assert_eq!(imag.width(), w);
    assert_eq!(imag.height(), h);
    let mut out = Image::<f32>::new(w, h);
    for i in 0..(w * h) {
        let r = real.as_slice()[i];
        let im = imag.as_slice()[i];
        out.as_mut_slice()[i] = (r * r + im * im).sqrt();
    }
    out
}
// Note: a generic Complex-based API can be added later if needed.
