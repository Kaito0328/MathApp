//! 有限体クレート用のWASMバインディング
//!
//! 方針: Matrix/Vector と同様に wasm_macros::wasm_class を最大限活用し、
//! - 直接委譲できる箇所は空ボディ宣言で自動生成
//! - 変換やエラー変換が必要な箇所だけ最小限の手動実装

#![allow(unused_macros)]  // マクロ内マクロのため警告を抑制

#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use finite_field::{
    gf256::{gf256_from_u8, gf256_modulus, GF256},
    gfext::GFExt,
    gfp::GFp,
};
#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use wasm_bindgen::prelude::*;
#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use std::sync::Arc;
#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use num_traits::{One, Zero};

// 内部型のエイリアス（属性マクロのパラメータで <...> を避けるため）
type InternalGF2 = finite_field::gfp::GFp<2>;
type InternalGF3 = finite_field::gfp::GFp<3>;
type InternalGF256 = finite_field::gf256::GF256; // = GFExt<GFp<2>>
type InternalGFExtGF2 = finite_field::gfext::GFExt<finite_field::gfp::GFp<2>>;

// ===============================
// 宣言マクロ（Matrix と同様の空ボディ委譲スタイル）
// ===============================

// GFp 共通の手動メソッド（最小限）
macro_rules! wasm_gfp_common_impl { ($RustName:ident) => {
    #[wasm_bindgen]
    impl $RustName {
        #[wasm_bindgen(getter)]
        pub fn value(&self) -> i64 { self.0.value() as i64 }
        pub fn div(&self, other: &Self) -> Result<Self, JsError> {
            self.0.checked_div(other.0).map(Self).map_err(|_| JsError::new("Division by zero"))
        }
        pub fn neg(&self) -> Self { Self(-self.0) }
        #[wasm_bindgen(getter = isZero)]
        pub fn is_zero(&self) -> bool { self.0.is_zero() }
        #[wasm_bindgen(getter = isOne)]
        pub fn is_one(&self) -> bool { self.0.value() == 1 }
    }
} }

// GF(2) - 属性ブロックは個別（確実な展開のため）
#[wasm_macros::wasm_class(
    internal = "InternalGF2",
    js_name = "WasmGF2",
    ops(Add, Sub, Mul),
    indexer = false,
    iterator = false
)]
impl WasmGF2 {
    #[constructor]
    pub fn new(value: i64) -> Self {}
    #[js_name = modulus]
    pub fn modulus() -> u16 {}
    // inv/zero/one は内部にあるので自動委譲
    pub fn inv(&self) -> Result<Self, JsValue> {}
    #[js_name = zero]
    pub fn zero() -> Self {}
    #[js_name = one]
    pub fn one() -> Self {}
}
wasm_gfp_common_impl!(WasmGF2);

// GF(3)
#[wasm_macros::wasm_class(
    internal = "InternalGF3",
    js_name = "WasmGF3",
    ops(Add, Sub, Mul),
    indexer = false,
    iterator = false
)]
impl WasmGF3 {
    #[constructor]
    pub fn new(value: i64) -> Self {}
    #[js_name = modulus]
    pub fn modulus() -> u16 {}
    pub fn inv(&self) -> Result<Self, JsValue> {}
    #[js_name = zero]
    pub fn zero() -> Self {}
    #[js_name = one]
    pub fn one() -> Self {}
}
wasm_gfp_common_impl!(WasmGF3);

// GF(256)
macro_rules! declare_wasm_gf256_impl { () => {
    #[wasm_macros::wasm_class(
        internal = "InternalGF256",
        js_name = "WasmGF256",
        ops(Add, Sub, Mul),
        indexer = false,
        iterator = false
    )]
    impl WasmGF256 {
        #[wasm_bindgen(getter = isZero)]
        pub fn is_zero(&self) -> bool {}
        #[wasm_bindgen(getter = isOne)]
        pub fn is_one(&self) -> bool {}
        #[wasm_bindgen(js_name = toU8)]
        pub fn to_u8(&self) -> u8 {}
        pub fn inv(&self) -> Result<Self, JsValue> {}
        #[js_name = zero]
        pub fn zero() -> Self {}
        #[js_name = one]
        pub fn one() -> Self {}
    }

    #[wasm_bindgen]
    impl WasmGF256 {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u8) -> Self { Self(gf256_from_u8(value)) }
        #[wasm_bindgen(getter)]
        pub fn value(&self) -> u8 { self.0.to_u8() }
        #[wasm_bindgen(getter)]
        pub fn coeffs(&self) -> Vec<u8> { self.0.coeffs().iter().map(|c| c.value() as u8).collect() }
        #[wasm_bindgen(js_name = fromCoeffs)]
        pub fn from_coeffs(coeffs: Vec<u8>) -> Self {
            let gf2_coeffs: Vec<GFp<2>> = coeffs.into_iter().map(|c| GFp::new(c as i64)).collect();
            Self(GF256::new(gf256_modulus(), gf2_coeffs))
        }
        #[wasm_bindgen(js_name = modulus)]
        pub fn modulus() -> Vec<u8> { gf256_modulus().iter().map(|c| c.value() as u8).collect() }
    }
} }

// GFExt(GF2)
macro_rules! declare_wasm_gfext_gf2_impl { () => {
    #[wasm_macros::wasm_class(
        internal = "InternalGFExtGF2",
        js_name = "WasmGFExtGF2",
        ops(Add, Sub, Mul),
        indexer = false,
        iterator = false
    )]
    impl WasmGFExtGF2 {
        // is_zero / is_one は委譲
        #[wasm_bindgen(getter = isZero)]
        pub fn is_zero(&self) -> bool {}
        #[wasm_bindgen(getter = isOne)]
        pub fn is_one(&self) -> bool {}
        pub fn inv(&self) -> Result<Self, JsValue> {}
        #[js_name = zero]
        pub fn zero() -> Self {}
        #[js_name = one]
        pub fn one() -> Self {}
    }

    #[wasm_bindgen]
    impl WasmGFExtGF2 {
        #[wasm_bindgen(constructor)]
        pub fn new(px_coeffs: Vec<u8>, coeffs: Vec<u8>) -> Self {
            let px = Arc::new(px_coeffs.into_iter().map(|c| GFp::new(c as i64)).collect());
            let coeffs = coeffs.into_iter().map(|c| GFp::new(c as i64)).collect();
            Self(GFExt::new(px, coeffs))
        }
        #[wasm_bindgen(js_name = fromBase)]
        pub fn from_base(px_coeffs: Vec<u8>, base_value: u8) -> Self {
            let px = Arc::new(px_coeffs.into_iter().map(|c| GFp::new(c as i64)).collect());
            let base = GFp::new(base_value as i64);
            Self(GFExt::from_base(px, base))
        }
        #[wasm_bindgen(getter)]
        pub fn coeffs(&self) -> Vec<u8> { self.0.coeffs().iter().map(|c| c.value() as u8).collect() }
        #[wasm_bindgen(getter)]
        pub fn px(&self) -> Vec<u8> { self.0.px().iter().map(|c| c.value() as u8).collect() }
    }
} }

// 共通: div と neg をまとめて提供（checked_div の Result を JsError に変換）
macro_rules! wasm_ff_common_div_neg_impl { ($RustName:ident) => {
    #[wasm_bindgen]
    impl $RustName {
        pub fn div(&self, other: &Self) -> Result<Self, JsError> {
            self.0.clone().checked_div(other.0.clone()).map(Self).map_err(|_| JsError::new("Division by zero"))
        }
        pub fn neg(&self) -> Self { Self(-self.0.clone()) }
    }
} }

// ===============================
// クラス定義（マクロ適用）
// ===============================

declare_wasm_gf256_impl!();
declare_wasm_gfext_gf2_impl!();
wasm_ff_common_div_neg_impl!(WasmGF256);
wasm_ff_common_div_neg_impl!(WasmGFExtGF2);