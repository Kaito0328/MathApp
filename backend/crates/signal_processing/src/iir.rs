use crate::signal::Signal;
use lti_systems::continuous::TransferFunction as ContinuousTransferFunction;
use lti_systems::discrete::TransferFunction as DiscreteTransferFunction;
use lti_systems::zpk;
use num_complex::Complex;
use polynomial::Polynomial;
use std::f64::consts::PI;

/// IIR フィルタ型（分子 b と分母 a の多項式）。係数は低次→高次。
#[derive(Clone, Debug, PartialEq)]
pub struct IIRFilter {
    tf: DiscreteTransferFunction,
}

impl IIRFilter {
    /// a[0] を 1 に正規化して保持（a[0] == 0 はパニック）。
    pub fn new(mut b: Polynomial<f64>, mut a: Polynomial<f64>) -> Self {
        let a0 = a.coeffs.first().copied().unwrap_or(1.0);
        assert!(a0 != 0.0, "a[0] must not be zero");
        if (a0 - 1.0).abs() > 0.0 {
            for c in b.coeffs.iter_mut() {
                *c /= a0;
            }
            for c in a.coeffs.iter_mut() {
                *c /= a0;
            }
        }
        Self {
            tf: DiscreteTransferFunction::new(b, a),
        }
    }

    /// サンプリング周波数を指定して生成
    pub fn new_with_fs(mut b: Polynomial<f64>, mut a: Polynomial<f64>, fs: f64) -> Self {
        let a0 = a.coeffs.first().copied().unwrap_or(1.0);
        assert!(a0 != 0.0, "a[0] must not be zero");
        if (a0 - 1.0).abs() > 0.0 {
            for c in b.coeffs.iter_mut() {
                *c /= a0;
            }
            for c in a.coeffs.iter_mut() {
                *c /= a0;
            }
        }
        Self {
            tf: DiscreteTransferFunction::new_with_fs(b, a, fs),
        }
    }

    /// 伝達関数から IIRFilter を生成
    pub fn from_transfer(tf: &DiscreteTransferFunction) -> Self {
        Self { tf: tf.clone() }
    }

    /// 自身を伝達関数として取得
    pub fn as_transfer(&self) -> DiscreteTransferFunction {
        self.tf.clone()
    }

    /// 直接型Iで適用した結果を返す（内部は既存差分方程式）。
    pub fn apply(&self, x: &Signal) -> Signal {
        let y_vec = self.tf.apply(x.data());
        Signal::new(y_vec, x.sample_rate())
    }

    /// デジタル Butterworth IIR を設計（プリワープ双一次）。
    /// - fs: サンプリング周波数 [Hz]
    /// - spec: 目標特性（Hz 指定）
    pub fn design_digital_butterworth(order: usize, fs: f64, spec: DigitalFilterSpec) -> Self {
        assert!(order >= 1);
        assert!(fs > 0.0);
        use DigitalFilterSpec as D;
        // 1) アナログ正規化LPをZPKで作成（wc=1）
        let zpk_lp = design_analog_butterworth_lp(order, 1.0);
        // 2) 仕様に応じてアナログZPKへ変換（必要な場合は周波数プリワープ）
        let ctf = match spec {
            D::Lowpass { fc_hz } => {
                let wc = 2.0 * std::f64::consts::PI * fc_hz;
                let zpk = lp_zpk_to(&zpk_lp, AnalogFilterSpec::Lowpass { wc });
                ContinuousTransferFunction::from_zpk(&zpk).to_discrete_bilinear_prewarp(fs, fc_hz)
            }
            D::Highpass { fc_hz } => {
                let wc = 2.0 * std::f64::consts::PI * fc_hz;
                let zpk = lp_zpk_to(&zpk_lp, AnalogFilterSpec::Highpass { wc });
                ContinuousTransferFunction::from_zpk(&zpk).to_discrete_bilinear_prewarp(fs, fc_hz)
            }
            D::Bandpass { f1_hz, f2_hz } => {
                assert!(f2_hz > f1_hz && f1_hz > 0.0);
                // エッジ周波数をプリワープしてアナログBPに変換 → 通常双一次
                let w1 = prewarp_rad_per_s(fs, f1_hz);
                let w2 = prewarp_rad_per_s(fs, f2_hz);
                let zpk = lp_zpk_to(&zpk_lp, AnalogFilterSpec::Bandpass { w1, w2 });
                ContinuousTransferFunction::from_zpk(&zpk).to_discrete_bilinear(fs)
            }
            D::Bandstop { f1_hz, f2_hz } => {
                assert!(f2_hz > f1_hz && f1_hz > 0.0);
                let w1 = prewarp_rad_per_s(fs, f1_hz);
                let w2 = prewarp_rad_per_s(fs, f2_hz);
                let zpk = lp_zpk_to(&zpk_lp, AnalogFilterSpec::Bandstop { w1, w2 });
                ContinuousTransferFunction::from_zpk(&zpk).to_discrete_bilinear(fs)
            }
        };
        IIRFilter::from_transfer(&ctf)
    }

    /// デジタル Chebyshev I IIR を設計（プリワープ双一次）。
    pub fn design_digital_chebyshev1(
        order: usize,
        ripple_db: f64,
        fs: f64,
        spec: DigitalFilterSpec,
    ) -> Self {
        assert!(order >= 1);
        assert!(ripple_db >= 0.0);
        assert!(fs > 0.0);
        use DigitalFilterSpec as D;
        let zpk_lp = design_analog_chebyshev1_lp(order, ripple_db, 1.0);
        let dtf = match spec {
            D::Lowpass { fc_hz } => {
                let wc = 2.0 * std::f64::consts::PI * fc_hz;
                let zpk = lp_zpk_to(&zpk_lp, AnalogFilterSpec::Lowpass { wc });
                ContinuousTransferFunction::from_zpk(&zpk).to_discrete_bilinear_prewarp(fs, fc_hz)
            }
            D::Highpass { fc_hz } => {
                let wc = 2.0 * std::f64::consts::PI * fc_hz;
                let zpk = lp_zpk_to(&zpk_lp, AnalogFilterSpec::Highpass { wc });
                ContinuousTransferFunction::from_zpk(&zpk).to_discrete_bilinear_prewarp(fs, fc_hz)
            }
            D::Bandpass { f1_hz, f2_hz } => {
                assert!(f2_hz > f1_hz && f1_hz > 0.0);
                let w1 = prewarp_rad_per_s(fs, f1_hz);
                let w2 = prewarp_rad_per_s(fs, f2_hz);
                let zpk = lp_zpk_to(&zpk_lp, AnalogFilterSpec::Bandpass { w1, w2 });
                ContinuousTransferFunction::from_zpk(&zpk).to_discrete_bilinear(fs)
            }
            D::Bandstop { f1_hz, f2_hz } => {
                assert!(f2_hz > f1_hz && f1_hz > 0.0);
                let w1 = prewarp_rad_per_s(fs, f1_hz);
                let w2 = prewarp_rad_per_s(fs, f2_hz);
                let zpk = lp_zpk_to(&zpk_lp, AnalogFilterSpec::Bandstop { w1, w2 });
                ContinuousTransferFunction::from_zpk(&zpk).to_discrete_bilinear(fs)
            }
        };
        IIRFilter::from_transfer(&dtf)
    }

    /// デジタル Chebyshev II IIR を設計（プリワープ双一次）。
    pub fn design_digital_chebyshev2(
        order: usize,
        stopband_atten_db: f64,
        fs: f64,
        spec: DigitalFilterSpec,
    ) -> Self {
        assert!(order >= 1);
        assert!(stopband_atten_db > 0.0);
        assert!(fs > 0.0);
        use DigitalFilterSpec as D;
        let zpk_lp = design_analog_chebyshev2_lp(order, stopband_atten_db, 1.0);
        let dtf = match spec {
            D::Lowpass { fc_hz } => {
                let wc = 2.0 * std::f64::consts::PI * fc_hz;
                let zpk = lp_zpk_to(&zpk_lp, AnalogFilterSpec::Lowpass { wc });
                ContinuousTransferFunction::from_zpk(&zpk).to_discrete_bilinear_prewarp(fs, fc_hz)
            }
            D::Highpass { fc_hz } => {
                let wc = 2.0 * std::f64::consts::PI * fc_hz;
                let zpk = lp_zpk_to(&zpk_lp, AnalogFilterSpec::Highpass { wc });
                ContinuousTransferFunction::from_zpk(&zpk).to_discrete_bilinear_prewarp(fs, fc_hz)
            }
            D::Bandpass { f1_hz, f2_hz } => {
                assert!(f2_hz > f1_hz && f1_hz > 0.0);
                let w1 = prewarp_rad_per_s(fs, f1_hz);
                let w2 = prewarp_rad_per_s(fs, f2_hz);
                let zpk = lp_zpk_to(&zpk_lp, AnalogFilterSpec::Bandpass { w1, w2 });
                ContinuousTransferFunction::from_zpk(&zpk).to_discrete_bilinear(fs)
            }
            D::Bandstop { f1_hz, f2_hz } => {
                assert!(f2_hz > f1_hz && f1_hz > 0.0);
                let w1 = prewarp_rad_per_s(fs, f1_hz);
                let w2 = prewarp_rad_per_s(fs, f2_hz);
                let zpk = lp_zpk_to(&zpk_lp, AnalogFilterSpec::Bandstop { w1, w2 });
                ContinuousTransferFunction::from_zpk(&zpk).to_discrete_bilinear(fs)
            }
        };
        IIRFilter::from_transfer(&dtf)
    }
}

impl Signal {
    /// IIR フィルタを適用。
    pub fn apply_iir(&self, filt: &IIRFilter) -> Signal {
        filt.apply(self)
    }
}

// ===== ローパス → 他特性 変換 (連続系, ZPKベース) =====

/// ターゲット特性の指定
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnalogFilterSpec {
    Lowpass { wc: f64 },
    Highpass { wc: f64 },
    Bandpass { w1: f64, w2: f64 },
    Bandstop { w1: f64, w2: f64 },
}

/// デジタルIIR設計用の仕様（Hz 単位）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DigitalFilterSpec {
    Lowpass { fc_hz: f64 },
    Highpass { fc_hz: f64 },
    Bandpass { f1_hz: f64, f2_hz: f64 },
    Bandstop { f1_hz: f64, f2_hz: f64 },
}

fn prewarp_rad_per_s(fs: f64, f_hz: f64) -> f64 {
    // ωa = 2 fs tan(π f/fs)
    assert!(fs > 0.0);
    if f_hz <= 0.0 {
        return 0.0;
    }
    let x = std::f64::consts::PI * f_hz / fs;
    2.0 * fs * x.tan()
}

fn zpk_lp_scale(zpk_lp: &zpk::ContinuousZpk, wc: f64) -> zpk::ContinuousZpk {
    let zeros: Vec<_> = zpk_lp.zeros.iter().map(|&r| r * wc).collect();
    let poles: Vec<_> = zpk_lp.poles.iter().map(|&r| r * wc).collect();
    let gain = zpk_lp.gain * wc.powi((zeros.len() as i32) - (poles.len() as i32));
    zpk::ContinuousZpk::new(zeros, poles, gain)
}

fn zpk_lp_to_hp(zpk_lp: &zpk::ContinuousZpk, wc: f64) -> zpk::ContinuousZpk {
    let z = &zpk_lp.zeros;
    let p = &zpk_lp.poles;
    let n = zpk_lp.poles.len();
    let mut zeros: Vec<Complex<f64>> = Vec::new();
    // map finite zeros
    for &zr in z {
        if zr == Complex::new(0.0, 0.0) {
            // LP 零点が原点なら HP では無限遠 → 無視（次数調整は原点零点で）
            continue;
        }
        zeros.push(wc / zr);
    }
    // add zeros at origin to match order
    while zeros.len() < n {
        zeros.push(Complex::new(0.0, 0.0));
    }
    let poles: Vec<_> = p.iter().map(|&pr| wc / pr).collect();
    // HP の高域利得を 1 に（k = 1）
    zpk::ContinuousZpk::new(zeros, poles, 1.0)
}

fn map_root_lp_to_bp(r: Complex<f64>, w0: f64, bw: f64) -> [Complex<f64>; 2] {
    // s^2 - (r*bw) s + w0^2 = 0
    let rbw = r * Complex::new(bw, 0.0);
    let disc = rbw * rbw - Complex::new(4.0 * w0 * w0, 0.0);
    let sqrt_disc = disc.sqrt();
    let s1 = (rbw + sqrt_disc) * Complex::new(0.5, 0.0);
    let s2 = (rbw - sqrt_disc) * Complex::new(0.5, 0.0);
    [s1, s2]
}

fn zpk_lp_to_bp(zpk_lp: &zpk::ContinuousZpk, w1: f64, w2: f64) -> zpk::ContinuousZpk {
    let z = &zpk_lp.zeros;
    let p = &zpk_lp.poles;
    assert!(w1 > 0.0 && w2 > 0.0 && w2 > w1);
    let w0 = (w1 * w2).sqrt();
    let bw = w2 - w1;

    let mut zeros: Vec<Complex<f64>> = Vec::new();
    for &zr in z {
        let pair = map_root_lp_to_bp(zr, w0, bw);
        zeros.push(pair[0]);
        zeros.push(pair[1]);
    }
    // all-pole LP → N 個の原点零点追加（BPでは N 個の s=0 零点と ∞ 零点が生じる）
    let n = p.len();
    for _ in 0..n {
        zeros.push(Complex::new(0.0, 0.0));
    }

    let mut poles: Vec<Complex<f64>> = Vec::with_capacity(2 * n);
    for &pr in p {
        let pair = map_root_lp_to_bp(pr, w0, bw);
        poles.push(pair[0]);
        poles.push(pair[1]);
    }

    // ゲインは |H(j w0)| = 1 に正規化
    let mut zpk_tmp = zpk::ContinuousZpk::new(zeros, poles, 1.0);
    let val = zpk_tmp.eval_s(Complex::new(0.0, w0));
    let k = 1.0 / val.norm();
    zpk_tmp.gain = k;
    zpk_tmp
}

fn map_root_lp_to_bs(r: Complex<f64>, w0: f64, bw: f64) -> [Complex<f64>; 2] {
    // r s^2 - bw s + r w0^2 = 0 → s = [bw ± sqrt(bw^2 - 4 r^2 w0^2)] / (2 r)
    let r2 = r * r;
    let disc = Complex::new(bw * bw, 0.0) - Complex::new(4.0 * w0 * w0, 0.0) * r2;
    let sqrt_disc = disc.sqrt();
    let num1 = Complex::new(bw, 0.0) + sqrt_disc;
    let num2 = Complex::new(bw, 0.0) - sqrt_disc;
    let denom = Complex::new(2.0, 0.0) * r;
    [num1 / denom, num2 / denom]
}

fn zpk_lp_to_bs(zpk_lp: &zpk::ContinuousZpk, w1: f64, w2: f64) -> zpk::ContinuousZpk {
    let z = &zpk_lp.zeros;
    let p = &zpk_lp.poles;
    let _k = zpk_lp.gain;
    assert!(w1 > 0.0 && w2 > 0.0 && w2 > w1);
    let w0 = (w1 * w2).sqrt();
    let bw = w2 - w1;

    let mut zeros: Vec<Complex<f64>> = Vec::new();
    for &zr in z {
        let pair = map_root_lp_to_bs(zr, w0, bw);
        zeros.push(pair[0]);
        zeros.push(pair[1]);
    }
    // all-pole LP → 零点を ±j w0 に N 個ずつ追加
    let n = p.len();
    for _ in 0..n {
        zeros.push(Complex::new(0.0, w0));
        zeros.push(Complex::new(0.0, -w0));
    }

    let mut poles: Vec<Complex<f64>> = Vec::with_capacity(2 * n);
    for &pr in p {
        let pair = map_root_lp_to_bs(pr, w0, bw);
        poles.push(pair[0]);
        poles.push(pair[1]);
    }

    // ゲイン: H(0)=1 に正規化
    let zpk_tmp = zpk::ContinuousZpk::new(zeros, poles, 1.0);
    let mut num_prod = Complex::new(1.0, 0.0);
    for z in &zpk_tmp.zeros {
        num_prod *= -z;
    }
    let mut den_prod = Complex::new(1.0, 0.0);
    for p in &zpk_tmp.poles {
        den_prod *= -p;
    }
    let k = if num_prod == Complex::new(0.0, 0.0) {
        1.0
    } else {
        (den_prod / num_prod).re
    };
    zpk::ContinuousZpk::new(zpk_tmp.zeros, zpk_tmp.poles, k)
}

/// ZPKのLPプロトタイプから指定特性へ（ZPKのまま）
pub fn lp_zpk_to(zpk_lp: &zpk::ContinuousZpk, spec: AnalogFilterSpec) -> zpk::ContinuousZpk {
    match spec {
        AnalogFilterSpec::Lowpass { wc } => zpk_lp_scale(zpk_lp, wc),
        AnalogFilterSpec::Highpass { wc } => zpk_lp_to_hp(zpk_lp, wc),
        AnalogFilterSpec::Bandpass { w1, w2 } => zpk_lp_to_bp(zpk_lp, w1, w2),
        AnalogFilterSpec::Bandstop { w1, w2 } => zpk_lp_to_bs(zpk_lp, w1, w2),
    }
}

// ===== 統一 API: Butterworth / Chebyshev I / Chebyshev II =====

pub fn design_analog_butterworth(
    order: usize,
    spec: AnalogFilterSpec,
) -> ContinuousTransferFunction {
    // 正規化LP(ZPK) → 変換(ZPK) → 最後に1回だけTF化
    let zpk_lp = design_analog_butterworth_lp(order, 1.0);
    let zpk_out = lp_zpk_to(&zpk_lp, spec);
    ContinuousTransferFunction::from_zpk(&zpk_out)
}

pub fn design_analog_chebyshev1(
    order: usize,
    ripple_db: f64,
    spec: AnalogFilterSpec,
) -> ContinuousTransferFunction {
    let zpk_lp = design_analog_chebyshev1_lp(order, ripple_db, 1.0);
    let zpk_out = lp_zpk_to(&zpk_lp, spec);
    ContinuousTransferFunction::from_zpk(&zpk_out)
}

pub fn design_analog_chebyshev2(
    order: usize,
    stopband_atten_db: f64,
    spec: AnalogFilterSpec,
) -> ContinuousTransferFunction {
    let zpk_lp = design_analog_chebyshev2_lp(order, stopband_atten_db, 1.0);
    let zpk_out = lp_zpk_to(&zpk_lp, spec);
    ContinuousTransferFunction::from_zpk(&zpk_out)
}

// ===== ZPK プロトタイプ（LP）作成 =====

/// Butterworth LP (ZPK)
pub fn design_analog_butterworth_lp(order: usize, cutoff_freq: f64) -> zpk::ContinuousZpk {
    assert!(cutoff_freq > 0.0);
    assert!(order >= 1);
    let mut poles: Vec<Complex<f64>> = Vec::with_capacity(order);
    for k in 0..order {
        let theta = PI * (2 * (k as i32) + 1) as f64 / (2.0 * order as f64);
        poles.push(Complex::new(
            -cutoff_freq * theta.sin(),
            cutoff_freq * theta.cos(),
        ));
    }
    let mut prod = Complex::new(1.0, 0.0);
    for p in &poles {
        prod *= -p;
    }
    let gain = prod.re;
    zpk::ContinuousZpk::new(Vec::new(), poles, gain)
}

/// Chebyshev Type I LP (ZPK)
pub fn design_analog_chebyshev1_lp(
    order: usize,
    ripple_db: f64,
    cutoff_freq: f64,
) -> zpk::ContinuousZpk {
    assert!(order >= 1);
    assert!(cutoff_freq > 0.0);
    assert!(ripple_db >= 0.0);
    let epsilon = (10.0_f64.powf(ripple_db / 10.0) - 1.0).sqrt();
    let u = (1.0 / epsilon).asinh() / (order as f64);
    let mut poles: Vec<Complex<f64>> = Vec::with_capacity(order);
    for m in 1..=order {
        let theta = PI * (2 * m - 1) as f64 / (2.0 * order as f64);
        poles.push(Complex::new(
            -cutoff_freq * u.sinh() * theta.sin(),
            cutoff_freq * u.cosh() * theta.cos(),
        ));
    }
    let mut prod = Complex::new(1.0, 0.0);
    for p in &poles {
        prod *= -p;
    }
    let gain = prod.re;
    zpk::ContinuousZpk::new(Vec::new(), poles, gain)
}

/// Chebyshev Type II (Inverse) LP (ZPK)
pub fn design_analog_chebyshev2_lp(
    order: usize,
    stopband_atten_db: f64,
    cutoff_freq: f64,
) -> zpk::ContinuousZpk {
    assert!(order >= 1);
    assert!(cutoff_freq > 0.0);
    assert!(stopband_atten_db > 0.0);
    let eps = 1.0 / (10.0_f64.powf(stopband_atten_db / 10.0) - 1.0).sqrt();
    let u = (1.0 / eps).asinh() / (order as f64);

    let mut zeros: Vec<Complex<f64>> = Vec::with_capacity(order);
    for m in 1..=order / 2 {
        let theta = PI * (2 * m - 1) as f64 / (2.0 * order as f64);
        let w = cutoff_freq / theta.cos();
        zeros.push(Complex::new(0.0, w));
        zeros.push(Complex::new(0.0, -w));
    }

    let mut poles: Vec<Complex<f64>> = Vec::with_capacity(order);
    for m in 1..=order {
        let theta = PI * (2 * m - 1) as f64 / (2.0 * order as f64);
        let a = u.sinh() * theta.sin();
        let b = u.cosh() * theta.cos();
        let q = Complex::new(-a, b);
        let sp = cutoff_freq * q.conj() / (q.norm_sqr());
        poles.push(sp);
    }

    let mut num_prod = Complex::new(1.0, 0.0);
    for z in &zeros {
        num_prod *= -z;
    }
    let mut den_prod = Complex::new(1.0, 0.0);
    for p in &poles {
        den_prod *= -p;
    }
    let k = if num_prod == Complex::new(0.0, 0.0) {
        1.0
    } else {
        (den_prod / num_prod).re
    };
    zpk::ContinuousZpk::new(zeros, poles, k)
}
