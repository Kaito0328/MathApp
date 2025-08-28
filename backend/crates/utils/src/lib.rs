#[macro_use]
pub mod macros;

use num_complex::Complex;

#[derive(Debug, Clone, PartialEq)]
pub enum ConvertError {
	ImaginaryComponentTooLarge { index: usize, value: f64, tol: f64 },
}

pub type Result<T> = std::result::Result<T, ConvertError>;

/// Vec<Complex<f64>> -> Vec<f64> への安全な変換。
/// 各要素の虚部の絶対値が tol 以下であれば実部のみを採用し、超える場合は Err を返す。
pub fn complex_vec_to_real(v: &[Complex<f64>], tol: f64) -> Result<Vec<f64>> {
	let mut out = Vec::with_capacity(v.len());
	for (i, c) in v.iter().enumerate() {
		if c.im.abs() > tol {
			return Err(ConvertError::ImaginaryComponentTooLarge { index: i, value: c.im, tol });
		}
		out.push(c.re);
	}
	Ok(out)
}
