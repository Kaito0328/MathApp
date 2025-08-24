use signal_processing::image::{
    convolution::Border,
    core::Image,
    filter::{bilateral_filter_u8, gaussian_blur_u8, median_filter_u8, unsharp_mask_u8},
};
use std::fs;
use std::path::Path;

fn pick_png(crate_dir: &str) -> Option<String> {
    let in_dir = Path::new(crate_dir).join("img_in");
    for e in fs::read_dir(&in_dir).ok()?.flatten() {
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
fn color_filter_pipeline() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let Some(path) = pick_png(crate_dir) else {
        eprintln!("no img_in/png; skip");
        return;
    };
    let rgb = match Image::<[u8; 3]>::load_from_path_as_rgb(&path) {
        Ok(i) => i,
        Err(_) => return,
    };
    let (w, h) = (rgb.width(), rgb.height());
    let mut r = Image::<u8>::new(w, h);
    let mut g = Image::<u8>::new(w, h);
    let mut b = Image::<u8>::new(w, h);
    for i in 0..(w * h) {
        let [rr, gg, bb] = rgb.as_slice()[i];
        r.as_mut_slice()[i] = rr;
        g.as_mut_slice()[i] = gg;
        b.as_mut_slice()[i] = bb;
    }
    let out_dir = Path::new(crate_dir)
        .join("img")
        .join("test_out")
        .join("color");
    let _ = fs::create_dir_all(&out_dir);
    let gr = gaussian_blur_u8(&r, 1.2, 2, Border::Reflect);
    let gg = gaussian_blur_u8(&g, 1.2, 2, Border::Reflect);
    let gb = gaussian_blur_u8(&b, 1.2, 2, Border::Reflect);
    let mut out = Image::<[u8; 3]>::new(w, h);
    for i in 0..(w * h) {
        out.as_mut_slice()[i] = [gr.as_slice()[i], gg.as_slice()[i], gb.as_slice()[i]];
    }
    assert!(out
        .save_png(out_dir.join("gaussian.png").to_str().unwrap())
        .is_ok());

    // basic other ops
    let ur = unsharp_mask_u8(&r, 1.2, 2, 0.6, Border::Reflect);
    let ug = unsharp_mask_u8(&g, 1.2, 2, 0.6, Border::Reflect);
    let ub = unsharp_mask_u8(&b, 1.2, 2, 0.6, Border::Reflect);
    for i in 0..(w * h) {
        out.as_mut_slice()[i] = [ur.as_slice()[i], ug.as_slice()[i], ub.as_slice()[i]];
    }
    assert!(out
        .save_png(out_dir.join("unsharp.png").to_str().unwrap())
        .is_ok());

    let mr = median_filter_u8(&r, 1, Border::Reflect);
    let mg = median_filter_u8(&g, 1, Border::Reflect);
    let mb = median_filter_u8(&b, 1, Border::Reflect);
    for i in 0..(w * h) {
        out.as_mut_slice()[i] = [mr.as_slice()[i], mg.as_slice()[i], mb.as_slice()[i]];
    }
    assert!(out
        .save_png(out_dir.join("median.png").to_str().unwrap())
        .is_ok());

    let br = bilateral_filter_u8(&r, 2, 2.0, 20.0, Border::Reflect);
    let bg = bilateral_filter_u8(&g, 2, 2.0, 20.0, Border::Reflect);
    let bb = bilateral_filter_u8(&b, 2, 2.0, 20.0, Border::Reflect);
    for i in 0..(w * h) {
        out.as_mut_slice()[i] = [br.as_slice()[i], bg.as_slice()[i], bb.as_slice()[i]];
    }
    assert!(out
        .save_png(out_dir.join("bilateral.png").to_str().unwrap())
        .is_ok());
}
