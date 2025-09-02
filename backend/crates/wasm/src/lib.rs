use wasm_bindgen::prelude::*;

pub mod classes;

// Re-export finite field classes
pub use classes::finite_field::{WasmGF2, WasmGF3, WasmGF256, WasmGFExtGF2};

#[wasm_bindgen(js_name = GF2)]
pub struct JsGF2(classes::finite_field::WasmGF2);
#[wasm_bindgen(js_class = "GF2")]
impl JsGF2 {
    #[wasm_bindgen(constructor)]
    pub fn new(value: i64) -> JsGF2 { JsGF2(classes::finite_field::WasmGF2::new(value)) }
    pub fn modulus() -> u16 { classes::finite_field::WasmGF2::modulus() }
    pub fn inv(&self) -> Result<JsGF2, JsValue> { self.0.inv().map(JsGF2) }
    pub fn zero() -> JsGF2 { JsGF2(classes::finite_field::WasmGF2::zero()) }
    pub fn one() -> JsGF2 { JsGF2(classes::finite_field::WasmGF2::one()) }
    pub fn add(&self, rhs: &JsGF2) -> JsGF2 { JsGF2(self.0.add(&rhs.0)) }
    pub fn sub(&self, rhs: &JsGF2) -> JsGF2 { JsGF2(self.0.sub(&rhs.0)) }
    pub fn mul(&self, rhs: &JsGF2) -> JsGF2 { JsGF2(self.0.mul(&rhs.0)) }
    pub fn div(&self, other: &JsGF2) -> Result<JsGF2, JsError> { self.0.div(&other.0).map(JsGF2) }
    pub fn neg(&self) -> JsGF2 { JsGF2(self.0.neg()) }
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i64 { self.0.value() }
    #[wasm_bindgen(getter = isZero)]
    pub fn is_zero(&self) -> bool { self.0.is_zero() }
    #[wasm_bindgen(getter = isOne)]
    pub fn is_one(&self) -> bool { self.0.is_one() }
}

#[wasm_bindgen(js_name = GF3)]
pub struct JsGF3(classes::finite_field::WasmGF3);
#[wasm_bindgen(js_class = "GF3")]
impl JsGF3 {
    #[wasm_bindgen(constructor)]
    pub fn new(value: i64) -> JsGF3 { JsGF3(classes::finite_field::WasmGF3::new(value)) }
    pub fn modulus() -> u16 { classes::finite_field::WasmGF3::modulus() }
    pub fn inv(&self) -> Result<JsGF3, JsValue> { self.0.inv().map(JsGF3) }
    pub fn zero() -> JsGF3 { JsGF3(classes::finite_field::WasmGF3::zero()) }
    pub fn one() -> JsGF3 { JsGF3(classes::finite_field::WasmGF3::one()) }
    pub fn add(&self, rhs: &JsGF3) -> JsGF3 { JsGF3(self.0.add(&rhs.0)) }
    pub fn sub(&self, rhs: &JsGF3) -> JsGF3 { JsGF3(self.0.sub(&rhs.0)) }
    pub fn mul(&self, rhs: &JsGF3) -> JsGF3 { JsGF3(self.0.mul(&rhs.0)) }
    pub fn div(&self, other: &JsGF3) -> Result<JsGF3, JsError> { self.0.div(&other.0).map(JsGF3) }
    pub fn neg(&self) -> JsGF3 { JsGF3(self.0.neg()) }
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i64 { self.0.value() }
    #[wasm_bindgen(getter = isZero)]
    pub fn is_zero(&self) -> bool { self.0.is_zero() }
    #[wasm_bindgen(getter = isOne)]
    pub fn is_one(&self) -> bool { self.0.is_one() }
}

#[wasm_bindgen(js_name = GFExtGF2)]
pub struct JsGFExtGF2(classes::finite_field::WasmGFExtGF2);
#[wasm_bindgen(js_class = "GFExtGF2")]
impl JsGFExtGF2 {
    #[wasm_bindgen(constructor)]
    pub fn new(px_coeffs: Vec<u8>, coeffs: Vec<u8>) -> JsGFExtGF2 { JsGFExtGF2(classes::finite_field::WasmGFExtGF2::new(px_coeffs, coeffs)) }
    #[wasm_bindgen(js_name = fromBase)]
    pub fn from_base(px_coeffs: Vec<u8>, base_value: u8) -> JsGFExtGF2 { JsGFExtGF2(classes::finite_field::WasmGFExtGF2::from_base(px_coeffs, base_value)) }
    pub fn inv(&self) -> Result<JsGFExtGF2, JsValue> { self.0.inv().map(JsGFExtGF2) }
    pub fn zero() -> JsGFExtGF2 { JsGFExtGF2(classes::finite_field::WasmGFExtGF2::zero()) }
    pub fn one() -> JsGFExtGF2 { JsGFExtGF2(classes::finite_field::WasmGFExtGF2::one()) }
    pub fn add(&self, rhs: &JsGFExtGF2) -> JsGFExtGF2 { JsGFExtGF2(self.0.add(&rhs.0)) }
    pub fn sub(&self, rhs: &JsGFExtGF2) -> JsGFExtGF2 { JsGFExtGF2(self.0.sub(&rhs.0)) }
    pub fn mul(&self, rhs: &JsGFExtGF2) -> JsGFExtGF2 { JsGFExtGF2(self.0.mul(&rhs.0)) }
    pub fn div(&self, other: &JsGFExtGF2) -> Result<JsGFExtGF2, JsError> { self.0.div(&other.0).map(JsGFExtGF2) }
    pub fn neg(&self) -> JsGFExtGF2 { JsGFExtGF2(self.0.neg()) }
    #[wasm_bindgen(getter)]
    pub fn coeffs(&self) -> Vec<u8> { self.0.coeffs() }
    #[wasm_bindgen(getter)]
    pub fn px(&self) -> Vec<u8> { self.0.px() }
}
// Re-export polynomial classes
pub use classes::polynomial::{PolynomialF64, PolynomialGF2, PolynomialGF256, PolynomialGFExtGF2};
// Re-export statistics classes
pub use classes::statistics::{
    WasmNormal,
    WasmGamma,
    WasmExponential,
    WasmBernoulli,
    WasmPoisson,
    WasmUniform,
    WasmStudentT,
    WasmChiSquare,
    WasmF,
    WasmBinomial,
    WasmCategorical,
};
//pub use test_linalg_impl::{TestMatrixF64, TestVectorF64, TestMatrixI32, TestVectorI32};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn __probe() -> i32 { 1 }

// Simple aliases for common types (Matrix/Vector) delegating to F64 versions
#[wasm_bindgen(js_name = Matrix)]
pub struct JsMatrix(classes::linalg::MatrixF64);

#[wasm_bindgen(js_class = "Matrix")]
impl JsMatrix {
    #[wasm_bindgen(constructor)]
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Result<JsMatrix, JsValue> {
        classes::linalg::MatrixF64::new(rows, cols, data).map(JsMatrix)
    }
    pub fn with_default(rows: usize, cols: usize) -> JsMatrix { JsMatrix(classes::linalg::MatrixF64::with_default(rows, cols)) }
    pub fn zeros(rows: usize, cols: usize) -> JsMatrix { JsMatrix(classes::linalg::MatrixF64::zeros(rows, cols)) }
    pub fn identity(size: usize) -> JsMatrix { JsMatrix(classes::linalg::MatrixF64::identity(size)) }
    pub fn rows(&self) -> usize { self.0.rows() }
    pub fn cols(&self) -> usize { self.0.cols() }
    pub fn is_square(&self) -> bool { self.0.is_square() }
    pub fn transpose(&self) -> JsMatrix { JsMatrix(self.0.transpose()) }
    pub fn trace(&self) -> Result<f64, JsValue> { self.0.trace() }
    pub fn determinant(&self) -> Result<f64, JsValue> { self.0.determinant() }
    pub fn rank(&self) -> Result<usize, JsValue> { self.0.rank() }
    pub fn inverse(&self) -> Option<JsMatrix> { self.0.inverse().map(JsMatrix) }
    pub fn frobenius_norm(&self) -> f64 { self.0.frobenius_norm() }
    pub fn expm(&self) -> JsMatrix { JsMatrix(self.0.expm()) }
    pub fn qr_decomposition(&self) -> Result<JsValue, JsValue> { self.0.qr_decomposition() }
    pub fn svd(&self) -> Result<JsValue, JsValue> { self.0.svd() }
    pub fn eigen_decomposition(&self) -> Result<JsValue, JsValue> { self.0.eigen_decomposition() }
    pub fn cholesky(&self) -> Result<JsMatrix, JsValue> { self.0.cholesky().map(JsMatrix) }
    pub fn pinv(&self) -> Result<JsMatrix, JsValue> { self.0.pinv().map(JsMatrix) }
}

#[wasm_bindgen(js_name = Vector)]
pub struct JsVector(classes::linalg::VectorF64);

#[wasm_bindgen(js_class = "Vector")]
impl JsVector {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<f64>) -> JsVector { JsVector(classes::linalg::VectorF64::new(data)) }
    pub fn zeros(dim: usize) -> JsVector { JsVector(classes::linalg::VectorF64::zeros(dim)) }
    pub fn ones(dim: usize) -> JsVector { JsVector(classes::linalg::VectorF64::ones(dim)) }
    pub fn dim(&self) -> usize { self.0.dim() }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn dot(&self, other: &JsVector) -> f64 { self.0.dot(&other.0) }
    pub fn argmax(&self) -> Option<usize> { self.0.argmax() }
    pub fn argmin(&self) -> Option<usize> { self.0.argmin() }
    pub fn max(&self) -> Option<f64> { self.0.max() }
    pub fn min(&self) -> Option<f64> { self.0.min() }
    pub fn norm(&self) -> f64 { self.0.norm() }
    pub fn normalize(&self) -> JsVector { JsVector(self.0.normalize()) }
    pub fn cosine_similarity(&self, other: &JsVector) -> f64 { self.0.cosine_similarity(&other.0) }
    pub fn mean(&self) -> Option<f64> { self.0.mean() }
    pub fn std(&self) -> f64 { self.0.std() }
    pub fn linspace(start: f64, end: f64, num: usize) -> Result<JsVector, JsValue> { classes::linalg::VectorF64::linspace(start, end, num).map(JsVector) }
    pub fn sum(&self) -> f64 { self.0.sum() }
    pub fn transpose(&self) -> classes::linalg::MatrixF64 { self.0.transpose() }
    pub fn to_column_matrix(&self) -> classes::linalg::MatrixF64 { self.0.to_column_matrix() }
    pub fn to_row_matrix(&self) -> classes::linalg::MatrixF64 { self.0.to_row_matrix() }
}

// ---- statsmodels convenience exports (ensure d.ts exposure) ----
#[wasm_bindgen(js_name = solveLinearSystem)]
pub fn solve_linear_system(rows: usize, cols: usize, a_data: Vec<f64>, b: Vec<f64>) -> Result<Vec<f64>, wasm_bindgen::JsValue> {
    crate::classes::statsmodels::solve_linear_system_js(rows, cols, a_data, b)
}

#[wasm_bindgen(js_name = ridgeRegression)]
pub fn ridge_regression(rows: usize, cols: usize, a_data: Vec<f64>, b: Vec<f64>, alpha: f64) -> Result<Vec<f64>, wasm_bindgen::JsValue> {
    crate::classes::statsmodels::ridge_regression_js(rows, cols, a_data, b, alpha)
}

#[wasm_bindgen(js_name = lassoRegression)]
pub fn lasso_regression(rows: usize, cols: usize, a_data: Vec<f64>, b: Vec<f64>, alpha: f64, max_iter: usize, tol: f64) -> Result<Vec<f64>, wasm_bindgen::JsValue> {
    crate::classes::statsmodels::lasso_regression_js(rows, cols, a_data, b, alpha, max_iter, tol)
}

// ---- statsmodels: additional estimators ----
#[wasm_bindgen(js_name = logisticFit)]
pub fn logistic_fit(rows: usize, cols: usize, x_data: Vec<f64>, y: Vec<f64>, lr: f64, max_iter: usize) -> Result<Vec<f64>, JsValue> {
    crate::classes::statsmodels::logistic_fit_js(rows, cols, x_data, y, lr, max_iter)
}
#[wasm_bindgen(js_name = logisticPredictProba)]
pub fn logistic_predict_proba(cols: usize, coeffs: Vec<f64>, x: Vec<f64>) -> Result<f64, JsValue> {
    crate::classes::statsmodels::logistic_predict_proba_js(cols, coeffs, x)
}

#[wasm_bindgen(js_name = gmmFit)]
pub fn gmm_fit(n_samples: usize, n_features: usize, data: Vec<f64>, k: usize, max_iter: usize, tol: f64) -> Result<Vec<f64>, JsValue> {
    crate::classes::statsmodels::gmm_fit_js(n_samples, n_features, data, k, max_iter, tol)
}
#[wasm_bindgen(js_name = gmmPredictProba)]
pub fn gmm_predict_proba(n_features: usize, params: Vec<f64>, x: Vec<f64>) -> Result<Vec<f64>, JsValue> {
    crate::classes::statsmodels::gmm_predict_proba_js(n_features, params, x)
}

#[wasm_bindgen(js_name = bayesianLinearPosterior)]
pub fn bayesian_linear_posterior(rows: usize, cols: usize, x_data: Vec<f64>, y: Vec<f64>, prior_mean: Vec<f64>, prior_cov: Vec<f64>, noise_cov: Vec<f64>) -> Result<Vec<f64>, JsValue> {
    crate::classes::statsmodels::bayesian_linear_posterior_js(rows, cols, x_data, y, prior_mean, prior_cov, noise_cov)
}

#[wasm_bindgen(js_name = kalmanPredict)]
pub fn kalman_predict(n: usize, f_flat: Vec<f64>, q_flat: Vec<f64>, x_flat: Vec<f64>, p_flat: Vec<f64>) -> Result<Vec<f64>, JsValue> {
    crate::classes::statsmodels::kalman_predict_js(n, f_flat, q_flat, x_flat, p_flat)
}
#[wasm_bindgen(js_name = kalmanUpdate)]
pub fn kalman_update(n: usize, h_flat: Vec<f64>, r_flat: Vec<f64>, z_flat: Vec<f64>, x_flat: Vec<f64>, p_flat: Vec<f64>) -> Result<Vec<f64>, JsValue> {
    crate::classes::statsmodels::kalman_update_js(n, h_flat, r_flat, z_flat, x_flat, p_flat)
}