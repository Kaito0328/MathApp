"use client"
import { getWasm } from './loader'
import type { Matrix, Vector } from '../widgets/dto/linalg'

export type MatrixDTO = Matrix
export type VectorDTO = Vector

export function mulMatVec(A: MatrixDTO, b: VectorDTO): VectorDTO {
  const m = A.rows, n = A.cols
  if (b.data.length !== n) throw new Error('dimension mismatch in mulMatVec')
  const out = new Array(m).fill(0)
  const a = A.data
  for (let i = 0; i < m; i++) {
    let s = 0
    const row = i * n
    for (let j = 0; j < n; j++) s += a[row + j] * b.data[j]
    out[i] = s
  }
  return { data: out }
}

function validateMatrix(A: MatrixDTO): string | null {
  if (!A || typeof A.rows !== 'number' || typeof A.cols !== 'number' || !Array.isArray(A.data)) return '入力不正: 行列データが不正です'
  if (A.rows <= 0 || A.cols <= 0) return 'サイズ不正: 行数・列数は正である必要があります'
  if (A.data.length !== A.rows * A.cols) return `サイズ不正: rows*cols(${A.rows}x${A.cols}=${A.rows * A.cols}) とデータ長(${A.data.length})が一致していません`
  return null
}

export async function inverse(A: MatrixDTO): Promise<MatrixDTO | { error: string }> {
  if (A.rows !== A.cols) return { error: 'サイズ不正: 逆行列は正方行列のみ計算できます' }
  const invalid = validateMatrix(A); if (invalid) return { error: invalid }
  const wasm: any = await getWasm()
  let mat: any
  try { mat = new wasm.MatrixF64(A.rows, A.cols, Float64Array.from(A.data)) } catch { return { error: 'サイズ不正: 行列データとサイズが一致しません' } }
  try {
    const inv = mat.inverse()
    if (!inv) return { error: '逆行列が存在しません（特異行列）' }
    const rows = inv.rows(), cols = inv.cols()
    const data = Array.from(inv.data() as Float64Array)
    inv.free?.()
    return { rows, cols, data }
  } catch {
    return { error: '計算に失敗しました（WASM内部エラー）' }
  } finally {
    mat.free?.()
  }
}

export async function pinv(A: MatrixDTO): Promise<MatrixDTO> {
  const invalid = validateMatrix(A); if (invalid) return { error: invalid } as any
  const wasm: any = await getWasm()
  let mat: any
  try { mat = new wasm.MatrixF64(A.rows, A.cols, Float64Array.from(A.data)) } catch { return { error: 'サイズ不正: 行列データとサイズが一致しません' } as any }
  try {
    const p = mat.pinv()
    const rows = p.rows(), cols = p.cols()
    const data = Array.from(p.data() as Float64Array)
    p.free?.()
    return { rows, cols, data }
  } finally {
    mat.free?.()
  }
}

export async function cholesky(A: MatrixDTO): Promise<MatrixDTO | { error: string }> {
  if (A.rows !== A.cols) return { error: 'サイズ不正: コレスキー分解は正方行列のみ有効です' }
  const invalid = validateMatrix(A); if (invalid) return { error: invalid }
  const wasm: any = await getWasm()
  let mat: any
  try { mat = new wasm.MatrixF64(A.rows, A.cols, Float64Array.from(A.data)) } catch { return { error: 'サイズ不正: 行列データとサイズが一致しません' } }
  try {
    let L: any
    try {
      L = mat.cholesky()
  } catch {
      return { error: '計算に失敗しました（対称正定値でない可能性）' }
    }
    const rows = L.rows(), cols = L.cols()
    const data = Array.from(L.data() as Float64Array)
    L.free?.()
    return { rows, cols, data }
  } catch {
    return { error: '計算に失敗しました（WASM内部エラー）' }
  } finally {
    mat.free?.()
  }
}

export async function solveAuto(A: MatrixDTO, b: VectorDTO): Promise<VectorDTO | { error: string }> {
  const wasm: any = await getWasm()
  if (A.rows !== b.data.length) return { error: 'サイズ不一致: A.rows と len(b) が一致する必要があります' }
  const x = wasm.solveLinearSystem(A.rows, A.cols, Float64Array.from(A.data), Float64Array.from(b.data)) as Float64Array
  return { data: Array.from(x) }
}

// Internal helpers to convert wasm objects to DTOs
function toMatrixDTO(m: any): MatrixDTO | null {
  if (m && typeof m.rows === 'function' && typeof m.cols === 'function' && typeof m.data === 'function') {
    const rows = m.rows(), cols = m.cols()
    const data = Array.from(m.data() as Float64Array)
    m.free?.()
    return { rows, cols, data }
  }
  if (m && typeof m.rows === 'number' && typeof m.cols === 'number' && m.data) {
    const data = Array.isArray(m.data) ? m.data.map((x: any) => Number(x)) : Array.from(m.data as Float64Array)
    return { rows: m.rows, cols: m.cols, data }
  }
  return null
}

function toVectorArray(v: any): number[] | null {
  if (!v) return null
  if (Array.isArray(v)) return v.map((x) => Number(x))
  if (v instanceof Float64Array) return Array.from(v)
  if (typeof v.data === 'function') {
    try { const arr = v.data(); const out = Array.from(arr as Float64Array); v.free?.(); return out } catch { /* ignore */ }
  }
  if (v && v.data && (Array.isArray(v.data) || v.data instanceof Float64Array)) {
    return Array.isArray(v.data) ? v.data.map((x: any) => Number(x)) : Array.from(v.data as Float64Array)
  }
  if (typeof v.length === 'number' && typeof v[0] !== 'undefined') {
    try { return Array.from(v as ArrayLike<number>).map((x) => Number(x)) } catch { return null }
  }
  if (typeof v.len === 'function' && typeof v.at === 'function') {
    // Some vector wrappers expose len()/at(i)
    const n = v.len()
    const out: number[] = []
    for (let i = 0; i < n; i++) out.push(Number(v.at(i)))
    v.free?.()
    return out
  }
  return null
}

export async function qr(A: MatrixDTO): Promise<{ Q?: MatrixDTO; R?: MatrixDTO; error?: string }> {
  const invalid = validateMatrix(A); if (invalid) return { error: invalid }
  const wasm: any = await getWasm()
  let mat: any
  try { mat = new wasm.MatrixF64(A.rows, A.cols, Float64Array.from(A.data)) } catch { return { error: 'サイズ不正: 行列データとサイズが一致しません' } }
  try {
    const res: any = mat.qr_decomposition()
    let q = toMatrixDTO(res?.Q ?? res?.q)
    let r = toMatrixDTO(res?.R ?? res?.r)
    if ((!q || !r) && Array.isArray(res)) {
      q = toMatrixDTO(res[0]) ?? q
      r = toMatrixDTO(res[1]) ?? r
    }
    if (!q || !r) return { error: 'QR の出力形式を解釈できません' }
    return { Q: q, R: r }
  } catch {
    return { error: '計算に失敗しました（WASM内部エラー）' }
  } finally {
    mat.free?.()
  }
}

export async function svd(A: MatrixDTO): Promise<{ U?: MatrixDTO; S?: { data: number[] }; V?: MatrixDTO; error?: string }> {
  const invalid = validateMatrix(A); if (invalid) return { error: invalid }
  const wasm: any = await getWasm()
  let mat: any
  try { mat = new wasm.MatrixF64(A.rows, A.cols, Float64Array.from(A.data)) } catch { return { error: 'サイズ不正: 行列データとサイズが一致しません' } }
  try {
    const res: any = mat.svd()
    let u = toMatrixDTO(res?.U ?? res?.u)
    let v = toMatrixDTO(res?.V ?? res?.v)
  const vt = toMatrixDTO(res?.Vt ?? res?.vt)
    let sArr = toVectorArray(res?.S ?? res?.sigma ?? res?.s)
    if ((!u || !sArr) && Array.isArray(res)) {
      u = toMatrixDTO(res[0]) ?? u
      // S could be vector or diagonal matrix
      const smaybe = res[1]
      sArr = toVectorArray(smaybe) ?? (typeof smaybe?.diagonal === 'function' ? (()=>{ const d = smaybe.diagonal(); const out = toVectorArray(d); d?.free?.(); return out })() : null) ?? sArr
      v = toMatrixDTO(res[2]) ?? v
    }
    if (!v && vt) {
      // transpose Vt to get V using JS helper
      const Vt = vt
      const V = transpose(Vt)
      v = V
    }
    if (!u || !v || !sArr) return { error: 'SVD の出力形式を解釈できません' }
    return { U: u, S: { data: sArr }, V: v }
  } catch {
    return { error: '計算に失敗しました（WASM内部エラー）' }
  } finally {
    mat.free?.()
  }
}

export async function eigen(A: MatrixDTO): Promise<{ Lambda?: { data: number[] }; V?: MatrixDTO; error?: string }> {
  if (A.rows !== A.cols) return { error: 'サイズ不正: 固有分解は正方行列のみ有効です' }
  const invalid = validateMatrix(A); if (invalid) return { error: invalid }
  // Fast path: identity matrix -> eigenvalues all 1, eigenvectors = I
  const n = A.rows
  let isId = true
  for (let i = 0; i < n && isId; i++) {
    for (let j = 0; j < n; j++) {
      const v = Number(A.data[i * n + j] ?? 0)
      const expect = i === j ? 1 : 0
      if (Math.abs(v - expect) > 1e-12) { isId = false; break }
    }
  }
  if (isId) {
    return { Lambda: { data: Array.from({ length: n }, () => 1) }, V: eye(n) }
  }
  const wasm: any = await getWasm()
  let mat: any
  try { mat = new wasm.MatrixF64(A.rows, A.cols, Float64Array.from(A.data)) } catch { return { error: 'サイズ不正: 行列データとサイズが一致しません' } }
  try {
  const res: any = mat.eigen_decomposition()
  // try common keys for eigenvectors and eigenvalues
  let vecs = toMatrixDTO(res?.V ?? res?.vectors ?? res?.eigvecs ?? res?.P ?? res?.eigenvectors)
  let vals = toVectorArray(res?.Lambda ?? res?.lambda ?? res?.values ?? res?.eigvals ?? res?.eigenvalues ?? res?.lambdas)
    // Try diagonal matrix D if values not directly provided
    if (!vals) {
  const Dobj: any = res?.D ?? res?.d ?? res?.diag ?? res?.LambdaDiag
      if (Dobj) {
        try {
          if (typeof Dobj.diagonal === 'function') {
            const dv = Dobj.diagonal()
            vals = toVectorArray(dv)
            dv?.free?.()
          } else {
            const Dm = toMatrixDTO(Dobj)
            if (Dm && Dm.rows === Dm.cols) {
              const d: number[] = []
              for (let i = 0; i < Dm.rows; i++) d.push(Dm.data[i * Dm.cols + i])
              vals = d
            }
          }
        } catch {
          // ignore extraction errors, will fall back to other shapes
        }
      }
    }
  if ((!vecs || !vals) && Array.isArray(res)) {
      // Heuristic: find which looks like vector vs matrix, also allow [V, D]
      const c0 = toVectorArray(res[0]); const c1 = toVectorArray(res[1])
      const m0 = toMatrixDTO(res[0]); const m1 = toMatrixDTO(res[1])
      if (c0 && m1) { vals = c0; vecs = m1 }
      else if (c1 && m0) { vals = c1; vecs = m0 }
      else {
        // try diagonal in array element
        const d0 = toMatrixDTO(res[0]); const d1 = toMatrixDTO(res[1])
        if (!vals && d0 && d0.rows === d0.cols) {
          vals = Array.from({ length: d0.rows }, (_, i) => d0.data[i * d0.cols + i])
          vecs = vecs ?? m1 ?? toMatrixDTO(res[1])
        } else if (!vals && d1 && d1.rows === d1.cols) {
          vals = Array.from({ length: d1.rows }, (_, i) => d1.data[i * d1.cols + i])
          vecs = vecs ?? m0 ?? toMatrixDTO(res[0])
        }
      }
    }
    // as last resort: if res looks like {D: matrix, V: matrix}
    if ((!vecs || !vals) && res && typeof res === 'object') {
      const Dm = toMatrixDTO((res as any).D ?? (res as any).d)
      const Vm = toMatrixDTO((res as any).V ?? (res as any).vectors ?? (res as any).eigvecs ?? (res as any).P)
      if (Dm && Dm.rows === Dm.cols) {
        vals = vals ?? Array.from({ length: Dm.rows }, (_, i) => Dm.data[i * Dm.cols + i])
        vecs = vecs ?? Vm ?? null as any
      }
    }
    if (!vecs || !vals) return { error: '固有分解の出力形式を解釈できません' }
    return { Lambda: { data: vals }, V: vecs }
  } catch {
    return { error: '計算に失敗しました（WASM内部エラー）' }
  } finally {
    mat.free?.()
  }
}

export async function solveWith(method: 'auto' | 'inverse' | 'pinv' | 'lu' | 'qr' | 'svd' | 'cholesky', A: MatrixDTO, b: VectorDTO): Promise<VectorDTO | { error: string }> {
  if (A.rows !== b.data.length) return { error: 'サイズ不一致: A.rows と len(b) が一致する必要があります' }
  try {
    switch (method) {
      case 'auto':
        return await solveAuto(A, b)
      case 'inverse': {
        const invA = await inverse(A)
        if ('error' in invA) return invA
        return mulMatVec(invA, b)
      }
      case 'pinv': {
        const p = await pinv(A)
        return mulMatVec(p, b)
      }
      case 'cholesky':
      case 'lu':
      case 'qr':
      case 'svd':
        // ひとまず安定な一般解法に委譲（今後、各分解ベースの解法に差し替え可能）
        return await solveAuto(A, b)
      default:
        return { error: '未対応' }
    }
  } catch {
    return { error: '計算中にエラーが発生しました' }
  }
}

// Scalars and other matrix ops
export async function determinant(A: MatrixDTO): Promise<number | { error: string }> {
  if (A.rows !== A.cols) return { error: 'サイズ不正: 正方行列のみ計算できます' }
  const invalid = validateMatrix(A); if (invalid) return { error: invalid }
  const wasm: any = await getWasm()
  let mat: any
  try { mat = new wasm.MatrixF64(A.rows, A.cols, Float64Array.from(A.data)) } catch { return { error: 'サイズ不正: 行列データとサイズが一致しません' } }
  try {
    return Number(mat.determinant())
  } catch {
    return { error: '計算に失敗しました（WASM内部エラー）' }
  } finally { mat.free?.() }
}

export async function rank(A: MatrixDTO): Promise<number | { error: string }> {
  const invalid = validateMatrix(A); if (invalid) return { error: invalid }
  const wasm: any = await getWasm()
  let mat: any
  try { mat = new wasm.MatrixF64(A.rows, A.cols, Float64Array.from(A.data)) } catch { return { error: 'サイズ不正: 行列データとサイズが一致しません' } }
  try {
    return Number(mat.rank())
  } catch {
    return { error: '計算に失敗しました（WASM内部エラー）' }
  } finally { mat.free?.() }
}

export async function frobeniusNorm(A: MatrixDTO): Promise<number | { error: string }> {
  const invalid = validateMatrix(A); if (invalid) return { error: invalid }
  const wasm: any = await getWasm()
  let mat: any
  try { mat = new wasm.MatrixF64(A.rows, A.cols, Float64Array.from(A.data)) } catch { return { error: 'サイズ不正: 行列データとサイズが一致しません' } }
  try {
    return Number(mat.frobenius_norm())
  } catch {
    return { error: '計算に失敗しました（WASM内部エラー）' }
  } finally { mat.free?.() }
}

export async function expm(A: MatrixDTO): Promise<MatrixDTO | { error: string }> {
  if (A.rows !== A.cols) return { error: 'サイズ不正: 正方行列のみ計算できます' }
  const invalid = validateMatrix(A); if (invalid) return { error: invalid }
  const wasm: any = await getWasm()
  let mat: any
  try { mat = new wasm.MatrixF64(A.rows, A.cols, Float64Array.from(A.data)) } catch { return { error: 'サイズ不正: 行列データとサイズが一致しません' } }
  try {
    const e = mat.expm()
    const rows = e.rows(), cols = e.cols()
    const data = Array.from(e.data() as Float64Array)
    e.free?.()
    return { rows, cols, data }
  } catch {
    return { error: '計算に失敗しました（WASM内部エラー）' }
  } finally { mat.free?.() }
}

// JS helpers for verification
export function matMul(A: MatrixDTO, B: MatrixDTO): MatrixDTO | { error: string } {
  if (A.cols !== B.rows) return { error: 'サイズ不一致: (A.cols != B.rows)' }
  const m = A.rows, n = A.cols, p = B.cols
  const C = new Array(m * p).fill(0)
  for (let i = 0; i < m; i++) {
    for (let k = 0; k < n; k++) {
      const aik = A.data[i * n + k]
      const row = i * p
      const bk = k * p
      for (let j = 0; j < p; j++) C[row + j] += aik * B.data[bk + j]
    }
  }
  return { rows: m, cols: p, data: C }
}

export function transpose(A: MatrixDTO): MatrixDTO {
  const out = new Array(A.rows * A.cols).fill(0)
  for (let i = 0; i < A.rows; i++) for (let j = 0; j < A.cols; j++) out[j * A.rows + i] = A.data[i * A.cols + j]
  return { rows: A.cols, cols: A.rows, data: out }
}

export function eye(n: number): MatrixDTO { return { rows: n, cols: n, data: Array.from({ length: n * n }, (_, i) => (Math.floor(i / n) === (i % n) ? 1 : 0)) } }

export function diagFrom(v: number[]): MatrixDTO { const n = v.length; const data = new Array(n * n).fill(0); for (let i = 0; i < n; i++) data[i * n + i] = v[i]; return { rows: n, cols: n, data } }
