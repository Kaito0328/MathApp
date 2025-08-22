use crate::signal::Signal;

// -------- Image <-> Vector (grayscale normalized [0,1]) --------
pub mod image_io {
    use image::{imageops::FilterType, DynamicImage, GrayImage, Luma, Rgb, RgbImage};

    pub fn load_grayscale_to_vec(path: &str) -> Result<(Vec<f64>, u32, u32), image::ImageError> {
        let img = image::open(path)?; // supports png/jpeg via features
        let gray = img.to_luma8();
        let (w, h) = gray.dimensions();
        let data: Vec<f64> = gray.pixels().map(|Luma([v])| (*v as f64) / 255.0).collect();
    Ok((data, w, h))
    }

    pub fn save_vec_to_grayscale(
        path: &str,
    v: &[f64],
        width: u32,
        height: u32,
    ) -> Result<(), image::ImageError> {
    assert_eq!(v.len(), (width * height) as usize);
        let mut img = GrayImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize;
        let mut val = v[idx];
                if val.is_nan() {
                    val = 0.0;
                }
                let clamped = (val.clamp(0.0, 1.0) * 255.0).round() as u8;
                img.put_pixel(x, y, Luma([clamped]));
            }
        }
        img.save(path)
    }

    pub fn resize_grayscale_vec(
    v: &[f64],
        w: u32,
        h: u32,
        new_w: u32,
        new_h: u32,
    ) -> Vec<f64> {
        let mut img = GrayImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let idx = (y * w + x) as usize;
        let mut vv = v[idx];
                if vv.is_nan() {
                    vv = 0.0;
                }
                let val = (vv.clamp(0.0, 1.0) * 255.0).round() as u8;
                img.put_pixel(x, y, Luma([val]));
            }
        }
        let dyn_img = DynamicImage::ImageLuma8(img);
        let resized = dyn_img
            .resize_exact(new_w, new_h, FilterType::Lanczos3)
            .to_luma8();
    let data: Vec<f64> = resized
            .pixels()
            .map(|Luma([v])| (*v as f64) / 255.0)
            .collect();
    data
    }

    // ---------- RGB (interleaved) <-> Vector (normalized [0,1]) ----------
    // Layout: [R0,G0,B0, R1,G1,B1, ...], length = width*height*3
    pub fn load_rgb_to_vec(path: &str) -> Result<(Vec<f64>, u32, u32), image::ImageError> {
        let img = image::open(path)?;
        let rgb = img.to_rgb8();
        let (w, h) = rgb.dimensions();
        let mut data = Vec::with_capacity((w * h * 3) as usize);
        for p in rgb.pixels() {
            let [r, g, b] = p.0;
            data.push(r as f64 / 255.0);
            data.push(g as f64 / 255.0);
            data.push(b as f64 / 255.0);
        }
    Ok((data, w, h))
    }

    pub fn save_vec_to_rgb(
        path: &str,
    v: &[f64],
        width: u32,
        height: u32,
    ) -> Result<(), image::ImageError> {
    assert_eq!(v.len(), (width * height * 3) as usize);
        let mut img = RgbImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let base = ((y * width + x) * 3) as usize;
                let mut r_f = v[base];
                let mut g_f = v[base + 1];
                let mut b_f = v[base + 2];
                if r_f.is_nan() {
                    r_f = 0.0;
                }
                if g_f.is_nan() {
                    g_f = 0.0;
                }
                if b_f.is_nan() {
                    b_f = 0.0;
                }
                let r = (r_f.clamp(0.0, 1.0) * 255.0).round() as u8;
                let g = (g_f.clamp(0.0, 1.0) * 255.0).round() as u8;
                let b = (b_f.clamp(0.0, 1.0) * 255.0).round() as u8;
                img.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        img.save(path)
    }

    pub fn resize_rgb_vec(v: &[f64], w: u32, h: u32, new_w: u32, new_h: u32) -> Vec<f64> {
        assert_eq!(v.len(), (w * h * 3) as usize);
        let mut img = RgbImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let base = ((y * w + x) * 3) as usize;
                let mut r_f = v[base];
                let mut g_f = v[base + 1];
                let mut b_f = v[base + 2];
                if r_f.is_nan() {
                    r_f = 0.0;
                }
                if g_f.is_nan() {
                    g_f = 0.0;
                }
                if b_f.is_nan() {
                    b_f = 0.0;
                }
                let r = (r_f.clamp(0.0, 1.0) * 255.0).round() as u8;
                let g = (g_f.clamp(0.0, 1.0) * 255.0).round() as u8;
                let b = (b_f.clamp(0.0, 1.0) * 255.0).round() as u8;
                img.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        let dyn_img = DynamicImage::ImageRgb8(img);
        let resized = dyn_img
            .resize_exact(new_w, new_h, FilterType::Lanczos3)
            .to_rgb8();
        let mut data = Vec::with_capacity((new_w * new_h * 3) as usize);
        for p in resized.pixels() {
            let [r, g, b] = p.0;
            data.push(r as f64 / 255.0);
            data.push(g as f64 / 255.0);
            data.push(b as f64 / 255.0);
        }
        data
    }
}

// -------- Audio <-> Vector (WAV mono f64 normalized [-1,1]) --------
pub mod audio_io {
    use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
    use std::fs::File;
    use std::io::BufWriter;

    pub struct WavInfo {
        pub sample_rate: u32,
        pub channels: u16,
        pub len_samples: usize,
    }

    pub fn load_wav_mono_to_vec(path: &str) -> Result<(Vec<f64>, WavInfo), hound::Error> {
        let mut reader = WavReader::open(path)?;
        let spec = reader.spec();
        let samples: Vec<f64> = match (spec.sample_format, spec.bits_per_sample) {
            (SampleFormat::Int, 16) => reader
                .samples::<i16>()
                .map(|s| (s.unwrap() as f64) / i16::MAX as f64)
                .collect(),
            (SampleFormat::Int, 24) => reader
                .samples::<i32>()
                .map(|s| (s.unwrap() as f64) / (1i64 << 23) as f64)
                .collect(),
            (SampleFormat::Int, 32) => reader
                .samples::<i32>()
                .map(|s| (s.unwrap() as f64) / i32::MAX as f64)
                .collect(),
            (SampleFormat::Float, 32) => {
                reader.samples::<f32>().map(|s| s.unwrap() as f64).collect()
            }
            _ => {
                // Fallback: read as i16
                reader
                    .samples::<i16>()
                    .map(|s| (s.unwrap() as f64) / i16::MAX as f64)
                    .collect()
            }
        };
        let info = WavInfo {
            sample_rate: spec.sample_rate,
            channels: spec.channels,
            len_samples: samples.len(),
        };
    Ok((samples, info))
    }

    pub fn save_wav_mono_from_vec(
        path: &str,
        v: &[f64],
        sample_rate: u32,
    ) -> Result<(), hound::Error> {
        let spec = WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };
        let writer = WavWriter::create(path, spec)?;
        write_i16_norm(writer, v)
    }

    fn write_i16_norm(
        mut writer: WavWriter<BufWriter<File>>,
        v: &[f64],
    ) -> Result<(), hound::Error> {
        for &s in v.iter() {
            let mut s = s;
            if s.is_nan() {
                s = 0.0;
            }
            let s_clamped = s.clamp(-1.0, 1.0);
            let i = (s_clamped * i16::MAX as f64) as i16;
            writer.write_sample(i)?;
        }
        writer.finalize()
    }
}

// -------- Public impls for Signal media I/O (moved from signal.rs) --------
impl Signal {
    /// 画像のグレースケールをベクトル化して 1D 信号に詰める（行優先）。値は[0,1]。
    pub fn from_image_grayscale(path: &str, sample_rate: f64) -> Result<Signal, image::ImageError> {
    let (v, _w, _h) = image_io::load_grayscale_to_vec(path)?;
    Ok(Signal::new(v, sample_rate))
    }

    /// 画像のRGB(インターリーブ)を 1D 信号に詰める。値は[0,1]、長さは w*h*3。
    pub fn from_image_rgb(path: &str, sample_rate: f64) -> Result<Signal, image::ImageError> {
    let (v, _w, _h) = image_io::load_rgb_to_vec(path)?;
    Ok(Signal::new(v, sample_rate))
    }

    /// WAV(モノラル)から読み込み。
    pub fn from_wav_mono(path: &str) -> Result<Signal, hound::Error> {
    let (v, info) = audio_io::load_wav_mono_to_vec(path)?;
    Ok(Signal::new(v, info.sample_rate as f64))
    }

    /// WAV(モノラル)として保存。内部値[-1,1]を 16bit PCM に量子化。
    pub fn save_wav_mono(&self, path: &str) -> Result<(), hound::Error> {
    audio_io::save_wav_mono_from_vec(path, self.data(), self.sample_rate() as u32)
    }

    /// グレースケール画像として保存。自己データ長は width*height を想定（[0,1] 範囲）。
    pub fn save_image_grayscale(
        &self,
        path: &str,
        width: u32,
        height: u32,
    ) -> Result<(), image::ImageError> {
    image_io::save_vec_to_grayscale(path, self.data(), width, height)
    }

    /// RGB画像として保存。自己データ長は width*height*3 を想定（[0,1] 範囲、R,G,B の順）。
    pub fn save_image_rgb(
        &self,
        path: &str,
        width: u32,
        height: u32,
    ) -> Result<(), image::ImageError> {
    image_io::save_vec_to_rgb(path, self.data(), width, height)
    }
}
