use signal_processing::image::{
    convolution::Border,
    core::Image,
    filter::{bilateral_filter_u8, gaussian_blur_u8, median_filter_u8, unsharp_mask_u8},
};
use std::fs;
use std::path::Path;

fn pick_first_png(in_dir: &Path) -> Option<String> {
    if let Ok(rd) = fs::read_dir(in_dir) {
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
    }
    None
}

fn split_rgb(img: &Image<[u8; 3]>) -> (Image<u8>, Image<u8>, Image<u8>) {
    let w = img.width();
    let h = img.height();
    let mut r = Image::<u8>::new(w, h);
    let mut g = Image::<u8>::new(w, h);
    let mut b = Image::<u8>::new(w, h);
    for i in 0..(w * h) {
        let [rr, gg, bb] = img.as_slice()[i];
        r.as_mut_slice()[i] = rr;
        g.as_mut_slice()[i] = gg;
        b.as_mut_slice()[i] = bb;
    }
    (r, g, b)
}

fn merge_rgb(r: &Image<u8>, g: &Image<u8>, b: &Image<u8>) -> Image<[u8; 3]> {
    let w = r.width();
    let h = r.height();
    assert_eq!(g.width(), w);
    assert_eq!(b.width(), w);
    assert_eq!(g.height(), h);
    assert_eq!(b.height(), h);
    let mut out = Image::<[u8; 3]>::new(w, h);
    for i in 0..(w * h) {
        out.as_mut_slice()[i] = [r.as_slice()[i], g.as_slice()[i], b.as_slice()[i]];
    }
    out
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let in_dir = Path::new(crate_dir).join("img_in");
    let out_dir = Path::new(crate_dir)
        .join("img")
        .join("processed")
        .join("color");
    fs::create_dir_all(&out_dir)?;

    let Some(path) = pick_first_png(&in_dir) else {
        eprintln!("img_in missing or no PNG; skipping example.");
        return Ok(());
    };

    // Load RGB
    let img_rgb = Image::<[u8; 3]>::load_from_path_as_rgb(&path)?;
    let (r, g, b) = split_rgb(&img_rgb);

    // Gaussian blur per channel
    let r_g = gaussian_blur_u8(&r, 1.4, 2, Border::Reflect);
    let g_g = gaussian_blur_u8(&g, 1.4, 2, Border::Reflect);
    let b_g = gaussian_blur_u8(&b, 1.4, 2, Border::Reflect);
    merge_rgb(&r_g, &g_g, &b_g).save_png(out_dir.join("gaussian.png").to_str().unwrap())?;

    // Unsharp mask per channel
    let r_u = unsharp_mask_u8(&r, 1.4, 2, 0.6, Border::Reflect);
    let g_u = unsharp_mask_u8(&g, 1.4, 2, 0.6, Border::Reflect);
    let b_u = unsharp_mask_u8(&b, 1.4, 2, 0.6, Border::Reflect);
    merge_rgb(&r_u, &g_u, &b_u).save_png(out_dir.join("unsharp.png").to_str().unwrap())?;

    // Median denoise per channel
    let r_m = median_filter_u8(&r, 1, Border::Reflect);
    let g_m = median_filter_u8(&g, 1, Border::Reflect);
    let b_m = median_filter_u8(&b, 1, Border::Reflect);
    merge_rgb(&r_m, &g_m, &b_m).save_png(out_dir.join("median.png").to_str().unwrap())?;

    // Bilateral (parameters are empirical for 8-bit range)
    let r_b = bilateral_filter_u8(&r, 2, 2.0, 20.0, Border::Reflect);
    let g_b = bilateral_filter_u8(&g, 2, 2.0, 20.0, Border::Reflect);
    let b_b = bilateral_filter_u8(&b, 2, 2.0, 20.0, Border::Reflect);
    merge_rgb(&r_b, &g_b, &b_b).save_png(out_dir.join("bilateral.png").to_str().unwrap())?;

    // Sobel magnitude on luminance (grayscale proxy), then replicate to RGB for visualization
    // Convert RGB to grayscale via DynamicImage to access to_luma8
    let gray_img = image::DynamicImage::ImageRgb8(img_rgb.to_rgb_image()).to_luma8();
    let gray = Image::<u8>::from_gray_image(gray_img);
    let sobel = signal_processing::image::filter::sobel_magnitude_u8(&gray, Border::Replicate);
    let mut sobel_rgb = Image::<[u8; 3]>::new(gray.width(), gray.height());
    for i in 0..(gray.width() * gray.height()) {
        let v = sobel.as_slice()[i];
        sobel_rgb.as_mut_slice()[i] = [v, v, v];
    }
    sobel_rgb.save_png(out_dir.join("sobel_gray.png").to_str().unwrap())?;

    println!("Wrote color filter outputs to {}", out_dir.display());
    Ok(())
}
