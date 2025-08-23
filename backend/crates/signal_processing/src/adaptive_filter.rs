use linalg::Vector;

use crate::signal::Signal;

pub struct AdaptiveFilterLMS {
    weights: Vector<f64>,
    buffer: Vec<f64>,
    head: usize, // ring buffer head index (points to most-recent sample)
    step_size: f64,
}

impl AdaptiveFilterLMS {
    pub fn new(taps: usize, step_size: f64) -> Self {
        Self {
            weights: Vector::zeros(taps),
            buffer: vec![0.0; taps],
            head: 0,
            step_size,
        }
    }

    pub fn process_sample(&mut self, input: f64, desired: f64) -> (f64, f64) {
        // push_front on ring buffer
        let n = self.buffer.len();
        self.head = if self.head == 0 { n - 1 } else { self.head - 1 };
        self.buffer[self.head] = input;

        // y = w^T x (x is buffer viewed from head, most recent first)
        let mut y = 0.0;
        for i in 0..n {
            let xi = self.buffer[(self.head + i) % n];
            y += self.weights.data[i] * xi;
        }
        let e = desired - y;

        // w <- w + 2 * mu * e * x
        let alpha = 2.0 * self.step_size * e;
        for i in 0..n {
            let xi = self.buffer[(self.head + i) % n];
            self.weights.data[i] += alpha * xi;
        }
        (y, e)
    }
}

/// 正規化LMS (NLMS) 適応フィルタ
pub struct AdaptiveFilterNLMS {
    weights: Vector<f64>,
    buffer: Vec<f64>,
    head: usize,
    step_size: f64,
    epsilon: f64,
}

impl AdaptiveFilterNLMS {
    /// taps 本数、ステップサイズ mu、正則化 epsilon で初期化
    /// 典型的には 0 < mu < 2、epsilon は 1e-6 程度
    pub fn new(taps: usize, step_size: f64, epsilon: f64) -> Self {
        assert!(taps > 0);
        assert!(step_size > 0.0);
        assert!(epsilon >= 0.0);
        Self {
            weights: Vector::zeros(taps),
            buffer: vec![0.0; taps],
            head: 0,
            step_size,
            epsilon,
        }
    }

    /// 1 サンプル処理。出力 y と誤差 e を返す。
    pub fn process_sample(&mut self, input: f64, desired: f64) -> (f64, f64) {
        // push_front into ring buffer
        let n = self.buffer.len();
        self.head = if self.head == 0 { n - 1 } else { self.head - 1 };
        self.buffer[self.head] = input;

        // 同時に y = w^T x と x^T x を計算
        let mut y = 0.0;
        let mut x2 = 0.0;
        for i in 0..n {
            let xi = self.buffer[(self.head + i) % n];
            y += self.weights.data[i] * xi;
            x2 += xi * xi;
        }
        let e = desired - y;

        // w <- w + mu * e * x / (epsilon + ||x||^2)
        let denom = self.epsilon + x2;
        if denom > 0.0 {
            let alpha = self.step_size * e / denom;
            for i in 0..n {
                let xi = self.buffer[(self.head + i) % n];
                self.weights.data[i] += alpha * xi;
            }
        }
        (y, e)
    }

    /// 現在の重みベクトル（参照）
    pub fn weights(&self) -> &[f64] {
        &self.weights.data
    }

    /// 現在の重みベクトル（所有権を持つコピー）
    pub fn weights_vec(&self) -> Vec<f64> {
        self.weights.data.clone()
    }
}

pub fn lms_filter(
    input_signal: &Signal,
    desired_signal: &Signal,
    taps: usize,
    step_size: f64,
) -> (Signal, Signal) {
    let mut adaptive_filter = AdaptiveFilterLMS::new(taps, step_size);

    let num_samples = input_signal.len().min(desired_signal.len());
    let mut output_samples = Vec::with_capacity(num_samples);
    let mut error_samples = Vec::with_capacity(num_samples);

    for n in 0..num_samples {
        let (y, e) =
            adaptive_filter.process_sample(input_signal.data()[n], desired_signal.data()[n]);
        output_samples.push(y);
        error_samples.push(e);
    }

    let output_signal = Signal::new(output_samples, input_signal.sample_rate());
    let error_signal = Signal::new(error_samples, input_signal.sample_rate());

    (output_signal, error_signal)
}

/// NLMS を信号全体に適用するユーティリティ
pub fn nlms_filter(
    input_signal: &Signal,
    desired_signal: &Signal,
    taps: usize,
    step_size: f64,
    epsilon: f64,
) -> (Signal, Signal) {
    let mut adaptive_filter = AdaptiveFilterNLMS::new(taps, step_size, epsilon);

    let num_samples = input_signal.len().min(desired_signal.len());
    let mut output_samples = Vec::with_capacity(num_samples);
    let mut error_samples = Vec::with_capacity(num_samples);

    for n in 0..num_samples {
        let (y, e) =
            adaptive_filter.process_sample(input_signal.data()[n], desired_signal.data()[n]);
        output_samples.push(y);
        error_samples.push(e);
    }

    let output_signal = Signal::new(output_samples, input_signal.sample_rate());
    let error_signal = Signal::new(error_samples, input_signal.sample_rate());

    (output_signal, error_signal)
}
