use signal_processing::dft::conv_auto_f64;
use signal_processing::fir::{design_fir_highpass, design_fir_lowpass};
use signal_processing::media::audio_io;
use signal_processing::window::WindowType;
use std::fs;

fn conv_same_1d(x: &[f64], h: &[f64]) -> Vec<f64> {
    let y = conv_auto_f64(x, h);
    let n = x.len();
    let m = h.len();
    let delay = (m - 1) / 2;
    let start = delay;
    y.into_iter().skip(start).take(n).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let audio_dir = format!("{crate_dir}/audio");
    fs::create_dir_all(&audio_dir)?;

    let wav_in = format!("{audio_dir}/input.wav");
    if !std::path::Path::new(&wav_in).exists() {
        eprintln!("Input WAV not found: {wav_in} (run generate_media_inputs example first)");
        return Ok(());
    }

    let (audio_v, info) = audio_io::load_wav_mono_to_vec(&wav_in)?;

    // FIR設計パラメータ
    let taps = 101usize; // 奇数
    let win = WindowType::Hamming;
    // 正規化カットオフ（Nyquist=0.5基準）
    let lp_cut = 0.12; // 低域通過
    let hp_cut = 0.12; // 高域通過

    let h_lp = design_fir_lowpass(taps, lp_cut, win);
    let h_hp = design_fir_highpass(taps, hp_cut, win);

    let y_lp = conv_same_1d(&audio_v, &h_lp);
    let y_hp = conv_same_1d(&audio_v, &h_hp);

    let out_lp = format!("{audio_dir}/output_lowpass.wav");
    audio_io::save_wav_mono_from_vec(&out_lp, &y_lp, info.sample_rate)?;
    println!("Wrote {out_lp}");

    let out_hp = format!("{audio_dir}/output_highpass.wav");
    audio_io::save_wav_mono_from_vec(&out_hp, &y_hp, info.sample_rate)?;
    println!("Wrote {out_hp}");

    Ok(())
}
