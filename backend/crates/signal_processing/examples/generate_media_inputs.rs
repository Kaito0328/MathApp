use image::{GrayImage, Luma, Rgb, RgbImage};
use signal_processing::media::audio_io;
use std::fs;

fn gen_image(path: &str, w: u32, h: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = GrayImage::new(w, h);
    // simple gradient + checkerboard mix
    for y in 0..h {
        for x in 0..w {
            let gx = (x as f64) / (w.saturating_sub(1).max(1) as f64);
            let gy = (y as f64) / (h.saturating_sub(1).max(1) as f64);
            let cb = if ((x / 16) + (y / 16)) % 2 == 0 {
                32
            } else {
                0
            };
            let v = ((0.5 * gx + 0.5 * (1.0 - gy)) * 255.0) as u32 + cb as u32;
            let v = v.min(255) as u8;
            img.put_pixel(x, y, Luma([v]));
        }
    }
    img.save(path)?;
    Ok(())
}

fn gen_color_image(path: &str, w: u32, h: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = RgbImage::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let fx = x as f64 / (w.saturating_sub(1).max(1) as f64);
            let fy = y as f64 / (h.saturating_sub(1).max(1) as f64);
            let r = (fx * 255.0).round() as u8; // 横方向グラデーション
            let g = (fy * 255.0).round() as u8; // 縦方向グラデーション
                                                // 斜めストライプ＋チェッカー風
            let stripes = (((x + y) / 16) % 2) * 48; // 0 or 48
            let b = ((fx * 0.5 + (1.0 - fy) * 0.5) * 255.0).round() as i32 + stripes as i32;
            let b = b.clamp(0, 255) as u8;
            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    img.save(path)?;
    Ok(())
}

fn gen_audio(path: &str, sr: u32, seconds: f64) -> Result<(), Box<dyn std::error::Error>> {
    let n = (sr as f64 * seconds).round() as usize;
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f64 / sr as f64;
        // 440 Hz tone + 880 Hz at lower amplitude
        let s = (2.0 * std::f64::consts::PI * 440.0 * t).sin()
            + 0.3 * (2.0 * std::f64::consts::PI * 880.0 * t).sin();
        v.push((s * 0.5).clamp(-1.0, 1.0));
    }
    let vec = linalg::Vector::new(v);
    audio_io::save_wav_mono_from_vec(path, &vec, sr)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let img_dir = format!("{crate_dir}/img");
    let audio_dir = format!("{crate_dir}/audio");
    fs::create_dir_all(&img_dir)?;
    fs::create_dir_all(&audio_dir)?;

    let img_path = format!("{img_dir}/input.png");
    gen_image(&img_path, 256, 192)?;
    println!("Wrote {img_path}");

    let img_color_path = format!("{img_dir}/input_color.png");
    gen_color_image(&img_color_path, 256, 192)?;
    println!("Wrote {img_color_path}");

    let wav_path = format!("{audio_dir}/input.wav");
    gen_audio(&wav_path, 16000, 2.0)?;
    println!("Wrote {wav_path}");

    Ok(())
}
