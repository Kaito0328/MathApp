"use client"
import React from 'react'
import PageContainer from '../../../../src/baseComponents/patterns/PageContainer'
import { Button } from '../../../../src/baseComponents/patterns/Button'
import RationalFunctionInput from '../../../../src/widgets/input/RationalFunctionInput'
import { CoreColorKey, SizeKey, VariantKey } from '../../../../src/design/tokens'
import MarkdownMath from '../../../../src/widgets/display/MarkdownMath'
import { formatRationalFunctionMarkdown } from '../../../../src/utils/format/markdown'
import { UnaryOperationLayout } from '../../../../src/baseComponents/patterns/OperationLayout'
import { getWasm } from '../../../../src/wasm/loader'
import { formatComplexMarkdown } from '../../../../src/utils/format/markdown'

export default function RationalUnaryPage() {
  const [F, setF] = React.useState({ numerator: { coeffs: [1,0] }, denominator: { coeffs: [1,1] } })
  const [op, setOp] = React.useState<'diff'|'simplify'|'zeros'|'poles'|'pfe'>('diff')
  const [out, setOut] = React.useState<typeof F | null>(null)
  const [extraMd, setExtraMd] = React.useState<string | null>(null)
  const [err, setErr] = React.useState<string | null>(null)

  const compute = async () => {
    setErr(null); setOut(null); setExtraMd(null)
    try {
      const wasm: any = await getWasm()
      const f = new wasm.RationalFunctionF64(Float64Array.from(F.numerator.coeffs), Float64Array.from(F.denominator.coeffs))
      if (op === 'diff') {
        const d = f.differentiate()
        d.simplify?.()
        setOut({ numerator: { coeffs: Array.from(d.numeratorCoeffs() as Float64Array) }, denominator: { coeffs: Array.from(d.denominatorCoeffs() as Float64Array) } })
        d.free?.()
      } else if (op === 'simplify') {
        f.simplify()
        setOut({ numerator: { coeffs: Array.from(f.numeratorCoeffs() as Float64Array) }, denominator: { coeffs: Array.from(f.denominatorCoeffs() as Float64Array) } })
      } else if (op === 'zeros') {
        // zeros are roots of numerator
        const numPoly = new wasm.PolynomialF64(f.numeratorCoeffs())
        const inter = numPoly.findRoots() as Float64Array
        numPoly.free?.()
        const zs: { re:number; im:number }[] = []
        for (let i=0;i<inter.length;i+=2) zs.push({ re: inter[i], im: inter[i+1] })
        const md = `ゼロ = ${zs.length? formatComplexMarkdown(zs as any, { orientation:'row' }) : 'なし'}`
        setExtraMd(md)
      } else if (op === 'poles') {
        const poles = f.findPoles() as any[]
        // poles: WasmRoot{re,im,multiplicity}
        const items = poles.map((p:any)=> `${p.multiplicity}重: (${p.re.toFixed(6)} ${p.im>=0?'+':'-'} ${Math.abs(p.im).toFixed(6)} i)`).join(', ')
        setExtraMd(`極 = ${items || 'なし'}`)
      } else if (op === 'pfe') {
        const res = f.partialFractionExpansion()
        // render via toString-like if available, else JSON
        const md = typeof res === 'string' ? res : `部分分数分解: ${JSON.stringify(res)}`
        setExtraMd(md)
      }
      f.free?.()
    } catch (e:any) { setErr(e?.message||String(e)) }
  }

  return (
    <PageContainer title="有理関数の単項演算" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        <UnaryOperationLayout
          header={(
            <div style={{ display:'flex', gap:8, alignItems:'center', flexWrap:'wrap' }}>
              <label>演算
                <select value={op} onChange={(e)=> setOp(e.target.value as any)}>
                  <option value="diff">微分</option>
                  <option value="simplify">約分</option>
                  <option value="zeros">ゼロ</option>
                  <option value="poles">極</option>
                  <option value="pfe">部分分数分解</option>
                </select>
              </label>
              <Button onClick={compute}>計算</Button>
            </div>
          )}
          input={(
            <div>
              <div style={{ fontWeight:600, marginBottom:6 }}>F</div>
              <RationalFunctionInput value={F} onChange={setF} />
            </div>
          )}
          result={(
            <div>
              {err && <div style={{ color:'crimson' }}>{err}</div>}
              {out && <MarkdownMath math={formatRationalFunctionMarkdown({ numerator: out.numerator, denominator: out.denominator } as any)} />}
              {extraMd && (
                <div style={{ display:'grid', gap:6, marginTop:8 }}>
                  <div style={{ display:'flex', alignItems:'center', gap:8 }}>
                    <div>結果</div>
                    <div style={{ marginLeft:'auto', display:'flex', gap:6 }}>
                      <Button aria-label="コピー" title="コピー" size={SizeKey.SM} color={CoreColorKey.Base} variant={VariantKey.Solid} onClick={()=> navigator.clipboard?.writeText(extraMd!)}>
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                          <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                          <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                        </svg>
                      </Button>
                    </div>
                  </div>
                  <MarkdownMath math={extraMd} />
                </div>
              )}
            </div>
          )}
        />
      </div>
    </PageContainer>
  )
}
