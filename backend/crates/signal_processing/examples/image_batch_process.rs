use signal_processing::image::{
    convolution::Border,
    core::Image,
    filter::{gaussian_blur_u8, sobel_magnitude_u8},
};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

fn list_pngs(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_file()
                && p.extension()
                    .and_then(OsStr::to_str)
                    .map(|s| s.eq_ignore_ascii_case("png"))
                    .unwrap_or(false)
            {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

fn stem(p: &Path) -> String {
    p.file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("image")
        .to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let in_dir = Path::new(crate_dir).join("img_in");
    let out_root = Path::new(crate_dir).join("img").join("processed");
    let out_blur_dir = out_root.join("blur");
    let out_sobel_dir = out_root.join("sobel");
    fs::create_dir_all(&out_blur_dir)?;
    fs::create_dir_all(&out_sobel_dir)?;

    if !in_dir.exists() {
        eprintln!("Input directory not found: {}", in_dir.display());
        eprintln!("Place PNG images under img_in/ and re-run.");
        return Ok(());
    }

    let files = list_pngs(&in_dir);
    if files.is_empty() {
        eprintln!("No PNG files in {}", in_dir.display());
        return Ok(());
    }

    for p in files {
        // Load as grayscale using Image<u8>
        let p_str = p.to_string_lossy();
        let img = Image::<u8>::load_from_path_as_gray(&p_str)?;
        // Simple processing
        let blur = gaussian_blur_u8(&img, 1.2, 2, Border::Reflect);
        let sobel = sobel_magnitude_u8(&img, Border::Replicate);

        let name = stem(&p);
    let out_blur = out_blur_dir.join(format!("{name}_blur.png"));
    let out_sobel = out_sobel_dir.join(format!("{name}_sobel.png"));
    let out_blur_str = out_blur.to_string_lossy();
    let out_sobel_str = out_sobel.to_string_lossy();
    blur.save_png(&out_blur_str)?;
    sobel.save_png(&out_sobel_str)?;
        println!(
            "Processed {} -> {}, {}",
            p.file_name().and_then(OsStr::to_str).unwrap_or("(unknown)"),
            out_blur.display(),
            out_sobel.display()
        );
    }

    Ok(())
}
