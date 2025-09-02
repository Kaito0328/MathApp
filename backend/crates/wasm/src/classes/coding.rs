//! coding クレートの WASM バインディング（最小セット）

use wasm_bindgen::prelude::*;

// 型エイリアス（<...> 回避）
type GF2 = finite_field::gfp::GFp<2>;
type GF256 = finite_field::gf256::GF256;
type MessageGF2 = coding::types::Message<GF2>;
type CodewordGF2 = coding::types::Codeword<GF2>;
type MessageGF256 = coding::types::Message<GF256>;
type CodewordGF256 = coding::types::Codeword<GF256>;

// Hamming(7,4)（GF(2) 固定）
#[wasm_bindgen(js_name = Hamming74)]
pub struct WasmHamming74(coding::Hamming74);

#[wasm_bindgen]
impl WasmHamming74 {
	#[wasm_bindgen(constructor)]
	pub fn new() -> WasmHamming74 { WasmHamming74(Default::default()) }

	// u: 0/1 array, returns 0/1 array length 7
	pub fn encode(&self, u: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let msg = MessageGF2::from(linalg::Vector::new(u.into_iter().map(|x| GF2::new(x as i64)).collect())) ;
		let cw = self.0.encode(&msg).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(cw.0.into_iter().map(|g| g.value() as u8).collect())
	}
}

// LinearCode<GF(2)> 固定（JS 利用が多い想定）
#[wasm_bindgen(js_name = LinearCode)]
pub struct WasmLinearCodeGF2(coding::LinearCode<GF2>);

#[wasm_bindgen(js_class = "LinearCode")]
impl WasmLinearCodeGF2 {
	// G は k×n 行列を行優先で 0/1 係数で受け取る
	#[wasm_bindgen(constructor)]
	pub fn new(k: usize, n: usize, g_data: Vec<u8>) -> Result<WasmLinearCodeGF2, JsError> {
		if g_data.len() != k * n { return Err(JsError::new("invalid generator length")); }
		let data: Vec<GF2> = g_data.into_iter().map(|x| GF2::new(x as i64)).collect();
		let g = linalg::Matrix::new(k, n, data).map_err(|e| JsError::new(&format!("{e}")))?;
		Ok(WasmLinearCodeGF2(coding::LinearCode::new(g)))
	}
	pub fn encode(&self, u: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let msg = MessageGF2::from(linalg::Vector::new(u.into_iter().map(|x| GF2::new(x as i64)).collect()));
		let cw = self.0.encode(&msg).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(cw.0.into_iter().map(|g| g.value() as u8).collect())
	}
}

// CyclicCode<GF(2)> 簡易（n と g 係数 0/1）
#[wasm_bindgen(js_name = CyclicCode)]
pub struct WasmCyclicCodeGF2(coding::CyclicCode<GF2>);

#[wasm_bindgen(js_class = "CyclicCode")]
impl WasmCyclicCodeGF2 {
	#[wasm_bindgen(constructor)]
	pub fn new(n: usize, g: Vec<u8>) -> WasmCyclicCodeGF2 {
		let g2: Vec<GF2> = g.into_iter().map(|x| GF2::new(x as i64)).collect();
		WasmCyclicCodeGF2(coding::CyclicCode::new(n, g2))
	}
	pub fn encode(&self, u: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let msg = MessageGF2::from(linalg::Vector::new(u.into_iter().map(|x| GF2::new(x as i64)).collect()));
		let cw = self.0.encode(&msg).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(cw.0.into_iter().map(|g| g.value() as u8).collect())
	}
	pub fn k(&self) -> usize { self.0.k() }
}

// ReedSolomon<GF256> 簡易（alphas は Uint8Array として渡す）
#[wasm_bindgen(js_name = ReedSolomon)]
pub struct WasmReedSolomonGF256(coding::ReedSolomon<GF256>);

#[wasm_bindgen(js_class = "ReedSolomon")]
impl WasmReedSolomonGF256 {
	#[wasm_bindgen(constructor)]
	pub fn new(k: usize, alphas: Vec<u8>) -> Result<WasmReedSolomonGF256, JsError> {
		let a: Vec<GF256> = alphas.into_iter().map(finite_field::gf256::gf256_from_u8).collect();
		let rs = coding::ReedSolomon::new(k, a).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(WasmReedSolomonGF256(rs))
	}
	pub fn encode(&self, f: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let msg = MessageGF256::from(linalg::Vector::new(f.into_iter().map(finite_field::gf256::gf256_from_u8).collect()));
		let cw = self.0.encode(&msg).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(cw.0.into_iter().map(|g| g.to_u8()).collect())
	}
	pub fn decode(&self, r: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let cw = CodewordGF256::from(linalg::Vector::new(r.into_iter().map(finite_field::gf256::gf256_from_u8).collect()));
		let out = self.0.decode(&cw).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(out.decoded.0.into_iter().map(|g| g.to_u8()).collect())
	}
	pub fn n(&self) -> usize { self.0.n }
	pub fn t(&self) -> usize { self.0.t }
}

// BCH Code over GF(2) with generator polynomial provided directly
#[wasm_bindgen(js_name = BCH)]
pub struct WasmBCHGF2(coding::BCHCode<GF2>);

#[wasm_bindgen(js_class = "BCH")]
impl WasmBCHGF2 {
	// Construct from n and generator polynomial coefficients (ascending order, 0/1)
	#[wasm_bindgen(constructor)]
	pub fn new(n: usize, g: Vec<u8>) -> WasmBCHGF2 {
		let g_vec: Vec<GF2> = g.into_iter().map(|x| GF2::new(x as i64)).collect();
		let poly = coding::Poly::new(g_vec);
		let deg = if poly.is_zero() { 0 } else { poly.deg() as usize };
		let t = (deg.max(1)) / 2; // heuristic; encode uses only n and g
		WasmBCHGF2(coding::BCHCode { n, t, g: poly })
	}
	pub fn encode(&self, u: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let msg = MessageGF2::from(linalg::Vector::new(u.into_iter().map(|x| GF2::new(x as i64)).collect()));
		let cw = self.0.encode(&msg).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(cw.0.into_iter().map(|g| g.value() as u8).collect())
	}
	pub fn k(&self) -> usize { self.0.k() }
	pub fn n(&self) -> usize { self.0.n }
	pub fn t(&self) -> usize { self.0.t }
}

// ---- code_utils helpers (GF2 specific) ----

#[wasm_bindgen(js_name = hammingDistanceGF2)]
pub fn hamming_distance_gf2(a: Vec<u8>, b: Vec<u8>) -> usize {
	let va = linalg::Vector::new(a.into_iter().map(|x| GF2::new(x as i64)).collect());
	let vb = linalg::Vector::new(b.into_iter().map(|x| GF2::new(x as i64)).collect());
	coding::code_utils::hamming_distance(&va, &vb)
}

// codebook_flat は長さ m*n の 0/1 配列、n は各符号語長
#[wasm_bindgen(js_name = weightDistributionGF2)]
pub fn weight_distribution_gf2(codebook_flat: Vec<u8>, n: usize) -> Result<Vec<usize>, JsError> {
	if n == 0 { return Err(JsError::new("n must be > 0")); }
	if codebook_flat.len() % n != 0 { return Err(JsError::new("codebook_flat length must be multiple of n")); }
	let m = codebook_flat.len() / n;
	let mut cws: Vec<CodewordGF2> = Vec::with_capacity(m);
	for i in 0..m {
		let start = i * n;
		let v = codebook_flat[start..start+n].iter().map(|&x| GF2::new(x as i64)).collect();
		cws.push(CodewordGF2::from(linalg::Vector::new(v)));
	}
	Ok(coding::code_utils::weight_distribution(&cws))
}

