//! concrete-math クレートの WASM バインディング（最小セット）

use num_complex::Complex;
use wasm_bindgen::prelude::*;
use concrete_math::sequence::core::{GeneralTerm as CMGeneralTerm, ClosedFormDisplay as CMClosedFormDisplay};

// ---- 数え上げ（数値） ----

#[wasm_bindgen(js_name = binom)]
pub fn binom_js(n: usize, k: usize) -> f64 { concrete_math::combinatorics::numbers::binom(n, k) }

#[wasm_bindgen(js_name = stirling2)]
pub fn stirling2_js(n: usize, k: usize) -> f64 { concrete_math::combinatorics::numbers::stirling2(n, k) }

// ---- 複素係数多項式: Float64Array を [re0, im0, re1, im1, ...] として受け渡し ----

fn flat_to_poly(mut flat: Vec<f64>) -> poly::polynomial::Polynomial<Complex<f64>> {
    if flat.len() % 2 != 0 { flat.pop(); }
    let mut coeffs: Vec<Complex<f64>> = Vec::with_capacity(flat.len() / 2);
    let mut i = 0usize;
    while i + 1 < flat.len() {
        coeffs.push(Complex::new(flat[i], flat[i + 1]));
        i += 2;
    }
    poly::polynomial::Polynomial::new(coeffs)
}

fn poly_to_flat(p: poly::polynomial::Polynomial<Complex<f64>>) -> Vec<f64> {
    let mut out = Vec::with_capacity(p.coeffs.len() * 2);
    for c in p.coeffs {
        out.push(c.re);
        out.push(c.im);
    }
    out
}

// ---- combinatorics::polynomials ----

#[wasm_bindgen(js_name = fallingFactorialPoly)]
pub fn falling_factorial_poly_js(m: usize) -> Vec<f64> {
    let p = concrete_math::combinatorics::polynomials::falling_factorial_poly(m);
    poly_to_flat(p)
}

#[wasm_bindgen(js_name = risingFactorialPoly)]
pub fn rising_factorial_poly_js(m: usize) -> Vec<f64> {
    let p = concrete_math::combinatorics::polynomials::rising_factorial_poly(m);
    poly_to_flat(p)
}

#[wasm_bindgen(js_name = shiftPolyXPlusH)]
pub fn shift_poly_x_plus_h_js(coeffs_flat: Vec<f64>, h: f64) -> Vec<f64> {
    let p = flat_to_poly(coeffs_flat);
    let q = concrete_math::combinatorics::polynomials::shift_poly_x_plus_h(&p, h);
    poly_to_flat(q)
}

#[wasm_bindgen(js_name = binomXPlusKChooseKPoly)]
pub fn binom_x_plus_k_choose_k_poly_js(k: usize) -> Vec<f64> {
    let p = concrete_math::combinatorics::polynomials::binom_x_plus_k_choose_k_poly(k);
    poly_to_flat(p)
}

// ---- sum::discrete ----

#[wasm_bindgen(js_name = discreteDiff)]
pub fn discrete_diff_js(coeffs_flat: Vec<f64>) -> Vec<f64> {
    let p = flat_to_poly(coeffs_flat);
    let q = concrete_math::sum::discrete::discrete_diff(&p);
    poly_to_flat(q)
}

#[wasm_bindgen(js_name = discreteSum)]
pub fn discrete_sum_js(coeffs_flat: Vec<f64>) -> Vec<f64> {
    let p = flat_to_poly(coeffs_flat);
    let q = concrete_math::sum::discrete::discrete_sum(&p);
    poly_to_flat(q)
}

// ---- sequence::recurrence をクラスとフラット配列で公開（d.ts安定） ----

#[wasm_bindgen(js_name = ClosedForm)]
pub struct WasmClosedForm(concrete_math::sequence::core::ClosedForm);

// JS 側のクラス名 ClosedForm にメソッドをぶら下げる
#[wasm_bindgen(js_class = "ClosedForm")]
impl WasmClosedForm {
    #[wasm_bindgen(js_name = termsCount)]
    pub fn terms_count(&self) -> usize { self.0.terms.len() }

    #[wasm_bindgen(js_name = termPoly)]
    pub fn term_poly(&self, i: usize) -> Result<Vec<f64>, JsError> {
        self.0.terms.get(i)
            .map(|t| poly_to_flat(t.polynomial.clone()))
            .ok_or_else(|| JsError::new("term index out of range"))
    }

    #[wasm_bindgen(js_name = termBase)]
    pub fn term_base(&self, i: usize) -> Result<Vec<f64>, JsError> {
        self.0.terms.get(i)
            .map(|t| vec![t.base.re, t.base.im])
            .ok_or_else(|| JsError::new("term index out of range"))
    }

    #[wasm_bindgen(js_name = term)]
    pub fn term(&self, n: u32) -> Vec<f64> {
        let v = self.0.term(n);
        vec![v.re, v.im]
    }

    /// 人が読める文字列表現（既定 var="n"）。上付き指数はオプション。
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string_js(&self, var: Option<String>, unicode_superscript: bool) -> String {
        let v_static: &'static str = match var.as_deref() {
            Some("n") => "n",
            Some("k") => "k",
            Some("x") => "x",
            Some("t") => "t",
            _ => "n",
        };
        format!("{}", CMClosedFormDisplay::new(&self.0, v_static).unicode_superscript(unicode_superscript))
    }

    /// 部分和（S(n) = sum_{i=0..n} a(i)）を ClosedForm として返す
    #[wasm_bindgen(js_name = partialSum)]
    pub fn partial_sum_method(&self) -> WasmClosedForm {
        WasmClosedForm(concrete_math::sum::partial_sum::partial_sum(&self.0))
    }
}

// 入力:
// - coeffs: 斉次係数 a_1..a_k（次数k）
// - nh_polys_flat: 連結した複素係数列 [re,im,...] を全非斉次項ぶん結合
// - nh_offsets: 各項の開始インデックス（Float64フラット配列のインデックス、最後は末尾）、長さ m+1
// - nh_bases: 長さ 2m の [re0,im0,re1,im1,...]
// - initial_values: 初期値列（長さは k 以上推奨）
#[wasm_bindgen(js_name = solveRecurrence)]
pub fn solve_recurrence(
    coeffs: Vec<f64>,
    nh_polys_flat: Vec<f64>,
    nh_offsets: Vec<u32>,
    nh_bases: Vec<f64>,
    initial_values: Vec<f64>,
) -> Result<WasmClosedForm, JsError> {
    if nh_bases.len() % 2 != 0 { return Err(JsError::new("nh_bases length must be even")); }
    let m = nh_bases.len() / 2;
    if nh_offsets.len() != m + 1 { return Err(JsError::new("nh_offsets length must be m+1")); }
    let last = *nh_offsets.last().unwrap_or(&0) as usize;
    if last != nh_polys_flat.len() { return Err(JsError::new("nh_offsets last must equal nh_polys_flat length")); }
    // 各区間は [re,im,...] で偶数長
    let mut terms = Vec::with_capacity(m);
    for i in 0..m {
        let s = nh_offsets[i] as usize;
        let e = nh_offsets[i+1] as usize;
        if (e < s) || ((e - s) % 2 != 0) { return Err(JsError::new("invalid nh_offsets segment")); }
        let poly = flat_to_poly(nh_polys_flat[s..e].to_vec());
        let base = num_complex::Complex::new(nh_bases[2*i], nh_bases[2*i+1]);
        terms.push(concrete_math::sequence::core::GeneralTerm { polynomial: poly, base });
    }
    let rr = concrete_math::sequence::recurrence_relation::RecurrenceRelation::new(coeffs, terms, initial_values);
    Ok(WasmClosedForm(rr.solve()))
}

/// 部分和（S(n) = sum_{i=0..n} a(i)）を ClosedForm として返す（自由関数版）
#[wasm_bindgen(js_name = partialSum)]
pub fn partial_sum_cf(cf: &WasmClosedForm) -> WasmClosedForm {
    WasmClosedForm(concrete_math::sum::partial_sum::partial_sum(&cf.0))
}

// ---- GeneralTerm / RecurrenceRelation をクラスで公開 ----

#[wasm_bindgen(js_name = GeneralTerm)]
pub struct WasmGeneralTerm(CMGeneralTerm);

#[wasm_bindgen(js_class = "GeneralTerm")]
impl WasmGeneralTerm {
    /// constructor from complex-coefficient polynomial (flat) and base (re, im)
    #[wasm_bindgen(constructor)]
    pub fn new(poly_flat: Vec<f64>, base_re: f64, base_im: f64) -> WasmGeneralTerm {
        let poly = flat_to_poly(poly_flat);
        let base = Complex::new(base_re, base_im);
        WasmGeneralTerm(CMGeneralTerm { polynomial: poly, base })
    }
    #[wasm_bindgen(js_name = polynomial)]
    pub fn polynomial_flat(&self) -> Vec<f64> { poly_to_flat(self.0.polynomial.clone()) }
    #[wasm_bindgen(js_name = base)]
    pub fn base_pair(&self) -> Vec<f64> { vec![self.0.base.re, self.0.base.im] }
}

#[wasm_bindgen(js_name = RecurrenceRelation)]
pub struct WasmRecurrenceRelation(concrete_math::sequence::recurrence_relation::RecurrenceRelation);

#[wasm_bindgen(js_class = "RecurrenceRelation")]
impl WasmRecurrenceRelation {
    /// constructor: coeffs (a1..ak), non-homogeneous terms, initial values
    #[wasm_bindgen(constructor)]
    pub fn new(coeffs: Vec<f64>, terms: Vec<WasmGeneralTerm>, initial_values: Vec<f64>) -> WasmRecurrenceRelation {
        let internal_terms: Vec<CMGeneralTerm> = terms.into_iter().map(|t| t.0).collect();
        let rr = concrete_math::sequence::recurrence_relation::RecurrenceRelation::new(coeffs, internal_terms, initial_values);
        WasmRecurrenceRelation(rr)
    }

    #[wasm_bindgen(js_name = solve)]
    pub fn solve_js(&self) -> WasmClosedForm { WasmClosedForm(self.0.solve()) }

    #[wasm_bindgen(js_name = coeffs)]
    pub fn coeffs(&self) -> Vec<f64> { self.0.coeffs.clone() }
}
