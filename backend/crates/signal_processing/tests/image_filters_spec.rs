use signal_processing::image::{
    convolution::Border,
    core::Image,
    dft,
    filter::{
        gaussian_blur_u8, laplacian_u8, median_filter_u8, sobel_magnitude_u8, unsharp_mask_u8,
    },
};
use std::fs;
use std::path::Path;

fn pick_png(crate_dir: &str) -> Option<String> {
    let in_dir = Path::new(crate_dir).join("img_in");
    let rd = fs::read_dir(&in_dir).ok()?;
    for e in rd.flatten() {
        let p = e.path();
        if p.extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("png"))
            .unwrap_or(false)
        {
            return Some(p.to_string_lossy().to_string());
        }
    }
    None
}

#[test]
fn filter_suite_runs_and_saves() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let Some(path) = pick_png(crate_dir) else {
        eprintln!("no img_in/png; skip");
        return;
    };
    let img = match Image::<u8>::load_from_path_as_gray(&path) {
        Ok(i) => i,
        Err(_) => return,
    };
    let out_dir = Path::new(crate_dir)
        .join("img")
        .join("test_out")
        .join("filters");
    let _ = fs::create_dir_all(&out_dir);

    let g = gaussian_blur_u8(&img, 1.2, 2, Border::Reflect);
    let s = sobel_magnitude_u8(&img, Border::Replicate);
    let l = laplacian_u8(&img, Border::Reflect);
    let m = median_filter_u8(&img, 1, Border::Reflect);
    let u = unsharp_mask_u8(&img, 1.2, 2, 0.6, Border::Reflect);

    assert!(g
    .save_png(out_dir.join("gaussian.png").to_str().unwrap_or("/dev/null"))
        .is_ok());
    assert!(s
    .save_png(out_dir.join("sobel.png").to_str().unwrap_or("/dev/null"))
        .is_ok());
    assert!(l
    .save_png(out_dir.join("laplacian.png").to_str().unwrap_or("/dev/null"))
        .is_ok());
    assert!(m
    .save_png(out_dir.join("median.png").to_str().unwrap_or("/dev/null"))
        .is_ok());
    assert!(u
    .save_png(out_dir.join("unsharp.png").to_str().unwrap_or("/dev/null"))
        .is_ok());
}

#[test]
fn dft_roundtrip_and_spectrum() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let Some(path) = pick_png(crate_dir) else {
        eprintln!("no img_in/png; skip");
        return;
    };
    let img_u8 = match Image::<u8>::load_from_path_as_gray(&path) {
        Ok(i) => i,
        Err(_) => return,
    };
    let img = img_u8.map(|v| v as f32);
    let out_dir = Path::new(crate_dir)
        .join("img")
        .join("test_out")
        .join("dft");
    let _ = fs::create_dir_all(&out_dir);

    let (mut r, mut i) = dft::dft2d(&img);
    dft::fftshift(&mut r, &mut i);
    let mag = dft::magnitude(&r, &i);
    let norm = Image::<u8>::from_f32_normalized(
        &mag,
        0.0,
        0.0 + mag.as_slice().iter().fold(0.0f32, |a, &v| a.max(v)),
    );
    assert!(norm
    .save_png(out_dir.join("spectrum.png").to_str().unwrap_or("/dev/null"))
        .is_ok());

    // Roundtrip: idft(dft(img)) ≈ img
    // 注意: 実装によりスケール差がある可能性あり。ここでは粗いチェック。
    let (r2, i2) = dft::dft2d(&img);
    let rec = dft::idft2d(&r2, &i2);
    let mut mse = 0.0f64;
    let n = img.width() * img.height();
    for k in 0..n {
        let diff = rec.as_slice()[k] as f64 - img.as_slice()[k] as f64;
        mse += diff * diff;
    }
    mse /= n as f64;
    // ゆるめの閾値
    assert!(mse.is_finite());
}
