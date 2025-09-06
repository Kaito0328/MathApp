// TypeScript mirrors for polynomial crate structures used via WASM DTOs
// Note: Rust側では Polynomial<F: Field> だが、WASM越しは実数(f64)や複素(Complex<f64>)に
// 限定したDTOでやりとりするのが現実的。

import type { Complex64 } from './signal'

// 低次→高次で係数を格納
export interface PolynomialR {
  coeffs: number[]
}

// Note: PolynomialC (complex coeffs) is not currently used on the frontend; remove until needed.

export interface RationalFunctionR {
  numerator: PolynomialR
  denominator: PolynomialR
}

export interface PartialFractionExpansion {
  polynomial_part: PolynomialR
  pole_terms: { pole: Complex64; coefficients: Complex64[] }[]
}
