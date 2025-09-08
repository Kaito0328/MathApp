"use client"
import { getWasm } from './loader'

export type PolyDTO = { coeffs: number[] }

function toDTO(p: any): PolyDTO {
  if (!p) return { coeffs: [] }
  try {
    const arr = (typeof p.coeffs === 'function') ? p.coeffs() : p.coeffs
    const out = Array.isArray(arr) ? arr.map(Number) : Array.from(arr as Float64Array)
    p.free?.()
    return { coeffs: out }
  } catch {
    // last resort try to access .coeffs as array-like
    const arr = (p as any).coeffs
    return { coeffs: Array.isArray(arr) ? arr.map(Number) : (arr ? Array.from(arr as Float64Array) : []) }
  }
}

export async function polyNormalize(coeffs: number[]): Promise<PolyDTO> {
  const wasm: any = await getWasm()
  const poly = new wasm.PolynomialF64(Float64Array.from(coeffs))
  return toDTO(poly)
}

export async function polyAdd(a: number[], b: number[]): Promise<PolyDTO> {
  const wasm: any = await getWasm()
  const pa = new wasm.PolynomialF64(Float64Array.from(a))
  const pb = new wasm.PolynomialF64(Float64Array.from(b))
  const pc = pa.add(pb)
  pa.free?.(); pb.free?.()
  return toDTO(pc)
}

export async function polySub(a: number[], b: number[]): Promise<PolyDTO> {
  const wasm: any = await getWasm()
  const pa = new wasm.PolynomialF64(Float64Array.from(a))
  const pb = new wasm.PolynomialF64(Float64Array.from(b))
  const pc = pa.sub(pb)
  pa.free?.(); pb.free?.()
  return toDTO(pc)
}

export async function polyMul(a: number[], b: number[]): Promise<PolyDTO> {
  const wasm: any = await getWasm()
  const pa = new wasm.PolynomialF64(Float64Array.from(a))
  const pb = new wasm.PolynomialF64(Float64Array.from(b))
  const pc = pa.mulAuto ? pa.mulAuto(pb) : pa.mul(pb)
  pa.free?.(); pb.free?.()
  return toDTO(pc)
}

export async function polyDiv(a: number[], b: number[]): Promise<PolyDTO> {
  const wasm: any = await getWasm()
  const pa = new wasm.PolynomialF64(Float64Array.from(a))
  const pb = new wasm.PolynomialF64(Float64Array.from(b))
  const pc = pa.div(pb)
  pa.free?.(); pb.free?.()
  return toDTO(pc)
}

export async function polyDivRem(a: number[], b: number[]): Promise<{ q: PolyDTO; r: PolyDTO }> {
  const wasm: any = await getWasm()
  const pa = new wasm.PolynomialF64(Float64Array.from(a))
  const pb = new wasm.PolynomialF64(Float64Array.from(b))
  const [q, r] = pa.divRem(pb) as any[]
  pa.free?.(); pb.free?.()
  return { q: toDTO(q), r: toDTO(r) }
}

export async function polyGcd(a: number[], b: number[]): Promise<PolyDTO> {
  const wasm: any = await getWasm()
  const pa = new wasm.PolynomialF64(Float64Array.from(a))
  const pb = new wasm.PolynomialF64(Float64Array.from(b))
  const g = wasm.PolynomialF64.gcd(pa, pb)
  pa.free?.(); pb.free?.()
  return toDTO(g)
}

export async function polyLcm(a: number[], b: number[]): Promise<PolyDTO> {
  const wasm: any = await getWasm()
  const pa = new wasm.PolynomialF64(Float64Array.from(a))
  const pb = new wasm.PolynomialF64(Float64Array.from(b))
  const g = wasm.PolynomialF64.lcm(pa, pb)
  pa.free?.(); pb.free?.()
  return toDTO(g)
}

export async function polyDiff(a: number[]): Promise<PolyDTO> {
  const wasm: any = await getWasm()
  const pa = new wasm.PolynomialF64(Float64Array.from(a))
  const d = pa.differentiate()
  pa.free?.()
  return toDTO(d)
}

export async function polyInt(a: number[]): Promise<PolyDTO> {
  const wasm: any = await getWasm()
  const pa = new wasm.PolynomialF64(Float64Array.from(a))
  const d = pa.integrate()
  pa.free?.()
  return toDTO(d)
}
