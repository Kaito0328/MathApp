//! fft-core クレートの WASM バインディング

use num_complex::Complex;
use wasm_bindgen::prelude::*;

// JS との受け渡しは Float64Array を [re0, im0, re1, im1, ...] とする

fn vec_to_complex(mut flat: Vec<f64>) -> Vec<Complex<f64>> {
	if flat.len() % 2 != 0 { flat.pop(); }
	let mut out = Vec::with_capacity(flat.len() / 2);
	let mut i = 0;
	while i + 1 < flat.len() {
		out.push(Complex::new(flat[i], flat[i + 1]));
		i += 2;
	}
	out
}

fn complex_to_vec(x: Vec<Complex<f64>>) -> Vec<f64> {
	let mut out = Vec::with_capacity(x.len() * 2);
	for c in x {
		out.push(c.re);
		out.push(c.im);
	}
	out
}

#[wasm_bindgen(js_name = dftComplexF64)]
pub fn dft_complex_f64(x_flat: Vec<f64>) -> Vec<f64> {
	let x = vec_to_complex(x_flat);
	let y = fft_core::dft(&x);
	complex_to_vec(y)
}

#[wasm_bindgen(js_name = iftComplexF64)]
pub fn ift_complex_f64(x_flat: Vec<f64>) -> Vec<f64> {
	let x = vec_to_complex(x_flat);
	let y = fft_core::ift(&x);
	complex_to_vec(y)
}
