"use client"
import { getWasm } from './loader'

// Helpers to map between real-coefficient arrays and WASM complex-interleaved flat arrays
function toWasmFlatReal(coeffs: number[]): Float64Array {
  const out: number[] = []
  for (let i = 0; i < coeffs.length; i++) { out.push(Number(coeffs[i] ?? 0), 0) }
  return Float64Array.from(out)
}

function fromWasmFlatToReal(arr: Float64Array): number[] {
  const n = Math.floor((arr?.length ?? 0) / 2)
  const out: number[] = new Array(n)
  for (let i = 0; i < n; i++) {
    const re = Number(arr[2 * i] ?? 0)
    // const im = Number(arr[2 * i + 1] ?? 0) // ignore, expected ~0 for these wrappers
    out[i] = re
  }
  // trim trailing ~0s to keep degree minimal
  while (out.length > 1 && Math.abs(out[out.length - 1]) < 1e-12) out.pop()
  return out
}

export async function cm_binom(n: number, k: number): Promise<number> {
  const wasm: any = await getWasm()
  return Number(wasm.binom(n, k))
}

export async function cm_stirling2(n: number, k: number): Promise<number> {
  const wasm: any = await getWasm()
  return Number(wasm.stirling2(n, k))
}

export async function cm_fallingFactorialPoly(m: number): Promise<number[]> {
  const wasm: any = await getWasm()
  const arr: Float64Array = wasm.fallingFactorialPoly(m)
  return fromWasmFlatToReal(arr)
}

export async function cm_risingFactorialPoly(m: number): Promise<number[]> {
  const wasm: any = await getWasm()
  const arr: Float64Array = wasm.risingFactorialPoly(m)
  return fromWasmFlatToReal(arr)
}

export async function cm_shiftPolyXPlusH(coeffs: number[], h: number): Promise<number[]> {
  const wasm: any = await getWasm()
  const out: Float64Array = wasm.shiftPolyXPlusH(toWasmFlatReal(coeffs), h)
  return fromWasmFlatToReal(out)
}

export async function cm_binomXPlusKChooseKPoly(k: number): Promise<number[]> {
  const wasm: any = await getWasm()
  const out: Float64Array = wasm.binomXPlusKChooseKPoly(k)
  return fromWasmFlatToReal(out)
}

export async function cm_discreteDiff(coeffs: number[]): Promise<number[]> {
  const wasm: any = await getWasm()
  const out: Float64Array = wasm.discreteDiff(toWasmFlatReal(coeffs))
  return fromWasmFlatToReal(out)
}

export async function cm_discreteSum(coeffs: number[]): Promise<number[]> {
  const wasm: any = await getWasm()
  const out: Float64Array = wasm.discreteSum(toWasmFlatReal(coeffs))
  return fromWasmFlatToReal(out)
}

// ClosedForm minimal wrapper helpers
export type ClosedForm = any

export async function cm_solveRecurrence_homogeneous(coeffs: number[], initial_values: number[]): Promise<ClosedForm> {
  const wasm: any = await getWasm()
  const nh_polys_flat = new Float64Array(0)
  const nh_offsets = new Uint32Array(0)
  const nh_bases = new Float64Array(0)
  return wasm.solveRecurrence(
    Float64Array.from(coeffs),
    nh_polys_flat,
    nh_offsets,
    nh_bases,
    Float64Array.from(initial_values)
  )
}

export async function cm_closedFormToString(cf: ClosedForm, variable = 'n', unicodeSuperscript = true): Promise<string> {
  // .toString may allocate, but we simply forward it
  return String(cf.toString?.(variable, unicodeSuperscript) ?? '')
}

export async function cm_partialSum(cf: ClosedForm): Promise<ClosedForm> {
  const wasm: any = await getWasm()
  // prefer instance method if exists; fallback to free function
  if (typeof cf.partialSum === 'function') return cf.partialSum()
  return wasm.partialSum(cf)
}
