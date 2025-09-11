"use client"
import React from 'react'
import PageContainer from '../../../../src/baseComponents/layout/PageContainer'
// removed unused Button/SizeKey/VariantKey in favor of OperationSetting
import { getWasm } from '../../../../src/wasm/loader'
import { formatComplexMarkdown, formatRationalFunctionMarkdown } from '../../../../src/utils/format/markdown'
import Stack from '../../../../src/baseComponents/layout/Stack'
import Panel from '../../../../src/baseComponents/layout/Panel'
import { RationalUnaryInputPanel, RationalUnaryResultPanel } from '../../../../src/components/rationalFunction/RationalUnaryPanels'
import OperationSetting from '../../../../src/components/operations/OperationSetting'
import Row from '../../../../src/baseComponents/layout/Row'

export default function RationalUnaryPage() {
  type RationalUnaryOp = 'diff'|'simplify'|'zeros'|'poles'|'pfe'
  const operations: { label: string; value: RationalUnaryOp }[] = [
    { label: '微分', value: 'diff' },
    { label: '約分', value: 'simplify' },
    { label: '根', value: 'zeros' },
    { label: '極', value: 'poles' },
    { label: '部分分数分解', value: 'pfe' },
  ]
  const [F, setF] = React.useState({ numerator: { coeffs: [1,0] }, denominator: { coeffs: [1,1] } })
  const [op, setOp] = React.useState<'diff'|'simplify'|'zeros'|'poles'|'pfe'>('diff')
  const [out, setOut] = React.useState<typeof F | null>(null)
  const [zerosList, setZerosList] = React.useState<string | null>(null)
  const [zerosFactor, setZerosFactor] = React.useState<string | null>(null)
  const [polesList, setPolesList] = React.useState<string | null>(null)
  const [polesFactor, setPolesFactor] = React.useState<string | null>(null)
  const [pfeLines, setPfeLines] = React.useState<string[] | null>(null)
  const [err, setErr] = React.useState<string | null>(null)
  const [precision, setPrecision] = React.useState<number>(6)

  const compute = async () => {
  setErr(null); setOut(null); setZerosList(null); setZerosFactor(null); setPolesList(null); setPolesFactor(null); setPfeLines(null)
    try {
      const wasm: any = await getWasm()
  const f = new wasm.RationalFunctionF64(Float64Array.from(F.numerator.coeffs), Float64Array.from(F.denominator.coeffs))
      const fmt = (x:number) => Number(x.toFixed(precision))
      const formatComplex = (re:number, im:number) => `${fmt(re)}${im>=0?'+':'-'}${fmt(Math.abs(im))}i`
      const factorPoly = (coeffs: number[]) => {
        try {
          if (!coeffs.length) return null
          // normalize leading coefficient separately
          let leading = coeffs[coeffs.length-1]
          if (Math.abs(leading) < 1e-12) return null
          const sign = leading < 0 ? -1 : 1
          coeffs = coeffs.map(c=> c/leading)
          if (sign < 0) leading = -leading
          const poly = new wasm.PolynomialF64(Float64Array.from(coeffs))
          const inter = poly.findRoots() as Float64Array
          const roots: {re:number; im:number}[] = []
          for (let i=0;i<inter.length;i+=2) {
            const re = inter[i]; const im = inter[i+1]
            if (!Number.isFinite(re) || !Number.isFinite(im)) continue
            roots.push({ re, im })
          }
          poly.free?.()
          if (!roots.length) return null
          const used = new Array(roots.length).fill(false)
          const factors: string[] = []
          for (let i=0;i<roots.length;i++) {
            if (used[i]) continue
            const r = roots[i]
            if (Math.abs(r.im) < 1e-9) {
              const a = fmt(r.re)
              const term = r.re >= 0 ? `(x - ${a})` : `(x + ${fmt(Math.abs(r.re))})`
              factors.push(term)
              used[i]=true
            } else {
              let found = -1
              for (let j=i+1;j<roots.length;j++) if (!used[j]) { const s=roots[j]; if (Math.abs(s.re-r.re)<1e-6 && Math.abs(s.im + r.im)<1e-6) { found=j; break } }
              if (found>=0) {
                const a=r.re, b=Math.abs(r.im)
                const c2 = fmt(a*a + b*b)
                const c1 = fmt(-2*a)
                // x^2 -2a x + (a^2+b^2)
                const c1str = Number(c1)===0? '' : ` ${Number(c1)>=0?'+':''}${c1} x`
                factors.push(`(x^2${c1str} + ${c2})`)
                used[i]=true; used[found]=true
              } else {
                const fc = formatComplex(r.re, r.im)
                factors.push(`(x - ${fc})`)
                used[i]=true
              }
            }
          }
          const leadStr = (Math.abs(leading-1) < 1e-12) ? '' : `${fmt(leading)}`
          return (leadStr? leadStr + ' ' : '') + factors.join(' ')
        } catch { return null }
      }
      if (op === 'diff') {
        const d = f.differentiate()
        d.simplify?.()
        setOut({ numerator: { coeffs: Array.from(d.numeratorCoeffs() as Float64Array) }, denominator: { coeffs: Array.from(d.denominatorCoeffs() as Float64Array) } })
        d.free?.()
      } else if (op === 'simplify') {
        f.simplify()
        setOut({ numerator: { coeffs: Array.from(f.numeratorCoeffs() as Float64Array) }, denominator: { coeffs: Array.from(f.denominatorCoeffs() as Float64Array) } })
  } else if (op === 'zeros') {
        const numPoly = new wasm.PolynomialF64(f.numeratorCoeffs())
        const inter = numPoly.findRoots() as Float64Array
        numPoly.free?.()
        const zs: { re:number; im:number }[] = []
        for (let i=0;i<inter.length;i+=2) {
          const re=inter[i], im=inter[i+1]
          if (!Number.isFinite(re) || !Number.isFinite(im)) continue
          zs.push({ re, im })
        }
        const list = zs.length ? formatComplexMarkdown(zs as any, { orientation:'row' }) : 'なし'
        const fact = factorPoly(Array.from(f.numeratorCoeffs() as Float64Array))
        setZerosList(zs.length ? list : 'ゼロなし')
        if (fact) setZerosFactor(fact)
      } else if (op === 'poles') {
        const poles = f.findPoles() as any[]
        const pts = poles.filter((p:any)=> Number.isFinite(p.re)&&Number.isFinite(p.im)).map((p:any)=> ({ re:p.re, im:p.im }))
        const list = pts.length ? formatComplexMarkdown(pts as any, { orientation:'row' }) : '極なし'
        const fact = factorPoly(Array.from(f.denominatorCoeffs() as Float64Array))
        setPolesList(list)
        if (fact) setPolesFactor(fact)
      } else if (op === 'pfe') {
        const res = f.partialFractionExpansion()
        let pieces: string[] = []
        if (Array.isArray(res)) {
          pieces = res.map((r:any)=> typeof r === 'string' ? r : JSON.stringify(r))
        } else if (typeof res === 'object') {
          pieces = Object.keys(res).map(k=> `${k}: ${JSON.stringify((res as any)[k])}`)
        } else if (typeof res === 'string') pieces=[res]
        // Attempt to parse terms like A/(x+a)^k or polynomials; keep simple join
        const cleanedTerms = pieces.map(p=> p.replace(/"/g,'').trim())
        const original = formatRationalFunctionMarkdown({ numerator:{ coeffs:Array.from(f.numeratorCoeffs() as Float64Array)}, denominator:{ coeffs:Array.from(f.denominatorCoeffs() as Float64Array)} } as any)
        const rhs = cleanedTerms.join(' + ')
        const aligned = `\\begin{aligned} ${original} &= ${rhs} \\end{aligned}`
        setPfeLines([aligned])
      }
      f.free?.()
    } catch (e:any) { setErr(e?.message||String(e)) }
  }

  return (
    <PageContainer title="有理関数の単項演算" stickyHeader>
      <Stack gap={12}>
        <Panel header={null}>
          <Row
            left={<div />}
            center={
              <OperationSetting
                operations={operations}
                operation={op}
                onOperationChange={(v)=> setOp(v as RationalUnaryOp)}
                accuracy={precision}
                onAccuracyChange={(n)=> setPrecision(Math.max(0, Math.min(12, Number(n)||0)))}
                accuracy_able
                onCalc={compute}
                calc_button_able
              />
            }
            right={<div />}
          />
        </Panel>
        <RationalUnaryInputPanel
          value={F}
          onChange={setF}
          buildSavePayload={()=> ({ kind:'rational', numerator: F.numerator.coeffs.slice(), denominator: F.denominator.coeffs.slice() })}
        />
        <RationalUnaryResultPanel
          result={out}
          error={err}
          zerosList={zerosList || undefined}
          zerosFactor={zerosFactor || undefined}
          polesList={polesList || undefined}
          polesFactor={polesFactor || undefined}
          pfeLines={pfeLines || undefined}
          buildSavePayload={()=> out ? { kind:'rational', numerator: out.numerator.coeffs.slice(), denominator: out.denominator.coeffs.slice() } : null}
        />
      </Stack>
    </PageContainer>
  )
}
