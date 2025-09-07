use crate::continuous::TransferFunction as ContinuousTransferFunction;
use crate::discrete::TransferFunction as DiscreteTransferFunction;
use num_complex::Complex;
use std::fs::File;
use std::io::{Result, Write};

impl DiscreteTransferFunction {
    /// Bode プロット（簡易版）: 既定の Hz 対数軸・アンラップONで `n_points` サンプル。
    pub fn plot_bode_svg_simple(
        &self,
        path: &str,
        width: u32,
        height: u32,
        n_points: usize,
    ) -> Result<()> {
        save_bode_svg(self, path, width, height, n_points)
    }

    /// Bode プロット（詳細オプション版）
    pub fn plot_bode_svg(
        &self,
        path: &str,
        width: u32,
        height: u32,
        opts: &crate::plot::DiscreteBodeOptions,
    ) -> Result<()> {
        save_bode_svg_discrete_multi(&[(self, "H(z)")], path, width, height, opts)
    }

    /// Bode プロット（複数系列）
    pub fn plot_bode_svg_multi(
        series: &[(&Self, &str)],
        path: &str,
        width: u32,
        height: u32,
        opts: &crate::plot::DiscreteBodeOptions,
    ) -> Result<()> {
        save_bode_svg_discrete_multi(series, path, width, height, opts)
    }

    /// Nyquist プロット（簡易版）: 角周波数 -π..π を `n_points` サンプル。
    pub fn plot_nyquist_svg_simple(
        &self,
        path: &str,
        width: u32,
        height: u32,
        n_points: usize,
    ) -> Result<()> {
        let opts = crate::plot::DiscreteNyquistOptions {
            n_points: n_points.max(2),
            legend: false,
            ..Default::default()
        };
        save_nyquist_svg_discrete_multi(&[(self, "H(z)")], path, width, height, &opts)
    }

    /// Nyquist プロット（詳細オプション版）
    pub fn plot_nyquist_svg(
        &self,
        path: &str,
        width: u32,
        height: u32,
        opts: &crate::plot::DiscreteNyquistOptions,
    ) -> Result<()> {
        save_nyquist_svg_discrete_multi(&[(self, "H(z)")], path, width, height, opts)
    }

    /// Nyquist プロット（複数系列）
    pub fn plot_nyquist_svg_multi(
        series: &[(&Self, &str)],
        path: &str,
        width: u32,
        height: u32,
        opts: &crate::plot::DiscreteNyquistOptions,
    ) -> Result<()> {
        save_nyquist_svg_discrete_multi(series, path, width, height, opts)
    }

    /// 単一ループブロック図（負帰還/正帰還）を SVG へ保存（離散）
    pub fn plot_block_feedback_svg(
        &self,
        path: &str,
        width: u32,
        height: u32,
        negative_feedback: bool,
        feedback_label: Option<&str>,
    ) -> Result<()> {
        let g_label = format!("{}", self.display());
        save_block_feedback_svg(
            &g_label,
            path,
            width,
            height,
            negative_feedback,
            feedback_label,
        )
    }
}

impl ContinuousTransferFunction {
    /// Bode プロット（簡易版）: f_min..f_max を対数で n_points サンプル
    pub fn plot_bode_svg_simple(
        &self,
        path: &str,
        width: u32,
        height: u32,
        f_min_hz: f64,
        f_max_hz: f64,
        n_points: usize,
    ) -> Result<()> {
        crate::plot::save_bode_svg_continuous(
            self, path, width, height, f_min_hz, f_max_hz, n_points,
        )
    }

    /// Bode プロット（詳細オプション版）
    pub fn plot_bode_svg(
        &self,
        path: &str,
        width: u32,
        height: u32,
        opts: &crate::plot::ContinuousBodeOptions,
    ) -> Result<()> {
        crate::plot::save_bode_svg_continuous_multi(&[(self, "G(s)")], path, width, height, opts)
    }

    /// Bode プロット（複数系列）
    pub fn plot_bode_svg_multi(
        series: &[(&Self, &str)],
        path: &str,
        width: u32,
        height: u32,
        opts: &crate::plot::ContinuousBodeOptions,
    ) -> Result<()> {
        crate::plot::save_bode_svg_continuous_multi(series, path, width, height, opts)
    }

    /// Nyquist プロット（簡易版）: f_min..f_max を対数で n_points サンプル
    pub fn plot_nyquist_svg_simple(
        &self,
        path: &str,
        width: u32,
        height: u32,
        f_min_hz: f64,
        f_max_hz: f64,
        n_points: usize,
    ) -> Result<()> {
        let opts = crate::plot::ContinuousNyquistOptions {
            n_points: n_points.max(2),
            f_min_hz,
            f_max_hz,
            legend: false,
            log_freq: true,
            ..Default::default()
        };
        crate::plot::save_nyquist_svg_continuous_multi(
            &[(self, "G(s)")],
            path,
            width,
            height,
            &opts,
        )
    }

    /// Nyquist プロット（詳細オプション版）
    pub fn plot_nyquist_svg(
        &self,
        path: &str,
        width: u32,
        height: u32,
        opts: &crate::plot::ContinuousNyquistOptions,
    ) -> Result<()> {
        crate::plot::save_nyquist_svg_continuous_multi(&[(self, "G(s)")], path, width, height, opts)
    }

    /// Nyquist プロット（複数系列）
    pub fn plot_nyquist_svg_multi(
        series: &[(&Self, &str)],
        path: &str,
        width: u32,
        height: u32,
        opts: &crate::plot::ContinuousNyquistOptions,
    ) -> Result<()> {
        crate::plot::save_nyquist_svg_continuous_multi(series, path, width, height, opts)
    }

    /// 単一ループブロック図（負帰還/正帰還）を SVG へ保存（連続）
    pub fn plot_block_feedback_svg(
        &self,
        path: &str,
        width: u32,
        height: u32,
        negative_feedback: bool,
        feedback_label: Option<&str>,
    ) -> Result<()> {
        let g_label = format!("{}", self.display());
        save_block_feedback_svg(
            &g_label,
            path,
            width,
            height,
            negative_feedback,
            feedback_label,
        )
    }
}

// ===== Public free functions for in-memory SVG generation (browser-friendly) =====
/// Discrete TF: Bode SVG as String
pub fn discrete_bode_svg_string(
    tf: &crate::discrete::TransferFunction,
    width: u32,
    height: u32,
    opts: &crate::plot::DiscreteBodeOptions,
) -> String {
    let mut buf: Vec<u8> = Vec::with_capacity(32 * 1024);
    let _ = write_bode_svg_discrete_multi(&[(tf, "H(z)")], &mut buf, width, height, opts);
    String::from_utf8(buf).unwrap_or_default()
}

/// Discrete TF: Nyquist SVG as String
pub fn discrete_nyquist_svg_string(
    tf: &crate::discrete::TransferFunction,
    width: u32,
    height: u32,
    opts: &crate::plot::DiscreteNyquistOptions,
) -> String {
    let mut buf: Vec<u8> = Vec::with_capacity(32 * 1024);
    let _ = write_nyquist_svg_discrete_multi(&[(tf, "H(z)")], &mut buf, width, height, opts);
    String::from_utf8(buf).unwrap_or_default()
}

/// Discrete TF: Block feedback diagram SVG as String
pub fn discrete_block_feedback_svg_string(
    tf: &crate::discrete::TransferFunction,
    width: u32,
    height: u32,
    negative_feedback: bool,
    feedback_label: Option<&str>,
) -> String {
    let mut buf: Vec<u8> = Vec::with_capacity(8 * 1024);
    let g_label = format!("{}", tf.display());
    let _ = write_block_feedback_svg(
        &g_label,
        &mut buf,
        width,
        height,
        negative_feedback,
        feedback_label,
    );
    String::from_utf8(buf).unwrap_or_default()
}

/// Continuous TF: Bode SVG as String
pub fn continuous_bode_svg_string(
    tf: &crate::continuous::TransferFunction,
    width: u32,
    height: u32,
    opts: &crate::plot::ContinuousBodeOptions,
) -> String {
    let mut buf: Vec<u8> = Vec::with_capacity(32 * 1024);
    let _ = write_bode_svg_continuous_multi(&[(tf, "G(s)")], &mut buf, width, height, opts);
    String::from_utf8(buf).unwrap_or_default()
}

/// Continuous TF: Nyquist SVG as String
pub fn continuous_nyquist_svg_string(
    tf: &crate::continuous::TransferFunction,
    width: u32,
    height: u32,
    opts: &crate::plot::ContinuousNyquistOptions,
) -> String {
    let mut buf: Vec<u8> = Vec::with_capacity(32 * 1024);
    let _ = write_nyquist_svg_continuous_multi(&[(tf, "G(s)")], &mut buf, width, height, opts);
    String::from_utf8(buf).unwrap_or_default()
}

/// Continuous TF: Block feedback diagram SVG as String
pub fn continuous_block_feedback_svg_string(
    tf: &crate::continuous::TransferFunction,
    width: u32,
    height: u32,
    negative_feedback: bool,
    feedback_label: Option<&str>,
) -> String {
    let mut buf: Vec<u8> = Vec::with_capacity(8 * 1024);
    let g_label = format!("{}", tf.display());
    let _ = write_block_feedback_svg(
        &g_label,
        &mut buf,
        width,
        height,
        negative_feedback,
        feedback_label,
    );
    String::from_utf8(buf).unwrap_or_default()
}

// ===== Block Diagram (simple single-loop) =====

fn xml_escape(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&apos;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

/// 単一ループ（前向き G、後向き H(任意)）のブロック図を描画
/// - negative_feedback: true で負帰還（サマで "+" を上、"-" を下）、false で正帰還
/// - feedback_label: Some("H") なら後向きに H ブロックを描く、None ならユニティ
fn save_block_feedback_svg(
    g_label: &str,
    path: &str,
    width: u32,
    height: u32,
    negative_feedback: bool,
    feedback_label: Option<&str>,
) -> Result<()> {
    let mut f = File::create(path)?;
    write_block_feedback_svg(g_label, &mut f, width, height, negative_feedback, feedback_label)
}

/// 単一ループ（前向き G、後向き H(任意)）のブロック図を SVG へ書き出し（任意のライター）
fn write_block_feedback_svg(
    g_label: &str,
    out: &mut dyn Write,
    width: u32,
    height: u32,
    negative_feedback: bool,
    feedback_label: Option<&str>,
) -> Result<()> {
    let w = width as f64;
    let h = height as f64;
    let mid_y = h * 0.5;

    // レイアウト定数
    let margin = 20.0;
    let sum_x = margin + 60.0;
    let sum_r = 12.0;
    let block_w = (w * 0.45).clamp(160.0, 460.0);
    let block_h = 60.0;
    let block_x = sum_x + 40.0;
    let block_y = mid_y - block_h / 2.0;
    let out_x = block_x + block_w + 40.0;

    let sign_up = "+";
    let sign_down = if negative_feedback { "-" } else { "+" };

    // SVG ヘッダ
    writeln!(
        out,
        "<svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}' viewBox='0 0 {width} {height}'>",
    )?;
    writeln!(
        out,
        "<defs><style>
    .bg {{ fill:#ffffff; }}
        .blk {{ fill:#ffffff; stroke:#222; stroke-width:1.5; }}
        .sum {{ fill:#fff; stroke:#222; stroke-width:1.5; }}
        .arrow {{ stroke:#222; stroke-width:1.5; fill:none; marker-end:url(#arrow); }}
        .wire {{ stroke:#222; stroke-width:1.5; fill:none; }}
        .text {{ font-family: 'DejaVu Sans', Arial, sans-serif; font-size:12px; fill:#222; }}
    </style>
    <marker id='arrow' markerWidth='10' markerHeight='8' refX='10' refY='4' orient='auto'>
      <path d='M 0 0 L 10 4 L 0 8 z' fill='#222'/>
    </marker></defs>"
    )?;
    // 背景（白）
    writeln!(
        out,
        "<rect class='bg' x='0' y='0' width='{width}' height='{height}' />"
    )?;

    // 入力 r → サマ
    let in_x = margin;
    writeln!(
        out,
        "<line class='arrow' x1='{in_x}' y1='{mid_y}' x2='{sum_x}' y2='{mid_y}' />"
    )?;
    writeln!(
        out,
        "<text class='text' x='{:.1}' y='{:.1}' text-anchor='start' dy='-6'>r</text>",
        in_x + 2.0,
        mid_y
    )?;

    // サマ（記号は円の外側に）
    writeln!(
        out,
        "<circle class='sum' cx='{sum_x}' cy='{mid_y}' r='{sum_r}' />"
    )?;
    let y_up = mid_y - sum_r - 4.0;
    let y_down = mid_y + sum_r + 12.0;
    // 下側記号はフィードバックの縦配線と重なりやすいので、少し右へずらす
    let x_down = sum_x + sum_r + 10.0;
    writeln!(
        out,
        "<text class='text' x='{sum_x:.1}' y='{y_up:.1}' text-anchor='middle'>{sign_up}</text>"
    )?;
    writeln!(out, "<text class='text' x='{x_down:.1}' y='{y_down:.1}' text-anchor='middle'>{sign_down}</text>")?;

    // サマ → G ブロック
    writeln!(
        out,
        "<line class='arrow' x1='{:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' />",
        sum_x + sum_r,
        mid_y,
        block_x,
        mid_y
    )?;

    // G ブロック
    writeln!(out, "<rect class='blk' x='{block_x}' y='{block_y}' width='{block_w}' height='{block_h}' rx='6' ry='6' />")?;
    let gtxt = xml_escape(g_label);
    writeln!(out, "<text class='text' x='{:.1}' y='{:.1}' text-anchor='middle' dominant-baseline='middle'>{}</text>", block_x + block_w / 2.0, mid_y - 6.0, gtxt)?;

    // 出力 → y
    writeln!(
        out,
        "<line class='arrow' x1='{:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' />",
        block_x + block_w,
        mid_y,
        out_x,
        mid_y
    )?;
    writeln!(
        out,
        "<text class='text' x='{:.1}' y='{:.1}' text-anchor='start' dy='-6'>y</text>",
        out_x + 2.0,
        mid_y
    )?;

    // フィードバック配線: 右から下、左、上
    let fb_y = mid_y + 80.0;
    // 出力ノードから下
    writeln!(
        out,
        "<path class='wire' d='M {out_x:.1} {mid_y:.1} V {fb_y:.1}' />"
    )?;
    // 右から左へ（H ブロック領域考慮）
    let fb_left_x = sum_x;
    // H ブロックがある場合は描画
    if let Some(h_label) = feedback_label {
        let h_w = 120.0;
        let h_h = 46.0;
        let h_x = block_x + block_w - h_w * 0.5; // 右側下流に配置
        let h_y = fb_y - h_h / 2.0;
        // 横線: 出力下から H 左端まで
        writeln!(
            out,
            "<line class='wire' x1='{out_x:.1}' y1='{fb_y:.1}' x2='{h_x:.1}' y2='{fb_y:.1}' />"
        )?;
        // H ブロック
        writeln!(
            out,
            "<rect class='blk' x='{h_x}' y='{h_y}' width='{h_w}' height='{h_h}' rx='6' ry='6' />"
        )?;
        let htxt = xml_escape(h_label);
        writeln!(out, "<text class='text' x='{:.1}' y='{:.1}' text-anchor='middle' dominant-baseline='middle'>{}</text>", h_x + h_w / 2.0, fb_y - 6.0, htxt)?;
        // H 右から左配線
        let hx2 = h_x + h_w;
        writeln!(
            out,
            "<line class='wire' x1='{hx2:.1}' y1='{fb_y:.1}' x2='{fb_left_x:.1}' y2='{fb_y:.1}' />"
        )?;
    } else {
        // ユニティ: そのまま左へ
        writeln!(out, "<line class='wire' x1='{out_x:.1}' y1='{fb_y:.1}' x2='{fb_left_x:.1}' y2='{fb_y:.1}' />")?;
    }
    // 上に戻る
    let mid_y_sr = mid_y + sum_r;
    writeln!(
        out,
        "<path class='wire' d='M {fb_left_x:.1} {fb_y:.1} V {mid_y_sr:.1}' />"
    )?;
    // サマへ矢印
    let mid_y_p = mid_y + 1.0;
    writeln!(out, "<line class='arrow' x1='{fb_left_x:.1}' y1='{mid_y_sr:.1}' x2='{fb_left_x:.1}' y2='{mid_y_p:.1}' />")?;

    // 終了
    writeln!(out, "</svg>")?;
    Ok(())
}

/// 色パレット（可読性の高い 10 色）
const COLORS: [&str; 10] = [
    "#1f77b4", // blue
    "#ff7f0e", // orange
    "#2ca02c", // green
    "#d62728", // red
    "#9467bd", // purple
    "#8c564b", // brown
    "#e377c2", // pink
    "#7f7f7f", // gray
    "#bcbd22", // yellow-green
    "#17becf", // cyan
];

/// Nyquist プロット設定（離散系）
#[derive(Clone, Debug)]
pub struct DiscreteNyquistOptions {
    pub n_points: usize,
    pub theta_min: f64, // rad
    pub theta_max: f64, // rad
    pub legend: bool,
    pub real_range: Option<(f64, f64)>,
    pub imag_range: Option<(f64, f64)>,
    pub show_minus_one: bool,
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
}

impl Default for DiscreteNyquistOptions {
    fn default() -> Self {
        Self {
            n_points: 512,
            theta_min: -std::f64::consts::PI,
            theta_max: std::f64::consts::PI,
            legend: true,
            real_range: None,
            imag_range: None,
            show_minus_one: true,
            title: Some("Nyquist (Discrete)".to_string()),
            x_label: Some("Re".to_string()),
            y_label: Some("Im".to_string()),
        }
    }
}

/// Nyquist プロット設定（連続系）
#[derive(Clone, Debug)]
pub struct ContinuousNyquistOptions {
    pub n_points: usize,
    pub f_min_hz: f64,
    pub f_max_hz: f64,
    pub legend: bool,
    pub real_range: Option<(f64, f64)>,
    pub imag_range: Option<(f64, f64)>,
    pub log_freq: bool,
    pub show_minus_one: bool,
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
}

impl Default for ContinuousNyquistOptions {
    fn default() -> Self {
        Self {
            n_points: 512,
            f_min_hz: 1e-2,
            f_max_hz: 1e3,
            legend: true,
            real_range: None,
            imag_range: None,
            log_freq: true,
            show_minus_one: true,
            title: Some("Nyquist (Continuous)".to_string()),
            x_label: Some("Re".to_string()),
            y_label: Some("Im".to_string()),
        }
    }
}

/// 位相アンラップ（度）: 連続に近い系列へ 360 度単位で補正
fn unwrap_phase_deg(ph: &[f64]) -> Vec<f64> {
    if ph.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(ph.len());
    let mut offset = 0.0_f64;
    let mut prev = ph[0];
    out.push(prev);
    for &p in ph.iter().skip(1) {
        let mut d = p - prev;
        while d > 180.0 {
            offset -= 360.0;
            d -= 360.0;
        }
        while d < -180.0 {
            offset += 360.0;
            d += 360.0;
        }
        out.push(p + offset);
        prev = p;
    }
    out
}

/// X 軸モード（離散系）
#[derive(Clone, Copy, Debug)]
pub enum DiscreteXAxis {
    /// 周波数 [Hz]（対数軸）
    Hz,
    /// 正規化角周波数 [rad/sample]（線形軸, 0..π）
    Radian,
    /// 正規化周波数 [cycles/sample]（線形軸, 0..0.5）
    Normalized,
}

/// Bode プロット設定（離散系）
#[derive(Clone, Debug)]
pub struct DiscreteBodeOptions {
    pub n_points: usize,
    pub x_axis: DiscreteXAxis,
    /// 位相アンラップ
    pub unwrap_phase: bool,
    /// |H| の dB 範囲（未指定なら自動）
    pub mag_db_range: Option<(f64, f64)>,
    /// 位相[deg] 範囲（未指定なら自動）
    pub phase_deg_range: Option<(f64, f64)>,
    /// 凡例の表示
    pub legend: bool,
    /// x 軸範囲（Hz のときのみ有効）
    pub f_min_hz: Option<f64>,
    pub f_max_hz: Option<f64>,
    /// x 軸範囲（Radian のときのみ有効）
    pub w_min: Option<f64>,
    pub w_max: Option<f64>,
    /// x 軸範囲（Normalized のときのみ有効, cycles/sample）
    pub norm_min: Option<f64>,
    pub norm_max: Option<f64>,
}

impl Default for DiscreteBodeOptions {
    fn default() -> Self {
        Self {
            n_points: 512,
            x_axis: DiscreteXAxis::Hz,
            unwrap_phase: true,
            mag_db_range: None,
            phase_deg_range: None,
            legend: true,
            f_min_hz: None,
            f_max_hz: None,
            w_min: None,
            w_max: None,
            norm_min: None,
            norm_max: None,
        }
    }
}

/// Bode プロット設定（連続系）
#[derive(Clone, Debug)]
pub struct ContinuousBodeOptions {
    pub n_points: usize,
    pub unwrap_phase: bool,
    pub mag_db_range: Option<(f64, f64)>,
    pub phase_deg_range: Option<(f64, f64)>,
    pub legend: bool,
    pub f_min_hz: f64,
    pub f_max_hz: f64,
}

impl Default for ContinuousBodeOptions {
    fn default() -> Self {
        Self {
            n_points: 512,
            unwrap_phase: true,
            mag_db_range: None,
            phase_deg_range: None,
            legend: true,
            f_min_hz: 1e-2,
            f_max_hz: 1e3,
        }
    }
}

/// Bode プロット（振幅[dB]/位相[deg]）を SVG に保存する簡易関数。
/// - 周波数サンプルは 0..Nyquist(=fs/2) を対数スケールで配置
/// - `n_points` はサンプル数（>= 2 推奨）
/// - 軸・凡例・グリッドを簡易描画
pub(crate) fn save_bode_svg(
    tf: &DiscreteTransferFunction,
    path: &str,
    width: u32,
    height: u32,
    n_points: usize,
) -> Result<()> {
    // 互換 API: Hz 対数軸、凡例なし、アンラップありのデフォルト
    let opts = DiscreteBodeOptions {
        n_points: n_points.max(2),
        legend: false,
        x_axis: DiscreteXAxis::Hz,
        ..Default::default()
    };
    save_bode_svg_discrete_multi(&[(tf, "H(z)")], path, width, height, &opts)
}

/// 連続伝達関数用: Bode プロット（振幅[dB]/位相[deg]）を SVG に保存。
/// f_min_hz..f_max_hz を対数スケールでサンプリング。
pub(crate) fn save_bode_svg_continuous(
    tf: &ContinuousTransferFunction,
    path: &str,
    width: u32,
    height: u32,
    f_min_hz: f64,
    f_max_hz: f64,
    n_points: usize,
) -> Result<()> {
    let opts = ContinuousBodeOptions {
        n_points: n_points.max(2),
        f_min_hz,
        f_max_hz,
        legend: false,
        ..Default::default()
    };
    save_bode_svg_continuous_multi(&[(tf, "H(s)")], path, width, height, &opts)
}

#[allow(clippy::too_many_arguments)]
fn draw_logx_series_multi(
    f: &mut dyn Write,
    x0: f64,
    y0: f64,
    w: f64,
    h: f64,
    x_hz: &[f64],
    series: &[(&[f64], &str, &str)], // (y, label, color)
    x_label: &str,
    y_label: &str,
    y_range: Option<(f64, f64)>,
    show_legend: bool,
) -> Result<()> {
    let margin_l = 56.0;
    let margin_r = 20.0;
    let margin_t = 18.0;
    let margin_b = 36.0;
    let plot_w = (w - margin_l - margin_r).max(1.0);
    let plot_h = (h - margin_t - margin_b).max(1.0);
    let origin_x = x0 + margin_l;
    let origin_y = y0 + margin_t;

    // y 範囲
    let (mut min_y, mut max_y) = if let Some(r) = y_range {
        r
    } else {
        (f64::INFINITY, f64::NEG_INFINITY)
    };
    if y_range.is_none() {
        for (ys, _, _) in series.iter() {
            for &v in ys.iter() {
                if v.is_finite() {
                    min_y = min_y.min(v);
                    max_y = max_y.max(v);
                }
            }
        }
    }
    if !min_y.is_finite() || !max_y.is_finite() || min_y == max_y {
        min_y = -1.0;
        max_y = 1.0;
    }

    // x 範囲（log10）
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    for &f_hz in x_hz.iter() {
        if f_hz > 0.0 {
            let lx = f_hz.log10();
            min_x = min_x.min(lx);
            max_x = max_x.max(lx);
        }
    }
    if !min_x.is_finite() || !max_x.is_finite() || min_x == max_x {
        min_x = 0.0;
        max_x = 1.0;
    }

    writeln!(f, "<g transform='translate({origin_x},{origin_y})'>")?;
    writeln!(f, "<rect x='0' y='0' width='{plot_w}' height='{plot_h}' fill='none' stroke='#888' stroke-width='1.5' />")?;

    // グリッド/目盛
    let text_color = "#333";
    let grid_color = "#eee";
    let n_ticks = 5;

    // Y ticks
    for i in 0..n_ticks {
        let t = i as f64 / (n_ticks - 1) as f64;
        let vy = min_y * (1.0 - t) + max_y * t;
        let y_pix = (1.0 - (vy - min_y) / (max_y - min_y)) * plot_h;
        writeln!(
            f,
            "<line x1='{:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' stroke='{grid_color}' />",
            0.0, y_pix, plot_w, y_pix
        )?;
        writeln!(f, "<text x='{:.1}' y='{:.1}' font-size='10' fill='{text_color}' text-anchor='end' dominant-baseline='middle'>{:.3}</text>", -6.0, y_pix, vy)?;
    }

    // X ticks at decades
    let decade_min = min_x.floor() as i32;
    let decade_max = max_x.ceil() as i32;
    for d in decade_min..=decade_max {
        let lx = d as f64;
        let x_pix = (lx - min_x) / (max_x - min_x) * plot_w;
        writeln!(
            f,
            "<line x1='{x_pix:.1}' y1='0' x2='{x_pix:.1}' y2='{plot_h:.1}' stroke='{grid_color}' />"
        )?;
        let val = 10f64.powf(lx);
        writeln!(
            f,
            "<text x='{x_pix:.1}' y='{plot_h:.1}' dy='14' font-size='10' fill='{text_color}' text-anchor='middle'>{val:.3}</text>"
        )?;
    }

    // data polylines
    for (idx, (ys, _label, color)) in series.iter().enumerate() {
        let color = if !color.is_empty() {
            *color
        } else {
            COLORS[idx % COLORS.len()]
        };
        write!(
            f,
            "<polyline fill='none' stroke='{color}' stroke-width='1.4' points='"
        )?;
        for (&fx, &vy) in x_hz.iter().zip(ys.iter()) {
            if fx <= 0.0 {
                continue;
            }
            let lx = fx.log10();
            let tx = (lx - min_x) / (max_x - min_x);
            let x_pix = tx * plot_w;
            let clamped = vy.clamp(min_y, max_y);
            let ty = if max_y == min_y {
                0.5
            } else {
                (clamped - min_y) / (max_y - min_y)
            };
            let y_pix = (1.0 - ty) * plot_h;
            write!(f, "{x_pix:.2},{y_pix:.2} ")?;
        }
    writeln!(f, "' />")?;
    }

    // legend (optional, top-right)
    if show_legend && !series.is_empty() {
        let items: Vec<(&str, &str)> = series
            .iter()
            .enumerate()
            .map(|(i, (_ys, label, color))| {
                let color = if !color.is_empty() {
                    *color
                } else {
                    COLORS[i % COLORS.len()]
                };
                (*label, color)
            })
            .collect();
    draw_legend(f, plot_w, 0.0, &items)?;
    }

    // labels
    writeln!(f, "<text x='{:.1}' y='{:.1}' font-size='11' fill='{text_color}' text-anchor='middle'>{}</text>", plot_w / 2.0, plot_h + 28.0, x_label)?;
    writeln!(
        f,
        "<text x='{:.1}' y='-8' font-size='11' fill='{text_color}' text-anchor='start'>{}</text>",
        0.0, y_label
    )?;

    writeln!(f, "</g>")?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn draw_linearx_series_multi(
    f: &mut dyn Write,
    x0: f64,
    y0: f64,
    w: f64,
    h: f64,
    x: &[f64],
    series: &[(&[f64], &str, &str)], // (y, label, color)
    x_label: &str,
    y_label: &str,
    y_range: Option<(f64, f64)>,
    show_legend: bool,
) -> Result<()> {
    let margin_l = 56.0;
    let margin_r = 20.0;
    let margin_t = 18.0;
    let margin_b = 36.0;
    let plot_w = (w - margin_l - margin_r).max(1.0);
    let plot_h = (h - margin_t - margin_b).max(1.0);
    let origin_x = x0 + margin_l;
    let origin_y = y0 + margin_t;

    // y 範囲
    let (mut min_y, mut max_y) = if let Some(r) = y_range {
        r
    } else {
        (f64::INFINITY, f64::NEG_INFINITY)
    };
    if y_range.is_none() {
        for (ys, _, _) in series.iter() {
            for &v in ys.iter() {
                if v.is_finite() {
                    min_y = min_y.min(v);
                    max_y = max_y.max(v);
                }
            }
        }
    }
    if !min_y.is_finite() || !max_y.is_finite() || min_y == max_y {
        min_y = -1.0;
        max_y = 1.0;
    }

    // x 範囲（線形）
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    for &vx in x.iter() {
        if vx.is_finite() {
            min_x = min_x.min(vx);
            max_x = max_x.max(vx);
        }
    }
    if !min_x.is_finite() || !max_x.is_finite() || min_x == max_x {
        min_x = 0.0;
        max_x = 1.0;
    }

    writeln!(f, "<g transform='translate({origin_x},{origin_y})'>")?;
    writeln!(f, "<rect x='0' y='0' width='{plot_w}' height='{plot_h}' fill='none' stroke='#888' stroke-width='1.5' />")?;

    // グリッド/目盛
    let text_color = "#333";
    let grid_color = "#eee";
    let n_ticks = 5;

    // Y ticks
    for i in 0..n_ticks {
        let t = i as f64 / (n_ticks - 1) as f64;
        let vy = min_y * (1.0 - t) + max_y * t;
        let y_pix = (1.0 - (vy - min_y) / (max_y - min_y)) * plot_h;
        writeln!(
            f,
            "<line x1='{:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' stroke='{grid_color}' />",
            0.0, y_pix, plot_w, y_pix
        )?;
        writeln!(f, "<text x='{:.1}' y='{:.1}' font-size='10' fill='{text_color}' text-anchor='end' dominant-baseline='middle'>{:.3}</text>", -6.0, y_pix, vy)?;
    }

    // X ticks linear
    for i in 0..n_ticks {
        let t = i as f64 / (n_ticks - 1) as f64;
        let vx = min_x * (1.0 - t) + max_x * t;
        let x_pix = (vx - min_x) / (max_x - min_x) * plot_w;
    writeln!(f, "<line x1='{x_pix:.1}' y1='0' x2='{x_pix:.1}' y2='{plot_h:.1}' stroke='{grid_color}' />")?;
        writeln!(f, "<text x='{x_pix:.1}' y='{plot_h:.1}' dy='14' font-size='10' fill='{text_color}' text-anchor='middle'>{vx:.3}</text>")?;
    }

    // polylines
    for (idx, (ys, _label, color)) in series.iter().enumerate() {
        let color = if !color.is_empty() {
            *color
        } else {
            COLORS[idx % COLORS.len()]
        };
        write!(
            f,
            "<polyline fill='none' stroke='{color}' stroke-width='1.4' points='"
        )?;
        for (&vx, &vy) in x.iter().zip(ys.iter()) {
            let tx = (vx - min_x) / (max_x - min_x);
            let x_pix = tx * plot_w;
            let clamped = vy.clamp(min_y, max_y);
            let ty = if max_y == min_y {
                0.5
            } else {
                (clamped - min_y) / (max_y - min_y)
            };
            let y_pix = (1.0 - ty) * plot_h;
            write!(f, "{x_pix:.2},{y_pix:.2} ")?;
        }
    writeln!(f, "' />")?;
    }

    if show_legend && !series.is_empty() {
        let items: Vec<(&str, &str)> = series
            .iter()
            .enumerate()
            .map(|(i, (_ys, label, color))| {
                let color = if !color.is_empty() {
                    *color
                } else {
                    COLORS[i % COLORS.len()]
                };
                (*label, color)
            })
            .collect();
    draw_legend(f, plot_w, 0.0, &items)?;
    }

    // labels
    writeln!(f, "<text x='{:.1}' y='{:.1}' font-size='11' fill='{text_color}' text-anchor='middle'>{}</text>", plot_w / 2.0, plot_h + 28.0, x_label)?;
    writeln!(
        f,
        "<text x='{:.1}' y='-8' font-size='11' fill='{text_color}' text-anchor='start'>{}</text>",
        0.0, y_label
    )?;

    writeln!(f, "</g>")?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn draw_complex_series_multi(
    f: &mut dyn Write,
    x0: f64,
    y0: f64,
    w: f64,
    h: f64,
    series: &[(&[f64], &[f64], &str, &str)], // (re[], im[], label, color)
    x_label: &str,
    y_label: &str,
    real_range: Option<(f64, f64)>,
    imag_range: Option<(f64, f64)>,
    show_legend: bool,
    show_minus_one: bool,
) -> Result<()> {
    let margin_l = 56.0;
    let margin_r = 20.0;
    let margin_t = 18.0;
    let margin_b = 36.0;
    let plot_w = (w - margin_l - margin_r).max(1.0);
    let plot_h = (h - margin_t - margin_b).max(1.0);
    let origin_x = x0 + margin_l;
    let origin_y = y0 + margin_t;

    // ranges
    let mut min_re = real_range.map(|r| r.0).unwrap_or(f64::INFINITY);
    let mut max_re = real_range.map(|r| r.1).unwrap_or(f64::NEG_INFINITY);
    let mut min_im = imag_range.map(|r| r.0).unwrap_or(f64::INFINITY);
    let mut max_im = imag_range.map(|r| r.1).unwrap_or(f64::NEG_INFINITY);
    if real_range.is_none() || imag_range.is_none() {
        for (re, im, _label, _color) in series.iter() {
            for (&x, &y) in re.iter().zip(im.iter()) {
                if x.is_finite() {
                    min_re = min_re.min(x);
                    max_re = max_re.max(x);
                }
                if y.is_finite() {
                    min_im = min_im.min(y);
                    max_im = max_im.max(y);
                }
            }
        }
    }
    if !min_re.is_finite() || !max_re.is_finite() || min_re == max_re {
        min_re = -1.0;
        max_re = 1.0;
    }
    if !min_im.is_finite() || !max_im.is_finite() || min_im == max_im {
        min_im = -1.0;
        max_im = 1.0;
    }

    writeln!(f, "<g transform='translate({origin_x},{origin_y})'>")?;
    writeln!(f, "<rect x='0' y='0' width='{plot_w}' height='{plot_h}' fill='none' stroke='#888' stroke-width='1.5' />")?;

    let text_color = "#333";
    let grid_color = "#eee";
    let axis_color = "#bbb";
    let n_ticks = 5;

    // grid and ticks (Y)
    for i in 0..n_ticks {
        let t = i as f64 / (n_ticks - 1) as f64;
        let vy = min_im * (1.0 - t) + max_im * t;
        let y_pix = (1.0 - (vy - min_im) / (max_im - min_im)) * plot_h;
        writeln!(
            f,
            "<line x1='0' y1='{y_pix:.1}' x2='{plot_w:.1}' y2='{y_pix:.1}' stroke='{grid_color}' />",
        )?;
        writeln!(f, "<text x='-6' y='{y_pix:.1}' font-size='10' fill='{text_color}' text-anchor='end' dominant-baseline='middle'>{vy:.3}</text>")?;
    }

    // grid and ticks (X)
    for i in 0..n_ticks {
        let t = i as f64 / (n_ticks - 1) as f64;
        let vx = min_re * (1.0 - t) + max_re * t;
        let x_pix = (vx - min_re) / (max_re - min_re) * plot_w;
        writeln!(
            f,
            "<line x1='{x_pix:.1}' y1='0' x2='{x_pix:.1}' y2='{plot_h:.1}' stroke='{grid_color}' />",
        )?;
        writeln!(f, "<text x='{x_pix:.1}' y='{plot_h:.1}' dy='14' font-size='10' fill='{text_color}' text-anchor='middle'>{vx:.3}</text>")?;
    }

    // draw axes at 0 if visible
    if min_re <= 0.0 && 0.0 <= max_re {
        let x0_pix = (0.0 - min_re) / (max_re - min_re) * plot_w;
        writeln!(f, "<line x1='{x0_pix:.1}' y1='0' x2='{x0_pix:.1}' y2='{plot_h:.1}' stroke='{axis_color}' />")?;
    }
    if min_im <= 0.0 && 0.0 <= max_im {
        let y0_pix = (1.0 - (0.0 - min_im) / (max_im - min_im)) * plot_h;
        writeln!(f, "<line x1='0' y1='{y0_pix:.1}' x2='{plot_w:.1}' y2='{y0_pix:.1}' stroke='{axis_color}' />")?;
    }

    // data polylines
    for (idx, (re, im, _label, color)) in series.iter().enumerate() {
        let color = if !color.is_empty() {
            *color
        } else {
            COLORS[idx % COLORS.len()]
        };
        write!(
            f,
            "<polyline fill='none' stroke='{color}' stroke-width='1.4' points='"
        )?;
        for (&xr, &yi) in re.iter().zip(im.iter()) {
            let tx = (xr - min_re) / (max_re - min_re);
            let x_pix = tx * plot_w;
            let ty = (yi - min_im) / (max_im - min_im);
            let y_pix = (1.0 - ty) * plot_h;
            write!(f, "{x_pix:.2},{y_pix:.2} ")?;
        }
    writeln!(f, "' />")?;
    }

    // -1+0j mark
    if show_minus_one && (min_re <= -1.0 && -1.0 <= max_re) && (min_im <= 0.0 && 0.0 <= max_im) {
        let x_pix = (-1.0 - min_re) / (max_re - min_re) * plot_w;
        let y_pix = (1.0 - (0.0 - min_im) / (max_im - min_im)) * plot_h;
        writeln!(f, "<circle cx='{x_pix:.1}' cy='{y_pix:.1}' r='3' fill='none' stroke='#d62728' stroke-width='1.4' />")?;
        writeln!(f, "<text x='{x_pix:.1}' y='{y_pix:.1}' dx='6' dy='-4' font-size='10' fill='{text_color}'>-1</text>")?;
    }

    // legend (optional, top-right)
    if show_legend && !series.is_empty() {
        let items: Vec<(&str, &str)> = series
            .iter()
            .enumerate()
            .map(|(i, (_re, _im, label, color))| {
                let color = if !color.is_empty() {
                    *color
                } else {
                    COLORS[i % COLORS.len()]
                };
                (*label, color)
            })
            .collect();
    draw_legend(f, plot_w, 0.0, &items)?;
    }

    // labels
    writeln!(f, "<text x='{:.1}' y='{:.1}' font-size='11' fill='{text_color}' text-anchor='middle'>{}</text>", plot_w / 2.0, plot_h + 28.0, x_label)?;
    writeln!(
        f,
        "<text x='{:.1}' y='-8' font-size='11' fill='{text_color}' text-anchor='start'>{}</text>",
        0.0, y_label
    )?;

    writeln!(f, "</g>")?;
    Ok(())
}

fn draw_legend(f: &mut dyn Write, plot_w: f64, y_top: f64, items: &[(&str, &str)]) -> Result<()> {
    if items.is_empty() {
        return Ok(());
    }
    // 簡易幅推定: 1 文字 ~ 7px + 色線分 + マージン
    let max_label = items.iter().map(|(s, _)| s.len()).max().unwrap_or(1) as f64;
    let item_h = 16.0_f64;
    let pad = 8.0_f64;
    let swatch_w = 16.0_f64;
    let est_text_w = max_label * 7.0;
    let legend_w = swatch_w + 8.0 + est_text_w + pad * 2.0;
    let legend_h = items.len() as f64 * item_h + pad * 2.0;
    let x = (plot_w - legend_w - 6.0).max(0.0);
    let y = (y_top + 6.0).max(0.0);
    writeln!(f, "<g transform='translate({x},{y})'>")?;
    writeln!(f, "<rect x='0' y='0' width='{legend_w:.1}' height='{legend_h:.1}' fill='#ffffffcc' stroke='#666' />")?;
    for (i, (label, color)) in items.iter().enumerate() {
        let yy = pad + i as f64 * item_h + item_h * 0.5;
    writeln!(f, "<line x1='{:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' stroke='{color}' stroke-width='2' />", pad, yy, pad + swatch_w, yy)?;
    writeln!(f, "<text x='{:.1}' y='{:.1}' font-size='11' fill='#222' dominant-baseline='middle'>{}</text>", pad + swatch_w + 6.0, yy, label)?;
    }
    writeln!(f, "</g>")?;
    Ok(())
}

/// 複数系列・任意軸モード対応（離散系）
pub(crate) fn save_bode_svg_discrete_multi(
    series: &[(&DiscreteTransferFunction, &str)],
    path: &str,
    width: u32,
    height: u32,
    opts: &DiscreteBodeOptions,
) -> Result<()> {
    let mut f = File::create(path)?;
    write_bode_svg_discrete_multi(series, &mut f, width, height, opts)
}

/// Bode（離散）複数系列を任意のライターへ出力
pub(crate) fn write_bode_svg_discrete_multi(
    series: &[(&DiscreteTransferFunction, &str)],
    f: &mut dyn Write,
    width: u32,
    height: u32,
    opts: &DiscreteBodeOptions,
) -> Result<()> {
    assert!(opts.n_points >= 2, "n_points must be >= 2");
    let w_all = width as f64;
    let h_all = height as f64;
    let h_each = (h_all / 2.0).floor();

    writeln!(f, "<svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}' viewBox='0 0 {width} {height}'>")?;
    writeln!(f, "<rect width='100%' height='100%' fill='white' />")?;

    // サンプリング x と評価方法
    let mut x_vals: Vec<f64> = Vec::with_capacity(opts.n_points);
    let fs_ref = series
        .first()
        .map(|(tf, _)| tf.sample_rate())
        .unwrap_or(1.0);
    match opts.x_axis {
        DiscreteXAxis::Hz => {
            let fs = fs_ref;
            let f_min = opts
                .f_min_hz
                .unwrap_or_else(|| (fs / 2.0) / 10_000.0)
                .max(1e-9);
            let f_max = opts.f_max_hz.unwrap_or(fs / 2.0).max(f_min * 10.0);
            for i in 0..opts.n_points {
                let t = i as f64 / (opts.n_points - 1) as f64;
                let lf = (f_min.ln() * (1.0 - t)) + (f_max.ln() * t);
                x_vals.push(lf.exp());
            }
        }
        DiscreteXAxis::Radian => {
            let w_min = opts.w_min.unwrap_or(1e-3);
            let w_max = opts.w_max.unwrap_or(std::f64::consts::PI);
            for i in 0..opts.n_points {
                let t = i as f64 / (opts.n_points - 1) as f64;
                x_vals.push(w_min * (1.0 - t) + w_max * t);
            }
        }
        DiscreteXAxis::Normalized => {
            let nmin = opts.norm_min.unwrap_or(1e-4);
            let nmax = opts.norm_max.unwrap_or(0.5);
            for i in 0..opts.n_points {
                let t = i as f64 / (opts.n_points - 1) as f64;
                x_vals.push(nmin * (1.0 - t) + nmax * t);
            }
        }
    }

    // 各系列の周波数応答を評価
    let mut mag_series: Vec<Vec<f64>> = Vec::with_capacity(series.len());
    let mut ph_series: Vec<Vec<f64>> = Vec::with_capacity(series.len());
    for (tf, _label) in series.iter() {
        let mut mags = Vec::with_capacity(opts.n_points);
        let mut phs = Vec::with_capacity(opts.n_points);
        for &x in x_vals.iter() {
            let h = match opts.x_axis {
                DiscreteXAxis::Hz => {
                    let omega = 2.0 * std::f64::consts::PI * x / tf.sample_rate();
                    let z = Complex::from_polar(1.0, omega);
                    tf.eval_z(z)
                }
                DiscreteXAxis::Radian => {
                    let z = Complex::from_polar(1.0, x);
                    tf.eval_z(z)
                }
                DiscreteXAxis::Normalized => {
                    let omega = 2.0 * std::f64::consts::PI * x; // cycles/sample -> rad/sample
                    let z = Complex::from_polar(1.0, omega);
                    tf.eval_z(z)
                }
            };
            let m = h.norm();
            let db = if m > 0.0 { 20.0 * m.log10() } else { -160.0 };
            mags.push(db);
            phs.push(h.arg().to_degrees());
        }
        if opts.unwrap_phase {
            phs = unwrap_phase_deg(&phs);
        }
        mag_series.push(mags);
        ph_series.push(phs);
    }

    // 上段: 振幅(dB)
    match opts.x_axis {
        DiscreteXAxis::Hz => {
            let series_refs: Vec<(&[f64], &str, &str)> = mag_series
                .iter()
                .zip(series.iter())
                .enumerate()
                .map(|(i, (ys, (_tf, label)))| (ys.as_slice(), *label, COLORS[i % COLORS.len()]))
                .collect();
            draw_logx_series_multi(
                f,
                0.0,
                0.0,
                w_all,
                h_each,
                &x_vals,
                &series_refs,
                "Frequency (Hz)",
                "Magnitude (dB)",
                opts.mag_db_range,
                opts.legend,
            )?;
        }
        _ => {
            let xlabel = match opts.x_axis {
                DiscreteXAxis::Radian => "Frequency (rad/sample)",
                DiscreteXAxis::Normalized => "Normalized frequency (cycles/sample)",
                DiscreteXAxis::Hz => unreachable!(),
            };
            let series_refs: Vec<(&[f64], &str, &str)> = mag_series
                .iter()
                .zip(series.iter())
                .enumerate()
                .map(|(i, (ys, (_tf, label)))| (ys.as_slice(), *label, COLORS[i % COLORS.len()]))
                .collect();
            draw_linearx_series_multi(
                f,
                0.0,
                0.0,
                w_all,
                h_each,
                &x_vals,
                &series_refs,
                xlabel,
                "Magnitude (dB)",
                opts.mag_db_range,
                opts.legend,
            )?;
        }
    }

    // 下段: 位相(deg)
    match opts.x_axis {
        DiscreteXAxis::Hz => {
            let series_refs: Vec<(&[f64], &str, &str)> = ph_series
                .iter()
                .zip(series.iter())
                .enumerate()
                .map(|(i, (ys, (_tf, label)))| (ys.as_slice(), *label, COLORS[i % COLORS.len()]))
                .collect();
            draw_logx_series_multi(
                f,
                0.0,
                h_each,
                w_all,
                h_each,
                &x_vals,
                &series_refs,
                "Frequency (Hz)",
                "Phase (deg)",
                opts.phase_deg_range,
                false, // 位相パネルでは凡例は省略
            )?;
        }
        _ => {
            let xlabel = match opts.x_axis {
                DiscreteXAxis::Radian => "Frequency (rad/sample)",
                DiscreteXAxis::Normalized => "Normalized frequency (cycles/sample)",
                DiscreteXAxis::Hz => unreachable!(),
            };
            let series_refs: Vec<(&[f64], &str, &str)> = ph_series
                .iter()
                .zip(series.iter())
                .enumerate()
                .map(|(i, (ys, (_tf, label)))| (ys.as_slice(), *label, COLORS[i % COLORS.len()]))
                .collect();
            draw_linearx_series_multi(
                f,
                0.0,
                h_each,
                w_all,
                h_each,
                &x_vals,
                &series_refs,
                xlabel,
                "Phase (deg)",
                opts.phase_deg_range,
                false,
            )?;
        }
    }

    writeln!(f, "</svg>")?;
    Ok(())
}

/// 複数系列対応（連続系）
pub(crate) fn save_bode_svg_continuous_multi(
    series: &[(&ContinuousTransferFunction, &str)],
    path: &str,
    width: u32,
    height: u32,
    opts: &ContinuousBodeOptions,
) -> Result<()> {
    let mut f = File::create(path)?;
    write_bode_svg_continuous_multi(series, &mut f, width, height, opts)
}

/// Bode（連続）複数系列を任意のライターへ出力
pub(crate) fn write_bode_svg_continuous_multi(
    series: &[(&ContinuousTransferFunction, &str)],
    f: &mut dyn Write,
    width: u32,
    height: u32,
    opts: &ContinuousBodeOptions,
) -> Result<()> {
    assert!(opts.n_points >= 2, "n_points must be >= 2");
    let f_min = opts.f_min_hz.max(1e-9);
    let f_max = opts.f_max_hz.max(f_min * 10.0);

    let mut freqs: Vec<f64> = Vec::with_capacity(opts.n_points);
    for i in 0..opts.n_points {
        let t = i as f64 / (opts.n_points - 1) as f64;
        let lf = (f_min.ln() * (1.0 - t)) + (f_max.ln() * t);
        freqs.push(lf.exp());
    }

    // evaluate
    let mut mag_series: Vec<Vec<f64>> = Vec::with_capacity(series.len());
    let mut ph_series: Vec<Vec<f64>> = Vec::with_capacity(series.len());
    for (tf, _label) in series.iter() {
        let mut mags = Vec::with_capacity(opts.n_points);
        let mut phs = Vec::with_capacity(opts.n_points);
        for &f_hz in freqs.iter() {
            let s = Complex::new(0.0, 2.0 * std::f64::consts::PI * f_hz);
            let h = tf.eval_s(s);
            let m = h.norm();
            mags.push(if m > 0.0 { 20.0 * m.log10() } else { -160.0 });
            phs.push(h.arg().to_degrees());
        }
        if opts.unwrap_phase {
            phs = unwrap_phase_deg(&phs);
        }
        mag_series.push(mags);
        ph_series.push(phs);
    }

    let w_all = width as f64;
    let h_all = height as f64;
    let h_each = (h_all / 2.0).floor();

    writeln!(f, "<svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}' viewBox='0 0 {width} {height}'>")?;
    writeln!(f, "<rect width='100%' height='100%' fill='white' />")?;

    // mag
    let mag_refs: Vec<(&[f64], &str, &str)> = mag_series
        .iter()
        .zip(series.iter())
        .enumerate()
        .map(|(i, (ys, (_tf, label)))| (ys.as_slice(), *label, COLORS[i % COLORS.len()]))
        .collect();
    draw_logx_series_multi(
        f,
        0.0,
        0.0,
        w_all,
        h_each,
        &freqs,
        &mag_refs,
        "Frequency (Hz)",
        "Magnitude (dB)",
        opts.mag_db_range,
        opts.legend,
    )?;

    // phase
    let ph_refs: Vec<(&[f64], &str, &str)> = ph_series
        .iter()
        .zip(series.iter())
        .enumerate()
        .map(|(i, (ys, (_tf, label)))| (ys.as_slice(), *label, COLORS[i % COLORS.len()]))
        .collect();
    draw_logx_series_multi(
        f,
        0.0,
        h_each,
        w_all,
        h_each,
        &freqs,
        &ph_refs,
        "Frequency (Hz)",
        "Phase (deg)",
        opts.phase_deg_range,
        false,
    )?;

    writeln!(f, "</svg>")?;
    Ok(())
}

/// Nyquist（離散）複数系列
pub(crate) fn save_nyquist_svg_discrete_multi(
    series: &[(&DiscreteTransferFunction, &str)],
    path: &str,
    width: u32,
    height: u32,
    opts: &crate::plot::DiscreteNyquistOptions,
) -> Result<()> {
    let mut file = File::create(path)?;
    write_nyquist_svg_discrete_multi(series, &mut file, width, height, opts)
}

/// Nyquist（離散）複数系列を任意のライターへ出力
pub(crate) fn write_nyquist_svg_discrete_multi(
    series: &[(&DiscreteTransferFunction, &str)],
    file: &mut dyn Write,
    width: u32,
    height: u32,
    opts: &crate::plot::DiscreteNyquistOptions,
) -> Result<()> {
    writeln!(file, "<svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}'>")?;
    // background
    writeln!(file, "<rect x='0' y='0' width='{width}' height='{height}' fill='#ffffff' />")?;
    if let Some(title) = &opts.title {
        let cx = (width as f64) / 2.0;
        writeln!(file, "<text x='{cx:.1}' y='16' font-size='14' fill='#333' text-anchor='middle'>{title}</text>")?;
    }

    let mut re_series: Vec<Vec<f64>> = Vec::with_capacity(series.len());
    let mut im_series: Vec<Vec<f64>> = Vec::with_capacity(series.len());
    for (tf, _label) in series.iter() {
        let n = opts.n_points.max(2);
        let mut re = Vec::with_capacity(n);
        let mut im = Vec::with_capacity(n);
        for k in 0..n {
            let t = k as f64 / (n - 1) as f64;
            let theta = opts.theta_min * (1.0 - t) + opts.theta_max * t;
            let z = Complex::from_polar(1.0, theta);
            let h = tf.eval_z(z);
            re.push(h.re);
            im.push(h.im);
        }
        re_series.push(re);
        im_series.push(im);
    }

    let packed: Vec<(&[f64], &[f64], &str, &str)> = series
        .iter()
        .enumerate()
        .map(|(i, (_tf, label))| {
            let color = COLORS[i % COLORS.len()];
            (
                re_series[i].as_slice(),
                im_series[i].as_slice(),
                *label,
                color,
            )
        })
        .collect();

    let x_label = opts.x_label.as_deref().unwrap_or("Re");
    let y_label = opts.y_label.as_deref().unwrap_or("Im");
    draw_complex_series_multi(
        file,
        0.0,
        if opts.title.is_some() { 10.0 } else { 0.0 },
        width as f64,
        height as f64,
        &packed,
        x_label,
        y_label,
        opts.real_range,
        opts.imag_range,
        opts.legend,
        opts.show_minus_one,
    )?;

    writeln!(file, "</svg>")?;
    Ok(())
}

/// Nyquist（連続）複数系列
pub(crate) fn save_nyquist_svg_continuous_multi(
    series: &[(&ContinuousTransferFunction, &str)],
    path: &str,
    width: u32,
    height: u32,
    opts: &crate::plot::ContinuousNyquistOptions,
) -> Result<()> {
    let mut file = File::create(path)?;
    write_nyquist_svg_continuous_multi(series, &mut file, width, height, opts)
}

/// Nyquist（連続）複数系列を任意のライターへ出力
pub(crate) fn write_nyquist_svg_continuous_multi(
    series: &[(&ContinuousTransferFunction, &str)],
    file: &mut dyn Write,
    width: u32,
    height: u32,
    opts: &crate::plot::ContinuousNyquistOptions,
) -> Result<()> {
    writeln!(file, "<svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}'>")?;
    // background
    writeln!(file, "<rect x='0' y='0' width='{width}' height='{height}' fill='#ffffff' />")?;
    if let Some(title) = &opts.title {
        let cx = (width as f64) / 2.0;
        writeln!(file, "<text x='{cx:.1}' y='16' font-size='14' fill='#333' text-anchor='middle'>{title}</text>")?;
    }

    let mut re_series: Vec<Vec<f64>> = Vec::with_capacity(series.len());
    let mut im_series: Vec<Vec<f64>> = Vec::with_capacity(series.len());
    for (tf, _label) in series.iter() {
        let n = opts.n_points.max(2);
        let mut re = Vec::with_capacity(n);
        let mut im = Vec::with_capacity(n);
        for k in 0..n {
            let t = if n == 1 {
                0.0
            } else {
                k as f64 / (n - 1) as f64
            };
            let f_hz = if opts.log_freq {
                let lo = opts.f_min_hz.max(1e-12).log10();
                let hi = opts.f_max_hz.max(opts.f_min_hz.max(1e-12) * 10.0).log10();
                10f64.powf(lo * (1.0 - t) + hi * t)
            } else {
                opts.f_min_hz * (1.0 - t) + opts.f_max_hz * t
            };
            let omega = 2.0 * std::f64::consts::PI * f_hz;
            let s = Complex::new(0.0, omega);
            let g = tf.eval_s(s);
            re.push(g.re);
            im.push(g.im);
        }
        re_series.push(re);
        im_series.push(im);
    }

    let packed: Vec<(&[f64], &[f64], &str, &str)> = series
        .iter()
        .enumerate()
        .map(|(i, (_tf, label))| {
            let color = COLORS[i % COLORS.len()];
            (
                re_series[i].as_slice(),
                im_series[i].as_slice(),
                *label,
                color,
            )
        })
        .collect();

    let x_label = opts.x_label.as_deref().unwrap_or("Re");
    let y_label = opts.y_label.as_deref().unwrap_or("Im");
    draw_complex_series_multi(
        file,
        0.0,
        if opts.title.is_some() { 10.0 } else { 0.0 },
        width as f64,
        height as f64,
        &packed,
        x_label,
        y_label,
        opts.real_range,
        opts.imag_range,
        opts.legend,
        opts.show_minus_one,
    )?;

    writeln!(file, "</svg>")?;
    Ok(())
}

// ===== SVG を String で取得する高レベル API =====

impl DiscreteTransferFunction {
    /// Bode（離散）SVG を文字列で返す（単系列）
    pub fn bode_svg_string(
        &self,
        width: u32,
        height: u32,
        opts: &crate::plot::DiscreteBodeOptions,
    ) -> String {
        let mut buf: Vec<u8> = Vec::with_capacity(32 * 1024);
        let _ = write_bode_svg_discrete_multi(&[(self, "H(z)")], &mut buf, width, height, opts);
        String::from_utf8(buf).unwrap_or_default()
    }

    /// Bode（離散）簡易設定 SVG を文字列で返す
    pub fn bode_svg_simple_string(
        &self,
        width: u32,
        height: u32,
        n_points: usize,
    ) -> String {
        let opts = crate::plot::DiscreteBodeOptions {
            n_points: n_points.max(2),
            legend: false,
            x_axis: crate::plot::DiscreteXAxis::Hz,
            ..Default::default()
        };
        self.bode_svg_string(width, height, &opts)
    }

    /// Nyquist（離散）SVG を文字列で返す（単系列）
    pub fn nyquist_svg_string(
        &self,
        width: u32,
        height: u32,
        opts: &crate::plot::DiscreteNyquistOptions,
    ) -> String {
        let mut buf: Vec<u8> = Vec::with_capacity(32 * 1024);
        let _ = write_nyquist_svg_discrete_multi(&[(self, "H(z)")], &mut buf, width, height, opts);
        String::from_utf8(buf).unwrap_or_default()
    }

    /// Nyquist（離散）簡易設定 SVG を文字列で返す
    pub fn nyquist_svg_simple_string(
        &self,
        width: u32,
        height: u32,
        n_points: usize,
    ) -> String {
        let opts = crate::plot::DiscreteNyquistOptions {
            n_points: n_points.max(2),
            legend: false,
            ..Default::default()
        };
        self.nyquist_svg_string(width, height, &opts)
    }

    /// ブロック図（離散）SVG を文字列で返す
    pub fn block_feedback_svg_string(
        &self,
        width: u32,
        height: u32,
        negative_feedback: bool,
        feedback_label: Option<&str>,
    ) -> String {
        let mut buf: Vec<u8> = Vec::with_capacity(8 * 1024);
        let g_label = format!("{}", self.display());
        let _ = write_block_feedback_svg(
            &g_label,
            &mut buf,
            width,
            height,
            negative_feedback,
            feedback_label,
        );
        String::from_utf8(buf).unwrap_or_default()
    }
}

impl ContinuousTransferFunction {
    /// Bode（連続）SVG を文字列で返す（単系列）
    pub fn bode_svg_string(
        &self,
        width: u32,
        height: u32,
        opts: &crate::plot::ContinuousBodeOptions,
    ) -> String {
        let mut buf: Vec<u8> = Vec::with_capacity(32 * 1024);
        let _ = write_bode_svg_continuous_multi(&[(self, "G(s)")], &mut buf, width, height, opts);
        String::from_utf8(buf).unwrap_or_default()
    }

    /// Bode（連続）簡易設定 SVG を文字列で返す
    pub fn bode_svg_simple_string(
        &self,
        width: u32,
        height: u32,
        f_min_hz: f64,
        f_max_hz: f64,
        n_points: usize,
    ) -> String {
        let opts = crate::plot::ContinuousBodeOptions {
            n_points: n_points.max(2),
            f_min_hz,
            f_max_hz,
            legend: false,
            ..Default::default()
        };
        self.bode_svg_string(width, height, &opts)
    }

    /// Nyquist（連続）SVG を文字列で返す（単系列）
    pub fn nyquist_svg_string(
        &self,
        width: u32,
        height: u32,
        opts: &crate::plot::ContinuousNyquistOptions,
    ) -> String {
        let mut buf: Vec<u8> = Vec::with_capacity(32 * 1024);
        let _ = write_nyquist_svg_continuous_multi(&[(self, "G(s)")], &mut buf, width, height, opts);
        String::from_utf8(buf).unwrap_or_default()
    }

    /// Nyquist（連続）簡易設定 SVG を文字列で返す
    pub fn nyquist_svg_simple_string(
        &self,
        width: u32,
        height: u32,
        f_min_hz: f64,
        f_max_hz: f64,
        n_points: usize,
    ) -> String {
        let opts = crate::plot::ContinuousNyquistOptions {
            n_points: n_points.max(2),
            f_min_hz,
            f_max_hz,
            legend: false,
            log_freq: true,
            ..Default::default()
        };
        self.nyquist_svg_string(width, height, &opts)
    }

    /// ブロック図（連続）SVG を文字列で返す
    pub fn block_feedback_svg_string(
        &self,
        width: u32,
        height: u32,
        negative_feedback: bool,
        feedback_label: Option<&str>,
    ) -> String {
        let mut buf: Vec<u8> = Vec::with_capacity(8 * 1024);
        let g_label = format!("{}", self.display());
        let _ = write_block_feedback_svg(
            &g_label,
            &mut buf,
            width,
            height,
            negative_feedback,
            feedback_label,
        );
        String::from_utf8(buf).unwrap_or_default()
    }
}
