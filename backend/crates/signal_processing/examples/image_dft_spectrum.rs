use signal_processing::image::{core::Image, dft};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let in_dir = Path::new(crate_dir).join("img_in");
    let out_dir = Path::new(crate_dir)
        .join("img")
        .join("processed")
        .join("spectrum");
    fs::create_dir_all(&out_dir)?;

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
        eprintln!("img_in missing or no PNG; skipping example.");
        return Ok(());
    };

    // Load gray and convert to f32
    let img_u8 = Image::<u8>::load_from_path_as_gray(&path)?;
    let img = img_u8.map(|v| v as f32);
    let (mut r, mut i) = dft::dft2d(&img);
    dft::fftshift(&mut r, &mut i);
    let mut mag = dft::magnitude(&r, &i);
    // Log scale to enhance visibility: log(1 + |F|)
    for v in mag.as_mut_slice().iter_mut() {
        let vv = (*v).max(0.0);
        *v = (1.0 + vv).ln();
    }
    // Robust min/max via percentiles (2%..98%)
    let mut vals: Vec<f32> = mag
        .as_slice()
        .iter()
        .copied()
        .filter(|x| x.is_finite())
        .collect();
    if vals.is_empty() {
        eprintln!("Spectrum has no finite values; skipping save.");
        return Ok(());
    }
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let lo = vals[(vals.len() as f32 * 0.02) as usize];
    let hi = vals[(vals.len() as f32 * 0.98) as usize];
    let norm = Image::<u8>::from_f32_normalized(&mag, lo, hi);
    let out = out_dir.join("spectrum.png");
    norm.save_png(out.to_str().unwrap())?;
    println!("Wrote {}", out.display());
    Ok(())
}
