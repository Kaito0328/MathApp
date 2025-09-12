"use client"
import React from 'react'
import Row from '../../../../../baseComponents/layout/Row'
import MarkdownMath from '../../../../../widgets/display/MarkdownMath'
import { getWasm } from '../../../../../wasm/loader'
import { formatNumberForMath, formatPolynomialMarkdown } from '../../../../../utils/format/markdown'

export interface ZerosPolesVerificationProps {
  kind: 'zeros' | 'poles'
  coeffs: number[]
  roots?: number[]
  precision?: number
}

const ZerosPolesVerification: React.FC<ZerosPolesVerificationProps> = ({ kind, coeffs, roots, precision }) => {
  const [tex, setTex] = React.useState<string | null>(null)
  const [error, setError] = React.useState<string | null>(null)

  React.useEffect(()=>{
    let cancelled = false
    const run = async () => {
      setError(null); setTex(null)
      try {
        const nz = coeffs.some(v=> Math.abs(v) > 1e-12)
        if (!nz) { setTex('\\begin{aligned} 0 &= 0 \\end{aligned}'); return }
        const wasm: any = await getWasm()
        let inter: ArrayLike<number>
        if (roots && roots.length>0) inter = roots as ArrayLike<number>
        else {
          const poly = new wasm.PolynomialF64(Float64Array.from(coeffs))
          inter = poly.findRoots() as Float64Array
          poly.free?.()
        }
        const list: { re:number; im:number }[] = []
        for (let i=0;i<inter.length;i+=2) {
          const re = inter[i]; const im = inter[i+1]
          if (!Number.isFinite(re) || !Number.isFinite(im)) continue
          list.push({ re, im })
        }
        const tol = typeof precision === 'number' ? 0.5 * Math.pow(10, -precision) : 1e-9
        const real: number[] = []
        const complex: { re:number; im:number }[] = []
        for (const r of list) { if (Math.abs(r.im) < tol) real.push(r.re); else complex.push(r) }

        // group by (re, im)
        const groups: { r:{re:number; im:number}; mult:number }[] = []
        for (const r of list) {
          const g = groups.find(g=> Math.abs(g.r.re - r.re) < 1e-6 && Math.abs(g.r.im - r.im) < 1e-6)
          if (g) g.mult += 1; else groups.push({ r, mult: 1 })
        }

        const fmt = (x:number)=> formatNumberForMath(x, precision)
        const x = 'x'
        // factors with complex allowed (linear)
        const cf: string[] = []
        for (const g of groups) {
          const a = g.r.re, b = g.r.im
          const isReal = Math.abs(b) < tol
          let linear: string
          if (isReal) {
            const aZero = fmt(Math.abs(a)) === '0'
            linear = aZero ? `(${x})` : `(${x} ${a>=0?'-':'+'} ${fmt(Math.abs(a))})`
          } else {
            const reZero = fmt(a) === '0'
            const imStr = fmt(Math.abs(b))
            const imPart = imStr === '1' ? 'i' : `${imStr}i`
            const inner = reZero ? `${b>=0?'+':'-'} ${imPart}` : `${fmt(a)} ${b>=0?'+':'-'} ${imPart}`
            linear = `(${x} - (${inner}))`
          }
          cf.push(g.mult>1 ? `${linear}^{${g.mult}}` : linear)
        }
        const complexForm = cf.join('') || '1'

        // real-coefficient pairing to quadratics
        const realGroups = groups.filter(g=> Math.abs(g.r.im) < 1e-6)
        const complexGroups = groups.filter(g=> Math.abs(g.r.im) >= 1e-6)
        const used = new Array(complexGroups.length).fill(false)
        const rf: string[] = []
        for (const g of realGroups) {
          const a = g.r.re
          const az = fmt(Math.abs(a)) === '0'
          const lin = az ? `(${x})` : `(${x} ${a>=0?'-':'+'} ${fmt(Math.abs(a))})`
          rf.push(g.mult>1 ? `${lin}^{${g.mult}}` : lin)
        }
        for (let i=0;i<complexGroups.length;i++) {
          if (used[i]) continue
          const g = complexGroups[i]
          const a = g.r.re, b = Math.abs(g.r.im)
          let cj = -1
          for (let j=i+1;j<complexGroups.length;j++) if (!used[j]) {
            const h = complexGroups[j]
            if (Math.abs(h.r.re - a) < 1e-6 && Math.abs(h.r.im + g.r.im) < 1e-6 && h.mult === g.mult) { cj = j; break }
          }
          if (cj>=0) {
            const c1n = -2*a
            const c2n = a*a + b*b
            const c1 = formatNumberForMath(c1n, precision)
            const c2 = formatNumberForMath(c2n, precision)
            const sign = c1n >= 0 ? '+' : ''
            const quad = `(${x}^2 ${sign}${c1} ${x} + ${c2})`
            rf.push(g.mult>1 ? `${quad}^{${g.mult}}` : quad)
            used[i]=used[cj]=true
          } else {
            const imStr = formatNumberForMath(Math.abs(g.r.im), precision)
            const imPart = imStr === '1' ? 'i' : `${imStr}i`
            const inner = `${formatNumberForMath(g.r.re, precision)} ${g.r.im>=0?'+':'-'} ${imPart}`
            const part = `(${x} - (${inner}))`
            rf.push(g.mult>1 ? `${part}^{${g.mult}}` : part)
            used[i]=true
          }
        }
        const realForm = rf.join('') || '1'

        // leading
        const leading = (()=>{ for(let i=coeffs.length-1;i>=0;i--){ const v=coeffs[i]; if(Math.abs(v)>1e-12) return v } return 0 })()
        const coefPrefix = Math.abs(leading - 1) < 1e-6 ? '' : (Math.abs(leading + 1) < 1e-6 ? '-' : formatNumberForMath(leading, precision))

        // expanded from roots (monic) then scale
        const arr: number[] = []; for (const r of list) { arr.push(r.re, r.im) }
        const poly = (await getWasm() as any).PolynomialF64.fromRoots(Float64Array.from(arr))
        const monicCoeffs = Array.from(poly.coeffs?.() ?? []) as number[]
        poly.free?.()
        const expandedCoeffs = monicCoeffs.map(c=> c*leading)
        const roundedCoeffs = expandedCoeffs.map(c=> typeof precision==='number'? (Object.is(Number(c.toFixed(precision)),-0)?0:Number(c.toFixed(precision))) : (Object.is(c,-0)?0:c))
        const expanded = formatPolynomialMarkdown(roundedCoeffs)

        const twoLines = complex.length>0
        const left = `${coefPrefix}${complexForm}`
        const mid = `${coefPrefix}${realForm}`
        const body = twoLines
          ? `\\begin{aligned} ${left} &= ${mid} \\\\ &= ${expanded} \\end{aligned}`
          : `\\begin{aligned} ${mid} &= ${expanded} \\end{aligned}`
        if (!cancelled) setTex(body)
      } catch(e:any) { if (!cancelled) setError(e?.message||String(e)) }
    }
    run(); return ()=>{ cancelled = true }
  }, [kind, coeffs, roots, precision])

  if (error) return <Row center={<span style={{ color:'crimson' }}>{error}</span>} />
  return tex ? <MarkdownMath math={tex} /> : null
}

export default ZerosPolesVerification
