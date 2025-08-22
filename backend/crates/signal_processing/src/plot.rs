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
    let w = width as f64;
    let h = height as f64;
    let margin_l = 48.0;
    let margin_r = 16.0;
    let margin_t = 16.0;
    let margin_b = 40.0;
    let plot_w = (w - margin_l - margin_r).max(1.0);
    let plot_h = (h - margin_t - margin_b).max(1.0);

    writeln!(f, "<svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}' viewBox='0 0 {width} {height}'>")?;
    writeln!(f, "<rect width='100%' height='100%' fill='white' />")?;

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
        writeln!(f, "<line x1='{:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' stroke='{grid_color}' stroke-width='1' />", margin_l, y, w - margin_r, y)?;
        writeln!(f, "<text x='{:.1}' y='{:.1}' font-size='10' fill='{text_color}' text-anchor='end' dominant-baseline='middle'>{:.3}</text>", margin_l - 6.0, y, y_val)?;
    }

    // X 軸目盛
    for i in 0..n_ticks {
        let t = i as f64 / (n_ticks - 1) as f64;
        let x = margin_l + t * plot_w;
        let x_val = x_to_val(t * x_max);
        writeln!(f, "<line x1='{:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' stroke='{grid_color}' stroke-width='1' />", x, margin_t, x, margin_t + plot_h)?;
        writeln!(f, "<text x='{:.1}' y='{:.1}' font-size='10' fill='{text_color}' text-anchor='middle'>{:.3}</text>", x, h - margin_b + 14.0, x_val)?;
    }

    // 軸線
    writeln!(f, "<rect x='{margin_l:.1}' y='{margin_t:.1}' width='{plot_w:.1}' height='{plot_h:.1}' fill='none' stroke='{axis_color}' stroke-width='1.5' />")?;
    writeln!(f, "<text x='{:.1}' y='{:.1}' font-size='11' fill='{text_color}' text-anchor='middle'>{}</text>", margin_l + plot_w / 2.0, h - 6.0, x_label)?;

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
            f,
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
                write!(f, "{x:.2},{y:.2} ")?;
            }
        }
        writeln!(f, "' />")?;
    }

    // 凡例（右上）
    let legend_x = margin_l + plot_w - 8.0;
    let mut legend_y = margin_t + 8.0;
    for (idx, s) in series.iter().enumerate() {
        let color = palette[idx % palette.len()];
        let y = legend_y;
        let x0 = legend_x - 120.0;
        let x1 = x0 + 18.0;
        writeln!(f, "<line x1='{x0:.1}' y1='{y:.1}' x2='{x1:.1}' y2='{y:.1}' stroke='{color}' stroke-width='2' />")?;
        writeln!(
            f,
            "<text x='{:.1}' y='{:.1}' font-size='10' fill='{text_color}'>{}</text>",
            x1 + 6.0,
            y + 3.0,
            escape_xml(s.label)
        )?;
        legend_y += 14.0;
    }

    writeln!(f, "</svg>")?;
    Ok(())
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
