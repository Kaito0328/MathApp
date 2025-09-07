//! statsmodels クレートの WASM バインディング

use wasm_bindgen::prelude::*;

use linalg::{Matrix, Vector};
use statsmodels::linear_model::{lasso, ols, ridge};
use statsmodels::estimation::{
	bayesian,
	gmm::GaussianMixtureModel,
	logistic::LogisticRegression,
};
use statistics::distribution::multivariate_continuous::core::MultivariateDistribution;

fn mat_from_flat(rows: usize, cols: usize, data: Vec<f64>) -> Result<Matrix<f64>, JsValue> {
	if data.len() != rows * cols {
		return Err(JsValue::from_str("data length must equal rows*cols"));
	}
	Matrix::new(rows, cols, data).map_err(|e| JsValue::from_str(&format!("{}", e)))
}

fn vec_from(b: Vec<f64>) -> Vector<f64> { Vector::new(b) }

#[wasm_bindgen]
pub struct WasmLinearModel;

#[wasm_bindgen]
impl WasmLinearModel {
	#[wasm_bindgen(js_name = solveLinearSystem)]
	pub fn solve_linear_system(rows: usize, cols: usize, a_data: Vec<f64>, b: Vec<f64>) -> Result<Vec<f64>, JsValue> {
		solve_linear_system_js(rows, cols, a_data, b)
	}

	#[wasm_bindgen(js_name = ridgeRegression)]
	pub fn ridge_regression(rows: usize, cols: usize, a_data: Vec<f64>, b: Vec<f64>, alpha: f64) -> Result<Vec<f64>, JsValue> {
		ridge_regression_js(rows, cols, a_data, b, alpha)
	}

	#[wasm_bindgen(js_name = lassoRegression)]
	pub fn lasso_regression(rows: usize, cols: usize, a_data: Vec<f64>, b: Vec<f64>, alpha: f64, max_iter: usize, tol: f64) -> Result<Vec<f64>, JsValue> {
		lasso_regression_js(rows, cols, a_data, b, alpha, max_iter, tol)
	}
}

pub fn solve_linear_system_js(rows: usize, cols: usize, a_data: Vec<f64>, b: Vec<f64>) -> Result<Vec<f64>, JsValue> {
	if b.len() != rows { return Err(JsValue::from_str("b length must equal rows")); }
	let a = mat_from_flat(rows, cols, a_data)?;
	let b = vec_from(b);
	let x = ols::solve_linear_system(&a, &b).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	Ok(x.as_slice().to_vec())
}

pub fn ridge_regression_js(rows: usize, cols: usize, a_data: Vec<f64>, b: Vec<f64>, alpha: f64) -> Result<Vec<f64>, JsValue> {
	if b.len() != rows { return Err(JsValue::from_str("b length must equal rows")); }
	let a = mat_from_flat(rows, cols, a_data)?;
	let b = vec_from(b);
	let x = ridge::ridge_regression_optimized(&a, &b, alpha).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	Ok(x.as_slice().to_vec())
}

pub fn lasso_regression_js(
	rows: usize,
	cols: usize,
	a_data: Vec<f64>,
	b: Vec<f64>,
	alpha: f64,
	max_iter: usize,
	tol: f64,
) -> Result<Vec<f64>, JsValue> {
	if b.len() != rows { return Err(JsValue::from_str("b length must equal rows")); }
	let a = mat_from_flat(rows, cols, a_data)?;
	let b = vec_from(b);
	let x = lasso::lasso_regression(&a, &b, alpha, max_iter, tol).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	Ok(x.as_slice().to_vec())
}

// ---- Logistic Regression ----
pub fn logistic_fit_js(
	rows: usize,
	cols: usize,
	x_data: Vec<f64>,
	y: Vec<f64>,
	lr: f64,
	max_iter: usize,
) -> Result<Vec<f64>, JsValue> {
	if y.len() != rows { return Err(JsValue::from_str("y length must equal rows")); }
	let x = mat_from_flat(rows, cols, x_data)?;
	let yv = vec_from(y);
	let model = LogisticRegression::fit(&x, &yv, lr, max_iter);
	Ok(model.coefficients().as_slice().to_vec())
}

pub fn logistic_predict_proba_js(
	cols: usize,
	coeffs: Vec<f64>,
	x: Vec<f64>,
) -> Result<f64, JsValue> {
	if x.len() != cols { return Err(JsValue::from_str("x length must equal cols")); }
	if coeffs.len() != cols + 1 { return Err(JsValue::from_str("coeffs length must equal cols+1 (intercept + weights)")); }
	let intercept = coeffs[0];
	let mut z = intercept;
	for j in 0..cols { z += coeffs[j + 1] * x[j]; }
	let p = 1.0 / (1.0 + (-z).exp());
	Ok(p)
}

// ---- Gaussian Mixture Model ----
fn vectors_from_flat(n_samples: usize, n_features: usize, data: Vec<f64>) -> Result<Vec<Vector<f64>>, JsValue> {
	if data.len() != n_samples * n_features {
		return Err(JsValue::from_str("data length must equal n_samples*n_features"));
	}
	let mut out = Vec::with_capacity(n_samples);
	for i in 0..n_samples {
		let start = i * n_features;
		let end = start + n_features;
		out.push(Vector::new(data[start..end].to_vec()));
	}
	Ok(out)
}

pub fn gmm_fit_js(
	n_samples: usize,
	n_features: usize,
	data: Vec<f64>,
	k: usize,
	max_iter: usize,
	tol: f64,
) -> Result<Vec<f64>, JsValue> {
	let dataset = vectors_from_flat(n_samples, n_features, data)?;
	let model = GaussianMixtureModel::fit(&dataset, k, max_iter, tol)
		.map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	let weights = model.weights().clone();
	let mut means: Vec<f64> = Vec::with_capacity(k * n_features);
	for mvn in model.distributions().iter() {
		let m = mvn.mean();
		means.extend_from_slice(m.as_slice());
	}
	let mut covs: Vec<f64> = Vec::with_capacity(k * n_features * n_features);
	for mvn in model.distributions().iter() {
		let c = mvn.covariance();
		covs.extend_from_slice(c.data.as_slice());
	}
	// パック形式: [k, d, weights(k), means(k*d), covariances(k*d*d)]
	let mut out = Vec::with_capacity(2 + weights.len() + means.len() + covs.len());
	out.push(k as f64);
	out.push(n_features as f64);
	out.extend_from_slice(&weights);
	out.extend_from_slice(&means);
	out.extend_from_slice(&covs);
	Ok(out)
}

pub fn gmm_predict_proba_js(
	n_features: usize,
	params: Vec<f64>,
	x: Vec<f64>,
) -> Result<Vec<f64>, JsValue> {
	if x.len() != n_features { return Err(JsValue::from_str("x length must equal n_features")); }
	if params.len() < 2 { return Err(JsValue::from_str("params too short")); }
	let k = params[0] as usize;
	let d = params[1] as usize;
	if d != n_features { return Err(JsValue::from_str("params d does not match n_features")); }
	let expected = 2 + k + k * d + k * d * d;
	if params.len() != expected { return Err(JsValue::from_str("params length mismatch")); }
	let mut idx = 2;
	let weights = params[idx..idx + k].to_vec();
	idx += k;
	let mut means = Vec::with_capacity(k);
	for _ in 0..k {
		means.push(Vector::new(params[idx..idx + d].to_vec()));
		idx += d;
	}
	let mut covs = Vec::with_capacity(k);
	for _ in 0..k {
		covs.push(Matrix::new(d, d, params[idx..idx + d * d].to_vec()).map_err(|e| JsValue::from_str(&format!("{}", e)))?);
		idx += d * d;
	}
	let model = GaussianMixtureModel::new(weights, means, covs)
		.map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	let xv = Vector::new(x);
	Ok(model.predict_proba(&xv))
}

// ---- Bayesian linear posterior ----
pub fn bayesian_linear_posterior_js(
	rows: usize,
	cols: usize,
	x_data: Vec<f64>,
	y: Vec<f64>,
	prior_mean: Vec<f64>,
	prior_cov: Vec<f64>,
	noise_cov: Vec<f64>,
) -> Result<Vec<f64>, JsValue> {
	if y.len() != rows { return Err(JsValue::from_str("y length must equal rows")); }
	if prior_mean.len() != cols { return Err(JsValue::from_str("prior_mean length must equal cols")); }
	if prior_cov.len() != cols * cols { return Err(JsValue::from_str("prior_cov must be cols*cols")); }
	if noise_cov.len() != rows * rows { return Err(JsValue::from_str("noise_cov must be rows*rows")); }
	let x = mat_from_flat(rows, cols, x_data)?;
	let yv = vec_from(y);
	let m0 = Vector::new(prior_mean);
	let p0 = Matrix::new(cols, cols, prior_cov).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	let r = Matrix::new(rows, rows, noise_cov).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	let mvn = bayesian::bayesian_estimation(&yv, &x, &m0, &p0, &r)
		.map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	let mut out = Vec::with_capacity(cols + cols * cols);
	out.extend_from_slice(mvn.mean().as_slice());
	out.extend_from_slice(mvn.covariance().data.as_slice());
	Ok(out)
}

// ---- Kalman filter: predict/update stateless helpers ----
pub fn kalman_predict_js(
	n: usize,
	f_flat: Vec<f64>,
	q_flat: Vec<f64>,
	x_flat: Vec<f64>,
	p_flat: Vec<f64>,
) -> Result<Vec<f64>, JsValue> {
	if x_flat.len() != n { return Err(JsValue::from_str("x length must equal n")); }
	if f_flat.len() != n * n { return Err(JsValue::from_str("F must be n*n")); }
	if q_flat.len() != n * n { return Err(JsValue::from_str("Q must be n*n")); }
	if p_flat.len() != n * n { return Err(JsValue::from_str("P must be n*n")); }
	let f = Matrix::new(n, n, f_flat).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	let q = Matrix::new(n, n, q_flat).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	let x = Vector::new(x_flat);
	let p = Matrix::new(n, n, p_flat).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	let x_new = &f * &x;
	let p_new = &f * &p * &f.transpose() + &q;
	let mut out = Vec::with_capacity(n + n * n);
	out.extend_from_slice(x_new.as_slice());
	out.extend_from_slice(p_new.data.as_slice());
	Ok(out)
}

pub fn kalman_update_js(
	n: usize,
	h_flat: Vec<f64>,
	r_flat: Vec<f64>,
	z_flat: Vec<f64>,
	x_flat: Vec<f64>,
	p_flat: Vec<f64>,
) -> Result<Vec<f64>, JsValue> {
	if x_flat.len() != n { return Err(JsValue::from_str("x length must equal n")); }
	if p_flat.len() != n * n { return Err(JsValue::from_str("P must be n*n")); }
	if r_flat.len() == 0 || h_flat.len() == 0 || z_flat.len() == 0 { return Err(JsValue::from_str("H, R, z must be non-empty")); }
	let m = z_flat.len();
	if h_flat.len() != m * n { return Err(JsValue::from_str("H must be m*n")); }
	if r_flat.len() != m * m { return Err(JsValue::from_str("R must be m*m")); }
	let h = Matrix::new(m, n, h_flat).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	let r = Matrix::new(m, m, r_flat).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	let z = Vector::new(z_flat);
	let x = Vector::new(x_flat);
	let p = Matrix::new(n, n, p_flat).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	// S = H P H^T + R
	let s = &h * &p * &h.transpose() + &r;
	// K = P H^T S^{-1} => solve S^T Y = (P H^T)^T, K = Y^T
	let pht = &p * &h.transpose();
	let y = s.transpose().solve_matrix(&pht.transpose())
		.map_err(|e| JsValue::from_str(&format!("{}", e)))?;
	let k = y.transpose();
	let x_new = &x + &k * (&z - &h * &x);
	let i = Matrix::identity(n);
	let p_new = &(&i - &(&k * &h)) * &p * &(&i - &(&k * &h)).transpose() + &(&k * &r) * &k.transpose();
	let mut out = Vec::with_capacity(n + n * n);
	out.extend_from_slice(x_new.as_slice());
	out.extend_from_slice(p_new.data.as_slice());
	Ok(out)
}

