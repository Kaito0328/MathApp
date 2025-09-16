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

#[wasm_bindgen(js_class = "Hamming74")]
impl WasmHamming74 {
	#[wasm_bindgen(constructor)]
	pub fn new() -> WasmHamming74 { WasmHamming74(Default::default()) }

	// u: 0/1 array, returns 0/1 array length 7
	pub fn encode(&self, u: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let msg = MessageGF2::from(linalg::Vector::new(u.into_iter().map(|x| GF2::new(x as i64)).collect())) ;
		let cw = self.0.encode(&msg).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(cw.0.into_iter().map(|g| g.value() as u8).collect())
	}

	/// H 行列（(n-k)×n）を行優先で返す
	#[wasm_bindgen(js_name = parityCheck)]
	pub fn parity_check(&self) -> Result<Vec<u8>, JsError> {
		let h = coding::code_utils::parity_check_from_generator(&self.0.g)
			.map_err(|e| JsError::new(&e.to_string()))?;
		Ok(h.0.data.into_iter().map(|x| x.value() as u8).collect())
	}

	/// 有界距離復号（t=1）で訂正したコード語を返す
	pub fn decode(&self, r: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let h = coding::code_utils::parity_check_from_generator(&self.0.g)
			.map_err(|e| JsError::new(&e.to_string()))?;
		let v = linalg::Vector::new(r.into_iter().map(|x| GF2::new(x as i64)).collect());
		let cw = CodewordGF2::from(v);
		let corrected = coding::code_utils::bounded_distance_decode_gf2(&h, &cw, 1)
			.map_err(|e| JsError::new(&e.to_string()))?;
		Ok(corrected.0.into_iter().map(|g| g.value() as u8).collect())
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

	/// H行列（(n-k)×n）を返す（標準形への変換を内部で行う）
	#[wasm_bindgen(js_name = parityCheck)]
	pub fn parity_check(&self) -> Result<Vec<u8>, JsError> {
		let h = coding::code_utils::parity_check_from_generator(&self.0.g)
			.map_err(|e| JsError::new(&e.to_string()))?;
		Ok(h.0.data.into_iter().map(|x| x.value() as u8).collect())
	}

	/// シンドローム復号（内部で H を構成）
	#[wasm_bindgen(js_name = decodeSyndrome)]
	pub fn decode_syndrome(&self, r: Vec<u8>, t: usize) -> Result<Vec<u8>, JsError> {
		let h = coding::code_utils::parity_check_from_generator(&self.0.g)
			.map_err(|e| JsError::new(&e.to_string()))?;
		let v = linalg::Vector::new(r.into_iter().map(|x| GF2::new(x as i64)).collect());
		let cw = CodewordGF2::from(v);
		let corrected = coding::code_utils::syndrome_decode_gf2(&h, &cw, t)
			.map_err(|e| JsError::new(&e.to_string()))?;
		Ok(corrected.0.into_iter().map(|g| g.value() as u8).collect())
	}

	/// 与えられた H を使って復号（(n-k)×n 行列を行優先、t は有界距離）
	#[wasm_bindgen(js_name = decodeWithH)]
	pub fn decode_with_h(&self, h_flat: Vec<u8>, rows: usize, r: Vec<u8>, t: usize) -> Result<Vec<u8>, JsError> {
		let n = self.0.n;
		if h_flat.len() != rows * n { return Err(JsError::new("invalid H length")); }
		let data: Vec<GF2> = h_flat.into_iter().map(|x| GF2::new(x as i64)).collect();
		let hmat = linalg::Matrix::new(rows, n, data).map_err(|e| JsError::new(&format!("{e}")))?;
		let h = coding::types::ParityCheckMatrix(hmat);
		let v = linalg::Vector::new(r.into_iter().map(|x| GF2::new(x as i64)).collect());
		let cw = CodewordGF2::from(v);
		let corrected = coding::code_utils::syndrome_decode_gf2(&h, &cw, t)
			.map_err(|e| JsError::new(&e.to_string()))?;
		Ok(corrected.0.into_iter().map(|g| g.value() as u8).collect())
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

	/// 与えられた H を使って復号（(n-k)×n 行列を行優先、t は有界距離）
	#[wasm_bindgen(js_name = decodeWithH)]
	pub fn decode_with_h(&self, h_flat: Vec<u8>, rows: usize, r: Vec<u8>, t: usize) -> Result<Vec<u8>, JsError> {
		let n = self.0.n;
		if h_flat.len() != rows * n { return Err(JsError::new("invalid H length")); }
		let data: Vec<GF2> = h_flat.into_iter().map(|x| GF2::new(x as i64)).collect();
		let hmat = linalg::Matrix::new(rows, n, data).map_err(|e| JsError::new(&format!("{e}")))?;
		let h = coding::types::ParityCheckMatrix(hmat);
		let v = linalg::Vector::new(r.into_iter().map(|x| GF2::new(x as i64)).collect());
		let cw = CodewordGF2::from(v);
		let corrected = coding::code_utils::syndrome_decode_gf2(&h, &cw, t)
			.map_err(|e| JsError::new(&e.to_string()))?;
		Ok(corrected.0.into_iter().map(|g| g.value() as u8).collect())
	}

	/// 生成多項式から内部でHを構成し、GF(2)シンドロームLUTで復号（t=(n-k)/2）
	#[wasm_bindgen(js_name = decodeLUT)]
	pub fn decode_lut(&self, r: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let v = linalg::Vector::new(r.into_iter().map(|x| GF2::new(x as i64)).collect());
		let cw = CodewordGF2::from(v);
		let corrected = self.0.decode_lut(&cw).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(corrected.0.into_iter().map(|g| g.value() as u8).collect())
	}
}

// ReedSolomon over GF(2^8)
#[wasm_bindgen(js_name = ReedSolomon)]
pub struct WasmReedSolomonGF256(coding::ReedSolomon);

#[wasm_bindgen(js_class = "ReedSolomon")]
impl WasmReedSolomonGF256 {
	#[wasm_bindgen(constructor)]
	pub fn new(k: usize, n: usize) -> Result<WasmReedSolomonGF256, JsError> {
		// Default: GF(2^8) AES modulus and primitive evaluation points
		let px = finite_field::gf256::gf256_modulus();
		let field = finite_field::field2m::FiniteField2m::new(px.as_ref());
		let rs = coding::ReedSolomon::new_with_field(k, n, &field)
			.map_err(|e| JsError::new(&e.to_string()))?;
		Ok(WasmReedSolomonGF256(rs))
	}
	/// Construct with a custom primitive polynomial over GF(2).
	/// `px` are GF(2) coefficients in low->high order, length m+1, with px[0]=px[m]=1.
	#[wasm_bindgen(js_name = newWithPrimitive)]
	pub fn new_with_primitive(px: Vec<u8>, k: usize, n: usize) -> Result<WasmReedSolomonGF256, JsError> {
		if px.len() < 3 { return Err(JsError::new("px length must be >= 3 (m >= 2)")); }
		let m = px.len() - 1;
		if m < 2 || m > 15 { return Err(JsError::new("supported m is 2..=15")); }
		if (px[0] & 1) == 0 || (px[m] & 1) == 0 { return Err(JsError::new("px[0] and px[m] must be 1")); }
		let coeffs: Vec<finite_field::gfp::GFp<2>> = px.into_iter().map(|b| finite_field::gfp::GFp::<2>((b & 1) as u16)).collect();
		let field = finite_field::field2m::FiniteField2m::new(&coeffs);
		let rs = coding::ReedSolomon::new_with_field(k, n, &field).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(WasmReedSolomonGF256(rs))
	}
	/// Auto-select minimal m with 2^m-1 >= n and build RS over GF(2^m)
	#[wasm_bindgen(js_name = newAuto)]
	pub fn new_auto(k: usize, n: usize) -> Result<WasmReedSolomonGF256, JsError> {
		let rs = coding::ReedSolomon::new_auto(k, n).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(WasmReedSolomonGF256(rs))
	}
	pub fn encode(&self, f: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let px = self.0.field.px_arc();
		let msg = MessageGF256::from(linalg::Vector::new(
			f.into_iter().map(|x| finite_field::gfext::GFExt::<finite_field::gfp::GFp<2>>::from_u8(px.clone(), x)).collect()
		));
		let cw = self.0.encode(&msg).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(cw.0.into_iter().map(|g| g.to_u8()).collect())
	}
	pub fn decode(&self, r: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let px = self.0.field.px_arc();
		let cw = CodewordGF256::from(linalg::Vector::new(
			r.into_iter().map(|x| finite_field::gfext::GFExt::<finite_field::gfp::GFp<2>>::from_u8(px.clone(), x)).collect()
		));
		let out = self.0.decode(&cw).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(out.decoded.0.into_iter().map(|g| g.to_u8()).collect())
	}
	pub fn n(&self) -> usize { self.0.n }
	pub fn t(&self) -> usize { self.0.t }
	pub fn k(&self) -> usize { self.0.k }

	/// Berlekamp–Massey ベースの代替復号器
	#[wasm_bindgen(js_name = decodeBM)]
	pub fn decode_bm(&self, r: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let px = self.0.field.px_arc();
		let cw = CodewordGF256::from(linalg::Vector::new(
			r.into_iter().map(|x| finite_field::gfext::GFExt::<finite_field::gfp::GFp<2>>::from_u8(px.clone(), x)).collect()
		));
		let out = self.0.decode_bm(&cw).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(out.decoded.0.into_iter().map(|g| g.to_u8()).collect())
	}
}

// BCH Code over GF(2) with generator polynomial provided directly
#[wasm_bindgen(js_name = BCH)]
pub struct WasmBCHGF2(coding::BCHCode);

#[wasm_bindgen(js_class = "BCH")]
impl WasmBCHGF2 {
	// Construct from n and generator polynomial coefficients (ascending order, 0/1)
	#[wasm_bindgen(constructor)]
	pub fn new(n: usize, g: Vec<u8>) -> WasmBCHGF2 {
		let g_vec: Vec<GF2> = g.into_iter().map(|x| GF2::new(x as i64)).collect();
		let poly = coding::Poly::new(g_vec);
		let deg = if poly.is_zero() { 0 } else { poly.deg() as usize };
		let t = (deg.max(1)) / 2; // heuristic upper-bound; exact t depends on design distance

		// Choose a GF(2^m) field consistent with n when possible (n must be 2^m-1 for BCH)
		// Otherwise, fall back to AES m=8 field for encode/LUT decode use-cases.
		let n_plus_1 = n + 1;
		let is_pow2 = n_plus_1.is_power_of_two();
		let field = if is_pow2 {
			let m = n_plus_1.trailing_zeros() as usize; // since n+1 = 2^m
			finite_field::field2m::FiniteField2m::new_auto(m)
		} else {
			let px = finite_field::gf256::gf256_modulus();
			finite_field::field2m::FiniteField2m::new(px.as_ref())
		};

		// Construct BCHCode directly to avoid internal asserts; override g, n, k, t as provided/derived
		let k = n.saturating_sub(poly.coeffs.len().saturating_sub(1));
		WasmBCHGF2(coding::BCHCode { field, n, k, t, g: poly })
	}

	/// Construct BCH automatically from m (GF(2^m)) and designed t (n = 2^m - 1). Narrow-sense (b=1).
	#[wasm_bindgen(js_name = newAuto)]
	pub fn new_auto(m: usize, t: usize) -> WasmBCHGF2 { WasmBCHGF2(coding::BCHCode::new_auto(m, t)) }
	pub fn encode(&self, u: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let msg = MessageGF2::from(linalg::Vector::new(u.into_iter().map(|x| GF2::new(x as i64)).collect()));
		let cw = self.0.encode(&msg).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(cw.0.into_iter().map(|g| g.value() as u8).collect())
	}
	pub fn k(&self) -> usize { self.0.k() }
	pub fn n(&self) -> usize { self.0.n }
	pub fn t(&self) -> usize { self.0.t }

	/// 与えられた H を使って復号（(n-k)×n 行列を行優先、t は有界距離）
	#[wasm_bindgen(js_name = decodeWithH)]
	pub fn decode_with_h(&self, h_flat: Vec<u8>, rows: usize, r: Vec<u8>, t: usize) -> Result<Vec<u8>, JsError> {
		let n = self.0.n;
		if h_flat.len() != rows * n { return Err(JsError::new("invalid H length")); }
		let data: Vec<GF2> = h_flat.into_iter().map(|x| GF2::new(x as i64)).collect();
		let hmat = linalg::Matrix::new(rows, n, data).map_err(|e| JsError::new(&format!("{e}")))?;
		let h = coding::types::ParityCheckMatrix(hmat);
		let v = linalg::Vector::new(r.into_iter().map(|x| GF2::new(x as i64)).collect());
		let cw = CodewordGF2::from(v);
		let corrected = coding::code_utils::syndrome_decode_gf2(&h, &cw, t)
			.map_err(|e| JsError::new(&e.to_string()))?;
		Ok(corrected.0.into_iter().map(|g| g.value() as u8).collect())
	}

	/// 生成多項式から内部で標準的な巡回G/Hを構成し、GF(2)シンドロームLUTで復号
	#[wasm_bindgen(js_name = decodeLUT)]
	pub fn decode_lut(&self, r: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let v = linalg::Vector::new(r.into_iter().map(|x| GF2::new(x as i64)).collect());
		let cw = CodewordGF2::from(v);
		let corrected = self.0.decode_lut(&cw).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(corrected.0.into_iter().map(|g| g.value() as u8).collect())
	}

	/// BM + Chien の復号（狭義BCH, b=1）
	#[wasm_bindgen(js_name = decodeBM)]
	pub fn decode_bm(&self, r: Vec<u8>) -> Result<Vec<u8>, JsError> {
		let v = linalg::Vector::new(r.into_iter().map(|x| GF2::new(x as i64)).collect());
		let cw = CodewordGF2::from(v);
		let corrected = self.0.decode_bm(&cw, 1).map_err(|e| JsError::new(&e.to_string()))?;
		Ok(corrected.0.into_iter().map(|g| g.value() as u8).collect())
	}
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

// ---- Additional utilities (GF2) ----

#[wasm_bindgen(js_name = parityCheckFromGeneratorGF2)]
pub fn parity_check_from_generator_gf2(k: usize, n: usize, g_flat: Vec<u8>) -> Result<Vec<u8>, JsError> {
	if g_flat.len() != k * n { return Err(JsError::new("invalid generator length")); }
	let data: Vec<GF2> = g_flat.into_iter().map(|x| GF2::new(x as i64)).collect();
	let g = linalg::Matrix::new(k, n, data).map_err(|e| JsError::new(&format!("{e}")))?;
	let gm = coding::types::GeneratorMatrix(g);
	let h = coding::code_utils::parity_check_from_generator(&gm).map_err(|e| JsError::new(&e.to_string()))?;
	Ok(h.0.data.into_iter().map(|x| x.value() as u8).collect())
}

#[wasm_bindgen(js_name = computeSyndromeGF2)]
pub fn compute_syndrome_gf2_js(h_flat: Vec<u8>, rows: usize, n: usize, r: Vec<u8>) -> Result<Vec<u8>, JsError> {
	if h_flat.len() != rows * n { return Err(JsError::new("invalid H length")); }
	if r.len() != n { return Err(JsError::new("invalid codeword length")); }
	let data: Vec<GF2> = h_flat.into_iter().map(|x| GF2::new(x as i64)).collect();
	let hmat = linalg::Matrix::new(rows, n, data).map_err(|e| JsError::new(&format!("{e}")))?;
	let h = coding::types::ParityCheckMatrix(hmat);
	let rv = CodewordGF2::from(linalg::Vector::new(r.into_iter().map(|x| GF2::new(x as i64)).collect()));
	let syn = coding::code_utils::compute_syndrome_gf2(&h, &rv);
	Ok(syn.0.into_iter().map(|x| x.value() as u8).collect())
}

#[wasm_bindgen(js_name = syndromeDecodeGF2)]
pub fn syndrome_decode_gf2_js(h_flat: Vec<u8>, rows: usize, n: usize, r: Vec<u8>, t: usize) -> Result<Vec<u8>, JsError> {
	if h_flat.len() != rows * n { return Err(JsError::new("invalid H length")); }
	if r.len() != n { return Err(JsError::new("invalid codeword length")); }
	let data: Vec<GF2> = h_flat.into_iter().map(|x| GF2::new(x as i64)).collect();
	let hmat = linalg::Matrix::new(rows, n, data).map_err(|e| JsError::new(&format!("{e}")))?;
	let h = coding::types::ParityCheckMatrix(hmat);
	let rv = CodewordGF2::from(linalg::Vector::new(r.into_iter().map(|x| GF2::new(x as i64)).collect()));
	let corrected = coding::code_utils::syndrome_decode_gf2(&h, &rv, t).map_err(|e| JsError::new(&e.to_string()))?;
	Ok(corrected.0.into_iter().map(|x| x.value() as u8).collect())
}

#[wasm_bindgen(js_name = boundedDistanceDecodeGF2)]
pub fn bounded_distance_decode_gf2_js(h_flat: Vec<u8>, rows: usize, n: usize, r: Vec<u8>, t: usize) -> Result<Vec<u8>, JsError> {
	if h_flat.len() != rows * n { return Err(JsError::new("invalid H length")); }
	if r.len() != n { return Err(JsError::new("invalid codeword length")); }
	let data: Vec<GF2> = h_flat.into_iter().map(|x| GF2::new(x as i64)).collect();
	let hmat = linalg::Matrix::new(rows, n, data).map_err(|e| JsError::new(&format!("{e}")))?;
	let h = coding::types::ParityCheckMatrix(hmat);
	let rv = CodewordGF2::from(linalg::Vector::new(r.into_iter().map(|x| GF2::new(x as i64)).collect()));
	let corrected = coding::code_utils::bounded_distance_decode_gf2(&h, &rv, t).map_err(|e| JsError::new(&e.to_string()))?;
	Ok(corrected.0.into_iter().map(|x| x.value() as u8).collect())
}

#[wasm_bindgen(js_name = hammingDMinGF2)]
pub fn hamming_d_min_gf2(codebook_flat: Vec<u8>, n: usize) -> Result<usize, JsError> {
	if n == 0 { return Err(JsError::new("n must be > 0")); }
	if codebook_flat.len() % n != 0 { return Err(JsError::new("codebook_flat length must be multiple of n")); }
	let m = codebook_flat.len() / n;
	let mut cws: Vec<CodewordGF2> = Vec::with_capacity(m);
	for i in 0..m {
		let start = i * n;
		let v = codebook_flat[start..start+n].iter().map(|&x| GF2::new(x as i64)).collect();
		cws.push(CodewordGF2::from(linalg::Vector::new(v)));
	}
	Ok(coding::code_utils::linear_hamming_d_min(&cws))
}

#[wasm_bindgen(js_name = codingRateFromGeneratorGF2)]
pub fn coding_rate_from_generator_gf2(k: usize, n: usize, g_flat: Vec<u8>) -> Result<f64, JsError> {
	if g_flat.len() != k * n { return Err(JsError::new("invalid generator length")); }
	let data: Vec<GF2> = g_flat.into_iter().map(|x| GF2::new(x as i64)).collect();
	let g = linalg::Matrix::new(k, n, data).map_err(|e| JsError::new(&format!("{e}")))?;
	let gm = coding::types::GeneratorMatrix(g);
	Ok(coding::code_utils::coding_rate_from_generator(&gm))
}

