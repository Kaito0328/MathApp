use wasm_bindgen::prelude::*;

// d.ts安定性のため、プリミティブ/フラット配列/usizeのみを使う
// 伝達関数は係数ベースで構築し、必要な解析/描画関数を提供

#[wasm_bindgen]
pub struct DiscreteTF {
    inner: lti_systems::DiscreteTransferFunction,
}

#[wasm_bindgen]
impl DiscreteTF {
    #[wasm_bindgen(constructor)]
    pub fn new(b: Vec<f64>, a: Vec<f64>, sample_rate: f64) -> DiscreteTF {
        let tf = lti_systems::discrete::TransferFunction::new_with_fs(
            lti_systems::Polynomial::new(b),
            lti_systems::Polynomial::new(a),
            sample_rate.max(1e-12),
        );
        DiscreteTF { inner: tf }
    }

    pub fn sample_rate(&self) -> f64 { self.inner.sample_rate() }

    pub fn set_sample_rate(&mut self, fs: f64) { self.inner.set_sample_rate(fs.max(1e-12)); }

    pub fn b_coeffs(&self) -> Vec<f64> { self.inner.b_coeffs().to_vec() }

    pub fn a_coeffs(&self) -> Vec<f64> { self.inner.a_coeffs().to_vec() }

    pub fn is_stable(&self) -> bool { self.inner.is_stable() }

    // 応答系
    pub fn impulse_response(&self, len: usize) -> Vec<f64> { self.inner.impulse_response(len) }

    pub fn step_response(&self, len: usize) -> Vec<f64> { self.inner.step_response(len) }

    pub fn frequency_response_mag_phase(&self, n_freqs: usize) -> Vec<f64> {
        // 返却: [mag0, phase0_deg, mag1, phase1, ...]
        use num_complex::Complex;
        let n = n_freqs.max(1);
        let mut out = Vec::with_capacity(n * 2);
        for k in 0..n {
            let omega = 2.0 * std::f64::consts::PI * (k as f64) / (n as f64);
            let z = Complex::from_polar(1.0, omega);
            let h = self.inner.eval_z(z);
            let m = h.norm();
            let ph = h.arg().to_degrees();
            out.push(m);
            out.push(ph);
        }
        out
    }

    // SVG描画（文字列返却）
    pub fn bode_svg(&self, width: u32, height: u32, n_points: usize, hz_axis: bool, legend: bool) -> String {
        let x_axis = if hz_axis { lti_systems::plot::DiscreteXAxis::Hz } else { lti_systems::plot::DiscreteXAxis::Radian };
        let opts = lti_systems::plot::DiscreteBodeOptions { n_points: n_points.max(2), legend, x_axis, ..Default::default() };
    let path = "/tmp/wasm_discrete_bode.svg";
    // 公開API: DiscreteTF::plot_bode_svg を使用
    let _ = self.inner.plot_bode_svg(path, width, height, &opts);
    std::fs::read_to_string(path).unwrap_or_default()
    }

    pub fn nyquist_svg(&self, width: u32, height: u32, n_points: usize, show_minus_one: bool, legend: bool) -> String {
        let opts = lti_systems::plot::DiscreteNyquistOptions { n_points: n_points.max(2), show_minus_one, legend, ..Default::default() };
    let path = "/tmp/wasm_discrete_nyquist.svg";
    let _ = self.inner.plot_nyquist_svg(path, width, height, &opts);
    std::fs::read_to_string(path).unwrap_or_default()
    }

    // 直列/並列/フィードバック（単純操作）
    pub fn series(&self, other: &DiscreteTF) -> DiscreteTF {
        DiscreteTF { inner: self.inner.series(&other.inner) }
    }
    pub fn parallel(&self, other: &DiscreteTF) -> DiscreteTF {
        DiscreteTF { inner: self.inner.parallel(&other.inner) }
    }
    pub fn feedback_unity(&self) -> DiscreteTF { DiscreteTF { inner: self.inner.feedback_unity() } }

    pub fn block_feedback_svg(&self, width: u32, height: u32, negative_feedback: bool, feedback_label: Option<String>) -> String {
    let path = "/tmp/wasm_discrete_blockfb.svg";
    let _ = self.inner.plot_block_feedback_svg(path, width, height, negative_feedback, feedback_label.as_deref());
        std::fs::read_to_string(path).unwrap_or_default()
    }
}

#[wasm_bindgen]
pub struct ContinuousTF {
    inner: lti_systems::ContinuousTransferFunction,
}

#[wasm_bindgen]
impl ContinuousTF {
    #[wasm_bindgen(constructor)]
    pub fn new(b: Vec<f64>, a: Vec<f64>) -> ContinuousTF {
        let tf = lti_systems::continuous::TransferFunction::new(
            lti_systems::Polynomial::new(b),
            lti_systems::Polynomial::new(a),
        );
        ContinuousTF { inner: tf }
    }

    pub fn b_coeffs(&self) -> Vec<f64> { self.inner.b_coeffs().to_vec() }
    pub fn a_coeffs(&self) -> Vec<f64> { self.inner.a_coeffs().to_vec() }
    pub fn is_stable(&self) -> bool { self.inner.is_stable() }

    pub fn impulse_response(&self, fs: f64, len: usize) -> Vec<f64> { self.inner.impulse_response(fs, len) }
    pub fn step_response(&self, fs: f64, len: usize) -> Vec<f64> { self.inner.step_response(fs, len) }

    pub fn frequency_response_mag_phase(&self, omega_max: f64, n_freqs: usize) -> Vec<f64> {
        let resp = self.inner.frequency_response(omega_max.max(0.0), n_freqs);
        let mut out = Vec::with_capacity(resp.len()*2);
        for h in resp { out.push(h.norm()); out.push(h.arg().to_degrees()); }
        out
    }

    pub fn to_discrete_bilinear(&self, fs: f64) -> DiscreteTF {
        DiscreteTF { inner: self.inner.to_discrete_bilinear(fs) }
    }
    pub fn to_discrete_bilinear_prewarp(&self, fs: f64, f_warp_hz: f64) -> DiscreteTF {
        DiscreteTF { inner: self.inner.to_discrete_bilinear_prewarp(fs, f_warp_hz) }
    }

    pub fn bode_svg(&self, width: u32, height: u32, f_min_hz: f64, f_max_hz: f64, n_points: usize, legend: bool) -> String {
    let opts = lti_systems::plot::ContinuousBodeOptions { n_points: n_points.max(2), f_min_hz, f_max_hz, legend, ..Default::default() };
    let path = "/tmp/wasm_cont_bode.svg";
    let _ = self.inner.plot_bode_svg(path, width, height, &opts);
        std::fs::read_to_string(path).unwrap_or_default()
    }
    pub fn nyquist_svg(&self, width: u32, height: u32, f_min_hz: f64, f_max_hz: f64, n_points: usize, log_freq: bool, legend: bool) -> String {
    let opts = lti_systems::plot::ContinuousNyquistOptions { n_points: n_points.max(2), f_min_hz, f_max_hz, log_freq, legend, ..Default::default() };
    let path = "/tmp/wasm_cont_nyquist.svg";
    let _ = self.inner.plot_nyquist_svg(path, width, height, &opts);
        std::fs::read_to_string(path).unwrap_or_default()
    }

    pub fn block_feedback_svg(&self, width: u32, height: u32, negative_feedback: bool, feedback_label: Option<String>) -> String {
    let path = "/tmp/wasm_cont_blockfb.svg";
    let _ = self.inner.plot_block_feedback_svg(path, width, height, negative_feedback, feedback_label.as_deref());
        std::fs::read_to_string(path).unwrap_or_default()
    }
}

// ZPK 表現の薄いバインディング（ゼロ/ポールは実部・虚部の交互配列で受け渡し）
#[wasm_bindgen]
pub struct ContinuousZpk { zeros: Vec<f64>, poles: Vec<f64>, gain: f64 }
#[wasm_bindgen]
impl ContinuousZpk {
    #[wasm_bindgen(constructor)]
    pub fn new(zeros_interleaved: Vec<f64>, poles_interleaved: Vec<f64>, gain: f64) -> ContinuousZpk {
        ContinuousZpk { zeros: zeros_interleaved, poles: poles_interleaved, gain }
    }
    pub fn from_tf(tf: &ContinuousTF) -> ContinuousZpk {
        let zpk = lti_systems::zpk::ContinuousZpk::from_transfer_function(&tf.inner);
        let mut z = Vec::with_capacity(zpk.zeros.len()*2);
        for c in zpk.zeros { z.push(c.re); z.push(c.im); }
        let mut p = Vec::with_capacity(zpk.poles.len()*2);
        for c in zpk.poles { p.push(c.re); p.push(c.im); }
        ContinuousZpk { zeros: z, poles: p, gain: zpk.gain }
    }
    pub fn to_tf(&self) -> ContinuousTF {
        use num_complex::Complex;
        let zeros = self.zeros.chunks_exact(2).map(|xy| Complex::new(xy[0], xy[1])).collect::<Vec<_>>();
        let poles = self.poles.chunks_exact(2).map(|xy| Complex::new(xy[0], xy[1])).collect::<Vec<_>>();
        let zpk = lti_systems::zpk::ContinuousZpk::new(zeros, poles, self.gain);
        ContinuousTF { inner: zpk.to_transfer_function() }
    }
    pub fn zeros_interleaved(&self) -> Vec<f64> { self.zeros.clone() }
    pub fn poles_interleaved(&self) -> Vec<f64> { self.poles.clone() }
    pub fn gain(&self) -> f64 { self.gain }
}

#[wasm_bindgen]
pub struct DiscreteZpk { zeros: Vec<f64>, poles: Vec<f64>, gain: f64, sample_rate: f64 }
#[wasm_bindgen]
impl DiscreteZpk {
    #[wasm_bindgen(constructor)]
    pub fn new(zeros_interleaved: Vec<f64>, poles_interleaved: Vec<f64>, gain: f64, sample_rate: f64) -> DiscreteZpk {
        DiscreteZpk { zeros: zeros_interleaved, poles: poles_interleaved, gain, sample_rate }
    }
    pub fn from_tf(tf: &DiscreteTF) -> DiscreteZpk {
        let zpk = lti_systems::zpk::DiscreteZpk::from_transfer_function(&tf.inner);
        let mut z = Vec::with_capacity(zpk.zeros.len()*2);
        for c in zpk.zeros { z.push(c.re); z.push(c.im); }
        let mut p = Vec::with_capacity(zpk.poles.len()*2);
        for c in zpk.poles { p.push(c.re); p.push(c.im); }
        DiscreteZpk { zeros: z, poles: p, gain: zpk.gain, sample_rate: tf.inner.sample_rate() }
    }
    pub fn to_tf(&self) -> DiscreteTF {
        use num_complex::Complex;
        let zeros = self.zeros.chunks_exact(2).map(|xy| Complex::new(xy[0], xy[1])).collect::<Vec<_>>();
        let poles = self.poles.chunks_exact(2).map(|xy| Complex::new(xy[0], xy[1])).collect::<Vec<_>>();
        let zpk = lti_systems::zpk::DiscreteZpk::new(zeros, poles, self.gain);
        DiscreteTF { inner: zpk.to_transfer_function(self.sample_rate) }
    }
    pub fn zeros_interleaved(&self) -> Vec<f64> { self.zeros.clone() }
    pub fn poles_interleaved(&self) -> Vec<f64> { self.poles.clone() }
    pub fn gain(&self) -> f64 { self.gain }
    pub fn sample_rate(&self) -> f64 { self.sample_rate }
}

// ===== State-Space bindings =====

#[wasm_bindgen]
pub struct ContinuousSS {
    a: Vec<f64>,
    b: Vec<f64>,
    c: Vec<f64>,
    d: Vec<f64>,
    na: usize,
    ma: usize,
    nb: usize,
    mb: usize,
    nc: usize,
    mc: usize,
    nd: usize,
    md: usize,
}

impl ContinuousSS {
    fn from_inner(ss: &lti_systems::ContinuousStateSpace) -> Self {
        Self {
            a: ss.a.data.clone(),
            b: ss.b.data.clone(),
            c: ss.c.data.clone(),
            d: ss.d.data.clone(),
            na: ss.a.rows, ma: ss.a.cols,
            nb: ss.b.rows, mb: ss.b.cols,
            nc: ss.c.rows, mc: ss.c.cols,
            nd: ss.d.rows, md: ss.d.cols,
        }
    }
}

#[wasm_bindgen]
impl ContinuousSS {
    #[wasm_bindgen(constructor)]
    pub fn new(a: Vec<f64>, na: usize, ma: usize,
               b: Vec<f64>, nb: usize, mb: usize,
               c: Vec<f64>, nc: usize, mc: usize,
               d: Vec<f64>, nd: usize, md: usize) -> ContinuousSS {
        // ここでは単なるホルダ（行列演算は内側へ変換するAPIで実施）
        ContinuousSS { a, b, c, d, na, ma, nb, mb, nc, mc, nd, md }
    }

    pub fn from_tf_siso(num: Vec<f64>, den: Vec<f64>) -> ContinuousSS {
        let nump = lti_systems::Polynomial::new(num);
        let denp = lti_systems::Polynomial::new(den);
        let css = lti_systems::statespace::ContinuousStateSpace::from_tf_siso(&nump, &denp);
        Self::from_inner(&css)
    }

    pub fn to_tf_siso(&self) -> Vec<f64> {
        // 返却は [num..., SEP=-1.0, den...] のフラット形式にする（d.ts安定）
        let a = linalg::matrix::Matrix::new(self.na, self.ma, self.a.clone()).unwrap();
        let b = linalg::matrix::Matrix::new(self.nb, self.mb, self.b.clone()).unwrap();
        let c = linalg::matrix::Matrix::new(self.nc, self.mc, self.c.clone()).unwrap();
        let d = linalg::matrix::Matrix::new(self.nd, self.md, self.d.clone()).unwrap();
        let css = lti_systems::statespace::ContinuousStateSpace { a, b, c, d };
        let rf = css.to_tf_siso();
        let mut out = rf.numerator.coeffs.clone();
        out.push(-1.0);
        out.extend_from_slice(&rf.denominator.coeffs);
        out
    }

    pub fn c2d_zoh(&self, fs: f64) -> DiscreteSS {
        let a = linalg::matrix::Matrix::new(self.na, self.ma, self.a.clone()).unwrap();
        let b = linalg::matrix::Matrix::new(self.nb, self.mb, self.b.clone()).unwrap();
        let c = linalg::matrix::Matrix::new(self.nc, self.mc, self.c.clone()).unwrap();
        let d = linalg::matrix::Matrix::new(self.nd, self.md, self.d.clone()).unwrap();
        let css = lti_systems::statespace::ContinuousStateSpace { a, b, c, d };
        let dss = css.c2d_zoh(fs);
        DiscreteSS::from_inner(&dss)
    }

    // getters（行列内容をそのまま返す）
    pub fn a_flat(&self) -> Vec<f64> { self.a.clone() }
    pub fn b_flat(&self) -> Vec<f64> { self.b.clone() }
    pub fn c_flat(&self) -> Vec<f64> { self.c.clone() }
    pub fn d_flat(&self) -> Vec<f64> { self.d.clone() }
    pub fn a_shape(&self) -> Vec<usize> { vec![self.na, self.ma] }
    pub fn b_shape(&self) -> Vec<usize> { vec![self.nb, self.mb] }
    pub fn c_shape(&self) -> Vec<usize> { vec![self.nc, self.mc] }
    pub fn d_shape(&self) -> Vec<usize> { vec![self.nd, self.md] }
}

#[wasm_bindgen]
pub struct DiscreteSS {
    a: Vec<f64>, b: Vec<f64>, c: Vec<f64>, d: Vec<f64>,
    na: usize, ma: usize, nb: usize, mb: usize, nc: usize, mc: usize, nd: usize, md: usize,
}

impl DiscreteSS {
    fn from_inner(ss: &lti_systems::DiscreteStateSpace) -> Self {
        Self {
            a: ss.a.data.clone(), b: ss.b.data.clone(), c: ss.c.data.clone(), d: ss.d.data.clone(),
            na: ss.a.rows, ma: ss.a.cols, nb: ss.b.rows, mb: ss.b.cols, nc: ss.c.rows, mc: ss.c.cols, nd: ss.d.rows, md: ss.d.cols,
        }
    }
}

#[wasm_bindgen]
impl DiscreteSS {
    #[wasm_bindgen(constructor)]
    pub fn new(a: Vec<f64>, na: usize, ma: usize,
               b: Vec<f64>, nb: usize, mb: usize,
               c: Vec<f64>, nc: usize, mc: usize,
               d: Vec<f64>, nd: usize, md: usize) -> DiscreteSS {
        DiscreteSS { a, b, c, d, na, ma, nb, mb, nc, mc, nd, md }
    }

    pub fn to_tf_siso(&self) -> Vec<f64> {
        // 参考実装は未完成のため、Dのみ返す形になるが形式は合わせる
        let a = linalg::matrix::Matrix::new(self.na, self.ma, self.a.clone()).unwrap();
        let b = linalg::matrix::Matrix::new(self.nb, self.mb, self.b.clone()).unwrap();
        let c = linalg::matrix::Matrix::new(self.nc, self.mc, self.c.clone()).unwrap();
        let d = linalg::matrix::Matrix::new(self.nd, self.md, self.d.clone()).unwrap();
    let mut dss = lti_systems::statespace::DiscreteStateSpace { a, b, c, d };
    let rf = dss.to_tf_siso();
    let mut out = rf.numerator.coeffs.clone();
        out.push(-1.0);
    out.extend_from_slice(&rf.denominator.coeffs);
        out
    }

    pub fn a_flat(&self) -> Vec<f64> { self.a.clone() }
    pub fn b_flat(&self) -> Vec<f64> { self.b.clone() }
    pub fn c_flat(&self) -> Vec<f64> { self.c.clone() }
    pub fn d_flat(&self) -> Vec<f64> { self.d.clone() }
    pub fn a_shape(&self) -> Vec<usize> { vec![self.na, self.ma] }
    pub fn b_shape(&self) -> Vec<usize> { vec![self.nb, self.mb] }
    pub fn c_shape(&self) -> Vec<usize> { vec![self.nc, self.mc] }
    pub fn d_shape(&self) -> Vec<usize> { vec![self.nd, self.md] }
}
