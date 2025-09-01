//! 有限体クレート用のWASMバインディング
//! 
//! # アーキテクチャ上の注意
//! external crateのtypeに対してwasm_classマクロを使用することはRustのorphan rulesにより制限される。
//! そのため、ここでは直接のwasm_bindgen実装を使用している。
//! これにより以下の利点がある：
//! - 外部型への制約を回避
//! - より柔軟な実装
//! - 型安全性の維持
//! - 実装の簡潔性

use finite_field::{
    gf256::{gf256_from_u8, gf256_modulus, GF256},
    gfext::GFExt,
    gfp::GFp,
};
use wasm_bindgen::prelude::*;
use std::sync::Arc;
use num_traits::{Zero, One};

// =============================================================================
// GF(2) - 2を法とする素体
// =============================================================================

#[wasm_bindgen]
pub struct WasmGF2(GFp<2>);

#[wasm_bindgen]
impl WasmGF2 {
    #[wasm_bindgen(constructor)]
    pub fn new(value: i64) -> Self {
        Self(GFp::new(value))
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> u16 {
        self.0.value()
    }

    pub fn add(&self, other: &WasmGF2) -> WasmGF2 {
        Self(self.0 + other.0)
    }

    pub fn sub(&self, other: &WasmGF2) -> WasmGF2 {
        Self(self.0 - other.0)
    }

    pub fn mul(&self, other: &WasmGF2) -> WasmGF2 {
        Self(self.0 * other.0)
    }

    pub fn div(&self, other: &WasmGF2) -> Result<WasmGF2, JsError> {
        match self.0.checked_div(other.0) {
            Ok(result) => Ok(Self(result)),
            Err(_) => Err(JsError::new("Division by zero")),
        }
    }

    pub fn neg(&self) -> WasmGF2 {
        Self(-self.0)
    }

    pub fn inv(&self) -> Result<WasmGF2, JsError> {
        match self.0.inv() {
            Ok(result) => Ok(Self(result)),
            Err(_) => Err(JsError::new("No multiplicative inverse")),
        }
    }

    #[wasm_bindgen(getter = isZero)]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[wasm_bindgen(getter = isOne)]
    pub fn is_one(&self) -> bool {
        self.0.is_one()
    }

    #[wasm_bindgen(js_name = zero)]
    pub fn zero() -> WasmGF2 {
        Self(GFp::zero())
    }

    #[wasm_bindgen(js_name = one)]
    pub fn one() -> WasmGF2 {
        Self(GFp::one())
    }

    #[wasm_bindgen(js_name = modulus)]
    pub fn modulus() -> u16 {
        2
    }
}

// =============================================================================
// GF(3) - 3を法とする素体
// =============================================================================

#[wasm_bindgen]
pub struct WasmGF3(GFp<3>);

#[wasm_bindgen]
impl WasmGF3 {
    #[wasm_bindgen(constructor)]
    pub fn new(value: i64) -> Self {
        Self(GFp::new(value))
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> u16 {
        self.0.value()
    }

    pub fn add(&self, other: &WasmGF3) -> WasmGF3 {
        Self(self.0 + other.0)
    }

    pub fn sub(&self, other: &WasmGF3) -> WasmGF3 {
        Self(self.0 - other.0)
    }

    pub fn mul(&self, other: &WasmGF3) -> WasmGF3 {
        Self(self.0 * other.0)
    }

    pub fn div(&self, other: &WasmGF3) -> Result<WasmGF3, JsError> {
        match self.0.checked_div(other.0) {
            Ok(result) => Ok(Self(result)),
            Err(_) => Err(JsError::new("Division by zero")),
        }
    }

    pub fn neg(&self) -> WasmGF3 {
        Self(-self.0)
    }

    pub fn inv(&self) -> Result<WasmGF3, JsError> {
        match self.0.inv() {
            Ok(result) => Ok(Self(result)),
            Err(_) => Err(JsError::new("No multiplicative inverse")),
        }
    }

    #[wasm_bindgen(getter = isZero)]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[wasm_bindgen(getter = isOne)]
    pub fn is_one(&self) -> bool {
        self.0.is_one()
    }

    #[wasm_bindgen(js_name = zero)]
    pub fn zero() -> WasmGF3 {
        Self(GFp::zero())
    }

    #[wasm_bindgen(js_name = one)]
    pub fn one() -> WasmGF3 {
        Self(GFp::one())
    }

    #[wasm_bindgen(js_name = modulus)]
    pub fn modulus() -> u16 {
        3
    }
}

// =============================================================================
// GF256 - GF(2^8)の特殊化
// =============================================================================

#[wasm_bindgen]
pub struct WasmGF256(GF256);

#[wasm_bindgen]
impl WasmGF256 {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u8) -> Self {
        Self(gf256_from_u8(value))
    }

    #[wasm_bindgen(js_name = fromCoeffs)]
    pub fn from_coeffs(coeffs: Vec<u8>) -> Self {
        let gf2_coeffs: Vec<GFp<2>> = coeffs.into_iter().map(|c| GFp::new(c as i64)).collect();
        Self(GF256::new(gf256_modulus(), gf2_coeffs))
    }

    #[wasm_bindgen(getter)]
    pub fn coeffs(&self) -> Vec<u8> {
        self.0.coeffs().iter().map(|c| c.value() as u8).collect()
    }

    #[wasm_bindgen(js_name = toU8)]
    pub fn to_u8(&self) -> u8 {
        self.0.to_u8()
    }

    pub fn add(&self, other: &WasmGF256) -> WasmGF256 {
        Self(self.0.clone() + other.0.clone())
    }

    pub fn sub(&self, other: &WasmGF256) -> WasmGF256 {
        Self(self.0.clone() - other.0.clone())
    }

    pub fn mul(&self, other: &WasmGF256) -> WasmGF256 {
        Self(self.0.clone() * other.0.clone())
    }

    pub fn div(&self, other: &WasmGF256) -> Result<WasmGF256, JsError> {
        match self.0.clone().checked_div(other.0.clone()) {
            Ok(result) => Ok(Self(result)),
            Err(_) => Err(JsError::new("Division by zero")),
        }
    }

    pub fn neg(&self) -> WasmGF256 {
        Self(-self.0.clone())
    }

    pub fn inv(&self) -> Result<WasmGF256, JsError> {
        match self.0.inv() {
            Ok(result) => Ok(Self(result)),
            Err(_) => Err(JsError::new("No multiplicative inverse")),
        }
    }

    #[wasm_bindgen(getter = isZero)]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[wasm_bindgen(getter = isOne)]
    pub fn is_one(&self) -> bool {
        self.0.is_one()
    }

    #[wasm_bindgen(js_name = zero)]
    pub fn zero() -> WasmGF256 {
        Self(GF256::from_base(gf256_modulus(), GFp::zero()))
    }

    #[wasm_bindgen(js_name = one)]
    pub fn one() -> WasmGF256 {
        Self(GF256::from_base(gf256_modulus(), GFp::one()))
    }

    #[wasm_bindgen(js_name = modulus)]
    pub fn modulus() -> Vec<u8> {
        let modulus_arc = gf256_modulus();
        modulus_arc.iter().map(|c| c.value() as u8).collect()
    }
}

// =============================================================================
// GFExt over GF(2) - GF(2)ベースの拡大体
// =============================================================================

#[wasm_bindgen]
pub struct WasmGFExtGF2(GFExt<GFp<2>>);

#[wasm_bindgen]
impl WasmGFExtGF2 {
    #[wasm_bindgen(constructor)]
    pub fn new(px_coeffs: Vec<u8>, coeffs: Vec<u8>) -> Self {
        let px = Arc::new(
            px_coeffs.into_iter().map(|c| GFp::new(c as i64)).collect()
        );
        let coeffs = coeffs.into_iter().map(|c| GFp::new(c as i64)).collect();
        Self(GFExt::new(px, coeffs))
    }

    #[wasm_bindgen(js_name = fromBase)]
    pub fn from_base(px_coeffs: Vec<u8>, base_value: u8) -> Self {
        let px = Arc::new(
            px_coeffs.into_iter().map(|c| GFp::new(c as i64)).collect()
        );
        let base = GFp::new(base_value as i64);
        Self(GFExt::from_base(px, base))
    }

    #[wasm_bindgen(getter)]
    pub fn coeffs(&self) -> Vec<u8> {
        self.0.coeffs().iter().map(|c| c.value() as u8).collect()
    }

    #[wasm_bindgen(getter)]
    pub fn px(&self) -> Vec<u8> {
        self.0.px().iter().map(|c| c.value() as u8).collect()
    }

    pub fn add(&self, other: &WasmGFExtGF2) -> WasmGFExtGF2 {
        Self(self.0.clone() + other.0.clone())
    }

    pub fn sub(&self, other: &WasmGFExtGF2) -> WasmGFExtGF2 {
        Self(self.0.clone() - other.0.clone())
    }

    pub fn mul(&self, other: &WasmGFExtGF2) -> WasmGFExtGF2 {
        Self(self.0.clone() * other.0.clone())
    }

    pub fn div(&self, other: &WasmGFExtGF2) -> Result<WasmGFExtGF2, JsError> {
        match self.0.clone().checked_div(other.0.clone()) {
            Ok(result) => Ok(Self(result)),
            Err(_) => Err(JsError::new("Division by zero")),
        }
    }

    pub fn neg(&self) -> WasmGFExtGF2 {
        Self(-self.0.clone())
    }

    pub fn inv(&self) -> Result<WasmGFExtGF2, JsError> {
        match self.0.inv() {
            Ok(result) => Ok(Self(result)),
            Err(_) => Err(JsError::new("No multiplicative inverse")),
        }
    }

    #[wasm_bindgen(getter = isZero)]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[wasm_bindgen(getter = isOne)]
    pub fn is_one(&self) -> bool {
        self.0.is_one()
    }
}