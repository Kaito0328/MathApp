// Pure formatting helpers to produce LaTeX-friendly strings for Markdown math blocks
// Consumers should wrap with $...$ or $$...$$ when rendering via remark-math/rehype-katex

// Structural type to avoid importing wasm types here; prefer DTOs used by widgets

import type { Vector, Matrix } from '../../widgets/dto/linalg'
import type { Signal, Spectrum } from '../../widgets/dto/signal_processing'
import type { RationalFunction } from '../../widgets/dto/polynomial'
import type { TransferFunction, Zpk } from '../../widgets/dto/lti-systems'
import type { Complex } from '../../widgets/dto/complex'

type PolyLike = { coeffs(): ArrayLike<number> }

// Overloads: accept either a Polynomial-like object or coefficient array-like
export function formatPolynomialMarkdown(polynomial: PolyLike, varName?: string, opts?: { trimZeros?: boolean }): string
export function formatPolynomialMarkdown(coeffs: ArrayLike<number>, varName?: string, opts?: { trimZeros?: boolean }): string
export function formatPolynomialMarkdown(arg: PolyLike | ArrayLike<number>, varName = 'x', opts?: { trimZeros?: boolean }): string {
  const coeffs: ArrayLike<number> = typeof (arg as PolyLike).coeffs === 'function' ? (arg as PolyLike).coeffs() : (arg as ArrayLike<number>)
  const trimZeros = opts?.trimZeros ?? true
  const n = coeffs.length
  if (n === 0) return '0'
  const deg = n - 1
  const terms: string[] = []
  for (let i = deg; i >= 0; i--) {
    const c = Number((coeffs as any)[i] ?? 0)
    if (trimZeros && c === 0) continue
    const isConst = i === 0
    const sign = c < 0 ? '-' : (terms.length ? '+' : '')
    const abs = Math.abs(c)
    const coeff = isConst ? `${abs}` : (abs === 1 ? '' : `${abs}`)
    const pow = !isConst ? (i === 1 ? varName : `${varName}^${i}`) : ''
    const piece = [sign, coeff, pow].join('').trim()
    terms.push(piece)
  }
  if (terms.length === 0) return '0'
  return terms.join(' ')
}

function fmtNumber(n: number, precision?: number): string {
  if (Number.isInteger(n)) return String(n)
  if (typeof precision === 'number') return n.toFixed(precision)
  // default: minimal string without trailing zeros for typical decimals
  const s = String(n)
  return s
}

function toArray(a: ArrayLike<number>): number[] {
  return Array.from({ length: a.length }, (_, i) => Number((a as any)[i]))
}

function buildArrayColSpec(n: number): string {
  // KaTeX-safe: only l/c/r and spaces
  return Array.from({ length: Math.max(1, n) }, () => 'c').join(' ')
}

function fmtComplex(re: number, im: number, precision?: number, imag: 'i' | 'j' = 'i'): string {
  if (!im) return fmtNumber(re, precision)
  const sign = im < 0 ? '-' : '+'
  const absIm = Math.abs(im)
  const imPart = absIm === 1 ? `${imag}` : `${fmtNumber(absIm, precision)}${imag}`
  if (!re) {
    return sign === '-' ? `-${imPart}` : imPart
  }
  return `${fmtNumber(re, precision)} ${sign} ${imPart}`
}

// Overloads to accept DTOs
export function formatVectorMarkdown(vec: Vector, opts?: { orientation?: 'row' | 'col'; precision?: number; paren?: boolean }): string
export function formatVectorMarkdown(values: ArrayLike<number>, opts?: { orientation?: 'row' | 'col'; precision?: number; paren?: boolean }): string
export function formatVectorMarkdown(valuesOrVector: ArrayLike<number> | Vector, opts?: { orientation?: 'row' | 'col'; precision?: number; paren?: boolean }): string {
  const orientation = opts?.orientation ?? 'row'
  const p = opts?.precision
  const paren = opts?.paren ?? false
  const nums = toArray((valuesOrVector as any).data ? (valuesOrVector as any).data : (valuesOrVector as ArrayLike<number>))
  if (orientation === 'row') {
    const spec = buildArrayColSpec(nums.length)
  const arr = `\\begin{array}{${spec}} ${nums.map((v) => fmtNumber(v, p)).join(' \\!&\\! ')} \\end{array}`
    return paren ? `\\left[ ${arr} \\right]` : arr
  } else {
    const spec = buildArrayColSpec(1)
  const arr = `\\begin{array}{${spec}} ${nums.map((v) => fmtNumber(v, p)).join(' \\\\ ')} \\end{array}`
    return paren ? `\\left[ ${arr} \\right]` : arr
  }
}

export function formatMatrixMarkdown(matrix: Matrix, opts?: { precision?: number; paren?: boolean }): string
export function formatMatrixMarkdown(rows: number, cols: number, data: ArrayLike<number>, opts?: { precision?: number; paren?: boolean }): string
export function formatMatrixMarkdown(a: number | Matrix, b?: number | { precision?: number; paren?: boolean }, c?: ArrayLike<number>, d?: { precision?: number; paren?: boolean }): string {
  const isMatrix = typeof a === 'object'
  const rows = isMatrix ? (a as Matrix).rows : (a as number)
  const cols = isMatrix ? (a as Matrix).cols : (b as number)
  const data = isMatrix ? (a as Matrix).data : (c as ArrayLike<number>)
  const opts = (isMatrix ? (b as { precision?: number; paren?: boolean } | undefined) : d) || undefined
  const p = opts?.precision
  const paren = (opts as any)?.paren ?? false
  const arr = toArray(data)
  const lines: string[] = []
  for (let r = 0; r < rows; r++) {
    const row: string[] = []
    for (let c = 0; c < cols; c++) row.push(fmtNumber(arr[r * cols + c] ?? 0, p))
    lines.push(row.join(' \\!&\\! '))
  }
  const spec = buildArrayColSpec(cols)
  const arrBody = `\\begin{array}{${spec}} ${lines.join(' \\\\ ')} \\end{array}`
  return paren ? `\\left[ ${arrBody} \\right]` : arrBody
}

// Thin wrappers per DTO/type
export function formatSignalMarkdown(signal: Signal, opts?: { orientation?: 'row' | 'col'; precision?: number }): string {
  return formatVectorMarkdown(signal.data, opts)
}

// Spectrum wrappers: represent as 2 x N matrix with first row Re, second row Im
export function formatSpectrumMarkdown(spec: Spectrum, opts?: { precision?: number }): string
export function formatSpectrumMarkdown(specInterleaved: ArrayLike<number>, opts?: { precision?: number }): string
export function formatSpectrumMarkdown(spec: Spectrum | ArrayLike<number>, opts?: { precision?: number }): string {
  const p = opts?.precision
  const list: { re: number; im: number }[] = []
  if ((spec as any).data) {
    const arr = (spec as Spectrum).data
    for (const z of arr) list.push({ re: z.re, im: z.im })
  } else {
    const inter = spec as ArrayLike<number>
    for (let i = 0; i < inter.length; i += 2) list.push({ re: Number((inter as any)[i] ?? 0), im: Number((inter as any)[i + 1] ?? 0) })
  }
  // render as row vector of complex entries with tighter spacing
  const parts = list.map((z) => fmtComplex(Number(z.re ?? 0), Number(z.im ?? 0), p, 'i'))
  const specCols = buildArrayColSpec(parts.length)
  return `\\begin{array}{${specCols}} ${parts.join(' \\!&\\! ')} \\end{array}`
}

export function formatRationalFunctionMarkdown(rf: RationalFunction, varName = 'x', opts?: { trimZeros?: boolean }): string {
  const num = formatPolynomialMarkdown(rf.numerator.coeffs, varName, opts)
  const den = formatPolynomialMarkdown(rf.denominator.coeffs, varName, opts)
  return `\\frac{${num}}{${den}}`
}

// Accept either { num, den, sample_time } or { numerator: {coeffs}, denominator: {coeffs}, sample_time? }
export function formatTransferFunctionMarkdown(
  tf: TransferFunction | { numerator: { coeffs: ArrayLike<number> }; denominator: { coeffs: ArrayLike<number> }; sample_time?: number | null },
  varName?: string,
  opts?: { trimZeros?: boolean }
): string {
  const st = (tf as any).sample_time
  const v = varName ?? (st == null ? 's' : 'z')
  const hasPoly = (tf as any).numerator && (tf as any).denominator
  const numCoeffs: ArrayLike<number> = hasPoly ? (tf as any).numerator.coeffs : (tf as any).num
  const denCoeffs: ArrayLike<number> = hasPoly ? (tf as any).denominator.coeffs : (tf as any).den
  const num = formatPolynomialMarkdown(numCoeffs, v, opts)
  const den = formatPolynomialMarkdown(denCoeffs, v, opts)
  return `\\frac{${num}}{${den}}`
}

export function formatZpkMarkdown(zpk: Zpk, varName?: string, opts?: { precision?: number; imag?: 'i' | 'j'; hideUnitGain?: boolean }): string {
  const v = varName ?? (zpk.sample_time == null ? 's' : 'z')
  const p = opts?.precision
  const imag = opts?.imag ?? 'i'
  const toPairs = (arr: number[]) => {
    const out: Array<[number, number]> = []
    for (let i = 0; i < arr.length; i += 2) out.push([arr[i] ?? 0, arr[i + 1] ?? 0])
    return out
  }
  const zeroPairs = toPairs(zpk.zeros || [])
  const polePairs = toPairs(zpk.poles || [])
  const zeroFactors = zeroPairs.map(([re, im]) => {
    if (!im) {
      const num = fmtNumber(Math.abs(re), p)
      return re < 0 ? `(${v} + ${num})` : `(${v} - ${num})`
    }
    return `(${v} - (${fmtComplex(re, im, p, imag)}))`
  })
  const poleFactors = polePairs.map(([re, im]) => {
    if (!im) {
      const num = fmtNumber(Math.abs(re), p)
      return re < 0 ? `(${v} + ${num})` : `(${v} - ${num})`
    }
    return `(${v} - (${fmtComplex(re, im, p, imag)}))`
  })
  const g = zpk.gain ?? 1
  const showGain = !(opts?.hideUnitGain) || Math.abs(g - 1) > 1e-12
  const gain = fmtNumber(g, p)
  const num = [showGain ? gain : undefined, ...zeroFactors].filter(Boolean).join(' \\cdot ') || '1'
  const den = (poleFactors.join(' \\cdot ')) || '1'
  return `\\frac{${num}}{${den}}`
}

// Complex formatting
// - Single complex: Complex or an interleaved pair [re, im]
// - List of complex: Complex[] or interleaved array [re0, im0, re1, im1, ...]
export function formatComplexMarkdown(z: Complex, opts?: { precision?: number; imag?: 'i' | 'j' }): string
export function formatComplexMarkdown(zPairInterleaved: ArrayLike<number>, opts?: { precision?: number; imag?: 'i' | 'j' }): string
export function formatComplexMarkdown(zList: ReadonlyArray<Complex>, opts?: { precision?: number; imag?: 'i' | 'j'; orientation?: 'row' | 'col' }): string
export function formatComplexMarkdown(zInterleavedList: ArrayLike<number>, opts?: { precision?: number; imag?: 'i' | 'j'; orientation?: 'row' | 'col' }): string
export function formatComplexMarkdown(
  arg: Complex | ReadonlyArray<Complex> | ArrayLike<number>,
  opts?: { precision?: number; imag?: 'i' | 'j'; orientation?: 'row' | 'col' }
): string {
  const p = opts?.precision
  const imag = opts?.imag ?? 'i'
  const orientation = opts?.orientation ?? 'row'
  // ArrayLike<number>
  if (typeof (arg as any).length === 'number' && !('re' in (arg as any))) {
    const arr = arg as ArrayLike<number>
    if (arr.length === 2) {
      const re = Number((arr as any)[0] ?? 0)
      const im = Number((arr as any)[1] ?? 0)
      return fmtComplex(re, im, p, imag)
    }
    // treat as interleaved list -> vector of formatted a+bi elements
    const parts: string[] = []
    for (let i = 0; i < arr.length; i += 2) {
      const re = Number((arr as any)[i] ?? 0)
      const im = Number((arr as any)[i + 1] ?? 0)
      parts.push(fmtComplex(re, im, p, imag))
    }
    if (orientation === 'row') {
  const spec = buildArrayColSpec(parts.length)
  return `\\begin{array}{${spec}} ${parts.join(' \\!&\\! ')} \\end{array}`
    } else {
  const spec = buildArrayColSpec(1)
  return `\\begin{array}{${spec}} ${parts.join(' \\\\ ')} \\end{array}`
    }
  }
  // Complex[]
  if (Array.isArray(arg)) {
    const list = arg as ReadonlyArray<Complex>
    const parts = list.map((z) => fmtComplex(Number(z.re ?? 0), Number(z.im ?? 0), p, imag))
    if (orientation === 'row') {
      const spec = buildArrayColSpec(parts.length)
      return `\\begin{array}{${spec}} ${parts.join(' & ')} \\end{array}`
    } else {
      const spec = buildArrayColSpec(1)
      return `\\begin{array}{${spec}} ${parts.join(' \\\\ ')} \\end{array}`
    }
  }
  // Single Complex
  const z = arg as Complex
  return fmtComplex(Number(z.re ?? 0), Number(z.im ?? 0), p, imag)
}
