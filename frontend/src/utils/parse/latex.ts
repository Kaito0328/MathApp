// Minimal LaTeX/Markdown matrix/vector parser
// Supports inputs like:
//  - \begin{array}{c c} 1 & 2 \\ 3 & 4 \end{array}
//  - \begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}
//  - \begin{bmatrix} 1 \\ 2 \\ 3 \end{bmatrix}
//  - \left[ \begin{array}{c} 1 \\ 2 \end{array} \right]
//  - Also tolerates our tight separators: \\!&\\!

export type ParsedMatrix = { kind: 'matrix'; rows: number; cols: number; data: number[] }
export type ParsedVector = { kind: 'vector'; data: number[] }
export type ParseResult = ParsedMatrix | ParsedVector

function stripWrappers(src: string): string {
  let s = src.trim()
  // Remove surrounding $...$ or $$...$$
  s = s.replace(/^\${1,2}/, '').replace(/\${1,2}$/,'')
  // Remove \[ ... \] or \( ... \)
  s = s.replace(/^\\\[|^\\\(/, '').replace(/\\\]$|\\\)$/, '')
  // Remove \left[ \right] or parentheses
  s = s.replace(/\\left\[/g, '').replace(/\\right\]/g, '')
  s = s.replace(/\\left\(/g, '').replace(/\\right\)/g, '')
  // Remove begin/end environments
  s = s.replace(/\\begin\{array\}\{[^}]*\}/g, '')
  s = s.replace(/\\begin\{pmatrix\}/g, '')
  s = s.replace(/\\begin\{bmatrix\}/g, '')
  s = s.replace(/\\end\{array\}/g, '')
  s = s.replace(/\\end\{pmatrix\}/g, '')
  s = s.replace(/\\end\{bmatrix\}/g, '')
  // Remove braces which sometimes wrap arrays
  s = s.replace(/[{}]/g, '')
  // Normalize tight spacing and separators (e.g., \!&\!, and stray \! around & or \\\)
  s = s.replace(/\\!\s*&\s*\\!/g, '&') // \!&\! -> &
  s = s.replace(/\s*&\s*/g, ' & ') // normalize around &
  s = s.replace(/\\!+/g, '') // remove leftover thin spaces
  // Remove common LaTeX spacing commands
  s = s.replace(/\\[ ,;:]/g, '')
  // Normalize row separators robustly
  // 1) Preserve existing \\\ first using a placeholder token
  const ROW = '__ROW__TOKEN__'
  s = s.replace(/\\\\/g, ROW)
  // 2) Any remaining single backslashes that aren't part of words act as row breaks (handles cases where one \\ got collapsed)
  s = s.replace(/\\(?![a-zA-Z])/g, ROW)
  // 3) Restore placeholders back to \\\ for downstream splitting
  s = s.replace(new RegExp(ROW, 'g'), '\\\\')
  // Accept newlines/semicolon later as fallbacks as well
  // Collapse multiple spaces
  s = s.replace(/[\t ]+/g, ' ').trim()
  return s
}

function splitRows(body: string): string[] {
  // Primary: explicit \\\\ row separator
  let parts = body.split(/\\\\/g).map((p) => p.trim()).filter(Boolean)
  if (parts.length > 1) return parts
  // Secondary: newline-delimited rows
  parts = body.split(/\n+/g).map((p) => p.trim()).filter(Boolean)
  if (parts.length > 1) return parts
  // Fallback: semicolon
  parts = body.split(/\s*;\s*/g).map((p) => p.trim()).filter(Boolean)
  return parts
}

function splitCols(row: string): string[] {
  if (row.includes('&')) return row.split(/\s*&\s*/g).map((s) => s.trim()).filter(Boolean)
  // Fallback: split on spaces
  return row.split(/[\s]+/g).map((s) => s.trim()).filter(Boolean)
}

function toNumber(tok: string): number {
  // Remove surrounding parentheses
  const t = tok.replace(/^\(+|\)+$/g, '')
  const n = Number(t)
  if (Number.isFinite(n)) return n
  // Try replacing commas as decimal separator (unlikely here, but safe)
  const n2 = Number(t.replace(',', '.'))
  return Number.isFinite(n2) ? n2 : NaN
}

export function parseLatexArray(text: string): ParseResult | { error: string } {
  if (!text || !text.trim()) return { error: '空の入力です' }
  // Accept bare matrix content such as `1 & 2 \\\\ 3 & 4` without any environment
  const body = stripWrappers(text)
  if (!body) return { error: '内容を解析できませんでした' }
  const rowsRaw = splitRows(body)
  if (!rowsRaw.length) return { error: '行の分割に失敗しました' }
  const rows: number[][] = []
  let maxCols = 0
  for (const r of rowsRaw) {
    const colsRaw = splitCols(r)
    if (!colsRaw.length) continue
    const nums = colsRaw.map(toNumber)
    if (nums.some((v) => !Number.isFinite(v))) {
      return { error: `数値の解析に失敗しました: "${r}"` }
    }
    rows.push(nums)
    if (nums.length > maxCols) maxCols = nums.length
  }
  if (!rows.length) return { error: '有効な数値が見つかりませんでした' }

  // Determine vector vs matrix
  const allSingleCol = rows.every((v) => v.length === 1)
  const singleRow = rows.length === 1
  if (allSingleCol || singleRow) {
    // Treat as vector (row or column). Flatten preserving order left-to-right, top-to-bottom.
    const data = (allSingleCol ? rows.map((r) => r[0]) : rows[0].slice())
    return { kind: 'vector', data }
  }
  // Normalize jagged rows by padding zeros to maxCols
  const data: number[] = []
  for (const r of rows) {
    const arr = r.slice()
    if (arr.length < maxCols) arr.push(...Array(maxCols - arr.length).fill(0))
    data.push(...arr)
  }
  return { kind: 'matrix', rows: rows.length, cols: maxCols, data }
}
