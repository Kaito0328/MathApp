// High-level, frontend-friendly API wrapping the low-level wasm exports
// Adapted from backend/docs to local paths

import { getWasm } from './bridge'

// ---------- small utils ----------
const f64 = (arr: number[] | Float64Array) =>
  arr instanceof Float64Array ? arr : new Float64Array(arr)

const flatten2D = (m: number[][]): Float64Array => {
  const rows = m.length
  const cols = rows ? m[0].length : 0
  const out = new Float64Array(rows * cols)
  let p = 0
  for (let i = 0; i < rows; i++) {
    const row = m[i]
    if (row.length !== cols) throw new Error('Non-rectangular matrix')
    out.set(row, p)
    p += cols
  }
  return out
}

export const toMatrix = (flat: Float64Array, rows: number, cols: number): number[][] => {
  if (flat.length !== rows * cols) throw new Error('Dimension mismatch')
  const out: number[][] = new Array(rows)
  for (let i = 0; i < rows; i++) {
    const start = i * cols
    out[i] = Array.from(flat.subarray(start, start + cols))
  }
  return out
}

// ---------- linear algebra ----------
export async function solveLinearSystem2D(A: number[][], b: number[]): Promise<number[]> {
  const wasm = await getWasm() as any
  const rows = A.length
  const cols = rows ? A[0].length : 0
  const x = wasm.solveLinearSystem(rows, cols, flatten2D(A), f64(b))
  return Array.from(x)
}

// ---------- statistics models ----------
export interface LogisticFitOptions { lr?: number; maxIter?: number }
export interface LogisticModel { intercept: number; weights: number[] }

export async function logisticFit2D(X: number[][], y: number[], opts: LogisticFitOptions = {}): Promise<LogisticModel> {
  const { lr = 0.1, maxIter = 200 } = opts
  const wasm = await getWasm() as any
  const rows = X.length
  const cols = rows ? X[0].length : 0
  const coeffs = wasm.logisticFit(rows, cols, flatten2D(X), f64(y), lr, maxIter)
  const arr = Array.from(coeffs) as number[]
  return { intercept: arr[0] ?? 0, weights: arr.slice(1) }
}

export async function logisticPredictProbaFromModel(model: LogisticModel, x: number[]): Promise<number> {
  const wasm = await getWasm() as any
  const cols = model.weights.length
  const coeffs = new Float64Array(1 + cols)
  coeffs[0] = model.intercept
  coeffs.set(model.weights, 1)
  return wasm.logisticPredictProba(cols, coeffs, f64(x))
}

// ---------- Gaussian Mixture Model ----------
export type GmmParamsPacked = Float64Array // [k, d, weights(k), means(k*d), covs(k*d*d)]
export interface GmmParams { k: number; d: number; weights: number[]; means: number[][]; covs: number[][][] }

export function unpackGmmParamsHigh(packed: GmmParamsPacked): GmmParams {
  const k = packed[0] | 0; const d = packed[1] | 0
  const offW = 2, offM = offW + k, offC = offM + k * d, end = offC + k * d * d
  if (packed.length !== end) throw new Error('Invalid GMM packed length')
  const weights = Array.from(packed.subarray(offW, offM))
  const means: number[][] = new Array(k)
  for (let i = 0; i < k; i++) {
    const s = offM + i * d; means[i] = Array.from(packed.subarray(s, s + d))
  }
  const covs: number[][][] = new Array(k)
  for (let i = 0; i < k; i++) {
    const blockStart = offC + i * d * d; const flat = packed.subarray(blockStart, blockStart + d * d)
    covs[i] = toMatrix(flat, d, d)
  }
  return { k, d, weights, means, covs }
}

export async function gmmFit2D(data: number[][], k: number, maxIter = 100, tol = 1e-6): Promise<{ packed: GmmParamsPacked; params: GmmParams }>{
  const wasm = await getWasm() as any
  const n = data.length; const d = n ? data[0].length : 0
  const packed = wasm.gmmFit(n, d, flatten2D(data), k, maxIter, tol)
  return { packed, params: unpackGmmParamsHigh(packed) }
}

export async function gmmPredictProbaFromPacked(packed: GmmParamsPacked, x: number[]): Promise<number[]> {
  const wasm = await getWasm() as any
  const d = packed[1] | 0
  const out = wasm.gmmPredictProba(d, packed, f64(x))
  return Array.from(out) as number[]
}

// ---------- Bayesian linear regression (posterior) ----------
export async function bayesianLinearPosterior2D(
  X: number[][],
  y: number[],
  priorMean: number[],
  priorCov: number[][],
  noiseCov: number[][],
): Promise<{ mean: number[]; cov: number[][] }>{
  const wasm = await getWasm() as any
  const rows = X.length; const cols = rows ? X[0].length : 0
  const out = wasm.bayesianLinearPosterior(
    rows, cols,
    flatten2D(X), f64(y),
    f64(priorMean), flatten2D(priorCov), flatten2D(noiseCov),
  )
  const mean = Array.from(out.subarray(0, cols)) as number[]
  const cov = toMatrix(out.subarray(cols), cols, cols)
  return { mean, cov }
}

// ---------- Kalman filter (stateless helpers) ----------
export async function kalmanPredict(
  F: number[][],
  Q: number[][],
  x: number[],
  P: number[][],
): Promise<{ x: number[]; P: number[][] }>{
  const wasm = await getWasm() as any
  const n = x.length
  const out = wasm.kalmanPredict(n, flatten2D(F), flatten2D(Q), f64(x), flatten2D(P))
  const xNew = Array.from(out.subarray(0, n)) as number[]
  const PNew = toMatrix(out.subarray(n), n, n)
  return { x: xNew, P: PNew }
}

export async function kalmanUpdate(
  H: number[][],
  R: number[][],
  z: number[],
  x: number[],
  P: number[][],
): Promise<{ x: number[]; P: number[][] }>{
  const wasm = await getWasm() as any
  const n = x.length; const m = z.length
  if (H.length !== m || (H[0]?.length ?? 0) !== n) throw new Error('H must be m x n')
  const out = wasm.kalmanUpdate(n, flatten2D(H), flatten2D(R), f64(z), f64(x), flatten2D(P))
  const xNew = Array.from(out.subarray(0, n)) as number[]
  const PNew = toMatrix(out.subarray(n), n, n)
  return { x: xNew, P: PNew }
}

// ---------- Statistics distributions (quick access) ----------
export async function createNormal(mu: number, sigma: number) {
  const wasm = await getWasm() as any
  const inst = new wasm.WasmNormal(mu, sigma)
  return {
    mean: () => inst.mean(),
    variance: () => inst.variance(),
    stdDev: () => inst.std_dev(),
    pdf: (x: number) => inst.pdf(x),
    cdf: (x: number) => inst.cdf(x),
    quantile: (p: number) => inst.quantile(p),
    pdfSvg: (w: number, h: number, samples = 200) => inst.pdf_svg(w, h, samples),
  }
}

export async function createUniform(a: number, b: number) {
  const wasm = await getWasm() as any
  const inst = new wasm.WasmUniform(a, b)
  return {
    mean: () => inst.mean(),
    variance: () => inst.variance(),
    stdDev: () => inst.std_dev(),
    pdf: (x: number) => inst.pdf(x),
    cdf: (x: number) => inst.cdf(x),
    quantile: (p: number) => inst.quantile(p),
    pdfSvg: (w: number, h: number, samples = 200) => inst.pdf_svg(w, h, samples),
  }
}

export async function createExponential(lambda: number) {
  const wasm = await getWasm() as any
  const inst = new wasm.WasmExponential(lambda)
  return {
    mean: () => inst.mean(),
    variance: () => inst.variance(),
    stdDev: () => inst.std_dev(),
    pdf: (x: number) => inst.pdf(x),
    cdf: (x: number) => inst.cdf(x),
    quantile: (p: number) => inst.quantile(p),
    pdfSvg: (w: number, h: number, samples = 200) => inst.pdf_svg(w, h, samples),
  }
}

export async function createGamma(shape: number, rate: number) {
  const wasm = await getWasm() as any
  const inst = new wasm.WasmGamma(shape, rate)
  return {
    mean: () => inst.mean(),
    variance: () => inst.variance(),
    stdDev: () => inst.std_dev(),
    pdf: (x: number) => inst.pdf(x),
    cdf: (x: number) => inst.cdf(x),
    quantile: (p: number) => inst.quantile(p),
    pdfSvg: (w: number, h: number, samples = 200) => inst.pdf_svg(w, h, samples),
  }
}

export async function createChiSquare(k: number) {
  const wasm = await getWasm() as any
  const inst = new wasm.WasmChiSquare(k)
  return {
    mean: () => inst.mean(),
    variance: () => inst.variance(),
    stdDev: () => inst.std_dev(),
    pdf: (x: number) => inst.pdf(x),
    cdf: (x: number) => inst.cdf(x),
    quantile: (p: number) => inst.quantile(p),
    pdfSvg: (w: number, h: number, samples = 200) => inst.pdf_svg(w, h, samples),
  }
}

export async function createStudentT(df: number) {
  const wasm = await getWasm() as any
  const inst = new wasm.WasmStudentT(df)
  return {
    mean: () => inst.mean(),
    variance: () => inst.variance(),
    stdDev: () => inst.std_dev(),
    pdf: (x: number) => inst.pdf(x),
    cdf: (x: number) => inst.cdf(x),
    quantile: (p: number) => inst.quantile(p),
    pdfSvg: (w: number, h: number, samples = 200) => inst.pdf_svg(w, h, samples),
  }
}

export async function createF(d1: number, d2: number) {
  const wasm = await getWasm() as any
  const inst = new wasm.WasmF(d1, d2)
  return {
    mean: () => inst.mean(),
    variance: () => inst.variance(),
    stdDev: () => inst.std_dev(),
    pdf: (x: number) => inst.pdf(x),
    cdf: (x: number) => inst.cdf(x),
    quantile: (p: number) => inst.quantile(p),
    pdfSvg: (w: number, h: number, samples = 200) => inst.pdf_svg(w, h, samples),
  }
}

export async function createBernoulli(p: number) {
  const wasm = await getWasm() as any
  const inst = new wasm.WasmBernoulli(p)
  return {
    mean: () => inst.mean(),
    variance: () => inst.variance(),
    stdDev: () => inst.std_dev(),
    pmf: (k: number) => inst.pmf(k),
    cdf: (k: number) => inst.cdf(k),
    quantile: (q: number) => inst.quantile(q),
    pmfSvg: (w: number, h: number) => inst.pmf_svg(w, h),
  }
}

export async function createBinomial(n: number, p: number) {
  const wasm = await getWasm() as any
  const inst = new wasm.WasmBinomial(n, p)
  return {
    mean: () => inst.mean(),
    variance: () => inst.variance(),
    stdDev: () => inst.std_dev(),
    pmf: (k: number) => inst.pmf(k),
    cdf: (k: number) => inst.cdf(k),
    quantile: (q: number) => inst.quantile(q),
    pmfSvg: (w: number, h: number) => inst.pmf_svg(w, h),
  }
}

export async function createPoisson(lambda: number) {
  const wasm = await getWasm() as any
  const inst = new wasm.WasmPoisson(lambda)
  return {
    mean: () => inst.mean(),
    variance: () => inst.variance(),
    stdDev: () => inst.std_dev(),
    pmf: (k: number) => inst.pmf(k),
    cdf: (k: number) => inst.cdf(k),
    quantile: (q: number) => inst.quantile(q),
    pmfSvg: (w: number, h: number) => inst.pmf_svg(w, h),
  }
}

export async function createCategorical(probs: number[]) {
  const wasm = await getWasm() as any
  const inst = new wasm.WasmCategorical(new Float64Array(probs))
  return {
    pmf: (k: number) => inst.pmf(k),
    cdf: (k: number) => inst.cdf(k),
    quantile: (q: number) => inst.quantile(q),
    pmfSvg: (w: number, h: number) => inst.pmf_svg(w, h),
  }
}
