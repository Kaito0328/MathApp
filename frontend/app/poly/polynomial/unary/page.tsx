"use client"
import React from 'react'
import { polyDiff, polyInt, polyNormalize } from '../../../../src/wasm/polynomial'
import { getWasm } from '../../../../src/wasm/loader'
import { formatPolynomialMarkdown } from '../../../../src/utils/format/markdown'
import PageContainer from '../../../../src/baseComponents/layout/PageContainer'
import { useVariableStore } from '../../../../src/state/VariableStore'
import { VariablePicker } from '../../../../src/components/variables/VariablePicker'
import Panel from '../../../../src/baseComponents/layout/Panel'
import Row from '../../../../src/baseComponents/layout/Row'
import { PolynomialUnaryInputPanel, PolynomialUnaryResultPanel } from '../../../../src/components/polynomial/PolynomialUnaryResult'
import type { PolyUnaryResultDU } from '../../../../src/components/polynomial/PolynomialUnaryResult'
import OperationSetting from '../../../../src/components/operations/OperationSetting'

export default function PolyUnaryPage() {
  type PolyUnaryOp = 'diff'|'int'|'deg'|'roots'
  const operations: { label: string; value: PolyUnaryOp }[] = [
    { label: '微分', value: 'diff' },
    { label: '積分', value: 'int' },
    { label: '次数', value: 'deg' },
    { label: '根', value: 'roots' },
  ]
  const [P, setP] = React.useState<{ coeffs: number[] }>({ coeffs: [1,0,-1] })
  const [out, setOut] = React.useState<number[] | null>(null)
  const [info, setInfo] = React.useState<string | null>(null)
  const [rootsDisplay, setRootsDisplay] = React.useState<string | null>(null)
  const [factorBlocks, setFactorBlocks] = React.useState<string[] | null>(null)
  const [verifyText, setVerifyText] = React.useState<string | null>(null)
  const [err, setErr] = React.useState<string | null>(null)
  const { get } = useVariableStore()
  const [op, setOp] = React.useState<PolyUnaryOp | 'noop'>('noop')

  const reset = () => { setErr(null); setInfo(null); setOut(null); setRootsDisplay(null); setFactorBlocks(null); setVerifyText(null) }
  const diff = async () => { reset(); try { setOut((await polyDiff(P.coeffs)).coeffs) } catch(e:any){ setErr(e?.message||String(e)) } }
  const integ = async () => { reset(); try { setOut((await polyInt(P.coeffs)).coeffs) } catch(e:any){ setErr(e?.message||String(e)) } }
  const degree = async () => { reset(); try { const p = await polyNormalize(P.coeffs); setInfo(`deg = ${Math.max(0, p.coeffs.length-1)}`) } catch(e:any){ setErr(e?.message||String(e)) } }
  const findRoots = async () => {
    reset()
    try {
      const wasm: any = await getWasm()
      const coeffs = P.coeffs.slice()
      const poly = new wasm.PolynomialF64(Float64Array.from(coeffs))
      const inter = poly.findRoots() as Float64Array
      poly.free?.()
      const roots: { re:number; im:number }[] = []
      for (let i=0;i<inter.length;i+=2) {
        const re = inter[i]; const im = inter[i+1]
        if (!Number.isFinite(re) || !Number.isFinite(im)) continue
        roots.push({ re, im })
      }
      const real: number[] = []
      const complex: { re:number; im:number }[] = []
      for (const r of roots) { if (Math.abs(r.im) < 1e-9) real.push(r.re); else complex.push(r) }
      const fmt = (x:number)=> Number(x.toPrecision(8)).toString().replace(/\.0+$/,'')
      const complexStrs: string[] = []
      const used = new Array(complex.length).fill(false)
      for (let i=0;i<complex.length;i++) {
        if (used[i]) continue
        const a = complex[i]
        let pairIndex = -1
        for (let j=i+1;j<complex.length;j++) if (!used[j]) { const b=complex[j]; if (Math.abs(b.re - a.re)<1e-6 && Math.abs(b.im + a.im)<1e-6) { pairIndex=j; break } }
        if (pairIndex>=0) {
          complexStrs.push(`${fmt(a.re)} ${a.im>=0?'+':'-'} ${fmt(Math.abs(a.im))}i`, `${fmt(a.re)} ${a.im>=0?'-':'+'} ${fmt(Math.abs(a.im))}i`)
          used[i]=true; used[pairIndex]=true
        } else {
          complexStrs.push(`${fmt(a.re)} ${a.im>=0?'+':'-'} ${fmt(Math.abs(a.im))}i`)
          used[i]=true
        }
      }
      const rootLine = [...real.map(r=> fmt(r)), ...complexStrs].join(', ') || 'なし'
      setRootsDisplay(rootLine)
      // multiplicities grouping
      const groups: { r:{re:number; im:number}; mult:number }[] = []
      for (const r of roots) {
        const existing = groups.find(g=> Math.abs(g.r.re - r.re)<1e-6 && Math.abs(g.r.im - r.im)<1e-6)
        if (existing) existing.mult += 1; else groups.push({ r, mult:1 })
      }
      const realGroups = groups.filter(g=> Math.abs(g.r.im) < 1e-9)
      const complexGroups = groups.filter(g=> Math.abs(g.r.im) >= 1e-9)
      const complexUsed = new Array(complexGroups.length).fill(false)
      const factorParts: string[] = []
      for (const g of realGroups) {
        const a = g.r.re
        const signPart = a >= 0 ? `(x - ${fmt(a)})` : `(x + ${fmt(Math.abs(a))})`
        factorParts.push(g.mult>1 ? `${signPart}^${g.mult}` : signPart)
      }
      for (let i=0;i<complexGroups.length;i++) {
        if (complexUsed[i]) continue
        const g = complexGroups[i]
        const a = g.r.re, b = Math.abs(g.r.im)
        let cj = -1
        for (let j=i+1;j<complexGroups.length;j++) if (!complexUsed[j]) { const h=complexGroups[j]; if (Math.abs(h.r.re - a)<1e-6 && Math.abs(h.r.im + g.r.im)<1e-6 && h.mult===g.mult) { cj=j; break } }
        if (cj>=0) {
          const c1 = fmt(-2*a)
          const c2 = fmt(a*a + b*b)
            const quad = `(x^2 ${Number(c1)>=0?'+':''}${c1} x + ${c2})`
          factorParts.push(g.mult>1 ? `${quad}^${g.mult}` : quad)
          complexUsed[i]=true; complexUsed[cj]=true
        } else {
          const part = `(x - (${fmt(a)} ${g.r.im>=0?'+':'-'} ${fmt(Math.abs(g.r.im))}i))`
          factorParts.push(g.mult>1 ? `${part}^${g.mult}` : part)
          complexUsed[i]=true
        }
      }
      const original = formatPolynomialMarkdown(P.coeffs)
      const hasComplex = complexGroups.length>0
      const factBody = factorParts.join('') || '1'
      let aligned: string
      if (hasComplex) {
        aligned = `\\begin{aligned} ${original} &= ${factBody} \\ &= ${factBody} \\end{aligned}`
      } else {
        aligned = `\\begin{aligned} ${original} &= ${factBody} \\end{aligned}`
      }
      setFactorBlocks([aligned])

      // verification: compare original P(x) and c*∏(x - r_i) at sample points
      const xs = [-2, -1, -0.5, 0, 0.5, 1, 2]
      const coeffsAsc = P.coeffs.slice()
      const leading = (()=>{
        for (let i=coeffsAsc.length-1;i>=0;i--) {
          const v = coeffsAsc[i]
          if (Math.abs(v) > 1e-12) return v
        }
        return 0
      })()
      const evalPoly = (cs:number[], x:number)=>{
        let acc = 0
        for (let i=cs.length-1;i>=0;i--) acc = acc * x + (cs[i]||0)
        return acc
      }
  // helper removed (unused)
      // compute with careful handling of complex pairs
      const evalProduct2 = (x:number)=>{
        let re = 1, im = 0
        const mulReal = (t:number)=>{ re *= t; im *= t }
        const mulComplex = (ar:number, ai:number)=>{
          const nr = re*ar - im*ai
          const ni = re*ai + im*ar
          re = nr; im = ni
        }
        // real groups
        for (const g of realGroups) {
          const t = x - g.r.re
          for (let k=0;k<g.mult;k++) mulReal(t)
        }
  // complex groups: try to pair conjugates with same multiplicity
  const used = new Array(complexGroups.length).fill(false)
        // pair pass
        for (let i=0;i<complexGroups.length;i++) {
          if (used[i]) continue
          const g = complexGroups[i]
          let cj = -1
          for (let j=i+1;j<complexGroups.length;j++) if (!used[j]) {
            const h = complexGroups[j]
            if (Math.abs(h.r.re - g.r.re)<1e-6 && Math.abs(h.r.im + g.r.im)<1e-6 && h.mult===g.mult) { cj=j; break }
          }
          if (cj>=0) {
            const a = g.r.re, b = Math.abs(g.r.im)
            const quad = (x:number)=> (x*x - 2*a*x + (a*a + b*b))
            for (let k=0;k<g.mult;k++) mulReal(quad(x))
            used[i]=used[cj]=true
          }
        }
        // leftover single complex roots
        for (let i=0;i<complexGroups.length;i++) {
          if (used[i]) continue
          const g = complexGroups[i]
          const a = g.r.re, b = g.r.im
          for (let k=0;k<g.mult;k++) mulComplex(x - a, -b)
          used[i]=true
        }
        return { re, im }
      }
      const errs: number[] = []
      for (const x of xs) {
        const p = evalPoly(coeffsAsc, x)
        const v = evalProduct2(x)
        const vr = leading * v.re
        const vi = leading * v.im
        const diff = Math.hypot(p - vr, vi)
        errs.push(diff)
      }
  const maxErr = Math.max(...errs)
  setVerifyText(`サンプル点 x ∈ { ${xs.join(', ')} } における最大誤差: ${maxErr.toExponential(2)}`)
    } catch(e:any) { setErr(e?.message||String(e)) }
  }

  const runOp = () => {
    if (op==='diff') return diff()
    if (op==='int') return integ()
    if (op==='deg') return degree()
    if (op==='roots') return findRoots()
  }

  let data: PolyUnaryResultDU | null = null
  if (op==='diff' || op==='int') {
    data = { op, value: out ?? undefined, error: err ?? undefined } as any
  } else if (op==='deg') {
    data = { op: 'deg', info: info ?? undefined, error: err ?? undefined }
  } else if (op==='roots') {
    data = { op: 'roots', rootsList: rootsDisplay ?? undefined, factorLines: factorBlocks ?? undefined, verifyText: verifyText ?? undefined, error: err ?? undefined }
  }

  return (
    <PageContainer title="多項式の単項演算" stickyHeader>
        <Panel header={null}>
          <Row
            left={
              <VariablePicker placeholder="変数から代入" allowedKinds={['polynomial']} onPick={(n)=>{ const v:any=get(n); if(v?.kind==='polynomial') setP({ coeffs: v.coeffs.slice() }) }} />
            }
            center={
              <OperationSetting
                operations={operations}
                operation={op}
                onOperationChange={(v)=> setOp(v as any)}
                onAccuracyChange={()=>{}}
                onCalc={runOp}
                calc_button_able
              />
            }
            right={<div />}
          />
        </Panel>
        <PolynomialUnaryInputPanel
          value={P}
          onChange={setP}
          buildSavePayload={()=> ({ kind:'polynomial', coeffs: P.coeffs.slice() })}
        />
  <PolynomialUnaryResultPanel data={data} />
    </PageContainer>
  )
}
