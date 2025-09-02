// Minimal ESM bridge for initializing the wasm package once and reusing it
// Adapted to use our core loader (src/wasm/core/loader)

import { getWasm as load } from './core/loader'

export async function getWasm() {
  return load()
}

// Convenience helpers (optional, mirrored from docs)
export async function solveLinearSystem(
  rows: number,
  cols: number,
  a: Float64Array,
  b: Float64Array,
): Promise<Float64Array> {
  const wasm = await getWasm() as any
  return wasm.solveLinearSystem(rows, cols, a, b)
}

export type GmmParamsPacked = Float64Array // [k, d, weights(k), means(k*d), covs(k*d*d)]
export interface GmmParams {
  k: number
  d: number
  weights: Float64Array // length k
  means: Float64Array   // length k*d, row-major: m0..m_{k-1}
  covs: Float64Array    // length k*d*d, blocks of dxd
}

export function unpackGmmParams(packed: GmmParamsPacked): GmmParams {
  const k = packed[0] | 0
  const d = packed[1] | 0
  const offW = 2
  const offM = offW + k
  const offC = offM + k * d
  const end = offC + k * d * d
  if (end !== packed.length) throw new Error('Invalid GMM packed length')
  return {
    k,
    d,
    weights: packed.slice(offW, offM),
    means: packed.slice(offM, offC),
    covs: packed.slice(offC, end),
  }
}
