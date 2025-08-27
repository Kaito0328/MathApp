use crate::distribution::{
    continuous::core::Distribution as ContDist, discrete::core::Distribution as DiscDist,
};

/// Options to customize SVG rendering.
#[derive(Clone, Debug)]
pub struct SvgOptions {
    /// Background color (e.g., "#ffffff").
    pub bg: &'static str,
    /// Axes color.
    pub axes: &'static str,
    /// Grid color.
    pub grid: &'static str,
    /// Show grid lines.
    pub show_grid: bool,
    /// Stroke color for continuous PDF line.
    pub line: &'static str,
    /// Fill color for discrete PMF bars.
    pub bar: &'static str,
    /// Optional x-range override.
    pub x_range: Option<(f64, f64)>,
    /// Number of samples for continuous curves.
    pub samples: usize,
}

impl Default for SvgOptions {
    fn default() -> Self {
        Self {
            bg: "#ffffff",
            axes: "#333333",
            grid: "#dddddd",
            show_grid: true,
            line: "#0a84ff",
            bar: "#34c759",
            x_range: None,
            samples: 400,
        }
    }
}

/// Render a continuous distribution's PDF to an SVG string.
/// - Chooses x-range via quantiles [q_lo, q_hi].
/// - Samples `samples` points and draws a polyline.
pub fn svg_continuous_pdf<D>(dist: &D, width: u32, height: u32, samples: usize) -> String
where
    D: ContDist<Item = f64>,
{
    // Delegate to options-based version with defaults
    let opts = SvgOptions {
        samples,
        ..Default::default()
    };
    svg_continuous_pdf_with(dist, width, height, &opts)
}

/// Render a continuous distribution's PDF to an SVG string with options.
pub fn svg_continuous_pdf_with<D>(dist: &D, width: u32, height: u32, opts: &SvgOptions) -> String
where
    D: ContDist<Item = f64>,
{
    let w = width.max(100) as f64;
    let h = height.max(80) as f64;
    let ml = 40.0; // margins
    let mr = 20.0;
    let mt = 20.0;
    let mb = 30.0;
    let iw = (w - ml - mr).max(1.0);
    let ih = (h - mt - mb).max(1.0);

    let (mut x_lo, mut x_hi) = if let Some((lo, hi)) = opts.x_range {
        (lo, hi)
    } else {
        (dist.quantile(0.001), dist.quantile(0.999))
    };
    if !x_lo.is_finite() || !x_hi.is_finite() || x_lo >= x_hi {
        // fallback range
        x_lo = -5.0;
        x_hi = 5.0;
    }
    let n = opts.samples.max(50);
    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);
    let mut y_max = 0.0f64;
    for i in 0..n {
        let t = i as f64 / (n - 1) as f64;
        let x = x_lo + t * (x_hi - x_lo);
        let y = dist.pdf(x).max(0.0);
        if y.is_finite() {
            xs.push(x);
            ys.push(y);
            if y > y_max {
                y_max = y;
            }
        } else {
            xs.push(x);
            ys.push(0.0);
        }
    }
    if y_max <= 0.0 {
        y_max = 1.0;
    }

    // Build path for polyline
    let mut d = String::new();
    for (i, (x, y)) in xs.iter().zip(ys.iter()).enumerate() {
        let sx = ml + (x - x_lo) / (x_hi - x_lo) * iw;
        let sy = mt + (1.0 - (y / y_max)) * ih;
        if i == 0 {
            d.push_str(&format!("M{sx:.2},{sy:.2}"));
        } else {
            d.push_str(&format!(" L{sx:.2},{sy:.2}"));
        }
    }

    // Background
    let bg = format!(
        "<rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\"/>",
        width, height, opts.bg
    );

    // Axes, ticks, and grid
    let mut overlay = String::new();
    overlay.push_str(&format!(
        "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1\"/>\
         <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1\"/>",
        ml, mt + ih, ml + iw, mt + ih, // x-axis
        opts.axes,
        ml, mt, ml, mt + ih,           // y-axis
        opts.axes
    ));
    let xticks = 5;
    let yticks = 4;
    for i in 0..=xticks {
        let t = i as f64 / xticks as f64;
        let x = ml + t * iw;
        if opts.show_grid {
            overlay.push_str(&format!(
                "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"1\" opacity=\"0.6\"/>",
                x, mt, x, mt + ih, opts.grid
            ));
        }
        let xv = x_lo + t * (x_hi - x_lo);
        overlay.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" font-size=\"10\" fill=\"{}\" text-anchor=\"middle\">{:.2}</text>",
            x, mt + ih + 14.0, opts.axes, xv
        ));
    }
    for j in 0..=yticks {
        let t = j as f64 / yticks as f64;
        let y = mt + (1.0 - t) * ih;
        if opts.show_grid {
            overlay.push_str(&format!(
                "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"1\" opacity=\"0.6\"/>",
                ml, y, ml + iw, y, opts.grid
            ));
        }
        let yv = t * y_max;
        overlay.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" font-size=\"10\" fill=\"{}\" text-anchor=\"end\">{:.2}</text>",
            ml - 6.0, y + 3.0, opts.axes, yv
        ));
    }

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">{}{}<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\"/></svg>",
        width, height, bg, overlay, d, opts.line
    )
}

/// Render a discrete distribution's PMF to an SVG string as bars.
/// - Chooses k-range by scanning CDF until 0.999 or a cap.
pub fn svg_discrete_pmf<D>(dist: &D, width: u32, height: u32) -> String
where
    D: DiscDist<Item = u64>,
{
    let opts = SvgOptions::default();
    svg_discrete_pmf_with(dist, width, height, &opts)
}

/// Render a discrete distribution's PMF to an SVG string as bars with options.
pub fn svg_discrete_pmf_with<D>(dist: &D, width: u32, height: u32, opts: &SvgOptions) -> String
where
    D: DiscDist<Item = u64>,
{
    let w = width.max(120) as f64;
    let h = height.max(100) as f64;
    let ml = 40.0;
    let mr = 20.0;
    let mt = 20.0;
    let mb = 30.0;
    let iw = (w - ml - mr).max(1.0);
    let ih = (h - mt - mb).max(1.0);

    // determine k range
    let mut k_max = 0u64;
    while dist.cdf(k_max) < 0.999 && k_max < 10_000 {
        k_max += 1;
    }
    if k_max == 0 {
        k_max = 10;
    }
    let k_min = 0u64;
    let span = (k_max - k_min + 1) as f64;
    let bar_w = (iw / span * 0.8).max(1.0);
    let mut y_max = 0.0f64;
    let mut pmfs: Vec<(u64, f64)> = Vec::new();
    for k in k_min..=k_max {
        let p = dist.pmf(k).max(0.0);
        if p.is_finite() {
            y_max = y_max.max(p);
        }
        pmfs.push((k, p));
    }
    if y_max <= 0.0 {
        y_max = 1.0;
    }

    // Background
    let bg = format!(
        "<rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\"/>",
        width, height, opts.bg
    );

    // Grid and axes/labels are separated so labels/axes can be drawn on top of bars.
    let mut grid = String::new();
    let mut axes_labels = String::new();
    axes_labels.push_str(&format!(
        "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1\"/>\
         <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1\"/>",
        ml, mt + ih, ml + iw, mt + ih, // x-axis
        opts.axes,
        ml, mt, ml, mt + ih,           // y-axis
        opts.axes
    ));
    let xticks = 5usize.min((span as usize).saturating_sub(1)).max(1);
    let yticks = 4;
    for i in 0..=xticks {
        let t = i as f64 / xticks as f64;
        let x = ml + t * iw;
        if opts.show_grid {
            grid.push_str(&format!(
                "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"1\" opacity=\"0.6\"/>",
                x, mt, x, mt + ih, opts.grid
            ));
        }
        let kv = k_min as f64 + t * (span - 1.0);
        axes_labels.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" font-size=\"10\" fill=\"{}\" text-anchor=\"middle\">{:.0}</text>",
            x, mt + ih + 14.0, opts.axes, kv
        ));
    }
    for j in 0..=yticks {
        let t = j as f64 / yticks as f64;
        let y = mt + (1.0 - t) * ih;
        if opts.show_grid {
            grid.push_str(&format!(
                "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"1\" opacity=\"0.6\"/>",
                ml, y, ml + iw, y, opts.grid
            ));
        }
        let yv = t * y_max;
        axes_labels.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" font-size=\"10\" fill=\"{}\" text-anchor=\"end\">{:.2}</text>",
            ml - 6.0, y + 3.0, opts.axes, yv
        ));
    }

    // Build rects
    let mut rects = String::new();
    for (k, p) in pmfs {
        let t = (k - k_min) as f64 / (span - 1.0);
        let cx = ml + t * iw;
        let bh = (p / y_max) * ih;
        let x = cx - bar_w / 2.0;
        let y = mt + (ih - bh);
        rects.push_str(&format!(
            "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"{}\"/>",
            x, y, bar_w, bh, opts.bar
        ));
    }

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\">{bg}{grid}{rects}{axes_labels}\n</svg>",
    )
}
