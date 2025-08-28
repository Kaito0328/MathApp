use signal_processing::media::image_io;
use std::fs;

// 1D 畳み込み（same出力）: 出力サイズは入力と同じ。端はゼロパディング相当のシフトで中央合わせ。
fn conv_same_1d(x: &[f64], h: &[f64]) -> Vec<f64> {
    let n = x.len();
    let m = h.len();
    let pad = (m - 1) / 2;
    let mut y = vec![0.0; n + m - 1];
    for i in 0..n {
        for k in 0..m {
            y[i + k] += x[i] * h[k];
        }
    }
    y.into_iter().skip(pad).take(n).collect()
}

// 簡易セパラブルぼかし（ボックスフィルタ）。
fn box_blur_channel(channel: &[f64], w: u32, h: u32, radius: usize) -> Vec<f64> {
    let n = (w * h) as usize;
    debug_assert_eq!(channel.len(), n);
    let size = 2 * radius + 1;
    let coeff = vec![1.0 / size as f64; size];

    // 横方向
    let mut out = vec![0.0; n];
    for y in 0..h as usize {
        let row = &channel[(y * w as usize)..((y + 1) * w as usize)];
        let conv = conv_same_1d(row, &coeff);
        out[(y * w as usize)..((y + 1) * w as usize)].copy_from_slice(&conv);
    }
    let mid = out;

    // 縦方向
    let mut out2 = vec![0.0; n];
    for x in 0..w as usize {
        let mut col = Vec::with_capacity(h as usize);
        for y in 0..h as usize {
            col.push(mid[y * w as usize + x]);
        }
        let conv = conv_same_1d(&col, &coeff);
        for y in 0..h as usize {
            out2[y * w as usize + x] = conv[y];
        }
    }
    out2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let img_dir = format!("{crate_dir}/img");
    fs::create_dir_all(&img_dir)?;

    let img_in = format!("{img_dir}/input.png");
    if !std::path::Path::new(&img_in).exists() {
        eprintln!("Input image not found: {img_in} (run generate_media_inputs example first)");
        return Ok(());
    }

    // RGBとして読み込み→各チャンネルに簡易ぼかし→保存
    let (v, w, h) = image_io::load_rgb_to_vec(&img_in)?;
    let mut r = Vec::with_capacity((w * h) as usize);
    let mut g = Vec::with_capacity((w * h) as usize);
    let mut b = Vec::with_capacity((w * h) as usize);
    for i in 0..(w * h) as usize {
        r.push(v[i * 3]);
        g.push(v[i * 3 + 1]);
        b.push(v[i * 3 + 2]);
    }
    let r = r;
    let g = g;
    let b = b;

    let radius = 2usize; // 5x5 ボックス
    let r_blur = box_blur_channel(&r, w, h, radius);
    let g_blur = box_blur_channel(&g, w, h, radius);
    let b_blur = box_blur_channel(&b, w, h, radius);

    // アンシャープマスク風の簡易シャープ化：sharp = orig + alpha*(orig - blur)
    let alpha = 0.6;
    let mut rgb_out = Vec::with_capacity((w * h * 3) as usize);
    for i in 0..(w * h) as usize {
        let rr = (r[i] + alpha * (r[i] - r_blur[i])).clamp(0.0, 1.0);
        let gg = (g[i] + alpha * (g[i] - g_blur[i])).clamp(0.0, 1.0);
        let bb = (b[i] + alpha * (b[i] - b_blur[i])).clamp(0.0, 1.0);
        rgb_out.push(rr);
        rgb_out.push(gg);
        rgb_out.push(bb);
    }

    let out_sharp = format!("{img_dir}/output_sharpened.png");
    image_io::save_vec_to_rgb(&out_sharp, &rgb_out, w, h)?;
    println!("Wrote {out_sharp}");

    // ぼかし結果自体も保存（低域通過の可視化）
    let mut rgb_blur = Vec::with_capacity((w * h * 3) as usize);
    for i in 0..(w * h) as usize {
        rgb_blur.push(r_blur[i]);
        rgb_blur.push(g_blur[i]);
        rgb_blur.push(b_blur[i]);
    }
    let out_blur = format!("{img_dir}/output_blurred.png");
    image_io::save_vec_to_rgb(&out_blur, &rgb_blur, w, h)?;
    println!("Wrote {out_blur}");

    Ok(())
}
