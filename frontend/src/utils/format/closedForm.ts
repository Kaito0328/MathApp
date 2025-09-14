"use client"
import { formatNumberForMath } from './markdown'

// 共通: ClosedForm から可読な TeX を生成（係数1の省略、\cdot の補完、整数底の括弧外し、丸め）
export function buildClosedFormTeX(cf: any, precision?: number): string {
  const p = Math.max(1, Number.isFinite(precision as number) ? (precision as number) : 4)
  const fmt = (v: number) => formatNumberForMath(Number(v || 0), p)
  const tol = Math.pow(10, -p) * 5 // tolerance tied to precision
  const fmtComplex = (re: number, im: number): string => {
    const r = Math.abs(re) < tol ? 0 : re
    const i = Math.abs(im) < tol ? 0 : im
    if (!i) return fmt(r)
    if (!r) {
      const ai = Math.abs(i)
      const coef = ai === 1 ? '' : `${fmt(ai)} \\cdot `
      return i < 0 ? `- ${coef}i` : `${coef}i`
    }
    const sign = i < 0 ? '-' : '+'
    const ai = Math.abs(i)
    const imag = ai === 1 ? 'i' : `${fmt(ai)} \\cdot i`
    return `${fmt(r)} ${sign} ${imag}`
  }
  const encloseIfNeeded = (s: string): string => /[ +-]/.test(s) ? `\\left(${s}\\right)` : s

  const roundNumbersInTeX = (tex: string): string =>
    tex.replace(/-?\d+(?:\.\d+)?(?:e[+-]?\d+)?/gi, (m) => {
      const v = Number(m)
      if (!isFinite(v)) return m
      return fmt(v)
    })

  try {
    const tcount: number = Number(cf.termsCount?.() ?? 0)
    const parts: string[] = []
    for (let t = 0; t < tcount; t++) {
      const polyFlat: Float64Array = cf.termPoly?.(t)
      const baseFlat: Float64Array = cf.termBase?.(t)
      const baseReRaw = Number(baseFlat?.[0] ?? 0)
      const baseImRaw = Number(baseFlat?.[1] ?? 0)
      const baseRe = Number(fmt(baseReRaw))
      const baseIm = Number(fmt(baseImRaw))
      const baseIsOne = Math.hypot(baseRe - 1, baseIm) < tol
      const baseStr = (() => {
        const r = Math.abs(baseRe) < tol ? 0 : baseRe
        const i = Math.abs(baseIm) < tol ? 0 : baseIm
        if (!i) return fmt(r)
        if (!r) return i < 0 ? '- i' : 'i'
        const sign = i < 0 ? '-' : '+'
        const ai = Math.abs(i)
        const imag = ai === 1 ? 'i' : `${fmt(ai)} \\cdot i`
        return `${fmt(r)} ${sign} ${imag}`
      })()
      // polyFlat is [re0, im0, re1, im1, ...] => re/im for n^0, n^1, ...
      const deg = Math.max(0, Math.floor((polyFlat?.length ?? 0) / 2) - 1)
      const termsQ: string[] = []
      for (let k = deg; k >= 0; k--) {
        const reRaw = Number(polyFlat?.[2 * k] ?? 0)
        const imRaw = Number(polyFlat?.[2 * k + 1] ?? 0)
        const re = Number(fmt(reRaw))
        const im = Number(fmt(imRaw))
        if (Math.abs(re) < tol && Math.abs(im) < tol) continue
        const coeff = fmtComplex(re, im)
        const pow = k === 0 ? '' : (k === 1 ? 'n' : `n^{${k}}`)
        const piece = pow
          ? (coeff === '1' ? `${pow}` : (coeff === '-1' ? `- ${pow}` : `${coeff} \\cdot ${pow}`))
          : `${coeff}`
        termsQ.push(piece)
      }
      const Qn = termsQ.length ? termsQ.join(' + ').replace(/\+ -/g, '- ') : '0'
      if (Qn === '0') {
        // skip zero term entirely
        continue
      }
      if (baseIsOne) {
        // 1^n -> 1, and hide 1· prefix
        parts.push(Qn)
      } else {
        const left = Qn === '1' ? '' : `${Qn} \\cdot `
        const right = `${encloseIfNeeded(baseStr)}^{n}`
        parts.push(`${left}${right}`)
      }
    }
    if (parts.length === 0) return '0'
    return roundNumbersInTeX(parts.join(' + ').replace(/\+ -/g, '- '))
  } catch {
  // Fallback: toString を整形
  let s = String(cf.toString?.('n', false) ?? '')
  // 1係数の省略
  s = s.replace(/\b1\s*\\cdot\s*/g, '')
    // 実部0の複素数の簡約: 0 + 1·i => i, 0 - 1·i => - i, 0 + i => i, 0 - i => - i
    s = s.replace(/\b0\s*\+\s*1\s*\\cdot\s*i\b/g, 'i')
    s = s.replace(/\b0\s*-\s*1\s*\\cdot\s*i\b/g, '- i')
    s = s.replace(/\b0\s*\+\s*i\b/g, 'i')
    s = s.replace(/\b0\s*-\s*i\b/g, '- i')
    // + 0 や - 0 を除去（項としての 0）
    s = s.replace(/\s\+\s*0(?![\d])/g, '')
    s = s.replace(/\s-\s*0(?![\d])/g, '')
  // 係数と変数の間に \cdot を入れる（単純な3段階置換）
  s = s.replace(/(\d)(?=[a-zA-Z\\])/g, '$1 \\cdot ')
  s = s.replace(/(\d)(?=\()/g, '$1 \\cdot ')
  s = s.replace(/(\d)(?=\\left\()/g, '$1 \\cdot ')
    // (整数)^n の括弧を外す
    s = s.replace(/\\left\((-?\d+)\\right\)\^\{n\}/g, '$1^{n}')
    s = s.replace(/\((-?\d+)\)\^\{n\}/g, '$1^{n}')
    s = s.replace(/\\left\((-?\d+)\\right\)\^n/g, '$1^n')
    s = s.replace(/\((-?\d+)\)\^n/g, '$1^n')
    return roundNumbersInTeX(s)
  }
}
