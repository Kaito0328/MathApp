use wasm_bindgen::prelude::*;

use signal_processing as sp;
use crate::classes::lti_systems::WasmDiscreteTF;

#[wasm_bindgen]
pub struct WasmSignal {
	data: Vec<f64>,
	fs: f64,
}

#[wasm_bindgen]
impl WasmSignal {
	#[wasm_bindgen(constructor)]
	pub fn new(data: Vec<f64>, sample_rate: f64) -> WasmSignal {
		WasmSignal { data, fs: sample_rate }
	}

	pub fn data(&self) -> Vec<f64> { self.data.clone() }
	pub fn sample_rate(&self) -> f64 { self.fs }
	pub fn len(&self) -> usize { self.data.len() }

	pub fn dft(&self) -> WasmSpectrum {
		let sig = sp::signal::Signal::new(self.data.clone(), self.fs);
		let spec = sp::dft::dft_signal(&sig);
		let (cvec, fs) = spec.into_inner();
		let mut out: Vec<f64> = Vec::with_capacity(2 * cvec.len());
		for c in cvec { out.push(c.re); out.push(c.im); }
		WasmSpectrum { data_interleaved: out, fs }
	}

	pub fn convolve(&self, h: &WasmSignal) -> WasmSignal {
		let x = sp::signal::Signal::new(self.data.clone(), self.fs);
		let hh = sp::signal::Signal::new(h.data.clone(), h.fs);
		let y = sp::dft::conv_signal_auto(&x, &hh);
		let (data, fs) = y.into_inner();
		WasmSignal { data, fs }
	}

	pub fn apply_fir(&self, taps: Vec<f64>) -> WasmSignal {
		use lti_systems::Polynomial;
		let x = sp::signal::Signal::new(self.data.clone(), self.fs);
		let y = x.apply_fir(&Polynomial::new(taps));
		let (data, fs) = y.into_inner();
		WasmSignal { data, fs }
	}

	// Resampling helpers
	pub fn downsample(&self, factor: usize, filter_taps: usize) -> WasmSignal {
		let y = sp::sampling::down_sample(self.data.as_slice(), factor, filter_taps, sp::window::WindowType::Hann);
		let fs = if factor>0 { self.fs / factor as f64 } else { self.fs };
		WasmSignal { data: y, fs }
	}

	pub fn upsample(&self, factor: usize, filter_taps: usize) -> WasmSignal {
		let y = sp::sampling::upsample(self.data.as_slice(), factor, filter_taps, sp::window::WindowType::Hann);
		let fs = self.fs * factor as f64;
		WasmSignal { data: y, fs }
	}

	pub fn resample(&self, upsample_factor: usize, downsample_factor: usize, filter_taps: usize) -> WasmSignal {
		let y = sp::sampling::resample(self.data.as_slice(), upsample_factor, downsample_factor, filter_taps, sp::window::WindowType::Hann);
		let fs = self.fs * upsample_factor as f64 / downsample_factor as f64;
		WasmSignal { data: y, fs }
	}

	pub fn decimate(&self, factor: usize) -> WasmSignal {
		let y = sp::sampling::decimate(self.data.as_slice(), factor);
		let fs = if factor>0 { self.fs / factor as f64 } else { self.fs };
		WasmSignal { data: y, fs }
	}

	pub fn expand(&self, factor: usize) -> WasmSignal {
		let y = sp::sampling::expand(self.data.as_slice(), factor);
		let fs = self.fs * factor as f64;
		WasmSignal { data: y, fs }
	}

	pub fn save_svg_simple(&self, width: u32, height: u32) -> String {
		let s = sp::plot::Series { y: &self.data, label: "signal" };
		let mut buf = Vec::<u8>::new();
		// use writer-based function
		let _ = sp::plot::write_svg_time_series_to(&mut buf, width, height, &[s], Some(self.fs));
		String::from_utf8(buf).unwrap_or_default()
	}

	pub fn save_svg_with_axes(&self, width: u32, height: u32, label: Option<String>) -> String {
		let lab = label.as_deref().unwrap_or("signal");
		let s = sp::plot::Series { y: &self.data, label: lab };
		let mut buf = Vec::<u8>::new();
		let _ = sp::plot::write_svg_time_series_to(&mut buf, width, height, &[s], Some(self.fs));
		String::from_utf8(buf).unwrap_or_default()
	}
}

#[wasm_bindgen]
pub struct WasmSpectrum {
	// complex array interleaved [re, im, re, im, ...]
	data_interleaved: Vec<f64>,
	fs: f64,
}

#[wasm_bindgen]
impl WasmSpectrum {
	#[wasm_bindgen(constructor)]
	pub fn new(data_interleaved: Vec<f64>, sample_rate: f64) -> WasmSpectrum {
		WasmSpectrum { data_interleaved, fs: sample_rate }
	}
	pub fn data_interleaved(&self) -> Vec<f64> { self.data_interleaved.clone() }
	pub fn sample_rate(&self) -> f64 { self.fs }
	pub fn len(&self) -> usize { self.data_interleaved.len() / 2 }

	pub fn ift(&self) -> WasmSignal {
		use num_complex::Complex;
		let mut v: Vec<Complex<f64>> = Vec::with_capacity(self.len());
		let di = &self.data_interleaved;
		for k in (0..di.len()).step_by(2) {
			v.push(Complex::new(di[k], di[k+1]));
		}
		let spc = sp::signal::Spectrum::new(v, self.fs);
		let sig = sp::dft::ift_spectrum(&spc);
		let (data, fs) = sig.into_inner();
		WasmSignal { data, fs }
	}

	pub fn magnitude_db_svg(&self, width: u32, height: u32, label: Option<String>) -> String {
		use num_complex::Complex;
		let mut v: Vec<Complex<f64>> = Vec::with_capacity(self.len());
		let di = &self.data_interleaved;
		for k in (0..di.len()).step_by(2) {
			v.push(Complex::new(di[k], di[k+1]));
		}
		let spc = sp::signal::Spectrum::new(v, self.fs);
	let mut buf = Vec::<u8>::new();
	let label = label.as_deref().unwrap_or("spectrum");
	// writerベースのスケールド描画を直接使用
		let mags: Vec<f64> = spc
			.data()
			.iter()
			.map(|c| { let m = c.norm(); if m>0.0 {20.0*m.log10()} else {-120.0} })
			.collect();
		let s2 = sp::plot::Series { y: &mags, label };
		let _ = sp::plot::write_svg_series_scaled_to(&mut buf, width, height, &[s2], "bin", (self.len().saturating_sub(1)) as f64);
		String::from_utf8(buf).unwrap_or_default()
	}
}

#[wasm_bindgen]
pub fn window_hann(size: usize) -> Vec<f64> { sp::window::generate_window(size, sp::window::WindowType::Hann) }

#[wasm_bindgen]
pub fn window_hamming(size: usize) -> Vec<f64> { sp::window::generate_window(size, sp::window::WindowType::Hamming) }

#[wasm_bindgen]
pub fn window_blackman(size: usize) -> Vec<f64> { sp::window::generate_window(size, sp::window::WindowType::Blackman) }

#[wasm_bindgen]
pub fn window_rectangular(size: usize) -> Vec<f64> { sp::window::generate_window(size, sp::window::WindowType::Rectangular) }

#[wasm_bindgen]
pub fn window_kaiser(size: usize, beta: f64) -> Vec<f64> { sp::window::generate_window(size, sp::window::WindowType::Kaiser{ beta }) }

// ==== FIR design helpers (return coefficients low->high) ====
#[wasm_bindgen]
pub fn sp_design_fir_lowpass(num_taps: usize, normalized_cutoff: f64) -> Vec<f64> {
	sp::fir::design_fir_lowpass(num_taps, normalized_cutoff, sp::window::WindowType::Hann)
}
#[wasm_bindgen]
pub fn sp_design_fir_highpass(num_taps: usize, normalized_cutoff: f64) -> Vec<f64> {
	sp::fir::design_fir_highpass(num_taps, normalized_cutoff, sp::window::WindowType::Hann)
}
#[wasm_bindgen]
pub fn sp_design_fir_bandpass(num_taps: usize, f1: f64, f2: f64) -> Vec<f64> {
	sp::fir::design_fir_bandpass(num_taps, f1, f2, sp::window::WindowType::Hann)
}
#[wasm_bindgen]
pub fn sp_design_fir_bandstop(num_taps: usize, f1: f64, f2: f64) -> Vec<f64> {
	sp::fir::design_fir_bandstop(num_taps, f1, f2, sp::window::WindowType::Hann)
}

// ==== IIR design helpers (return lti WasmDiscreteTF) ====
#[wasm_bindgen]
pub fn sp_design_iir_butter_lowpass(order: usize, fs: f64, fc_hz: f64) -> WasmDiscreteTF {
	use sp::iir::{DigitalFilterSpec, IIRFilter};
	let filt = IIRFilter::design_digital_butterworth(order, fs, DigitalFilterSpec::Lowpass{ fc_hz });
	let tf = filt.as_transfer();
	let b = tf.b_coeffs().to_vec();
	let a = tf.a_coeffs().to_vec();
	WasmDiscreteTF::new(b, a, tf.sample_rate())
}
#[wasm_bindgen]
pub fn sp_design_iir_butter_highpass(order: usize, fs: f64, fc_hz: f64) -> WasmDiscreteTF {
	use sp::iir::{DigitalFilterSpec, IIRFilter};
	let filt = IIRFilter::design_digital_butterworth(order, fs, DigitalFilterSpec::Highpass{ fc_hz });
	let tf = filt.as_transfer();
	WasmDiscreteTF::new(tf.b_coeffs().to_vec(), tf.a_coeffs().to_vec(), tf.sample_rate())
}
#[wasm_bindgen]
pub fn sp_design_iir_butter_bandpass(order: usize, fs: f64, f1_hz: f64, f2_hz: f64) -> WasmDiscreteTF {
	use sp::iir::{DigitalFilterSpec, IIRFilter};
	let filt = IIRFilter::design_digital_butterworth(order, fs, DigitalFilterSpec::Bandpass{ f1_hz, f2_hz });
	let tf = filt.as_transfer();
	WasmDiscreteTF::new(tf.b_coeffs().to_vec(), tf.a_coeffs().to_vec(), tf.sample_rate())
}
#[wasm_bindgen]
pub fn sp_design_iir_butter_bandstop(order: usize, fs: f64, f1_hz: f64, f2_hz: f64) -> WasmDiscreteTF {
	use sp::iir::{DigitalFilterSpec, IIRFilter};
	let filt = IIRFilter::design_digital_butterworth(order, fs, DigitalFilterSpec::Bandstop{ f1_hz, f2_hz });
	let tf = filt.as_transfer();
	WasmDiscreteTF::new(tf.b_coeffs().to_vec(), tf.a_coeffs().to_vec(), tf.sample_rate())
}

#[wasm_bindgen]
pub fn sp_design_iir_cheby1_lowpass(order: usize, ripple_db: f64, fs: f64, fc_hz: f64) -> WasmDiscreteTF {
	use sp::iir::{DigitalFilterSpec, IIRFilter};
	let filt = IIRFilter::design_digital_chebyshev1(order, ripple_db, fs, DigitalFilterSpec::Lowpass{ fc_hz});
	let tf = filt.as_transfer();
	WasmDiscreteTF::new(tf.b_coeffs().to_vec(), tf.a_coeffs().to_vec(), tf.sample_rate())
}
#[wasm_bindgen]
pub fn sp_design_iir_cheby2_lowpass(order: usize, stop_atten_db: f64, fs: f64, fc_hz: f64) -> WasmDiscreteTF {
	use sp::iir::{DigitalFilterSpec, IIRFilter};
	let filt = IIRFilter::design_digital_chebyshev2(order, stop_atten_db, fs, DigitalFilterSpec::Lowpass{ fc_hz});
	let tf = filt.as_transfer();
	WasmDiscreteTF::new(tf.b_coeffs().to_vec(), tf.a_coeffs().to_vec(), tf.sample_rate())
}

// ==== Adaptive filters (LMS / NLMS) ====
#[wasm_bindgen]
pub struct WasmLMS {
	inner: sp::adaptive_filter::AdaptiveFilterLMS,
}

#[wasm_bindgen]
impl WasmLMS {
	#[wasm_bindgen(constructor)]
	pub fn new(taps: usize, step_size: f64) -> WasmLMS { WasmLMS { inner: sp::adaptive_filter::AdaptiveFilterLMS::new(taps, step_size) } }

	pub fn process_sample(&mut self, input: f64, desired: f64) -> Vec<f64> {
		let (y,e) = self.inner.process_sample(input, desired);
		vec![y,e]
	}

	pub fn process_series(&mut self, input: Vec<f64>, desired: Vec<f64>) -> Vec<f64> {
		let n = input.len().min(desired.len());
		let mut out = Vec::with_capacity(2*n);
		for i in 0..n {
			let (y,e) = self.inner.process_sample(input[i], desired[i]);
			out.push(y); out.push(e);
		}
		out
	}
}

#[wasm_bindgen]
pub struct WasmNLMS {
	inner: sp::adaptive_filter::AdaptiveFilterNLMS,
}

#[wasm_bindgen]
impl WasmNLMS {
	#[wasm_bindgen(constructor)]
	pub fn new(taps: usize, step_size: f64, epsilon: f64) -> WasmNLMS { WasmNLMS { inner: sp::adaptive_filter::AdaptiveFilterNLMS::new(taps, step_size, epsilon) } }

	pub fn process_sample(&mut self, input: f64, desired: f64) -> Vec<f64> {
		let (y,e) = self.inner.process_sample(input, desired);
		vec![y,e]
	}

	pub fn process_series(&mut self, input: Vec<f64>, desired: Vec<f64>) -> Vec<f64> {
		let n = input.len().min(desired.len());
		let mut out = Vec::with_capacity(2*n);
		for i in 0..n {
			let (y,e) = self.inner.process_sample(input[i], desired[i]);
			out.push(y); out.push(e);
		}
		out
	}
}

// ==== 2D image processing bindings (f32) ====
#[wasm_bindgen]
pub enum WasmBorder { ConstantZero, Replicate, Reflect }

fn border_from(kind: WasmBorder) -> sp::image::convolution::Border {
	use sp::image::convolution::Border as B;
	match kind {
		WasmBorder::ConstantZero => B::Constant(0.0),
		WasmBorder::Replicate => B::Replicate,
		WasmBorder::Reflect => B::Reflect,
	}
}

#[wasm_bindgen]
pub fn img_convolve2d_f32_simple(src: Vec<f64>, width: usize, height: usize, kernel: Vec<f64>, kw: usize, kh: usize, border: WasmBorder) -> Vec<f64> {
	let s32: Vec<f32> = src.into_iter().map(|v| v as f32).collect();
	let k32: Vec<f32> = kernel.into_iter().map(|v| v as f32).collect();
	let img = sp::image::core::Image::<f32>::from_vec(s32, width, height);
	let ker = sp::image::convolution::Kernel::from_vec(k32, kw, kh);
	let y = sp::image::convolution::convolve2d_simple_f32(&img, &ker, border_from(border));
	y.as_slice().iter().map(|&v| v as f64).collect()
}

#[wasm_bindgen]
pub fn img_convolve2d_f32(src: Vec<f64>, width: usize, height: usize, kernel: Vec<f64>, kw: usize, kh: usize, border: WasmBorder) -> Vec<f64> {
	let s32: Vec<f32> = src.into_iter().map(|v| v as f32).collect();
	let k32: Vec<f32> = kernel.into_iter().map(|v| v as f32).collect();
	let img = sp::image::core::Image::<f32>::from_vec(s32, width, height);
	let ker = sp::image::convolution::Kernel::from_vec(k32, kw, kh);
	let y = sp::image::convolution::convolve2d_f32(&img, &ker, border_from(border));
	y.as_slice().iter().map(|&v| v as f64).collect()
}

#[wasm_bindgen]
pub fn img_convolve2d_u8(src: Vec<u8>, width: usize, height: usize, kernel: Vec<f64>, kw: usize, kh: usize, border: WasmBorder) -> Vec<u8> {
	let img = sp::image::core::Image::<u8>::from_vec(src, width, height);
	let k32: Vec<f32> = kernel.into_iter().map(|v| v as f32).collect();
	let ker = sp::image::convolution::Kernel::from_vec(k32, kw, kh);
	let y = sp::image::convolution::convolve2d_u8(&img, &ker, border_from(border));
	y.as_slice().to_vec()
}

#[wasm_bindgen]
pub fn img_dft2d(src: Vec<f64>, width: usize, height: usize) -> Vec<f64> {
	let s32: Vec<f32> = src.into_iter().map(|v| v as f32).collect();
	let img = sp::image::core::Image::<f32>::from_vec(s32, width, height);
	let (rr, ri) = sp::image::dft::dft2d(&img);
	let mut out = Vec::with_capacity(width*height*2);
	for i in 0..(width*height) {
		out.push(rr.as_slice()[i] as f64);
		out.push(ri.as_slice()[i] as f64);
	}
	out
}

#[wasm_bindgen]
pub fn img_idft2d(spec_interleaved: Vec<f64>, width: usize, height: usize) -> Vec<f64> {
	assert_eq!(spec_interleaved.len(), width*height*2);
	let mut rr = vec![0.0f32; width*height];
	let mut ri = vec![0.0f32; width*height];
	for i in 0..(width*height) {
		rr[i] = spec_interleaved[2*i] as f32;
		ri[i] = spec_interleaved[2*i+1] as f32;
	}
	let rimg = sp::image::core::Image::<f32>::from_vec(rr, width, height);
	let iimg = sp::image::core::Image::<f32>::from_vec(ri, width, height);
	let out = sp::image::dft::idft2d(&rimg, &iimg);
	out.as_slice().iter().map(|&v| v as f64).collect()
}

#[wasm_bindgen]
pub fn img_fftshift(spec_interleaved: Vec<f64>, width: usize, height: usize) -> Vec<f64> {
	assert_eq!(spec_interleaved.len(), width*height*2);
	let mut rr = vec![0.0f32; width*height];
	let mut ri = vec![0.0f32; width*height];
	for i in 0..(width*height) {
		rr[i] = spec_interleaved[2*i] as f32;
		ri[i] = spec_interleaved[2*i+1] as f32;
	}
	let mut rimg = sp::image::core::Image::<f32>::from_vec(rr, width, height);
	let mut iimg = sp::image::core::Image::<f32>::from_vec(ri, width, height);
	sp::image::dft::fftshift(&mut rimg, &mut iimg);
	let mut out = Vec::with_capacity(width*height*2);
	for i in 0..(width*height) {
		out.push(rimg.as_slice()[i] as f64);
		out.push(iimg.as_slice()[i] as f64);
	}
	out
}

#[wasm_bindgen]
pub fn img_magnitude(spec_interleaved: Vec<f64>, width: usize, height: usize) -> Vec<f64> {
	assert_eq!(spec_interleaved.len(), width*height*2);
	let mut rr = vec![0.0f32; width*height];
	let mut ri = vec![0.0f32; width*height];
	for i in 0..(width*height) {
		rr[i] = spec_interleaved[2*i] as f32;
		ri[i] = spec_interleaved[2*i+1] as f32;
	}
	let rimg = sp::image::core::Image::<f32>::from_vec(rr, width, height);
	let iimg = sp::image::core::Image::<f32>::from_vec(ri, width, height);
	let mag = sp::image::dft::magnitude(&rimg, &iimg);
	mag.as_slice().iter().map(|&v| v as f64).collect()
}

// ==== Image Filters (grayscale) ====
#[wasm_bindgen]
pub fn img_gaussian_blur_f32(src: Vec<f64>, width: usize, height: usize, sigma: f64, radius: usize, border: WasmBorder) -> Vec<f64> {
	let s32: Vec<f32> = src.into_iter().map(|v| v as f32).collect();
	let img = sp::image::core::Image::<f32>::from_vec(s32, width, height);
	let y = sp::image::filter::gaussian_blur_f32(&img, sigma as f32, radius, border_from(border));
	y.as_slice().iter().map(|&v| v as f64).collect()
}

#[wasm_bindgen]
pub fn img_gaussian_blur_u8(src: Vec<u8>, width: usize, height: usize, sigma: f64, radius: usize, border: WasmBorder) -> Vec<u8> {
	let img = sp::image::core::Image::<u8>::from_vec(src, width, height);
	let y = sp::image::filter::gaussian_blur_u8(&img, sigma as f32, radius, border_from(border));
	y.as_slice().to_vec()
}

#[wasm_bindgen]
pub fn img_box_filter_f32(src: Vec<f64>, width: usize, height: usize, radius: usize, border: WasmBorder) -> Vec<f64> {
	let s32: Vec<f32> = src.into_iter().map(|v| v as f32).collect();
	let img = sp::image::core::Image::<f32>::from_vec(s32, width, height);
	let y = sp::image::filter::box_filter_f32(&img, radius, border_from(border));
	y.as_slice().iter().map(|&v| v as f64).collect()
}

#[wasm_bindgen]
pub fn img_box_filter_u8(src: Vec<u8>, width: usize, height: usize, radius: usize, border: WasmBorder) -> Vec<u8> {
	let img = sp::image::core::Image::<u8>::from_vec(src, width, height);
	let y = sp::image::filter::box_filter_u8(&img, radius, border_from(border));
	y.as_slice().to_vec()
}

#[wasm_bindgen]
pub fn img_unsharp_mask_f32(src: Vec<f64>, width: usize, height: usize, sigma: f64, radius: usize, amount: f64, border: WasmBorder) -> Vec<f64> {
	let s32: Vec<f32> = src.into_iter().map(|v| v as f32).collect();
	let img = sp::image::core::Image::<f32>::from_vec(s32, width, height);
	let y = sp::image::filter::unsharp_mask_f32(&img, sigma as f32, radius, amount as f32, border_from(border));
	y.as_slice().iter().map(|&v| v as f64).collect()
}

#[wasm_bindgen]
pub fn img_unsharp_mask_u8(src: Vec<u8>, width: usize, height: usize, sigma: f64, radius: usize, amount: f64, border: WasmBorder) -> Vec<u8> {
	let img = sp::image::core::Image::<u8>::from_vec(src, width, height);
	let y = sp::image::filter::unsharp_mask_u8(&img, sigma as f32, radius, amount as f32, border_from(border));
	y.as_slice().to_vec()
}

#[wasm_bindgen]
pub fn img_sobel_magnitude_f32(src: Vec<f64>, width: usize, height: usize, border: WasmBorder) -> Vec<f64> {
	let s32: Vec<f32> = src.into_iter().map(|v| v as f32).collect();
	let img = sp::image::core::Image::<f32>::from_vec(s32, width, height);
	let y = sp::image::filter::sobel_magnitude_f32(&img, border_from(border));
	y.as_slice().iter().map(|&v| v as f64).collect()
}

#[wasm_bindgen]
pub fn img_sobel_magnitude_u8(src: Vec<u8>, width: usize, height: usize, border: WasmBorder) -> Vec<u8> {
	let img = sp::image::core::Image::<u8>::from_vec(src, width, height);
	let y = sp::image::filter::sobel_magnitude_u8(&img, border_from(border));
	y.as_slice().to_vec()
}

#[wasm_bindgen]
pub fn img_laplacian_f32(src: Vec<f64>, width: usize, height: usize, border: WasmBorder) -> Vec<f64> {
	let s32: Vec<f32> = src.into_iter().map(|v| v as f32).collect();
	let img = sp::image::core::Image::<f32>::from_vec(s32, width, height);
	let y = sp::image::filter::laplacian_f32(&img, border_from(border));
	y.as_slice().iter().map(|&v| v as f64).collect()
}

#[wasm_bindgen]
pub fn img_laplacian_u8(src: Vec<u8>, width: usize, height: usize, border: WasmBorder) -> Vec<u8> {
	let img = sp::image::core::Image::<u8>::from_vec(src, width, height);
	let y = sp::image::filter::laplacian_u8(&img, border_from(border));
	y.as_slice().to_vec()
}

#[wasm_bindgen]
pub fn img_median_filter_f32(src: Vec<f64>, width: usize, height: usize, radius: usize, border: WasmBorder) -> Vec<f64> {
	let s32: Vec<f32> = src.into_iter().map(|v| v as f32).collect();
	let img = sp::image::core::Image::<f32>::from_vec(s32, width, height);
	let y = sp::image::filter::median_filter_f32(&img, radius, border_from(border));
	y.as_slice().iter().map(|&v| v as f64).collect()
}

#[wasm_bindgen]
pub fn img_median_filter_u8(src: Vec<u8>, width: usize, height: usize, radius: usize, border: WasmBorder) -> Vec<u8> {
	let img = sp::image::core::Image::<u8>::from_vec(src, width, height);
	let y = sp::image::filter::median_filter_u8(&img, radius, border_from(border));
	y.as_slice().to_vec()
}

#[wasm_bindgen]
pub fn img_bilateral_filter_f32(src: Vec<f64>, width: usize, height: usize, radius: usize, sigma_s: f64, sigma_r: f64, border: WasmBorder) -> Vec<f64> {
	let s32: Vec<f32> = src.into_iter().map(|v| v as f32).collect();
	let img = sp::image::core::Image::<f32>::from_vec(s32, width, height);
	let y = sp::image::filter::bilateral_filter_f32(&img, radius, sigma_s as f32, sigma_r as f32, border_from(border));
	y.as_slice().iter().map(|&v| v as f64).collect()
}

#[wasm_bindgen]
pub fn img_bilateral_filter_u8(src: Vec<u8>, width: usize, height: usize, radius: usize, sigma_s: f64, sigma_r: f64, border: WasmBorder) -> Vec<u8> {
	let img = sp::image::core::Image::<u8>::from_vec(src, width, height);
	let y = sp::image::filter::bilateral_filter_u8(&img, radius, sigma_s as f32, sigma_r as f32, border_from(border));
	y.as_slice().to_vec()
}


// ==== Convenience: RGB/RGBA <-> Grayscale helpers (flat arrays) ====
#[wasm_bindgen]
pub fn rgb_u8_to_gray_f64(rgb: Vec<u8>, width: usize, height: usize) -> Vec<f64> {
	let n = width * height;
	assert_eq!(rgb.len(), n * 3, "rgb length must be width*height*3");
	let mut out = Vec::with_capacity(n);
	for i in 0..n {
		let r = rgb[3 * i] as f64;
		let g = rgb[3 * i + 1] as f64;
		let b = rgb[3 * i + 2] as f64;
		// ITU-R BT.601 luma
		out.push(0.299 * r + 0.587 * g + 0.114 * b);
	}
	out
}

#[wasm_bindgen]
pub fn rgba_u8_to_gray_f64(rgba: Vec<u8>, width: usize, height: usize) -> Vec<f64> {
	let n = width * height;
	assert_eq!(rgba.len(), n * 4, "rgba length must be width*height*4");
	let mut out = Vec::with_capacity(n);
	for i in 0..n {
		let r = rgba[4 * i] as f64;
		let g = rgba[4 * i + 1] as f64;
		let b = rgba[4 * i + 2] as f64;
		// Aは無視（非プリマルチ想定）
		out.push(0.299 * r + 0.587 * g + 0.114 * b);
	}
	out
}

#[wasm_bindgen]
pub fn gray_f64_to_rgba_u8(gray: Vec<f64>, width: usize, height: usize) -> Vec<u8> {
	let n = width * height;
	assert_eq!(gray.len(), n, "gray length must be width*height");
	let mut out = Vec::with_capacity(n * 4);
	for i in 0..n {
		let v = gray[i].round().clamp(0.0, 255.0) as u8;
		out.push(v);
		out.push(v);
		out.push(v);
		out.push(255);
	}
	out
}

#[wasm_bindgen]
pub fn u8_to_gray_f64(pixels: Vec<u8>) -> Vec<f64> {
	pixels.into_iter().map(|v| v as f64).collect()
}

#[wasm_bindgen]
pub fn gray_f64_to_u8_clamped(gray: Vec<f64>) -> Vec<u8> {
	gray
		.into_iter()
		.map(|v| v.round().clamp(0.0, 255.0) as u8)
		.collect()
}

// ==== f32 I/O variants (lower transfer overhead) ====
#[wasm_bindgen]
pub fn img_convolve2d_f32_io(src: Vec<f32>, width: usize, height: usize, kernel: Vec<f32>, kw: usize, kh: usize, border: WasmBorder) -> Vec<f32> {
	let img = sp::image::core::Image::<f32>::from_vec(src, width, height);
	let ker = sp::image::convolution::Kernel::from_vec(kernel, kw, kh);
	let y = sp::image::convolution::convolve2d_f32(&img, &ker, border_from(border));
	y.as_slice().to_vec()
}

#[wasm_bindgen]
pub fn img_gaussian_blur_f32_io(src: Vec<f32>, width: usize, height: usize, sigma: f64, radius: usize, border: WasmBorder) -> Vec<f32> {
	let img = sp::image::core::Image::<f32>::from_vec(src, width, height);
	let y = sp::image::filter::gaussian_blur_f32(&img, sigma as f32, radius, border_from(border));
	y.as_slice().to_vec()
}

