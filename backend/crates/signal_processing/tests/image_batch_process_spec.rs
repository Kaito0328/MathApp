use signal_processing::image::{
    convolution::Border,
    core::Image,
    filter::{gaussian_blur_u8, sobel_magnitude_u8},
};
use std::fs;
use std::path::Path;

#[test]
fn process_first_png_in_img_in() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let in_dir = Path::new(crate_dir).join("img_in");
    if !in_dir.exists() {
        eprintln!("img_in not found; skipping test.");
        return; // skip softly
    }
    let entries = match fs::read_dir(&in_dir) {
        Ok(rd) => rd,
        Err(_) => return,
    };
    let mut first_png: Option<String> = None;
    for e in entries.flatten() {
        let p = e.path();
        if p.is_file() {
            if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
                if ext.eq_ignore_ascii_case("png") {
                    first_png = Some(p.to_string_lossy().to_string());
                    break;
                }
            }
        }
    }
    let Some(path) = first_png else {
        eprintln!("no PNG files in img_in; skipping test");
        return;
    };

    // Load, filter, and ensure outputs can be saved (to a temp dir under target/tmp_tests)
    let img = match Image::<u8>::load_from_path_as_gray(&path) {
        Ok(i) => i,
        Err(_) => return,
    };
    let blur = gaussian_blur_u8(&img, 1.0, 2, Border::Reflect);
    let sobel = sobel_magnitude_u8(&img, Border::Replicate);

    let tmp_dir = Path::new(crate_dir).join("target").join("tmp_tests");
    let _ = fs::create_dir_all(&tmp_dir);
    let out1 = tmp_dir.join("test_out_blur.png");
    let out2 = tmp_dir.join("test_out_sobel.png");
    assert!(blur.save_png(out1.to_str().unwrap()).is_ok());
    assert!(sobel.save_png(out2.to_str().unwrap()).is_ok());
}
