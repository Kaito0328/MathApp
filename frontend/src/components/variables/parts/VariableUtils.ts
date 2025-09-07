import { formatMatrixMarkdown, formatVectorMarkdown } from '../../../utils/format/markdown'

export function variableLabel(v: any): string {
  if (v?.kind === 'matrix') return `matrix [${v.rows} x ${v.cols}]`
  if (v?.kind === 'vector') return `vector [${v.length}]`
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
  return ''
}

export function variableToMarkdown(v: any): string {
  if (v?.kind === 'matrix') return formatMatrixMarkdown(v.rows, v.cols, v.data)
  if (v?.kind === 'vector') return formatVectorMarkdown({ data: v.data }, { orientation: 'col', paren: true })
  return ''
}
