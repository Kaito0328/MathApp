// Interop helpers for working with wasm-bindgen generated API (src/wasm-pkg)
// - Array/TypedArray normalization
// - 2D matrix flatten/unflatten
// - BigInt coercion for finite fields
// - Thin wrappers for a few common classes

import { getWasm } from './core/loader'

// ---------- basic coercions ----------
export type NumArray = number[] | ReadonlyArray<number> | Float64Array | Float32Array | Int32Array | Uint8Array

export const toF64 = (arr: NumArray): Float64Array =>
  arr instanceof Float64Array ? arr : new Float64Array(arr as ArrayLike<number>)

export const toU8 = (arr: ArrayBuffer | ArrayLike<number>): Uint8Array =>
  arr instanceof Uint8Array ? arr : new Uint8Array(arr as ArrayLike<number>)

export const toI32 = (arr: NumArray): Int32Array =>
  arr instanceof Int32Array ? arr : new Int32Array(arr as ArrayLike<number>)

export const toBigInt = (v: number | string | bigint): bigint => {
  if (typeof v === 'bigint') return v
  if (typeof v === 'number') return BigInt(v)
  // allow hex like '0xff', decimal, or bigint-like strings
  return BigInt(v)
}

// ---------- 2D helpers ----------
export const flatten2D = (m: number[][]): Float64Array => {
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

export const unflatten2D = (flat: Float64Array | number[], rows: number, cols: number): number[][] => {
  const f = toF64(flat as any)
  if (f.length !== rows * cols) throw new Error('Dimension mismatch')
  const out: number[][] = new Array(rows)
  for (let i = 0; i < rows; i++) {
    const s = i * cols
    out[i] = Array.from(f.subarray(s, s + cols))
  }
  return out
}

// ---------- tuple helpers (some bindings return [val, err_ptr, err_len]) ----------
export const first = <T>(t: [T, number, number]): T => t[0]
export const ok = <T>(t: [T, number, number]): T => {
  // wasm-bindgen pattern: [value, err_ptr, err_len]
  const [, errPtr, errLen] = t
  if (errPtr && errLen) throw new Error('WASM error (see console)')
  return t[0]
}

// ---------- thin wrappers ----------
export class DirectMatrix {
  private inner: any
  readonly rows: number
  readonly cols: number

  private constructor(inner: any, rows: number, cols: number) {
    this.inner = inner
    this.rows = rows
    this.cols = cols
  }

  static async from2D(m: number[][]) {
    const wasm = await getWasm() as any
    const rows = m.length; const cols = rows ? m[0].length : 0
    const data = flatten2D(m)
    const obj = new wasm.TestMatrixF64Direct(rows, cols, data)
    return new DirectMatrix(obj, rows, cols)
  }

  transpose(): DirectMatrix {
    const t = this.inner.transpose()
    return new DirectMatrix(t, this.cols, this.rows)
  }

  free() { this.inner?.free?.(); this.inner = null }
}

export const GF = {
  async gf2(v: number | string | bigint) {
    const wasm = await getWasm() as any
    return new wasm.WasmGF2(toBigInt(v))
  },
  async gf3(v: number | string | bigint) {
    const wasm = await getWasm() as any
    return new wasm.WasmGF3(toBigInt(v))
  },
  async gf256(v: number) {
    const wasm = await getWasm() as any
    return new wasm.WasmGF256(v & 0xff)
  },
  async gf256FromCoeffs(coeffs: ArrayLike<number>) {
    const wasm = await getWasm() as any
    return wasm.WasmGF256.fromCoeffs(toU8(coeffs))
  },
}

// ---------- DTO wrappers ----------
export interface MatrixDTO { rows: number; cols: number; data: Float64Array }

export async function newTestMatrixFromDTO(dto: MatrixDTO) {
  const wasm = await getWasm() as any
  return new wasm.TestMatrixF64Direct(dto.rows, dto.cols, toF64(dto.data))
}

export function testMatrixToDTO(inst: any): MatrixDTO {
  // TestMatrixF64Direct: expose rows/cols getters
  const rows = inst.get_rows() as number
  const cols = inst.get_cols() as number
  // No direct data accessor in bindings; return empty data placeholder
  return { rows, cols, data: new Float64Array(rows * cols) }
}

