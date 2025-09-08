//! Polynomial crate の WASM バインディング

#![allow(unused_macros)]

use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;
use num_complex::Complex;
use poly::rational_function::{RationalFunction as InternalRF64, PartialFractionExpansion as InternalPFE, PoleTerm as InternalPoleTerm};

// 内部型エイリアス（<...> を避けるため）
type InternalPolyF64 = poly::Polynomial<f64>;
type InternalPolyGF2 = poly::Polynomial<finite_field::gfp::GFp<2>>;
type InternalPolyGF256 = poly::Polynomial<finite_field::gf256::GF256>;
type InternalPolyGFExtGF2 = poly::Polynomial<finite_field::gfext::GFExt<finite_field::gfp::GFp<2>>>;

// 共通メソッド宣言（空ボディ→wasm_class が委譲）
macro_rules! poly_common_methods { ($t:ty) => {
	#[constructor]
	pub fn new(coeffs: Vec<$t>) -> Self {}
	#[js_name = zero]
	pub fn zero() -> Self {}
	#[js_name = one]
	pub fn one() -> Self {}
	pub fn is_zero(&self) -> bool {}
	pub fn deg(&self) -> isize {}
	pub fn get(&self, i: usize) -> $t {}
	pub fn eval(&self, x: $t) -> $t {}
	pub fn monic(&self) -> Self {}
}; }

// f64 専用メソッド
macro_rules! poly_f64_methods { () => {
	pub fn differentiate(&self) -> Self {}
	pub fn integrate(&self) -> Self {}
	pub fn mul_simple(&self, other: &Self) -> Self {}
	pub fn mul_fft(&self, other: &Self) -> Self {}
	pub fn mul_auto(&self, other: &Self) -> Self {}
} }

// F64 用クラス
#[wasm_macros::wasm_class(
	internal = "InternalPolyF64",
	js_name = "PolynomialF64",
	ops(Add, Sub, Mul, Div),
	indexer = false,
	iterator = false
)]
impl PolynomialF64 {
	poly_common_methods!(f64);
}

// GF2/GF256/GFExtGF2 の基本は共通宣言で生成（追加の手動は最小限）

#[wasm_macros::wasm_class(
	internal = "InternalPolyGF2",
	js_name = "PolynomialGF2",
	ops(Add, Sub, Mul, Div),
	indexer = false,
	iterator = false
)]
impl PolynomialGF2 { poly_common_methods!(finite_field::gfp::GFp<2>); }

#[wasm_macros::wasm_class(
	internal = "InternalPolyGF256",
	js_name = "PolynomialGF256",
	ops(Add, Sub, Mul, Div),
	indexer = false,
	iterator = false
)]
impl PolynomialGF256 { poly_common_methods!(finite_field::gf256::GF256); }

#[wasm_macros::wasm_class(
	internal = "InternalPolyGFExtGF2",
	js_name = "PolynomialGFExtGF2",
	ops(Add, Sub, Mul, Div),
	indexer = false,
	iterator = false
)]
impl PolynomialGFExtGF2 { poly_common_methods!(finite_field::gfext::GFExt<finite_field::gfp::GFp<2>>); }

// 手動が必要な箇所: div_rem の戻り値を (q, r) を JsValue にパック
macro_rules! poly_divrem_json_impl { ($RustName:ident) => {
	#[wasm_bindgen]
	impl $RustName {
		#[wasm_bindgen(js_name = divRem)]
		pub fn div_rem_json(&self, other: &$RustName) -> Vec<$RustName> {
			let (q, r) = self.0.div_rem(&other.0);
			vec![$RustName(q), $RustName(r)]
		}
	}
} }

poly_divrem_json_impl!(PolynomialF64);
poly_divrem_json_impl!(PolynomialGF2);
poly_divrem_json_impl!(PolynomialGF256);
poly_divrem_json_impl!(PolynomialGFExtGF2);

// JS から new できるように f64 版のみ明示のコンストラクタを提供
#[wasm_bindgen]
impl PolynomialF64 {
	#[wasm_bindgen(constructor)]
	pub fn new(coeffs: Vec<f64>) -> Self { Self(InternalPolyF64::new(coeffs)) }

	pub fn deg(&self) -> i32 { self.0.deg() as i32 }
	pub fn get(&self, i: usize) -> f64 { self.0.get(i) }
	pub fn eval(&self, x: f64) -> f64 { self.0.eval(x) }

	// 明示的に公開（d.ts 生成の安定化）
	pub fn differentiate(&self) -> Self { Self(self.0.differentiate()) }
	pub fn integrate(&self) -> Self { Self(self.0.integrate()) }
	/// 係数ベクトル（低次→高次）を返す
	#[wasm_bindgen(js_name = coeffs)]
	pub fn coeffs_js(&self) -> Vec<f64> { self.0.coeffs.clone() }

	// ---- 追加公開API ----
	/// 実根列から多項式を生成（roots は実数）
	#[wasm_bindgen(js_name = fromRoots)]
	pub fn from_roots_real(roots: Vec<f64>) -> Self {
		Self(InternalPolyF64::from_roots(roots))
	}

	/// 多項式の最大公約多項式（静的メソッド）
	pub fn gcd(a: &PolynomialF64, b: &PolynomialF64) -> PolynomialF64 {
		PolynomialF64(InternalPolyF64::gcd(&a.0, &b.0))
	}

	/// 多項式の最小公倍多項式（静的メソッド）
	pub fn lcm(a: &PolynomialF64, b: &PolynomialF64) -> PolynomialF64 {
		PolynomialF64(InternalPolyF64::lcm(&a.0, &b.0))
	}

	/// 複素根を返す（re, im の交互並びのフラット配列: [re0, im0, re1, im1, ...]）
	#[wasm_bindgen(js_name = findRoots)]
	pub fn find_roots_js(&self) -> Vec<f64> {
		let roots = self.0.find_roots();
		let mut out = Vec::with_capacity(roots.len() * 2);
		for r in roots { out.push(r.re); out.push(r.im); }
		out
	}

	/// 渡された複素根列をクラスタリングして重複度付き根情報を返す
	/// inputs: roots_interleaved = [re0, im0, re1, im1, ...], tolerance
	#[wasm_bindgen(js_name = groupRoots)]
	pub fn group_roots_js(roots_interleaved: Vec<f64>, tolerance: f64) -> Vec<WasmRoot> {
		let mut roots: Vec<Complex<f64>> = Vec::with_capacity(roots_interleaved.len()/2);
		for xy in roots_interleaved.chunks_exact(2) { roots.push(Complex::new(xy[0], xy[1])); }
		let grouped = InternalPolyF64::group_roots(&roots, tolerance);
		grouped
			.into_iter()
			.map(|g| WasmRoot { re: g.value.re, im: g.value.im, multiplicity: g.multiplicity as u32 })
			.collect()
	}
}

/// JS へ返す重複度付き根
#[wasm_bindgen]
pub struct WasmRoot { re: f64, im: f64, multiplicity: u32 }

#[wasm_bindgen]
impl WasmRoot {
	#[wasm_bindgen(getter)]
	pub fn re(&self) -> f64 { self.re }
	#[wasm_bindgen(getter)]
	pub fn im(&self) -> f64 { self.im }
	#[wasm_bindgen(getter)]
	pub fn multiplicity(&self) -> u32 { self.multiplicity }
}

// 速度重視の f64 専用乗算を公開
#[wasm_bindgen]
impl PolynomialF64 {
	#[wasm_bindgen(js_name = mulSimple)]
	pub fn mul_simple_js(&self, other: &PolynomialF64) -> PolynomialF64 { PolynomialF64(self.0.mul_simple(&other.0)) }
	#[wasm_bindgen(js_name = mulFft)]
	pub fn mul_fft_js(&self, other: &PolynomialF64) -> PolynomialF64 { PolynomialF64(self.0.mul_fft(&other.0)) }
	#[wasm_bindgen(js_name = mulAuto)]
	pub fn mul_auto_js(&self, other: &PolynomialF64) -> PolynomialF64 { PolynomialF64(self.0.mul_auto(&other.0)) }
}

// ===================== RationalFunction<f64> =====================
#[wasm_bindgen(js_name = RationalFunctionF64)]
pub struct RationalFunctionF64(InternalRF64<f64>);

#[wasm_bindgen(js_class = "RationalFunctionF64")]
impl RationalFunctionF64 {
	/// constructor from numerator/denominator coeff arrays (low->high)
	#[wasm_bindgen(constructor)]
	pub fn new(numerator: Vec<f64>, denominator: Vec<f64>) -> RationalFunctionF64 {
		let num = InternalPolyF64::new(numerator);
		let den = InternalPolyF64::new(denominator);
		RationalFunctionF64(InternalRF64::new(num, den))
	}

	/// numerator coefficients (low->high)
	#[wasm_bindgen(js_name = numeratorCoeffs)]
	pub fn numerator_coeffs(&self) -> Vec<f64> { self.0.numerator.coeffs.clone() }
	/// denominator coefficients (low->high)
	#[wasm_bindgen(js_name = denominatorCoeffs)]
	pub fn denominator_coeffs(&self) -> Vec<f64> { self.0.denominator.coeffs.clone() }

	pub fn simplify(&mut self) { self.0.simplify(); }
	pub fn is_zero(&self) -> bool { self.0.is_zero() }
	pub fn inverse(&self) -> RationalFunctionF64 { RationalFunctionF64(self.0.inverse()) }

	/// evaluate at x (Some(y) or None if denominator(x)==0)
	pub fn eval(&self, x: f64) -> Option<f64> { self.0.eval(x) }

	/// derivative using quotient rule from crate
	pub fn differentiate(&self) -> RationalFunctionF64 { RationalFunctionF64(self.0.differentiate()) }

	/// basic ops
	pub fn add(&self, rhs: &RationalFunctionF64) -> RationalFunctionF64 { RationalFunctionF64(&self.0 + &rhs.0) }
	pub fn sub(&self, rhs: &RationalFunctionF64) -> RationalFunctionF64 { RationalFunctionF64(&self.0 - &rhs.0) }
	pub fn mul(&self, rhs: &RationalFunctionF64) -> RationalFunctionF64 { RationalFunctionF64(&self.0 * &rhs.0) }
	pub fn div(&self, rhs: &RationalFunctionF64) -> RationalFunctionF64 { RationalFunctionF64(&self.0 / &rhs.0) }

	/// multiply/divide by a polynomial
	#[wasm_bindgen(js_name = mulPoly)]
	pub fn mul_poly(&self, poly_coeffs: Vec<f64>) -> RationalFunctionF64 {
		let p = InternalPolyF64::new(poly_coeffs);
		RationalFunctionF64(self.0.mul_poly(&p))
	}
	#[wasm_bindgen(js_name = divPoly)]
	pub fn div_poly(&self, poly_coeffs: Vec<f64>) -> RationalFunctionF64 {
		let p = InternalPolyF64::new(poly_coeffs);
		RationalFunctionF64(self.0.div_poly(&p))
	}

	/// find poles (value + multiplicity)
	#[wasm_bindgen(js_name = findPoles)]
	pub fn find_poles_js(&self) -> Vec<WasmRoot> {
		self.0
			.find_poles()
			.into_iter()
			.map(|p| WasmRoot { re: p.value.re, im: p.value.im, multiplicity: p.multiplicity as u32 })
			.collect()
	}

	/// partial fraction expansion result
	#[wasm_bindgen(js_name = partialFractionExpansion)]
	pub fn partial_fraction_expansion_js(&self) -> JsValue {
		let pfe: InternalPFE = self.0.partial_fraction_expansion();
		// Serialize a friendly shape to JS: { polynomialPart: number[], terms: Array<{pole:[re,im], coeffs:number[]}> }
		use serde::Serialize;
	#[derive(Serialize)]
	struct JsPoleTerm { pole: [f64; 2], coefficients: Vec<[f64; 2]> }
	#[derive(Serialize)]
	struct JsPFE { #[serde(rename = "polynomialPart")] polynomial_part: Vec<f64>, terms: Vec<JsPoleTerm> }

		let terms: Vec<JsPoleTerm> = pfe
			.pole_terms
			.into_iter()
			.map(|InternalPoleTerm { pole, coefficients }| JsPoleTerm {
				pole: [pole.re, pole.im],
				coefficients: coefficients.into_iter().map(|c| [c.re, c.im]).collect(),
			})
			.collect();

	let obj = JsPFE { polynomial_part: pfe.polynomial_part.coeffs, terms };
		serde_wasm_bindgen::to_value(&obj).unwrap()
	}
}

// GF(2) 係数の多項式: Uint8Array(0/1) から構築し、u8 で値を返却
#[wasm_bindgen]
impl PolynomialGF2 {
	#[wasm_bindgen(constructor)]
	pub fn new(coeffs: Vec<u8>) -> Self {
		let v: Vec<finite_field::gfp::GFp<2>> = coeffs
			.into_iter()
			.map(|c| finite_field::gfp::GFp::<2>::new((c as i64) % 2))
			.collect();
		Self(InternalPolyGF2::new(v))
	}
	pub fn deg(&self) -> i32 { self.0.deg() as i32 }
	pub fn get(&self, i: usize) -> u8 { self.0.get(i).value() as u8 }
	pub fn eval(&self, x: u8) -> u8 {
		let gx = finite_field::gfp::GFp::<2>::new(x as i64);
		self.0.eval(gx).value() as u8
	}
	/// 係数ベクトル（低次→高次, 0/1）を返す
	#[wasm_bindgen(js_name = coeffs)]
	pub fn coeffs_js(&self) -> Vec<u8> {
		self.0.coeffs.iter().map(|c| c.value() as u8).collect()
	}
}

// GF(256) 係数の多項式: Uint8Array から構築し、u8 で値を返却
#[wasm_bindgen]
impl PolynomialGF256 {
	#[wasm_bindgen(constructor)]
	pub fn new(coeffs: Vec<u8>) -> Self {
		let v: Vec<finite_field::gf256::GF256> = coeffs
			.into_iter()
			.map(finite_field::gf256::gf256_from_u8)
			.collect();
		Self(InternalPolyGF256::new(v))
	}
	pub fn deg(&self) -> i32 { self.0.deg() as i32 }
	pub fn get(&self, i: usize) -> u8 { self.0.get(i).to_u8() }
	pub fn eval(&self, x: u8) -> u8 {
		let gx = finite_field::gf256::gf256_from_u8(x);
		self.0.eval(gx).to_u8()
	}
	/// 係数ベクトル（低次→高次, u8）を返す
	#[wasm_bindgen(js_name = coeffs)]
	pub fn coeffs_js(&self) -> Vec<u8> {
		self.0.coeffs.iter().map(|c| c.to_u8()).collect()
	}
}

// GFExt(GF2) 係数の多項式: px(Uint8Array) と coeffs(Array<Uint8Array>) で構築
#[wasm_bindgen]
impl PolynomialGFExtGF2 {
	#[wasm_bindgen(constructor)]
	pub fn new(px: Vec<u8>, coeffs: Vec<Uint8Array>) -> Self {
		use finite_field::gfp::GFp;
		use finite_field::gfext::GFExt;
		let px_gf2: std::sync::Arc<Vec<GFp<2>>> = std::sync::Arc::new(
			px.into_iter().map(|c| GFp::<2>::new(c as i64)).collect()
		);
		let coeffs_gfext: Vec<GFExt<GFp<2>>> = coeffs
			.into_iter()
			.map(|arr| {
				let mut v = vec![0u8; arr.length() as usize];
				arr.copy_to(&mut v[..]);
				let coeffs_gf2: Vec<GFp<2>> = v.into_iter().map(|c| GFp::<2>::new(c as i64)).collect();
				GFExt::new(px_gf2.clone(), coeffs_gf2)
			})
			.collect();
		Self(InternalPolyGFExtGF2::new(coeffs_gfext))
	}

	pub fn deg(&self) -> i32 { self.0.deg() as i32 }

	pub fn get(&self, i: usize) -> Vec<u8> {
		self.0.get(i).coeffs().iter().map(|c| c.value() as u8).collect()
	}

	pub fn eval(&self, x_coeffs: Vec<u8>) -> Vec<u8> {
		use finite_field::gfp::GFp;
		use finite_field::gfext::GFExt;
		let px = self
			.0
			.coeffs
			.first()
			.map(|e| e.px().clone())
			.unwrap_or_else(|| std::sync::Arc::new(vec![GFp::<2>::new(1)]));
		let x = GFExt::new(px, x_coeffs.into_iter().map(|c| GFp::<2>::new(c as i64)).collect());
		self.0.eval(x).coeffs().iter().map(|c| c.value() as u8).collect()
	}

	/// 係数ベクトル（各係数は GFExt(GF2) の係数列を Uint8Array として返す）
	#[wasm_bindgen(js_name = coeffs)]
	pub fn coeffs_js(&self) -> Vec<Uint8Array> {
		self
			.0
			.coeffs
			.iter()
			.map(|e| {
				let v: Vec<u8> = e.coeffs().iter().map(|c| c.value() as u8).collect();
				Uint8Array::from(v.as_slice())
			})
			.collect()
	}
}

