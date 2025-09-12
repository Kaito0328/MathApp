"use client"
import { getWasm } from './loader'

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
  return Array.from(arr)
}

export async function cm_risingFactorialPoly(m: number): Promise<number[]> {
  const wasm: any = await getWasm()
  const arr: Float64Array = wasm.risingFactorialPoly(m)
  return Array.from(arr)
}

export async function cm_shiftPolyXPlusH(coeffs: number[], h: number): Promise<number[]> {
  const wasm: any = await getWasm()
  const out: Float64Array = wasm.shiftPolyXPlusH(Float64Array.from(coeffs), h)
  return Array.from(out)
}

export async function cm_binomXPlusKChooseKPoly(k: number): Promise<number[]> {
  const wasm: any = await getWasm()
  const out: Float64Array = wasm.binomXPlusKChooseKPoly(k)
  return Array.from(out)
}

export async function cm_discreteDiff(coeffs: number[]): Promise<number[]> {
  const wasm: any = await getWasm()
  const out: Float64Array = wasm.discreteDiff(Float64Array.from(coeffs))
  return Array.from(out)
}

export async function cm_discreteSum(coeffs: number[]): Promise<number[]> {
  const wasm: any = await getWasm()
  const out: Float64Array = wasm.discreteSum(Float64Array.from(coeffs))
  return Array.from(out)
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
