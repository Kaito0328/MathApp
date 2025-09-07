use wasm_bindgen::prelude::*;

use special_functions::{
    beta::{beta as sf_beta, log_beta as sf_log_beta, regularized_beta as sf_reg_beta},
    erf::{erf as sf_erf, erfc as sf_erfc, erf_inv as sf_erf_inv},
    gamma::{gamma as sf_gamma, log_gamma as sf_log_gamma, regularized_gamma as sf_reg_gamma},
};

#[wasm_bindgen(js_name = erf)]
pub fn wasm_erf(x: f64) -> f64 { sf_erf(x) }

#[wasm_bindgen(js_name = erfc)]
pub fn wasm_erfc(x: f64) -> f64 { sf_erfc(x) }

#[wasm_bindgen(js_name = erfInv)]
pub fn wasm_erf_inv(y: f64) -> f64 { sf_erf_inv(y) }

#[wasm_bindgen(js_name = gamma)]
pub fn wasm_gamma(x: f64) -> f64 { sf_gamma(x) }

#[wasm_bindgen(js_name = logGamma)]
pub fn wasm_log_gamma(x: f64) -> f64 { sf_log_gamma(x) }

#[wasm_bindgen(js_name = regularizedGamma)]
pub fn wasm_regularized_gamma(s: f64, x: f64) -> f64 { sf_reg_gamma(s, x) }

#[wasm_bindgen(js_name = beta)]
pub fn wasm_beta(a: f64, b: f64) -> f64 { sf_beta(a, b) }

#[wasm_bindgen(js_name = logBeta)]
pub fn wasm_log_beta(a: f64, b: f64) -> f64 { sf_log_beta(a, b) }

#[wasm_bindgen(js_name = regularizedBeta)]
pub fn wasm_regularized_beta(a: f64, b: f64, x: f64) -> f64 { sf_reg_beta(a, b, x) }
