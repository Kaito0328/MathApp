use signal_processing::signal::Signal;
use signal_processing::window::WindowType;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let img_dir = format!("{crate_dir}/img");
    let audio_dir = format!("{crate_dir}/audio");
    let _ = fs::create_dir_all(&img_dir);
    let _ = fs::create_dir_all(&audio_dir);

    // ---------- Image: grayscale + RGB demos ----------
    // Grayscale (load -> save copy)
    let img_in = format!("{img_dir}/input.png");
    if std::path::Path::new(&img_in).exists() {
        let s = Signal::from_image_grayscale(&img_in, 1.0)?;
        let (w, h) = (64u32, 64u32); // 保存例として固定のサイズ（既存ベクタをそのまま保存も可）
        let img_out = format!("{img_dir}/output_resized.png");
        s.save_image_grayscale(&img_out, w, h)?;
        println!("Wrote {img_out}");
    } else {
        println!("Skip grayscale image demo: input not found.");
    }

    // RGB (load -> save copy)
    let img_color_in = format!("{img_dir}/input_color.png");
    if std::path::Path::new(&img_color_in).exists() {
        let s = Signal::from_image_rgb(&img_color_in, 1.0)?;
        let (w, h) = (64u32, 64u32);
        let img_out = format!("{img_dir}/output_resized_color.png");
        s.save_image_rgb(&img_out, w, h)?;
        println!("Wrote {img_out}");
    } else {
        println!("Skip RGB image demo: input_color.png not found.");
    }

    // ---------- Audio: load WAV -> vector -> resample -> save ----------
    let wav_in = format!("{audio_dir}/input.wav");
    if std::path::Path::new(&wav_in).exists() {
        let audio = Signal::from_wav_mono(&wav_in)?;
        let l = 3usize;
        let m = 2usize;
        let taps = 41usize;
        let win = WindowType::Hamming;
        let audio_resamp = audio.resample(l, m, taps, win);
        let wav_out = format!("{audio_dir}/output_resampled.wav");
        audio_resamp.save_wav_mono(&wav_out)?;
        println!("Wrote {wav_out}");
    } else {
        println!("Skip audio demo: input not found.");
    }

    Ok(())
}
