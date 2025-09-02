use wasm_bindgen::prelude::*;

use statistics as stats;
// Bring trait methods (mean, variance, pdf, cdf, quantile, etc.) into scope
use stats::distribution::continuous::core::Distribution as ContDistribution;
use stats::distribution::discrete::core::Distribution as DiscDistribution;

// 連続分布: Normal
#[wasm_bindgen(js_name = Normal)]
pub struct WasmNormal {
	inner: stats::distribution::continuous::normal::Normal,
}

// 連続分布: Uniform(a,b)
#[wasm_bindgen(js_name = Uniform)]
pub struct WasmUniform { inner: stats::distribution::continuous::uniform::Uniform }
#[wasm_bindgen(js_class = "Uniform")]
impl WasmUniform {
	#[wasm_bindgen(constructor)]
	pub fn new(a: f64, b: f64) -> Result<WasmUniform, JsValue> {
		stats::distribution::continuous::uniform::Uniform::new(a, b)
			.map(|inner| WasmUniform { inner })
			.map_err(|e| JsValue::from_str(&format!("{}", e)))
	}
	pub fn mean(&self) -> f64 { self.inner.mean() }
	pub fn variance(&self) -> f64 { self.inner.variance() }
	pub fn std_dev(&self) -> f64 { self.inner.std_dev() }
	pub fn pdf(&self, x: f64) -> f64 { self.inner.pdf(x) }
	pub fn cdf(&self, x: f64) -> f64 { self.inner.cdf(x) }
	pub fn quantile(&self, p: f64) -> f64 { self.inner.quantile(p) }
	pub fn pdf_svg(&self, width: u32, height: u32, samples: usize) -> String {
		stats::plot::svg_continuous_pdf(&self.inner, width, height, samples)
	}
}

// 連続分布: Student's t(df)
#[wasm_bindgen(js_name = StudentT)]
pub struct WasmStudentT { inner: stats::distribution::continuous::t::T }
#[wasm_bindgen(js_class = "StudentT")]
impl WasmStudentT {
	#[wasm_bindgen(constructor)]
	pub fn new(df: u32) -> Result<WasmStudentT, JsValue> {
		stats::distribution::continuous::t::T::new(df as usize)
			.map(|inner| WasmStudentT { inner })
			.map_err(|e| JsValue::from_str(&format!("{}", e)))
	}
	pub fn mean(&self) -> f64 { self.inner.mean() }
	pub fn variance(&self) -> f64 { self.inner.variance() }
	pub fn std_dev(&self) -> f64 { self.inner.std_dev() }
	pub fn pdf(&self, x: f64) -> f64 { self.inner.pdf(x) }
	pub fn cdf(&self, x: f64) -> f64 { self.inner.cdf(x) }
	pub fn quantile(&self, p: f64) -> f64 { self.inner.quantile(p) }
	pub fn pdf_svg(&self, width: u32, height: u32, samples: usize) -> String {
		stats::plot::svg_continuous_pdf(&self.inner, width, height, samples)
	}
}

// 連続分布: ChiSquare(k)
#[wasm_bindgen(js_name = ChiSquare)]
pub struct WasmChiSquare { inner: stats::distribution::continuous::chi_square::ChiSquare }
#[wasm_bindgen(js_class = "ChiSquare")]
impl WasmChiSquare {
	#[wasm_bindgen(constructor)]
	pub fn new(k: u32) -> Result<WasmChiSquare, JsValue> {
		stats::distribution::continuous::chi_square::ChiSquare::new(k as usize)
			.map(|inner| WasmChiSquare { inner })
			.map_err(|e| JsValue::from_str(&format!("{}", e)))
	}
	pub fn mean(&self) -> f64 { self.inner.mean() }
	pub fn variance(&self) -> f64 { self.inner.variance() }
	pub fn std_dev(&self) -> f64 { self.inner.std_dev() }
	pub fn pdf(&self, x: f64) -> f64 { self.inner.pdf(x) }
	pub fn cdf(&self, x: f64) -> f64 { self.inner.cdf(x) }
	pub fn quantile(&self, p: f64) -> f64 { self.inner.quantile(p) }
	pub fn pdf_svg(&self, width: u32, height: u32, samples: usize) -> String {
		stats::plot::svg_continuous_pdf(&self.inner, width, height, samples)
	}
}

// 連続分布: F(d1, d2)
#[wasm_bindgen(js_name = F)]
pub struct WasmF { inner: stats::distribution::continuous::f::F }
#[wasm_bindgen(js_class = "F")]
impl WasmF {
	#[wasm_bindgen(constructor)]
	pub fn new(d1: u32, d2: u32) -> Result<WasmF, JsValue> {
		stats::distribution::continuous::f::F::new(d1 as usize, d2 as usize)
			.map(|inner| WasmF { inner })
			.map_err(|e| JsValue::from_str(&format!("{}", e)))
	}
	pub fn mean(&self) -> f64 { self.inner.mean() }
	pub fn variance(&self) -> f64 { self.inner.variance() }
	pub fn std_dev(&self) -> f64 { self.inner.std_dev() }
	pub fn pdf(&self, x: f64) -> f64 { self.inner.pdf(x) }
	pub fn cdf(&self, x: f64) -> f64 { self.inner.cdf(x) }
	pub fn quantile(&self, p: f64) -> f64 { self.inner.quantile(p) }
	pub fn pdf_svg(&self, width: u32, height: u32, samples: usize) -> String {
		stats::plot::svg_continuous_pdf(&self.inner, width, height, samples)
	}
}

// 離散分布: Binomial(n,p)
#[wasm_bindgen(js_name = Binomial)]
pub struct WasmBinomial { inner: stats::distribution::discrete::binomial::Binomial }
#[wasm_bindgen(js_class = "Binomial")]
impl WasmBinomial {
	#[wasm_bindgen(constructor)]
	pub fn new(n: u32, p: f64) -> Result<WasmBinomial, JsValue> {
		stats::distribution::discrete::binomial::Binomial::new(n as u64, p)
			.map(|inner| WasmBinomial { inner })
			.map_err(|e| JsValue::from_str(&format!("{}", e)))
	}
	pub fn mean(&self) -> f64 { self.inner.mean() }
	pub fn variance(&self) -> f64 { self.inner.variance() }
	pub fn std_dev(&self) -> f64 { self.inner.std_dev() }
	pub fn pmf(&self, k: u32) -> f64 { self.inner.pmf(k as u64) }
	pub fn cdf(&self, k: u32) -> f64 { self.inner.cdf(k as u64) }
	pub fn quantile(&self, p: f64) -> u32 { self.inner.quantile(p) as u32 }
	pub fn pmf_svg(&self, width: u32, height: u32) -> String {
		stats::plot::svg_discrete_pmf(&self.inner, width, height)
	}
}

// 離散分布: Categorical(probs)
#[wasm_bindgen(js_name = Categorical)]
pub struct WasmCategorical { inner: stats::distribution::discrete::categorical::Categorical }
#[wasm_bindgen(js_class = "Categorical")]
impl WasmCategorical {
	#[wasm_bindgen(constructor)]
	pub fn new(probs: Vec<f64>) -> Result<WasmCategorical, JsValue> {
		stats::distribution::discrete::categorical::Categorical::new(probs)
			.map(|inner| WasmCategorical { inner })
			.map_err(|e| JsValue::from_str(&format!("{}", e)))
	}
	pub fn pmf(&self, k: u32) -> f64 { self.inner.pmf(k as u64) }
	pub fn log_pmf(&self, k: u32) -> f64 { self.inner.log_pmf(k as u64) }
	pub fn cdf(&self, k: u32) -> f64 { self.inner.cdf(k as u64) }
	pub fn quantile(&self, p: f64) -> u32 { self.inner.quantile(p) as u32 }
	pub fn pmf_svg(&self, width: u32, height: u32) -> String {
		stats::plot::svg_discrete_pmf(&self.inner, width, height)
	}
}

#[wasm_bindgen(js_class = "Normal")]
impl WasmNormal {
	#[wasm_bindgen(constructor)]
	pub fn new(mu: f64, sigma: f64) -> Result<WasmNormal, JsValue> {
		stats::distribution::continuous::normal::Normal::new(mu, sigma)
			.map(|inner| WasmNormal { inner })
			.map_err(|e| JsValue::from_str(&format!("{}", e)))
	}
	pub fn mean(&self) -> f64 { self.inner.mean() }
	pub fn variance(&self) -> f64 { self.inner.variance() }
	pub fn std_dev(&self) -> f64 { self.inner.std_dev() }
	pub fn pdf(&self, x: f64) -> f64 { self.inner.pdf(x) }
	pub fn cdf(&self, x: f64) -> f64 { self.inner.cdf(x) }
	pub fn quantile(&self, p: f64) -> f64 { self.inner.quantile(p) }

	pub fn pdf_svg(&self, width: u32, height: u32, samples: usize) -> String {
		stats::plot::svg_continuous_pdf(&self.inner, width, height, samples)
	}
}

// 連続分布: Gamma(k, rate)
#[wasm_bindgen(js_name = Gamma)]
pub struct WasmGamma {
	inner: stats::distribution::continuous::gamma::Gamma,
}

#[wasm_bindgen(js_class = "Gamma")]
impl WasmGamma {
	#[wasm_bindgen(constructor)]
	pub fn new(shape: f64, rate: f64) -> Result<WasmGamma, JsValue> {
		stats::distribution::continuous::gamma::Gamma::new(shape, rate)
			.map(|inner| WasmGamma { inner })
			.map_err(|e| JsValue::from_str(&format!("{}", e)))
	}
	pub fn mean(&self) -> f64 { self.inner.mean() }
	pub fn variance(&self) -> f64 { self.inner.variance() }
	pub fn std_dev(&self) -> f64 { self.inner.std_dev() }
	pub fn pdf(&self, x: f64) -> f64 { self.inner.pdf(x) }
	pub fn cdf(&self, x: f64) -> f64 { self.inner.cdf(x) }
	pub fn quantile(&self, p: f64) -> f64 { self.inner.quantile(p) }

	pub fn pdf_svg(&self, width: u32, height: u32, samples: usize) -> String {
		stats::plot::svg_continuous_pdf(&self.inner, width, height, samples)
	}
}

// 連続分布: Exponential(lambda)
#[wasm_bindgen(js_name = Exponential)]
pub struct WasmExponential {
	inner: stats::distribution::continuous::exponential::Exponential,
}

#[wasm_bindgen(js_class = "Exponential")]
impl WasmExponential {
	#[wasm_bindgen(constructor)]
	pub fn new(lambda: f64) -> Result<WasmExponential, JsValue> {
		stats::distribution::continuous::exponential::Exponential::new(lambda)
			.map(|inner| WasmExponential { inner })
			.map_err(|e| JsValue::from_str(&format!("{}", e)))
	}
	pub fn mean(&self) -> f64 { self.inner.mean() }
	pub fn variance(&self) -> f64 { self.inner.variance() }
	pub fn std_dev(&self) -> f64 { self.inner.std_dev() }
	pub fn pdf(&self, x: f64) -> f64 { self.inner.pdf(x) }
	pub fn cdf(&self, x: f64) -> f64 { self.inner.cdf(x) }
	pub fn quantile(&self, p: f64) -> f64 { self.inner.quantile(p) }

	pub fn pdf_svg(&self, width: u32, height: u32, samples: usize) -> String {
		stats::plot::svg_continuous_pdf(&self.inner, width, height, samples)
	}
}

// 離散分布: Bernoulli(p)
#[wasm_bindgen(js_name = Bernoulli)]
pub struct WasmBernoulli {
	inner: stats::distribution::discrete::bernoulli::Bernoulli,
}

#[wasm_bindgen(js_class = "Bernoulli")]
impl WasmBernoulli {
	#[wasm_bindgen(constructor)]
	pub fn new(p: f64) -> Result<WasmBernoulli, JsValue> {
		stats::distribution::discrete::bernoulli::Bernoulli::new(p)
			.map(|inner| WasmBernoulli { inner })
			.map_err(|e| JsValue::from_str(&format!("{}", e)))
	}
	pub fn mean(&self) -> f64 { self.inner.mean() }
	pub fn variance(&self) -> f64 { self.inner.variance() }
	pub fn std_dev(&self) -> f64 { self.inner.std_dev() }
	pub fn pmf(&self, k: u32) -> f64 { self.inner.pmf(k as u64) }
	pub fn cdf(&self, k: u32) -> f64 { self.inner.cdf(k as u64) }
	pub fn quantile(&self, p: f64) -> u32 { self.inner.quantile(p) as u32 }

	pub fn pmf_svg(&self, width: u32, height: u32) -> String {
		stats::plot::svg_discrete_pmf(&self.inner, width, height)
	}
}

// 離散分布: Poisson(lambda)
#[wasm_bindgen(js_name = Poisson)]
pub struct WasmPoisson {
	inner: stats::distribution::discrete::poisson::Poisson,
}

#[wasm_bindgen(js_class = "Poisson")]
impl WasmPoisson {
	#[wasm_bindgen(constructor)]
	pub fn new(lambda: f64) -> Result<WasmPoisson, JsValue> {
		stats::distribution::discrete::poisson::Poisson::new(lambda)
			.map(|inner| WasmPoisson { inner })
			.map_err(|e| JsValue::from_str(&format!("{}", e)))
	}
	pub fn mean(&self) -> f64 { self.inner.mean() }
	pub fn variance(&self) -> f64 { self.inner.variance() }
	pub fn std_dev(&self) -> f64 { self.inner.std_dev() }
	pub fn pmf(&self, k: u32) -> f64 { self.inner.pmf(k as u64) }
	pub fn log_pmf(&self, k: u32) -> f64 { self.inner.log_pmf(k as u64) }
	pub fn cdf(&self, k: u32) -> f64 { self.inner.cdf(k as u64) }
	pub fn quantile(&self, p: f64) -> u32 { self.inner.quantile(p) as u32 }

	pub fn pmf_svg(&self, width: u32, height: u32) -> String {
		stats::plot::svg_discrete_pmf(&self.inner, width, height)
	}
}
