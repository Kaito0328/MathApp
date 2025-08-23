use signal_processing::image::{core::Image, dft};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let img_dir = format!("{crate_dir}/img");
    fs::create_dir_all(&img_dir)?;

    // 入力画像（なければ合成チェッカーボードを作る）
    let input_path = format!("{img_dir}/input.png");
    let gray_u8 = match Image::<u8>::load_from_path_as_gray(&input_path) {
        Ok(img) => img,
        Err(_) => {
            // 256x256 チェッカーボード
            let w = 256usize;
            let h = 256usize;
            let tile = 16usize;
            let mut img = Image::<u8>::new(w, h);
            for y in 0..h {
                for x in 0..w {
                    let cx = (x / tile) % 2;
                    let cy = (y / tile) % 2;
                    let v = if (cx ^ cy) == 0 { 32 } else { 224 };
                    img.as_mut_slice()[y * w + x] = v;
                }
            }
            let synth_path = format!("{img_dir}/synthetic_checker.png");
            let _ = img.save_to_path(&synth_path);
            img
        }
    };

    // f32に変換してDFT
    let gray_f32: Image<f32> = gray_u8.map(|p| p as f32);
    let (mut real, mut imag) = dft::dft2d(&gray_f32);
    dft::fftshift(&mut real, &mut imag);
    let mag = dft::magnitude(&real, &imag);

    // 対数スケール log(1+mag) で可視化しやすく
    let logmag = mag.map(|v| (1.0 + v).ln());
    let (vmin, vmax) = {
        let mut mn = f32::INFINITY;
        let mut mx = f32::NEG_INFINITY;
        for &v in logmag.as_slice() {
            if v.is_finite() {
                if v < mn {
                    mn = v;
                }
                if v > mx {
                    mx = v;
                }
            }
        }
        if !mn.is_finite() || !mx.is_finite() || mn == mx {
            (0.0, 1.0)
        } else {
            (mn, mx)
        }
    };
    let vis = Image::<u8>::from_f32_normalized(&logmag, vmin, vmax);
    let out_path = format!("{img_dir}/dft_magnitude.png");
    vis.save_to_path(&out_path)?;
    println!("Wrote {out_path}");

    Ok(())
}
