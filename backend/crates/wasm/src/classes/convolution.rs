//! convolution クレートの WASM バインディング

use wasm_bindgen::prelude::*;

// 実数 1D 畳み込み（naive / FFT / auto）を公開
// Vec<f64> は JS 側では Float64Array と相互変換される

#[wasm_bindgen(js_name = convolveNaiveF64)]
pub fn convolve_naive_f64_js(x: Vec<f64>, h: Vec<f64>) -> Vec<f64> {
	convolution::convolve_naive_f64(&x, &h)
}

#[wasm_bindgen(js_name = convolveFftF64)]
pub fn convolve_fft_f64_js(x: Vec<f64>, h: Vec<f64>) -> Result<Vec<f64>, JsError> {
	convolution::convolve_fft_f64(&x, &h)
		.map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen(js_name = convolveAutoF64)]
pub fn convolve_auto_f64_js(x: Vec<f64>, h: Vec<f64>, threshold: usize) -> Result<Vec<f64>, JsError> {
	convolution::convolve_auto_f64(&x, &h, threshold)
		.map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen(js_name = defaultConvolutionThreshold)]
pub fn default_convolution_threshold() -> usize { convolution::DEFAULT_THRESHOLD }
