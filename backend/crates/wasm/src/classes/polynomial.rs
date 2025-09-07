//! Polynomial crate の WASM バインディング

#![allow(unused_macros)]

use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

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
	poly_f64_methods!();
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
	/// 係数ベクトル（低次→高次）を返す
	#[wasm_bindgen(js_name = coeffs)]
	pub fn coeffs_js(&self) -> Vec<f64> { self.0.coeffs.clone() }
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

