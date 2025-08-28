use super::core::Image;
use super::dft;

#[derive(Clone, Copy, Debug)]
pub enum Border {
    Constant(f32),
    Replicate,
    Reflect,
}

#[derive(Clone, Debug)]
pub struct Kernel<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Kernel<T> {
    pub fn from_vec(data: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(data.len(), width * height);
        Self {
            data,
            width,
            height,
        }
    }
}

#[inline]
fn sample_with_border(img: &Image<f32>, x: isize, y: isize, border: Border) -> f32 {
    let w = img.width() as isize;
    let h = img.height() as isize;
    let mut xi = x;
    let mut yi = y;
    if xi >= 0 && xi < w && yi >= 0 && yi < h {
        return img.as_slice()[yi as usize * img.width() + xi as usize];
    }
    match border {
        Border::Constant(c) => c,
        Border::Replicate => {
            if xi < 0 {
                xi = 0;
            } else if xi >= w {
                xi = w - 1;
            }
            if yi < 0 {
                yi = 0;
            } else if yi >= h {
                yi = h - 1;
            }
            img.as_slice()[yi as usize * img.width() + xi as usize]
        }
        Border::Reflect => {
            let mut xr = xi;
            let mut yr = yi;
            if w > 1 {
                while xr < 0 || xr >= w {
                    xr = if xr < 0 { -xr - 1 } else { 2 * w - xr - 1 };
                }
            } else {
                xr = 0;
            }
            if h > 1 {
                while yr < 0 || yr >= h {
                    yr = if yr < 0 { -yr - 1 } else { 2 * h - yr - 1 };
                }
            } else {
                yr = 0;
            }
            img.as_slice()[yr as usize * img.width() + xr as usize]
        }
    }
}

/// 2D 畳み込み（f32 グレースケール）素朴法（空間直接法）
pub fn convolve2d_simple_f32(src: &Image<f32>, kernel: &Kernel<f32>, border: Border) -> Image<f32> {
    let w = src.width();
    let h = src.height();
    let kw = kernel.width as isize;
    let kh = kernel.height as isize;
    let kcx = kw / 2;
    let kcy = kh / 2;
    let mut out = Image::<f32>::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let mut acc = 0.0f32;
            for j in 0..kh {
                for i in 0..kw {
                    let sx = x as isize + i - kcx;
                    let sy = y as isize + j - kcy;
                    let s = sample_with_border(src, sx, sy, border);
                    let k = kernel.data[(j as usize) * (kernel.width) + (i as usize)];
                    acc += s * k;
                }
            }
            out.as_mut_slice()[y * w + x] = acc;
        }
    }
    out
}

/// 2D 畳み込み（u8 入出力）。内部計算は f32、0..255 にクランプ。
pub fn convolve2d_simple_u8(src: &Image<u8>, kernel: &Kernel<f32>, border: Border) -> Image<u8> {
    // convert to f32
    let src32 = src.map(|v| v as f32);
    let y32 = convolve2d_simple_f32(&src32, kernel, border);
    y32.map(|v| v.round().clamp(0.0, 255.0) as u8)
}

/// 分離可能カーネルのための 1D 畳み込み（横）
pub fn convolve_horizontal_f32(src: &Image<f32>, taps: &[f32], border: Border) -> Image<f32> {
    let w = src.width();
    let h = src.height();
    let n = taps.len() as isize;
    let c = n / 2;
    let mut out = Image::<f32>::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let mut acc = 0.0;
            for i in 0..n {
                let sx = x as isize + i - c;
                let s = sample_with_border(src, sx, y as isize, border);
                acc += s * taps[i as usize];
            }
            out.as_mut_slice()[y * w + x] = acc;
        }
    }
    out
}

/// 分離可能カーネルのための 1D 畳み込み（縦）
pub fn convolve_vertical_f32(src: &Image<f32>, taps: &[f32], border: Border) -> Image<f32> {
    let w = src.width();
    let h = src.height();
    let n = taps.len() as isize;
    let c = n / 2;
    let mut out = Image::<f32>::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let mut acc = 0.0;
            for j in 0..n {
                let sy = y as isize + j - c;
                let s = sample_with_border(src, x as isize, sy, border);
                acc += s * taps[j as usize];
            }
            out.as_mut_slice()[y * w + x] = acc;
        }
    }
    out
}

// ===== FFT-based 2D convolution (zero-padded, correlation-style; same-size crop) =====

#[inline]
fn pad_image_top_left(src: &Image<f32>, pw: usize, ph: usize) -> Image<f32> {
    let mut out = Image::<f32>::new(pw, ph);
    let w = src.width().min(pw);
    let h = src.height().min(ph);
    for y in 0..h {
        let dst_row = &mut out.as_mut_slice()[y * pw..y * pw + w];
        let src_row = &src.as_slice()[y * src.width()..y * src.width() + w];
        dst_row.copy_from_slice(src_row);
    }
    out
}

#[inline]
fn pad_kernel_shift_center(kernel: &Kernel<f32>, pw: usize, ph: usize) -> Image<f32> {
    // Correlation準拠（カーネルを反転しない）で、中心(kcx,kcy)が原点(0,0)に来るよう循環シフト
    let kcx = (kernel.width as isize) / 2;
    let kcy = (kernel.height as isize) / 2;
    let mut out = Image::<f32>::new(pw, ph);
    let pw_i = pw as isize;
    let ph_i = ph as isize;
    for j in 0..kernel.height as isize {
        for i in 0..kernel.width as isize {
            let v = kernel.data[(j as usize) * kernel.width + (i as usize)];
            let xx = (i - kcx).rem_euclid(pw_i) as usize;
            let yy = (j - kcy).rem_euclid(ph_i) as usize;
            out.as_mut_slice()[yy * pw + xx] = v;
        }
    }
    out
}

#[inline]
fn complex_mul_pointwise(
    xr: &Image<f32>,
    xi: &Image<f32>,
    hr: &Image<f32>,
    hi: &Image<f32>,
) -> (Image<f32>, Image<f32>) {
    let w = xr.width();
    let h = xr.height();
    debug_assert_eq!(xi.width(), w);
    debug_assert_eq!(hr.width(), w);
    debug_assert_eq!(hi.width(), w);
    debug_assert_eq!(xi.height(), h);
    debug_assert_eq!(hr.height(), h);
    debug_assert_eq!(hi.height(), h);
    let mut yr = Image::<f32>::new(w, h);
    let mut yi = Image::<f32>::new(w, h);
    let a = xr.as_slice();
    let b = xi.as_slice();
    let c = hr.as_slice();
    let d = hi.as_slice();
    for idx in 0..(w * h) {
        // (a+jb)*(c+jd) = (ac-bd) + j(ad+bc)
        let re = a[idx] * c[idx] - b[idx] * d[idx];
        let im = a[idx] * d[idx] + b[idx] * c[idx];
        yr.as_mut_slice()[idx] = re;
        yi.as_mut_slice()[idx] = im;
    }
    (yr, yi)
}

#[inline]
fn crop_same(src: &Image<f32>, w: usize, h: usize, kcx: usize, kcy: usize) -> Image<f32> {
    // "same" 取り出し: (kcx,kcy) から w×h を切り出す
    let mut out = Image::<f32>::new(w, h);
    let pw = src.width();
    let _ph = src.height();
    for y in 0..h {
        let sy = y + kcy;
        let dst_row = &mut out.as_mut_slice()[y * w..y * w + w];
        let src_row = &src.as_slice()[sy * pw + kcx..sy * pw + kcx + w];
        dst_row.copy_from_slice(src_row);
    }
    out
}

/// 2D FFT 畳み込み（f32）。ゼロパディング → 周波数領域乗算 → 逆変換 → "same" サイズ切り出し。
/// 境界はゼロ拡張相当（Borderは扱いません）。
pub fn convolve2d_fft_f32(src: &Image<f32>, kernel: &Kernel<f32>) -> Image<f32> {
    let w = src.width();
    let h = src.height();
    let kw = kernel.width;
    let kh = kernel.height;
    let out_w = w + kw - 1;
    let out_h = h + kh - 1;

    // pad
    let xpad = pad_image_top_left(src, out_w, out_h);
    let hpad = pad_kernel_shift_center(kernel, out_w, out_h);

    // DFTs
    let (xr, xi) = dft::dft2d(&xpad);
    let (hr, hi) = dft::dft2d(&hpad);

    // Multiply in frequency domain
    let (yr, yi) = complex_mul_pointwise(&xr, &xi, &hr, &hi);

    // Inverse DFT
    let y = dft::idft2d(&yr, &yi);

    // Crop to same size (align kernel center)
    let kcx = kw / 2;
    let kcy = kh / 2;
    crop_same(&y, w, h, kcx, kcy)
}

/// 2D FFT 畳み込み（u8 入出力）。内部計算は f32。
pub fn convolve2d_fft_u8(src: &Image<u8>, kernel: &Kernel<f32>) -> Image<u8> {
    let src32 = src.map(|v| v as f32);
    let y32 = convolve2d_fft_f32(&src32, kernel);
    y32.map(|v| v.round().clamp(0.0, 255.0) as u8)
}

/// カーネルサイズに応じて空間/FFTを自動選択（f32）。
/// - Border::Constant(0.0) 以外は空間畳み込みにフォールバック。
/// - カーネルが大きい場合（AREA >= THRESHOLD_AREA）はFFTを使用。
fn convolve2d_auto_f32(src: &Image<f32>, kernel: &Kernel<f32>, border: Border) -> Image<f32> {
    const THRESHOLD_AREA: usize = 31 * 31; // 簡易しきい値。必要に応じて調整/ベンチ。
    let area = kernel.width * kernel.height;
    let use_fft =
        matches!(border, Border::Constant(c) if c.abs() < f32::EPSILON) && area >= THRESHOLD_AREA;
    if use_fft {
        convolve2d_fft_f32(src, kernel)
    } else {
        convolve2d_simple_f32(src, kernel, border)
    }
}

/// カーネルサイズに応じて空間/FFTを自動選択（u8）。
fn convolve2d_auto_u8(src: &Image<u8>, kernel: &Kernel<f32>, border: Border) -> Image<u8> {
    const THRESHOLD_AREA: usize = 31 * 31;
    let area = kernel.width * kernel.height;
    let use_fft =
        matches!(border, Border::Constant(c) if c.abs() < f32::EPSILON) && area >= THRESHOLD_AREA;
    if use_fft {
        convolve2d_fft_u8(src, kernel)
    } else {
        convolve2d_simple_u8(src, kernel, border)
    }
}

/// 2D 畳み込み（f32）デフォルト：自動切替
pub fn convolve2d_f32(src: &Image<f32>, kernel: &Kernel<f32>, border: Border) -> Image<f32> {
    convolve2d_auto_f32(src, kernel, border)
}

/// 2D 畳み込み（u8）デフォルト：自動切替
pub fn convolve2d_u8(src: &Image<u8>, kernel: &Kernel<f32>, border: Border) -> Image<u8> {
    convolve2d_auto_u8(src, kernel, border)
}
