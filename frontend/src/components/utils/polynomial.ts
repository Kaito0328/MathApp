export function formatPolynomial(coeffs: number[], varName = 'x', opts?: { useSup?: boolean; trimZeros?: boolean }): string {
  const useSup = opts?.useSup ?? false
  const trimZeros = opts?.trimZeros ?? true
  const n = coeffs.length
  if (n === 0) return '0'
  const deg = n - 1
  const terms: string[] = []
  for (let i = deg; i >= 0; i--) {
    const c = coeffs[i] ?? 0
    if (trimZeros && c === 0) continue
    const isConst = i === 0
    const sign = c < 0 ? '-' : (terms.length ? '+' : '')
    const abs = Math.abs(c)
    const coeff = isConst ? `${abs}` : (abs === 1 ? '' : `${abs}`)
    let pow = ''
    if (!isConst) {
      if (i === 1) pow = varName
      else pow = useSup ? `${varName}<sup>${i}</sup>` : `${varName}^${i}`
    }
    const piece = [sign, coeff, pow].join('').trim()
    terms.push(piece)
  }
  if (terms.length === 0) return '0'
  return terms.join(' ') // e.g., "2x^2 + x - 1"
}

export function formatPolynomialMarkdown(coeffs: number[], varName = 'x', opts?: { trimZeros?: boolean }): string {
  const trimZeros = opts?.trimZeros ?? true
  const n = coeffs.length
  if (n === 0) return '0'
  const deg = n - 1
  const terms: string[] = []
  for (let i = deg; i >= 0; i--) {
    const c = coeffs[i] ?? 0
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
