use signal_processing::media::{audio_io, image_io};
use signal_processing::sampling::resample;
use signal_processing::window::WindowType;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let img_dir = format!("{crate_dir}/img");
    let audio_dir = format!("{crate_dir}/audio");
    let _ = fs::create_dir_all(&img_dir);
    let _ = fs::create_dir_all(&audio_dir);

    // ---------- Image: grayscale + RGB resize demos ----------
    // Grayscale
    let img_in = format!("{img_dir}/input.png");
    if std::path::Path::new(&img_in).exists() {
        let (v, w, h) = image_io::load_grayscale_to_vec(&img_in)?;
        let vv = image_io::resize_grayscale_vec(&v, w, h, w * 2, h * 2);
        let img_out = format!("{img_dir}/output_resized.png");
        image_io::save_vec_to_grayscale(&img_out, &vv, w * 2, h * 2)?;
        println!("Wrote {img_out}");
    } else {
        println!("Skip grayscale image demo: input not found.");
    }

    // RGB
    let img_color_in = format!("{img_dir}/input_color.png");
    if std::path::Path::new(&img_color_in).exists() {
        let (v, w, h) = image_io::load_rgb_to_vec(&img_color_in)?;
        let vv = image_io::resize_rgb_vec(&v, w, h, w * 2, h * 2);
        let img_out = format!("{img_dir}/output_resized_color.png");
        image_io::save_vec_to_rgb(&img_out, &vv, w * 2, h * 2)?;
        println!("Wrote {img_out}");
    } else {
        println!("Skip RGB image demo: input_color.png not found.");
    }

    // ---------- Audio: load WAV -> vector -> resample -> save ----------
    let wav_in = format!("{audio_dir}/input.wav");
    if std::path::Path::new(&wav_in).exists() {
        let (audio, info) = audio_io::load_wav_mono_to_vec(&wav_in)?;
        let l = 3usize;
        let m = 2usize;
        let taps = 41usize;
        let win = WindowType::Hamming;
        let audio_resamp = resample(&audio, l, m, taps, win);
        let out_sr = info.sample_rate * l as u32 / m as u32;
        let wav_out = format!("{audio_dir}/output_resampled.wav");
        audio_io::save_wav_mono_from_vec(&wav_out, &audio_resamp, out_sr)?;
        println!("Wrote {wav_out}");
    } else {
        println!("Skip audio demo: input not found.");
    }

    Ok(())
}
