use wasm_bindgen::prelude::*;
use num_complex::Complex;
use signal_processing::signal::{Signal, Spectrum};
use serde::{Deserialize, Serialize};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// ===== Exports to JS =====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DftResult {
    /// Interleaved [re, im, re, im, ...]
    pub spectrum: Vec<f64>,
    pub sample_rate: f64,
}

#[wasm_bindgen]
pub fn dft_real_obj(input: Vec<f64>, sample_rate: f64) -> JsValue {
    let sig = Signal::new(input, sample_rate);
    let spec: Spectrum = sig.dft();
    let complex_vec: Vec<Complex<f64>> = spec.into();
    let spectrum: Vec<f64> = complex_vec.into_iter().flat_map(|c| [c.re, c.im]).collect();
    let out = DftResult { spectrum, sample_rate };
    serde_wasm_bindgen::to_value(&out).unwrap()
}

#[wasm_bindgen]
pub fn dft_real(input: Vec<f64>) -> Vec<f64> {
    let sig = Signal::new(input, 1.0);
    let spec: Spectrum = sig.dft();
    let complex_vec: Vec<Complex<f64>> = spec.into();
    complex_vec.into_iter().flat_map(|c| [c.re, c.im]).collect()
}

#[wasm_bindgen]
pub fn ift_real(real_im_pairs: Vec<f64>, sample_rate: f64) -> Vec<f64> {
    let pairs: Vec<Complex<f64>> = real_im_pairs
        .chunks(2)
        .map(|ch| Complex::new(*ch.get(0).unwrap_or(&0.0), *ch.get(1).unwrap_or(&0.0)))
        .collect();
    let spec = Spectrum::new(pairs, sample_rate);
    let sig = spec.ift();
    sig.data().to_vec()
}

#[wasm_bindgen]
pub fn conv_real(x: Vec<f64>, h: Vec<f64>) -> Vec<f64> {
    use signal_processing::dft::conv_auto_f64;
    conv_auto_f64(&x, &h)
}
