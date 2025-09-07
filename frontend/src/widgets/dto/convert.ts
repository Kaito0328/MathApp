// Helpers to convert between WASM classes/TypedArrays and DTOs used by widgets
// These are intentionally minimal and safe for UI/state usage.
// Note: we accept wasm-bindgen classes directly via structural typing or type-only imports.

// Type-only imports to describe wasm classes (no runtime cost)
import type {
  Matrix as WasmMatrix,
  MatrixF64 as WasmMatrixF64,
  MatrixF32 as WasmMatrixF32,
  MatrixI32 as WasmMatrixI32,
  Vector as WasmVector,
  VectorF64 as WasmVectorF64,
  VectorF32 as WasmVectorF32,
  VectorI32 as WasmVectorI32,
  WasmSignal,
  WasmSpectrum,
  PolynomialF64,
  ContinuousTF,
  DiscreteTF,
  ContinuousZpk,
  DiscreteZpk,
  Normal as WasmNormal,
  Uniform as WasmUniform,
  Exponential as WasmExponential,
  Gamma as WasmGamma,
  ChiSquare as WasmChiSquare,
  StudentT as WasmStudentT,
  F as WasmF,
  Bernoulli as WasmBernoulli,
  Binomial as WasmBinomial,
  Poisson as WasmPoisson,
  Categorical as WasmCategorical,
} from '../../wasm-pkg/wasm'
import type { Matrix as MatrixDTO } from './linalg'
import type { Signal as SignalDTO, Spectrum as SpectrumDTO } from './signal_processing'
import type { Complex } from './complex'
import type { Polynomial as PolynomialDTO, RationalFunction as RationalFunctionDTO } from './polynomial'
import type { TransferFunction as TransferFunctionDTO, Zpk as ZpkDTO } from './lti-systems'
import type {
  NormalParams,
  UniformParams,
  ExponentialParams,
  GammaParams,
  ChiSquareParams,
  StudentTParams,
  FParams,
  BernoulliParams,
  BinomialParams,
  PoissonParams,
  CategoricalParams,
} from './statistics'

export function toArray(a: ArrayLike<number>): number[] {
  return Array.from({ length: a.length }, (_, i) => Number((a as any)[i]))
}

export function toFloat64Array(a: ArrayLike<number>): Float64Array {
  if (a instanceof Float64Array) return a
  return new Float64Array(toArray(a))
}

// Internal structural guards to detect wasm instances
function isWasmVectorLike(x: any): x is { data(): ArrayLike<number> } {
  return x && typeof x.data === 'function'
}

function isWasmMatrixLike(x: any): x is { rows(): number; cols(): number; data(): ArrayLike<number> } {
  return x && typeof x.rows === 'function' && typeof x.cols === 'function' && typeof x.data === 'function'
}

// Complex helpers
export function complexArrayFromInterleaved(data: ArrayLike<number>): Complex[] {
  const out: Complex[] = []
  for (let i = 0; i < data.length; i += 2) {
    out.push({ re: Number((data as any)[i] ?? 0), im: Number((data as any)[i + 1] ?? 0) })
  }
  return out
}

export function interleavedFromComplexArray(arr: ReadonlyArray<Complex>): number[] {
  const out: number[] = []
  for (const z of arr) {
    out.push(Number(z.re ?? 0), Number(z.im ?? 0))
  }
  return out
}

// Sample time helper (Hz -> s). Returns null if fs <= 0 or not finite.
export function sampleTimeFromRate(fs: number | undefined): number | null {
  if (typeof fs !== 'number' || !Number.isFinite(fs) || fs <= 0) return null
  return 1 / fs
}

// Vector DTOs are typically number[]; accept either raw ArrayLike or wasm Vector classes
export function vectorFromWasm(data: ArrayLike<number>): number[]
export function vectorFromWasm(v: WasmVector | WasmVectorF64 | WasmVectorF32 | WasmVectorI32): number[]
export function vectorFromWasm(arg: ArrayLike<number> | (WasmVector | WasmVectorF64 | WasmVectorF32 | WasmVectorI32)): number[] {
  if (isWasmVectorLike(arg)) return toArray(arg.data())
  return toArray(arg)
}

// Matrix: accept either (rows, cols, data) or a wasm Matrix class directly
export function matrixFromWasm(rows: number, cols: number, data: ArrayLike<number>): MatrixDTO
export function matrixFromWasm(m: WasmMatrix | WasmMatrixF64 | WasmMatrixF32 | WasmMatrixI32): MatrixDTO
export function matrixFromWasm(
  a: number | (WasmMatrix | WasmMatrixF64 | WasmMatrixF32 | WasmMatrixI32),
  b?: number,
  c?: ArrayLike<number>
): MatrixDTO {
  if (isWasmMatrixLike(a)) {
    return { rows: a.rows(), cols: a.cols(), data: toArray(a.data()) }
  }
  return { rows: a as number, cols: b as number, data: toArray(c as ArrayLike<number>) }
}

// “Constructor-like” helpers (factory functions) to keep interface DTOs but improve ergonomics
export function Matrix_fromWasm(m: WasmMatrix | WasmMatrixF64 | WasmMatrixF32 | WasmMatrixI32): MatrixDTO {
  return matrixFromWasm(m)
}
export function Matrix_from(rows: number, cols: number, data: ArrayLike<number>): MatrixDTO {
  return matrixFromWasm(rows, cols, data)
}
export function Vector_fromWasm(v: WasmVector | WasmVectorF64 | WasmVectorF32 | WasmVectorI32 | ArrayLike<number>): { data: number[] } {
  return { data: vectorFromWasm(v as any) }
}

// Signal/Spectrum
export function signalFromWasm(sig: WasmSignal): SignalDTO {
  return { data: toArray(sig.data()), sample_rate: sig.sample_rate() }
}

export function spectrumFromWasm(spec: WasmSpectrum): SpectrumDTO {
  const inter = spec.data_interleaved()
  return { data: complexArrayFromInterleaved(inter), sample_rate: spec.sample_rate() }
}

// Polynomial / Rational Function
export function polynomialFromWasm(p: PolynomialF64 | ArrayLike<number>): PolynomialDTO {
  const coeffs = Array.isArray(p as any) || 'length' in (p as any) && typeof (p as any).length === 'number'
    ? toArray(p as ArrayLike<number>)
    : toArray((p as PolynomialF64).coeffs())
  return { coeffs }
}

export function rationalFunctionFromWasm(numerator: PolynomialF64 | ArrayLike<number>, denominator: PolynomialF64 | ArrayLike<number>): RationalFunctionDTO {
  return { numerator: polynomialFromWasm(numerator), denominator: polynomialFromWasm(denominator) }
}

// Transfer Function (ContinuousTF / DiscreteTF)
export function transferFunctionFromWasm(tf: ContinuousTF | DiscreteTF): TransferFunctionDTO {
  // wasm: b = numerator, a = denominator
  const num = toArray('b_coeffs' in tf ? tf.b_coeffs() : (tf as any).b_coeffs())
  const den = toArray('a_coeffs' in tf ? tf.a_coeffs() : (tf as any).a_coeffs())
  const fs = (tf as DiscreteTF).sample_rate ? (tf as DiscreteTF).sample_rate() : undefined
  const sample_time = sampleTimeFromRate(fs)
  return { num, den, sample_time }
}

// ZPK (ContinuousZpk / DiscreteZpk)
export function zpkFromWasm(z: ContinuousZpk | DiscreteZpk): ZpkDTO {
  const zeros = toArray(z.zeros_interleaved())
  const poles = toArray(z.poles_interleaved())
  const gain = z.gain()
  const fs = (z as DiscreteZpk).sample_rate ? (z as DiscreteZpk).sample_rate() : undefined
  const sample_time = sampleTimeFromRate(fs)
  return { zeros, poles, gain, sample_time }
}

// Statistics parameter DTOs from wasm distributions
export function normalParamsFromWasm(d: WasmNormal): NormalParams {
  return { mu: d.mu, sigma: d.sigma }
}
export function uniformParamsFromWasm(d: WasmUniform): UniformParams {
  return { a: d.a, b: d.b }
}
export function exponentialParamsFromWasm(d: WasmExponential): ExponentialParams {
  return { lambda: d.lambda }
}
export function gammaParamsFromWasm(d: WasmGamma): GammaParams {
  return { shape: d.shape, rate: d.rate }
}
export function chiSquareParamsFromWasm(d: WasmChiSquare): ChiSquareParams {
  return { k: d.k_param }
}
export function studentTParamsFromWasm(d: WasmStudentT): StudentTParams {
  return { df: d.df }
}
export function fParamsFromWasm(d: WasmF): FParams {
  return { d1: d.d1, d2: d.d2 }
}
export function bernoulliParamsFromWasm(d: WasmBernoulli): BernoulliParams {
  return { p: d.p }
}
export function binomialParamsFromWasm(d: WasmBinomial): BinomialParams {
  return { n: d.n, p: d.p }
}
export function poissonParamsFromWasm(d: WasmPoisson): PoissonParams {
  return { lambda: d.lambda }
}
export function categoricalParamsFromWasm(d: WasmCategorical): CategoricalParams {
  return { probs: toArray(d.probs) }
}
