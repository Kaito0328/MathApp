use signal_processing::image::{
    convolution::Border,
    core::Image,
    filter::{bilateral_filter_u8, median_filter_u8},
};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let in_dir = Path::new(crate_dir).join("img_in");
    let out_dir = Path::new(crate_dir).join("img").join("processed").join("denoise");
    fs::create_dir_all(&out_dir)?;
    let mut picked: Option<String> = None;
    if let Ok(rd) = fs::read_dir(&in_dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension().and_then(|s| s.to_str()).map(|s| s.eq_ignore_ascii_case("png")).unwrap_or(false) {
                picked = Some(p.to_string_lossy().to_string());
                break;
            }
        }
    }
    let Some(path) = picked else {
        eprintln!("img_in missing or no PNG; skipping example.");
        return Ok(());
    };
    let img = Image::<u8>::load_from_path_as_gray(&path)?;
    let med = median_filter_u8(&img, 1, Border::Reflect);
    let bi = bilateral_filter_u8(&img, 2, 2.0, 20.0, Border::Reflect);
    let out_med = out_dir.join("median.png");
    let out_bi = out_dir.join("bilateral.png");
    med.save_png(out_med.to_str().unwrap())?;
    bi.save_png(out_bi.to_str().unwrap())?;
    println!("Wrote {} and {}", out_med.display(), out_bi.display());
    Ok(())
}
