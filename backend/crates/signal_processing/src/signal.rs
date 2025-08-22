use crate::fir::{self, FIRFilter};
use crate::plot::{self, Series};
use num_complex::Complex;

/// 時間領域の実数信号。サンプル列とサンプルレート(Hz)を保持。
#[derive(Clone, Debug, PartialEq)]
pub struct Signal {
    data: Vec<f64>,
    sample_rate: f64,
}

impl Signal {
    pub fn new(data: Vec<f64>, sample_rate: f64) -> Self {
        Self { data, sample_rate }
    }
    pub fn from_slice(data: &[f64], sample_rate: f64) -> Self {
        Self::new(data.to_vec(), sample_rate)
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn sample_rate(&self) -> f64 {
        self.sample_rate
    }
    pub fn duration(&self) -> f64 {
        if self.sample_rate > 0.0 {
            self.len() as f64 / self.sample_rate
        } else {
            0.0
        }
    }
    pub fn data(&self) -> &[f64] {
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut [f64] {
        &mut self.data
    }
    pub fn into_inner(self) -> (Vec<f64>, f64) {
        (self.data, self.sample_rate)
    }
    /// 複素数列へ拡張（虚部0）
    pub fn to_complex_vec(&self) -> Vec<Complex<f64>> {
        self.data.iter().map(|&v| Complex::new(v, 0.0)).collect()
    }

    /// SVG 折れ線グラフとして保存（簡易）。幅・高さはピクセル、y は[-1,1]想定でスケール。
    pub fn save_svg(&self, path: &str, width: u32, height: u32) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;
        let mut f = File::create(path)?;
        let w = width as f64;
        let h = height as f64;
        writeln!(f, "<svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}' viewBox='0 0 {width} {height}'>")?;
        writeln!(f, "<rect width='100%' height='100%' fill='white' />")?;
        if self.len() > 1 {
            let max_n = (self.len() - 1) as f64;
            write!(
                f,
                "<polyline fill='none' stroke='black' stroke-width='1' points='"
            )?;
            for (i, &v) in self.data.iter().enumerate() {
                let x = (i as f64 / max_n) * w;
                // y in [-1,1] を [h,0] にマップ（上が小さい）
                let y = h * (1.0 - (v.clamp(-1.0, 1.0) + 1.0) * 0.5);
                write!(f, "{x:.3},{y:.3} ")?;
            }
            writeln!(f, "' />")?;
        }
        writeln!(f, "</svg>")?;
        Ok(())
    }

    /// 軸・凡例・複数系列対応の SVG。self を 1 系列として描画。
    pub fn save_svg_with_axes(
        &self,
        path: &str,
        width: u32,
        height: u32,
        label: &str,
    ) -> std::io::Result<()> {
        let s = Series {
            y: &self.data,
            label,
        };
        plot::save_svg_time_series(path, width, height, &[s], Some(self.sample_rate))
    }

    /// 複数の信号を重ね描きして SVG 出力。全系列のサンプルレートが同一前提。
    pub fn save_svg_multi(
        &self,
        path: &str,
        width: u32,
        height: u32,
        labeled_signals: &[(&Signal, &str)],
    ) -> std::io::Result<()> {
        if labeled_signals.is_empty() {
            return Ok(());
        }
        let sr = self.sample_rate;
        let series: Vec<Series> = labeled_signals
            .iter()
            .map(|(sig, label)| Series {
                y: sig.data(),
                label,
            })
            .collect();
        plot::save_svg_time_series(path, width, height, &series, Some(sr))
    }

    /// FIR 係数（Polynomial の係数を低次→高次でタップとみなす）で自己に畳み込み適用。
    pub fn apply_fir(&self, taps: &lti_systems::Polynomial<f64>) -> Signal {
        fir::apply_fir_signal(taps, self)
    }

    /// FIRFilter を適用（内部は直接法）。
    pub fn apply_fir_filter(&self, filter: &FIRFilter) -> Signal {
        filter.apply(self)
    }
}

impl AsRef<[f64]> for Signal {
    fn as_ref(&self) -> &[f64] {
        &self.data
    }
}

impl From<(Vec<f64>, f64)> for Signal {
    fn from(value: (Vec<f64>, f64)) -> Self {
        Signal::new(value.0, value.1)
    }
}

impl From<Signal> for Vec<f64> {
    fn from(value: Signal) -> Self {
        value.data
    }
}

/// 周波数領域の複素スペクトル。DFT/FFTの出力と同形。
#[derive(Clone, Debug, PartialEq)]
pub struct Spectrum {
    data: Vec<Complex<f64>>, // 長さN（DFTサイズ）
    sample_rate: f64,        // 入力信号と同じサンプルレート
}

impl Spectrum {
    pub fn new(data: Vec<Complex<f64>>, sample_rate: f64) -> Self {
        Self { data, sample_rate }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn sample_rate(&self) -> f64 {
        self.sample_rate
    }
    pub fn data(&self) -> &[Complex<f64>] {
        &self.data
    }
    pub fn into_inner(self) -> (Vec<Complex<f64>>, f64) {
        (self.data, self.sample_rate)
    }
    /// k番目のビンの周波数(Hz)
    pub fn bin_hz(&self, k: usize) -> f64 {
        if self.is_empty() {
            return 0.0;
        }
        (k as f64) * self.sample_rate / (self.len() as f64)
    }
    pub fn magnitude(&self, k: usize) -> f64 {
        self.data.get(k).map(|c| c.norm()).unwrap_or(0.0)
    }
    pub fn magnitudes(&self) -> Vec<f64> {
        self.data.iter().map(|c| c.norm()).collect()
    }

    /// 対数振幅(dB)の簡易SVGプロット。
    pub fn save_svg_magnitude_db(
        &self,
        path: &str,
        width: u32,
        height: u32,
    ) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;
        let mut f = File::create(path)?;
        let w = width as f64;
        let h = height as f64;
        let n = self.len();
        writeln!(f, "<svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}' viewBox='0 0 {width} {height}'>")?;
        writeln!(f, "<rect width='100%' height='100%' fill='white' />")?;
        if n > 1 {
            let max_k = (n - 1) as f64;
            write!(
                f,
                "<polyline fill='none' stroke='black' stroke-width='1' points='"
            )?;
            for (k, c) in self.data.iter().enumerate() {
                let x = (k as f64 / max_k) * w;
                let mag = c.norm();
                let db = if mag > 0.0 {
                    20.0 * mag.log10()
                } else {
                    -120.0
                };
                // dB を [0, -120] → [0,h] にマップ（0dBが上）
                let y = h * (1.0 - ((db + 120.0) / 120.0));
                write!(f, "{x:.3},{y:.3} ")?;
            }
            writeln!(f, "' />")?;
        }
        writeln!(f, "</svg>")?;
        Ok(())
    }

    /// 軸・凡例・複数系列対応の SVG（スペクトル用）。self の振幅(dB)を 1 系列として描画。
    pub fn save_svg_magnitude_db_with_axes(
        &self,
        path: &str,
        width: u32,
        height: u32,
        label: &str,
    ) -> std::io::Result<()> {
        let mags: Vec<f64> = self
            .data
            .iter()
            .map(|c| {
                let m = c.norm();
                if m > 0.0 {
                    20.0 * m.log10()
                } else {
                    -120.0
                }
            })
            .collect();
        let s = Series { y: &mags, label };
        plot::save_svg_time_series(path, width, height, &[s], Some(self.sample_rate))
    }

    /// 複数スペクトルの dB 振幅を重ね描き。全系列のサンプルレートが同一前提。
    pub fn save_svg_magnitude_db_multi(
        path: &str,
        width: u32,
        height: u32,
        labeled_specs: &[(&Spectrum, &str)],
    ) -> std::io::Result<()> {
        if labeled_specs.is_empty() {
            return Ok(());
        }
        let sr = labeled_specs[0].0.sample_rate;
        let mag_vecs: Vec<Vec<f64>> = labeled_specs
            .iter()
            .map(|(sp, _)| {
                sp.data
                    .iter()
                    .map(|c| {
                        let m = c.norm();
                        if m > 0.0 {
                            20.0 * m.log10()
                        } else {
                            -120.0
                        }
                    })
                    .collect()
            })
            .collect();
        let series: Vec<Series> = mag_vecs
            .iter()
            .zip(labeled_specs.iter())
            .map(|(v, (_, label))| Series {
                y: v.as_slice(),
                label,
            })
            .collect();
        plot::save_svg_time_series(path, width, height, &series, Some(sr))
    }
}

impl AsRef<[Complex<f64>]> for Spectrum {
    fn as_ref(&self) -> &[Complex<f64>] {
        &self.data
    }
}

impl From<(Vec<Complex<f64>>, f64)> for Spectrum {
    fn from(value: (Vec<Complex<f64>>, f64)) -> Self {
        Spectrum::new(value.0, value.1)
    }
}

impl From<Spectrum> for Vec<Complex<f64>> {
    fn from(value: Spectrum) -> Self {
        value.data
    }
}
