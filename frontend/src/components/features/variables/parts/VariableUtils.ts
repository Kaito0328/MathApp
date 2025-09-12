import { formatMatrixMarkdown, formatVectorMarkdown, formatPolynomialMarkdown, formatRationalFunctionMarkdown } from '../../../../utils/format/markdown'

export function variableLabel(v: any): string {
  if (v?.kind === 'matrix') return `matrix [${v.rows} x ${v.cols}]`
  if (v?.kind === 'vector') return `vector [${v.length}]`
  if (v?.kind === 'polynomial') return `polynomial [deg ${Math.max(0, (v.coeffs?.length ?? 1) - 1)}]`
  if (v?.kind === 'rational') return `rational`
  return 'unknown'
}

export function variablePreview(v: any): string {
  if (v?.kind === 'matrix') {
    const rows = Math.min(v.rows ?? 0, 3)
    const cols = Math.min(v.cols ?? 0, 6)
    return Array.from({ length: rows }, (_, r) =>
      Array.from({ length: cols }, (_, c) => v.data?.[r * (v.cols ?? 0) + c] ?? 0).join(' ')
    ).join(' | ')
  }
  if (v?.kind === 'vector') {
    const n = Math.min(v.length ?? (v.data?.length ?? 0), 8)
    return Array.from({ length: n }, (_, i) => v.data?.[i] ?? 0).join(' ')
  }
  if (v?.kind === 'polynomial') {
    const n = Math.min(v.coeffs?.length ?? 0, 6)
    return (v.coeffs ?? []).slice(0, n).join(' ')
  }
  if (v?.kind === 'rational') {
    const n1 = Math.min(v.numerator?.length ?? 0, 4)
    const n2 = Math.min(v.denominator?.length ?? 0, 4)
    return `num: ${(v.numerator ?? []).slice(0, n1).join(' ')} / den: ${(v.denominator ?? []).slice(0, n2).join(' ')}`
  }
  return ''
}

export function variableToMarkdown(v: any): string {
  if (v?.kind === 'matrix') return formatMatrixMarkdown(v.rows, v.cols, v.data)
  if (v?.kind === 'vector') return formatVectorMarkdown({ data: v.data }, { orientation: 'col', paren: true })
  if (v?.kind === 'polynomial') return formatPolynomialMarkdown(v.coeffs)
  if (v?.kind === 'rational') return formatRationalFunctionMarkdown({ numerator: { coeffs: v.numerator }, denominator: { coeffs: v.denominator } } as any)
  return ''
}
