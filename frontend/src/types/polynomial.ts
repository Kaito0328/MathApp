// TypeScript mirrors for polynomial crate structures used via WASM DTOs
// Note: Rust側では Polynomial<F: Field> だが、WASM越しは実数(f64)や複素(Complex<f64>)に
// 限定したDTOでやりとりするのが現実的。

import type { Complex64 } from './signal'

// 低次→高次で係数を格納
export interface PolynomialR {
  coeffs: number[]
}

export interface PolynomialC {
  coeffs: Complex64[]
}

export interface RationalFunctionR {
  numerator: PolynomialR
  denominator: PolynomialR
}

export interface Root {
  value: Complex64
  multiplicity: number
}

export type Pole = Root

export interface PoleTerm {
  pole: Complex64
  // C_j のリスト（(s - p)^j の分子係数）
  coefficients: Complex64[]
}

export interface PartialFractionExpansion {
  polynomial_part: PolynomialR
  pole_terms: PoleTerm[]
}
