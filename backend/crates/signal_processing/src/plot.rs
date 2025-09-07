use image::{ImageResult, Luma, Rgb, RgbImage};
use std::fs::File;
use std::io::{Result, Write};

pub struct Series<'a> {
    pub y: &'a [f64],
    pub label: &'a str,
}

/// 複数系列の時系列を SVG に描画（軸・目盛・凡例付き）。
/// sample_rate が指定されると x 軸は秒。未指定ならサンプル番号。
pub fn save_svg_time_series(
    path: &str,
    width: u32,
    height: u32,
    series: &[Series<'_>],
    sample_rate: Option<f64>,
) -> Result<()> {
    let mut f = File::create(path)?;
    write_svg_time_series_to(&mut f, width, height, series, sample_rate)
}

/// WASM等での文字列返却に使えるよう、任意の Write に出力するバージョン。
pub fn write_svg_time_series_to<W: Write>(
    mut out: W,
    width: u32,
    height: u32,
    series: &[Series<'_>],
    sample_rate: Option<f64>,
) -> Result<()> {
    let wf = width as f64;
    let hf = height as f64;
    let margin_l = 48.0;
    let margin_r = 16.0;
    let margin_t = 16.0;
    let margin_b = 40.0;
    let plot_w = (wf - margin_l - margin_r).max(1.0);
    let plot_h = (hf - margin_t - margin_b).max(1.0);

    writeln!(out, "<svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}' viewBox='0 0 {width} {height}'>")?;
    writeln!(out, "<rect width='100%' height='100%' fill='white' />")?;

    // データの統計
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut max_len = 0usize;
    for s in series {
        for &v in s.y.iter() {
            if !v.is_nan() {
                min_y = min_y.min(v);
                max_y = max_y.max(v);
            }
        }
        max_len = max_len.max(s.y.len());
    }
    if !min_y.is_finite() || !max_y.is_finite() || min_y == max_y {
        min_y = -1.0;
        max_y = 1.0;
    }

    // x 範囲
    let x_max = if max_len > 1 {
        (max_len - 1) as f64
    } else {
        1.0
    };
    let (x_label, x_to_val): (&str, Box<dyn Fn(f64) -> f64>) = if let Some(sr) = sample_rate {
        ("time (s)", Box::new(move |x| x / sr))
    } else {
        ("sample", Box::new(|x| x))
    };

    // 軸とグリッド
    let axis_color = "#888";
    let grid_color = "#eee";
    let text_color = "#333";
    let n_ticks = 5;

    // Y 軸目盛
    for i in 0..n_ticks {
        let t = i as f64 / (n_ticks - 1) as f64;
        let y_val = min_y * (1.0 - t) + max_y * t;
    let y = margin_t + (1.0 - t) * plot_h;
    writeln!(out, "<line x1='{:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' stroke='{grid_color}' stroke-width='1' />", margin_l, y, wf - margin_r, y)?;
    writeln!(out, "<text x='{:.1}' y='{:.1}' font-size='10' fill='{text_color}' text-anchor='end' dominant-baseline='middle'>{:.3}</text>", margin_l - 6.0, y, y_val)?;
    }

    // X 軸目盛
    for i in 0..n_ticks {
        let t = i as f64 / (n_ticks - 1) as f64;
    let x = margin_l + t * plot_w;
        let x_val = x_to_val(t * x_max);
    writeln!(out, "<line x1='{:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' stroke='{grid_color}' stroke-width='1' />", x, margin_t, x, margin_t + plot_h)?;
    writeln!(out, "<text x='{:.1}' y='{:.1}' font-size='10' fill='{text_color}' text-anchor='middle'>{:.3}</text>", x, hf - margin_b + 14.0, x_val)?;
    }

    // 軸線
    writeln!(out, "<rect x='{margin_l:.1}' y='{margin_t:.1}' width='{plot_w:.1}' height='{plot_h:.1}' fill='none' stroke='{axis_color}' stroke-width='1.5' />")?;
    writeln!(out, "<text x='{:.1}' y='{:.1}' font-size='11' fill='{text_color}' text-anchor='middle'>{}</text>", margin_l + plot_w / 2.0, hf - 6.0, x_label)?;

    // 色サイクル
    let palette = [
        "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd", "#8c564b", "#e377c2", "#7f7f7f",
        "#bcbd22", "#17becf",
    ];

    // データ線
    for (idx, s) in series.iter().enumerate() {
        let color = palette[idx % palette.len()];
        // polyline points
        write!(
            out,
            "<polyline fill='none' stroke='{color}' stroke-width='1.2' points='"
        )?;
        let len = s.y.len();
        if len > 1 {
            let denom = (len - 1) as f64;
            for (i, &v) in s.y.iter().enumerate() {
                let t = i as f64 / denom;
                let x = margin_l + t * plot_w;
                let vv = v.clamp(min_y, max_y);
                let ty = if max_y == min_y {
                    0.5
                } else {
                    (vv - min_y) / (max_y - min_y)
                };
                let y = margin_t + (1.0 - ty) * plot_h;
        write!(out, "{x:.2},{y:.2} ")?;
            }
        }
    writeln!(out, "' />")?;
    }

    // 凡例（右上）
    let legend_x = margin_l + plot_w - 8.0;
    let mut legend_y = margin_t + 8.0;
    for (idx, s) in series.iter().enumerate() {
        let color = palette[idx % palette.len()];
        let y = legend_y;
        let x0 = legend_x - 120.0;
        let x1 = x0 + 18.0;
    writeln!(out, "<line x1='{x0:.1}' y1='{y:.1}' x2='{x1:.1}' y2='{y:.1}' stroke='{color}' stroke-width='2' />")?;
    writeln!(
        out,
            "<text x='{:.1}' y='{:.1}' font-size='10' fill='{text_color}'>{}</text>",
            x1 + 6.0,
            y + 3.0,
            escape_xml(s.label)
        )?;
        legend_y += 14.0;
    }

    writeln!(out, "</svg>")?;
    Ok(())
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// 2D データを PNG グレースケールで保存（data は row-major、長さ = width*height）。
/// vmin..vmax に正規化（クランプあり）。invert_y=true で Y 軸を画像上で上向きに。
pub fn save_png_grayscale(
    path: &str,
    width: u32,
    height: u32,
    data: &[f64],
    vmin: f64,
    vmax: f64,
    invert_y: bool,
) -> ImageResult<()> {
    assert_eq!(data.len(), (width as usize) * (height as usize));
    let mut img = image::GrayImage::new(width, height);
    let denom = if vmax > vmin { vmax - vmin } else { 1.0 };
    for y in 0..height {
        let src_y = if invert_y { height - 1 - y } else { y } as usize;
        let row_off = src_y * (width as usize);
        for x in 0..width {
            let v = data[row_off + x as usize];
            let t = ((v - vmin) / denom).clamp(0.0, 1.0);
            let g = (t * 255.0).round() as u8;
            img.put_pixel(x, y, Luma([g]));
        }
    }
    img.save(path)
}

/// 2D データを PNG ヒートマップで保存（簡易 viridis 風の補間）。
pub fn save_png_heatmap(
    path: &str,
    width: u32,
    height: u32,
    data: &[f64],
    vmin: f64,
    vmax: f64,
    invert_y: bool,
) -> ImageResult<()> {
    assert_eq!(data.len(), (width as usize) * (height as usize));
    let mut img: RgbImage = RgbImage::new(width, height);
    let denom = if vmax > vmin { vmax - vmin } else { 1.0 };
    for y in 0..height {
        let src_y = if invert_y { height - 1 - y } else { y } as usize;
        let row_off = src_y * (width as usize);
        for x in 0..width {
            let v = data[row_off + x as usize];
            let t = ((v - vmin) / denom).clamp(0.0, 1.0) as f32;
            let (r, g, b) = viridis_approx(t);
            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    img.save(path)
}

// ごく簡易な viridis 近似（5点の線形補間）。
fn viridis_approx(t: f32) -> (u8, u8, u8) {
    // anchor colors from viridis at t ~ [0, 0.25, 0.5, 0.75, 1]
    const C: [(u8, u8, u8); 5] = [
        (68, 1, 84),    // purple
        (59, 82, 139),  // blue
        (33, 145, 140), // teal
        (94, 201, 98),  // green
        (253, 231, 37), // yellow
    ];
    let t = t.clamp(0.0, 1.0);
    let seg = (t * 4.0).floor() as usize;
    if seg >= 4 {
        return C[4];
    }
    let local = t * 4.0 - seg as f32; // in [0,1)
    let (r0, g0, b0) = C[seg];
    let (r1, g1, b1) = C[seg + 1];
    let lerp = |a: u8, b: u8| -> u8 { (a as f32 + local * (b as f32 - a as f32)).round() as u8 };
    (lerp(r0, r1), lerp(g0, g1), lerp(b0, b1))
}

/// 任意スケールの等間隔 X 軸で複数系列を SVG に描画（軸・目盛・凡例付き）。
/// x 軸の範囲は [0, x_max]、ラベルは x_label を使用します。
pub fn save_svg_series_scaled(
    path: &str,
    width: u32,
    height: u32,
    series: &[Series<'_>],
    x_label: &str,
    x_max: f64,
) -> Result<()> {
    let mut f = File::create(path)?;
    write_svg_series_scaled_to(&mut f, width, height, series, x_label, x_max)
}

/// 任意の Write に描画するバージョン。
pub fn write_svg_series_scaled_to<W: Write>(
    mut out: W,
    width: u32,
    height: u32,
    series: &[Series<'_>],
    x_label: &str,
    x_max: f64,
) -> Result<()> {
    let wf = width as f64;
    let hf = height as f64;
    let margin_l = 48.0;
    let margin_r = 16.0;
    let margin_t = 16.0;
    let margin_b = 40.0;
    let plot_w = (wf - margin_l - margin_r).max(1.0);
    let plot_h = (hf - margin_t - margin_b).max(1.0);

    writeln!(
        out,
        "<svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}' viewBox='0 0 {width} {height}'>"
    )?;
    writeln!(out, "<rect width='100%' height='100%' fill='white' />")?;

    // データの統計
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut max_len = 0usize;
    for s in series {
        for &v in s.y.iter() {
            if !v.is_nan() {
                min_y = min_y.min(v);
                max_y = max_y.max(v);
            }
        }
        max_len = max_len.max(s.y.len());
    }
    if !min_y.is_finite() || !max_y.is_finite() || min_y == max_y {
        min_y = -1.0;
        max_y = 1.0;
    }

    // 軸とグリッド
    let axis_color = "#888";
    let grid_color = "#eee";
    let text_color = "#333";
    let n_ticks = 5;

    // Y 軸目盛
    for i in 0..n_ticks {
        let t = i as f64 / (n_ticks - 1) as f64;
        let y_val = min_y * (1.0 - t) + max_y * t;
        let y = margin_t + (1.0 - t) * plot_h;
        writeln!(
            out,
            "<line x1='{:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' stroke='{grid_color}' stroke-width='1' />",
            margin_l, y, wf - margin_r, y
        )?;
        writeln!(
            out,
            "<text x='{:.1}' y='{:.1}' font-size='10' fill='{text_color}' text-anchor='end' dominant-baseline='middle'>{:.3}</text>",
            margin_l - 6.0,
            y,
            y_val
        )?;
    }

    // X 軸目盛（0..x_max）
    for i in 0..n_ticks {
        let t = i as f64 / (n_ticks - 1) as f64;
        let x = margin_l + t * plot_w;
        let x_val = t * x_max;
        writeln!(
            out,
            "<line x1='{:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' stroke='{grid_color}' stroke-width='1' />",
            x, margin_t, x, margin_t + plot_h
        )?;
        writeln!(
            out,
            "<text x='{:.1}' y='{:.1}' font-size='10' fill='{text_color}' text-anchor='middle'>{:.3}</text>",
            x, hf - margin_b + 14.0, x_val
        )?;
    }

    // 枠・ラベル
    writeln!(
    out,
        "<rect x='{margin_l:.1}' y='{margin_t:.1}' width='{plot_w:.1}' height='{plot_h:.1}' fill='none' stroke='{axis_color}' stroke-width='1.5' />"
    )?;
    writeln!(
    out,
        "<text x='{:.1}' y='{:.1}' font-size='11' fill='{text_color}' text-anchor='middle'>{}</text>",
    margin_l + plot_w / 2.0,
    hf - 6.0,
        escape_xml(x_label)
    )?;

    // 色サイクル
    let palette = [
        "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd", "#8c564b", "#e377c2", "#7f7f7f",
        "#bcbd22", "#17becf",
    ];

    // データ線
    for (idx, s) in series.iter().enumerate() {
        let color = palette[idx % palette.len()];
        write!(
            out,
            "<polyline fill='none' stroke='{color}' stroke-width='1.2' points='"
        )?;
        let len = s.y.len();
        if len > 1 {
            let denom = (len - 1) as f64;
            for (i, &v) in s.y.iter().enumerate() {
                let t = i as f64 / denom;
                let x = margin_l + t * plot_w;
                let vv = v.clamp(min_y, max_y);
                let ty = if max_y == min_y {
                    0.5
                } else {
                    (vv - min_y) / (max_y - min_y)
                };
                let y = margin_t + (1.0 - ty) * plot_h;
        write!(out, "{x:.2},{y:.2} ")?;
            }
        }
    writeln!(out, "' />")?;
    }

    // 凡例（右上）
    let legend_x = margin_l + plot_w - 8.0;
    let mut legend_y = margin_t + 8.0;
    for (idx, s) in series.iter().enumerate() {
        let color = palette[idx % palette.len()];
        let y = legend_y;
        let x0 = legend_x - 120.0;
        let x1 = x0 + 18.0;
    writeln!(out, "<line x1='{x0:.1}' y1='{y:.1}' x2='{x1:.1}' y2='{y:.1}' stroke='{color}' stroke-width='2' />")?;
    writeln!(
        out,
            "<text x='{:.1}' y='{:.1}' font-size='10' fill='{text_color}'>{}</text>",
            x1 + 6.0,
            y + 3.0,
            escape_xml(s.label)
        )?;
        legend_y += 14.0;
    }

    writeln!(out, "</svg>")?;
    Ok(())
}
