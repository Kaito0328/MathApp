"use client"
import React from 'react'
import { getWasm } from '../../../../../wasm/loader'
import Row from '../../../../../baseComponents/layout/Row'
import MarkdownMath from '../../../../../widgets/display/MarkdownMath'
import { formatPolynomialMarkdown, formatNumberForMath } from '../../../../../utils/format/markdown'

export interface RootsVerificationProps {
  coeffs: number[]
  // Optional precomputed interleaved roots [re0, im0, ...]
  precomputedRoots?: number[]
  precision?: number
}

const RootsVerification: React.FC<RootsVerificationProps> = ({ coeffs, precomputedRoots, precision }) => {
  const [tex, setTex] = React.useState<string | null>(null)
  const [error, setError] = React.useState<string | null>(null)

  React.useEffect(()=>{
    let cancelled = false
    const run = async ()=>{
  setError(null); setTex(null)
      try {
        const wasm: any = await getWasm()
        // detect zero polynomial early
        const nz = coeffs.some(v=> Math.abs(v) > 1e-12)
        if (!nz) {
          if (!cancelled) setTex('\\begin{aligned} 0 &= 0 \\end{aligned}')
          return
        }
        let inter: ArrayLike<number>
        if (precomputedRoots && precomputedRoots.length>0) {
          inter = precomputedRoots as ArrayLike<number>
        } else {
          const poly = new wasm.PolynomialF64(Float64Array.from(coeffs))
          inter = poly.findRoots() as Float64Array
          poly.free?.()
        }
  const roots: { re:number; im:number }[] = []
        for (let i=0;i<inter.length;i+=2) {
          const re = inter[i]; const im = inter[i+1]
          if (!Number.isFinite(re) || !Number.isFinite(im)) continue
          roots.push({ re, im })
        }
        const tol = typeof precision === 'number' ? 0.5 * Math.pow(10, -precision) : 1e-9
        const real: number[] = []
        const complex: { re:number; im:number }[] = []
        for (const r of roots) { if (Math.abs(r.im) < tol) real.push(r.re); else complex.push(r) }
  const eps = 1e-6
  const fmt = (x:number)=> formatNumberForMath(x, precision)

        // multiplicity grouping
        const groups: { r:{re:number; im:number}; mult:number }[] = []
        for (const r of roots) {
          const existing = groups.find(g=> Math.abs(g.r.re - r.re)<eps && Math.abs(g.r.im - r.im)<eps)
          if (existing) existing.mult += 1; else groups.push({ r, mult:1 })
        }
        // Build factorization strings
  const varName = 'x'
        // Complex-root factorization: product of linear factors (x - r)
        const complexFactors: string[] = []
        for (const g of groups) {
          const a = g.r.re, b = g.r.im
          const isReal = Math.abs(b) < tol
          let linear: string
          if (isReal) {
            const aShownZero = fmt(Math.abs(a)) === '0'
            linear = aShownZero ? `(${varName})` : `(${varName} ${a>=0?'-':'+'} ${fmt(Math.abs(a))})`
          } else {
            const reZero = fmt(a) === '0'
            const imStr = fmt(Math.abs(b))
            const imPart = imStr === '1' ? 'i' : `${imStr}i`
            const inner = reZero ? `${b>=0?'+':'-'} ${imPart}` : `${fmt(a)} ${b>=0?'+':'-'} ${imPart}`
            linear = `(${varName} - (${inner}))`
          }
          complexFactors.push(g.mult>1 ? `${linear}^{${g.mult}}` : linear)
        }
        const complexForm = complex.length>0 ? (complexFactors.join('') || '1') : ''

        // Real-coefficient factorization: pair conjugates into quadratics
        const realGroups = groups.filter(g=> Math.abs(g.r.im) < eps)
        const complexGroups = groups.filter(g=> Math.abs(g.r.im) >= eps)
        const usedC = new Array(complexGroups.length).fill(false)
        const realCoefFactors: string[] = []
        // linear real roots
        for (const g of realGroups) {
          const a = g.r.re
          const aShownZero = fmt(Math.abs(a)) === '0'
          const lin = aShownZero ? `(${varName})` : `(${varName} ${a>=0?'-':'+'} ${fmt(Math.abs(a))})`
          realCoefFactors.push(g.mult>1 ? `${lin}^{${g.mult}}` : lin)
        }
        // pair conjugates
        for (let i=0;i<complexGroups.length;i++) {
          if (usedC[i]) continue
          const g = complexGroups[i]
          const a = g.r.re, b = Math.abs(g.r.im)
          let cj = -1
          for (let j=i+1;j<complexGroups.length;j++) if (!usedC[j]) {
            const h = complexGroups[j]
            if (Math.abs(h.r.re - a)<eps && Math.abs(h.r.im + g.r.im)<eps && h.mult===g.mult) { cj=j; break }
          }
          if (cj>=0) {
            // Note: c1 and c2 are numeric strings; compute numerically then format
            const c1n = -2*a
            const c2n = a*a + b*b
            const c1 = formatNumberForMath(c1n, precision)
            const c2 = formatNumberForMath(c2n, precision)
            const signC1 = c1n >= 0 ? '+' : ''
            const quad = `(${varName}^2 ${signC1}${c1} ${varName} + ${c2})`
            realCoefFactors.push(g.mult>1 ? `${quad}^{${g.mult}}` : quad)
            usedC[i]=usedC[cj]=true
          } else {
            // fallback: single complex (shouldn't happen) -> linear complex
            const part = `(${varName} - (${fmt(a)} ${g.r.im>=0?'+':'-'} ${fmt(Math.abs(g.r.im))}i))`
            realCoefFactors.push(g.mult>1 ? `${part}^{${g.mult}}` : part)
            usedC[i]=true
          }
        }
        const realCoefForm = realCoefFactors.join('') || '1'

        // leading coefficient
        const leading = (()=>{
          for (let i=coeffs.length-1;i>=0;i--) {
            const v = coeffs[i]
            if (Math.abs(v) > 1e-12) return v
          }
          return 0
        })()
        // Coefficient prefix for factorization: -1 -> '-', 1 -> '', others -> number string (rounded)
        const coefPrefix = Math.abs(leading - 1) < eps
          ? ''
          : (Math.abs(leading + 1) < eps ? '-' : formatNumberForMath(leading, precision))

        // Build expanded polynomial (monic) by multiplying linear/quadratic real-coefficient factors
        // Factors: real root a -> (x - a) with coeffs [-a, 1]
        //          conjugate pair a±ib -> x^2 - 2a x + (a^2 + b^2) with coeffs [a^2+b^2, -2a, 1]
        const factors: number[][] = [] // each factor coeffs low->high
        // linear real roots
        for (const g of realGroups) {
          for (let k = 0; k < g.mult; k++) factors.push([-g.r.re, 1])
        }
        // conjugate pairs
        for (let i=0;i<complexGroups.length;i++) {
          if (usedC[i]) continue
          const g = complexGroups[i]
          const a = g.r.re, b = Math.abs(g.r.im)
          let cj = -1
          for (let j=i+1;j<complexGroups.length;j++) if (!usedC[j]) {
            const h = complexGroups[j]
            if (Math.abs(h.r.re - a)<eps && Math.abs(h.r.im + g.r.im)<eps && h.mult===g.mult) { cj=j; break }
          }
          if (cj>=0) {
            const c0 = a*a + b*b // constant term
            const c1 = -2*a
            for (let k=0;k<g.mult;k++) factors.push([c0, c1, 1])
            usedC[i]=usedC[cj]=true
          } else {
            // unmatched complex root (rare): skip to keep real coefficients
            usedC[i]=true
          }
        }
        // multiply all factors
        const polyMul = (A: number[], B: number[]) => {
          const out = new Array(A.length + B.length - 1).fill(0)
          for (let i=0;i<A.length;i++) for (let j=0;j<B.length;j++) out[i+j] += A[i]*B[j]
          return out
        }
        let monicCoeffs = [1]
        for (const f of factors) monicCoeffs = polyMul(monicCoeffs, f)
        const expandedCoeffs = monicCoeffs.map(c=> c * leading)
        // clean tiny numbers
        for (let i=0;i<expandedCoeffs.length;i++) if (Math.abs(expandedCoeffs[i]) < 1e-12) expandedCoeffs[i] = 0
        // apply precision rounding for display
        const roundedCoeffs = expandedCoeffs.map((c)=>{
          if (typeof precision === 'number') {
            const r = Number(c.toFixed(precision))
            return Object.is(r, -0) ? 0 : r
          }
          return Object.is(c, -0) ? 0 : c
        })
  const expanded = formatPolynomialMarkdown(roundedCoeffs)
        let texOut: string
        if (complex.length>0) {
          const left = `${coefPrefix}${complexForm}`
          const mid = `${coefPrefix}${realCoefForm}`
          texOut = `\\begin{aligned} ${left} &= ${mid} \\\\ &= ${expanded} \\end{aligned}`
        } else {
          const left = `${coefPrefix}${realCoefForm}`
          texOut = `\\begin{aligned} ${left} &= ${expanded} \\end{aligned}`
        }
        if (cancelled) return
        setTex(texOut)
      } catch (e:any) {
        if (!cancelled) setError(e?.message || String(e))
      }
    }
    run()
    return ()=>{ cancelled = true }
  }, [coeffs, precomputedRoots, precision])

  if (error) return <Row center={<span style={{ color:'crimson' }}>{error}</span>} />

  return tex ? <MarkdownMath math={tex} /> : null
}

export default RootsVerification
