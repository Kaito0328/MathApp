use signal_processing::image::{
    convolution::Border,
    core::Image,
    filter::{gaussian_blur_u8, unsharp_mask_u8},
};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let in_dir = Path::new(crate_dir).join("img_in");
    let out_dir = Path::new(crate_dir)
        .join("img")
        .join("processed")
        .join("unsharp");
    fs::create_dir_all(&out_dir)?;
    // Use first PNG
    let mut picked: Option<String> = None;
    if let Ok(rd) = fs::read_dir(&in_dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("png"))
                .unwrap_or(false)
            {
                picked = Some(p.to_string_lossy().to_string());
                break;
            }
        }
    }
    let Some(path) = picked else {
        eprintln!("img_in missing or no PNG found; skipping example.");
        return Ok(());
    };
    let img = Image::<u8>::load_from_path_as_gray(&path)?;
    let blur = gaussian_blur_u8(&img, 1.4, 2, Border::Reflect);
    let sharp = unsharp_mask_u8(&img, 1.4, 2, 0.6, Border::Reflect);
    let out_blur = out_dir.join("blur.png");
    let out_sharp = out_dir.join("unsharp.png");
    let out_blur_str = match out_blur.to_str() { Some(s) => s, None => return Ok(()) };
    let out_sharp_str = match out_sharp.to_str() { Some(s) => s, None => return Ok(()) };
    blur.save_png(out_blur_str)?;
    sharp.save_png(out_sharp_str)?;
    println!("Wrote {} and {}", out_blur.display(), out_sharp.display());
    Ok(())
}
