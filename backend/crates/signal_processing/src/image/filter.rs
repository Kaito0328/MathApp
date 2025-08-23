use super::convolution::{convolve_horizontal_f32, convolve_vertical_f32, Border, Kernel};
use super::convolve2d_auto_f32 as convolve2d_f32;
use super::core::Image;

/// ガウシアンフィルタ（分離可能）。sigma と半径 r を指定（taps 長は 2*r+1）。
pub fn gaussian_separable_taps(sigma: f32, radius: usize) -> Vec<f32> {
    let n = 2 * radius + 1;
    let s2 = 2.0 * sigma * sigma;
    let mut taps = Vec::with_capacity(n);
    let mut sum = 0.0f32;
    for i in 0..n {
        let d = i as isize - radius as isize;
        let v = (-(d as f32 * d as f32) / s2).exp();
        taps.push(v);
        sum += v;
    }
    for t in &mut taps {
        *t /= sum;
    }
    taps
}

/// ガウシアン（f32、分離適用）
pub fn gaussian_blur_f32(
    src: &Image<f32>,
    sigma: f32,
    radius: usize,
    border: Border,
) -> Image<f32> {
    let taps = gaussian_separable_taps(sigma, radius);
    let tmp = convolve_horizontal_f32(src, &taps, border);
    convolve_vertical_f32(&tmp, &taps, border)
}

/// ガウシアン（u8、内部 f32）
pub fn gaussian_blur_u8(src: &Image<u8>, sigma: f32, radius: usize, border: Border) -> Image<u8> {
    let src32 = src.map(|v| v as f32);
    let y32 = gaussian_blur_f32(&src32, sigma, radius, border);
    y32.map(|v| v.round().clamp(0.0, 255.0) as u8)
}

/// ボックスフィルタ（平均）
pub fn box_filter_f32(src: &Image<f32>, radius: usize, border: Border) -> Image<f32> {
    let n = 2 * radius + 1;
    let taps = vec![1.0f32 / n as f32; n];
    let tmp = convolve_horizontal_f32(src, &taps, border);
    convolve_vertical_f32(&tmp, &taps, border)
}

pub fn box_filter_u8(src: &Image<u8>, radius: usize, border: Border) -> Image<u8> {
    let src32 = src.map(|v| v as f32);
    let y32 = box_filter_f32(&src32, radius, border);
    y32.map(|v| v.round().clamp(0.0, 255.0) as u8)
}

/// シャープ化（アンシャープマスク）
pub fn unsharp_mask_f32(
    src: &Image<f32>,
    sigma: f32,
    radius: usize,
    amount: f32,
    border: Border,
) -> Image<f32> {
    let blurred = gaussian_blur_f32(src, sigma, radius, border);
    // dst = src + amount * (src - blurred)
    let mut out = Image::<f32>::new(src.width(), src.height());
    for (o, (&s, &b)) in out
        .as_mut_slice()
        .iter_mut()
        .zip(src.as_slice().iter().zip(blurred.as_slice().iter()))
    {
        *o = s + amount * (s - b);
    }
    out
}

pub fn unsharp_mask_u8(
    src: &Image<u8>,
    sigma: f32,
    radius: usize,
    amount: f32,
    border: Border,
) -> Image<u8> {
    let src32 = src.map(|v| v as f32);
    let y32 = unsharp_mask_f32(&src32, sigma, radius, amount, border);
    y32.map(|v| v.round().clamp(0.0, 255.0) as u8)
}

/// ソーベルエッジ（勾配強度）
pub fn sobel_magnitude_f32(src: &Image<f32>, border: Border) -> Image<f32> {
    // Gx, Gy カーネル
    let kx = Kernel::from_vec(vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0], 3, 3);
    let ky = Kernel::from_vec(vec![-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0], 3, 3);
    let gx = convolve2d_f32(src, &kx, border);
    let gy = convolve2d_f32(src, &ky, border);

    let mut out = Image::<f32>::new(src.width(), src.height());
    for i in 0..out.as_slice().len() {
        let v = (gx.as_slice()[i] * gx.as_slice()[i] + gy.as_slice()[i] * gy.as_slice()[i]).sqrt();
        out.as_mut_slice()[i] = v;
    }
    out
}

pub fn sobel_magnitude_u8(src: &Image<u8>, border: Border) -> Image<u8> {
    let src32 = src.map(|v| v as f32);
    let y32 = sobel_magnitude_f32(&src32, border);
    // 強度は広がるのでスケール調整は利用側で。ここでは単純にクリップ。
    y32.map(|v| v.round().clamp(0.0, 255.0) as u8)
}

// --- ここから追加フィルタ群: Laplacian / Median / Bilateral ---

#[inline]
fn sample_with_border_f32(img: &Image<f32>, x: isize, y: isize, border: Border) -> f32 {
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

#[inline]
fn sample_with_border_u8(img: &Image<u8>, x: isize, y: isize, border: Border) -> u8 {
    let w = img.width() as isize;
    let h = img.height() as isize;
    let mut xi = x;
    let mut yi = y;
    if xi >= 0 && xi < w && yi >= 0 && yi < h {
        return img.as_slice()[yi as usize * img.width() + xi as usize];
    }
    match border {
        Border::Constant(c) => c.round().clamp(0.0, 255.0) as u8,
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

/// ラプラシアン（4近傍: [0 -1 0; -1 4 -1; 0 -1 0]）
pub fn laplacian_f32(src: &Image<f32>, border: Border) -> Image<f32> {
    let k = Kernel::from_vec(vec![0.0, -1.0, 0.0, -1.0, 4.0, -1.0, 0.0, -1.0, 0.0], 3, 3);
    convolve2d_f32(src, &k, border)
}

pub fn laplacian_u8(src: &Image<u8>, border: Border) -> Image<u8> {
    let src32 = src.map(|v| v as f32);
    let y32 = laplacian_f32(&src32, border);
    y32.map(|v| v.round().clamp(0.0, 255.0) as u8)
}

/// メディアンフィルタ（正方ウィンドウ）
pub fn median_filter_f32(src: &Image<f32>, radius: usize, border: Border) -> Image<f32> {
    let w = src.width();
    let h = src.height();
    let mut out = Image::<f32>::new(w, h);
    let r = radius as isize;
    let size = (2 * radius + 1) as isize;
    let n = (size * size) as usize;
    let mid = n / 2;
    let mut window = Vec::<f32>::with_capacity(n);
    for y in 0..h {
        for x in 0..w {
            window.clear();
            for j in -r..=r {
                for i in -r..=r {
                    window.push(sample_with_border_f32(
                        src,
                        x as isize + i,
                        y as isize + j,
                        border,
                    ));
                }
            }
            let (_, m, _) = window.select_nth_unstable_by(mid, |a, b| a.partial_cmp(b).unwrap());
            out.as_mut_slice()[y * w + x] = *m;
        }
    }
    out
}

pub fn median_filter_u8(src: &Image<u8>, radius: usize, border: Border) -> Image<u8> {
    let w = src.width();
    let h = src.height();
    let mut out = Image::<u8>::new(w, h);
    let r = radius as isize;
    let size = (2 * radius + 1) as isize;
    let n = (size * size) as usize;
    let mid = n / 2;
    let mut window = Vec::<u8>::with_capacity(n);
    for y in 0..h {
        for x in 0..w {
            window.clear();
            for j in -r..=r {
                for i in -r..=r {
                    window.push(sample_with_border_u8(
                        src,
                        x as isize + i,
                        y as isize + j,
                        border,
                    ));
                }
            }
            let (_, m, _) = window.select_nth_unstable(mid);
            out.as_mut_slice()[y * w + x] = *m;
        }
    }
    out
}

/// バイラテラルフィルタ（f32 グレースケール）
/// radius: 近傍半径、sigma_s: 空間、sigma_r: 輝度
pub fn bilateral_filter_f32(
    src: &Image<f32>,
    radius: usize,
    sigma_s: f32,
    sigma_r: f32,
    border: Border,
) -> Image<f32> {
    let w = src.width();
    let h = src.height();
    let mut out = Image::<f32>::new(w, h);
    let rs2 = 2.0 * sigma_s * sigma_s;
    let rr2 = 2.0 * sigma_r * sigma_r;
    // 事前に空間カーネルを用意
    let r = radius as isize;
    let size = 2 * radius + 1;
    let mut spatial = vec![0.0f32; size * size];
    for j in -r..=r {
        for i in -r..=r {
            let idx = (j + radius as isize) as usize * size + (i + radius as isize) as usize;
            let d2 = (i * i + j * j) as f32;
            spatial[idx] = (-(d2) / rs2).exp();
        }
    }
    for y in 0..h {
        for x in 0..w {
            let center = sample_with_border_f32(src, x as isize, y as isize, border);
            let mut num = 0.0f32;
            let mut den = 0.0f32;
            for j in -r..=r {
                for i in -r..=r {
                    let s = sample_with_border_f32(src, x as isize + i, y as isize + j, border);
                    let diff = s - center;
                    let wr = (-(diff * diff) / rr2).exp();
                    let ws = spatial
                        [(j + radius as isize) as usize * size + (i + radius as isize) as usize];
                    let wgt = ws * wr;
                    num += wgt * s;
                    den += wgt;
                }
            }
            out.as_mut_slice()[y * w + x] = if den > 0.0 { num / den } else { center };
        }
    }
    out
}

pub fn bilateral_filter_u8(
    src: &Image<u8>,
    radius: usize,
    sigma_s: f32,
    sigma_r: f32,
    border: Border,
) -> Image<u8> {
    let src32 = src.map(|v| v as f32);
    let y32 = bilateral_filter_f32(&src32, radius, sigma_s, sigma_r, border);
    y32.map(|v| v.round().clamp(0.0, 255.0) as u8)
}
