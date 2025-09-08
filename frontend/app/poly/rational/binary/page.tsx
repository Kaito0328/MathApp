"use client"
import React from 'react'
import PageContainer from '../../../../src/baseComponents/patterns/PageContainer'
import { Button } from '../../../../src/baseComponents/patterns/Button'
import { CoreColorKey, SizeKey, VariantKey } from '../../../../src/design/tokens'
import RationalFunctionInput from '../../../../src/widgets/input/RationalFunctionInput'
import MarkdownMath from '../../../../src/widgets/display/MarkdownMath'
import { formatRationalFunctionMarkdown } from '../../../../src/utils/format/markdown'
import { VariablePicker } from '../../../../src/components/variables/VariablePicker'
import { useVariableStore } from '../../../../src/state/VariableStore'
import { BinaryOperationLayout } from '../../../../src/baseComponents/patterns/OperationLayout'
import { getWasm } from '../../../../src/wasm/loader'

export default function RationalBinaryPage() {
  const [F, setF] = React.useState({ numerator: { coeffs: [1,0] }, denominator: { coeffs: [1] } })
  const [G, setG] = React.useState({ numerator: { coeffs: [1] }, denominator: { coeffs: [1,1] } })
  const [op, setOp] = React.useState<'add'|'sub'|'mul'|'div'>('add')
  const [precision, setPrecision] = React.useState<number>(6)
  const [out, setOut] = React.useState<typeof F | null>(null)
  const [err, setErr] = React.useState<string | null>(null)
  const { get, upsert } = useVariableStore()

  const compute = async () => {
    setErr(null); setOut(null)
    try {
      const wasm: any = await getWasm()
      const f = new wasm.RationalFunctionF64(Float64Array.from(F.numerator.coeffs), Float64Array.from(F.denominator.coeffs))
      const g = new wasm.RationalFunctionF64(Float64Array.from(G.numerator.coeffs), Float64Array.from(G.denominator.coeffs))
      let h: any
      if (op==='add') h = f.add(g)
      else if (op==='sub') h = f.sub(g)
      else if (op==='mul') h = f.mul(g)
      else if (op==='div') h = f.div(g)
      else h = f
      h.simplify?.()
      const num = Array.from(h.numeratorCoeffs() as Float64Array)
      const den = Array.from(h.denominatorCoeffs() as Float64Array)
      setOut({ numerator: { coeffs: num }, denominator: { coeffs: den } })
      f.free?.(); g.free?.(); h.free?.()
    } catch (e:any) { setErr(e?.message || String(e)) }
  }

  const show = (rf: typeof F | null) => rf ? formatRationalFunctionMarkdown({ numerator: { coeffs: rf.numerator.coeffs.map(x=> Number(x.toFixed(precision))) }, denominator: { coeffs: rf.denominator.coeffs.map(x=> Number(x.toFixed(precision))) } } as any) : ''

  return (
    <PageContainer title="有理関数の二項演算" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        <BinaryOperationLayout
          header={(
            <div style={{ display:'grid', gridTemplateColumns:'1fr auto 1fr', gap:8, alignItems:'center' }}>
              <div style={{ display:'flex', gap:8, alignItems:'center', flexWrap:'wrap' }}>
                <span>左:</span>
                <VariablePicker placeholder="変数から代入" allowedKinds={['rational']} onPick={(n)=>{ const v:any=get(n); if(v?.kind==='rational') setF({ numerator:{ coeffs: v.numerator.slice() }, denominator:{ coeffs: v.denominator.slice() } }) }} />
              </div>
              <div style={{ justifySelf:'center', display:'flex', gap:12, alignItems:'center', flexWrap:'wrap' }}>
                <label>演算
                  <select value={op} onChange={(e)=> setOp(e.target.value as any)}>
                    <option value="add">+</option>
                    <option value="sub">-</option>
                    <option value="mul">×</option>
                    <option value="div">÷</option>
                  </select>
                </label>
                <label>精度
                  <input type="number" min={0} max={12} value={precision} onChange={(e)=> setPrecision(Math.max(0, Math.min(12, Number(e.target.value||0))))} style={{ width:72 }} />
                </label>
                <Button onClick={compute}>計算</Button>
              </div>
              <div style={{ display:'flex', gap:8, alignItems:'center', flexWrap:'wrap', justifyContent:'flex-end' }}>
                <span>右:</span>
                <VariablePicker placeholder="変数から代入" allowedKinds={['rational']} onPick={(n)=>{ const v:any=get(n); if(v?.kind==='rational') setG({ numerator:{ coeffs: v.numerator.slice() }, denominator:{ coeffs: v.denominator.slice() } }) }} />
              </div>
            </div>
          )}
          leftOperand={(
            <div>
              <div style={{ display:'flex', alignItems:'center', gap:8, marginBottom:6 }}>
                <div style={{ fontWeight:600 }}>左オペランド</div>
                <div style={{ marginLeft:'auto', display:'flex', gap:6 }}>
                  <Button aria-label="保存" title="保存" size={SizeKey.SM} variant={VariantKey.Solid} onClick={()=>{ const name = window.prompt('保存する変数名')?.trim(); if(!name) return; upsert(name, { kind:'rational', numerator: F.numerator.coeffs.slice(), denominator: F.denominator.coeffs.slice() }) }}>
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                      <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
                      <polyline points="17 21 17 13 7 13 7 21"/>
                      <polyline points="7 3 7 8 15 8"/>
                    </svg>
                  </Button>
                  <Button aria-label="コピー" title="コピー" size={SizeKey.SM} color={CoreColorKey.Base} variant={VariantKey.Solid} onClick={()=> navigator.clipboard?.writeText(show(F))}>
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                      <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                      <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                    </svg>
                  </Button>
                </div>
              </div>
              <RationalFunctionInput value={F} onChange={setF} />
            </div>
          )}
          rightOperand={(
            <div>
              <div style={{ display:'flex', alignItems:'center', gap:8, marginBottom:6 }}>
                <div style={{ fontWeight:600 }}>右オペランド</div>
                <div style={{ marginLeft:'auto', display:'flex', gap:6 }}>
                  <Button aria-label="保存" title="保存" size={SizeKey.SM} variant={VariantKey.Solid} onClick={()=>{ const name = window.prompt('保存する変数名')?.trim(); if(!name) return; upsert(name, { kind:'rational', numerator: G.numerator.coeffs.slice(), denominator: G.denominator.coeffs.slice() }) }}>
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                      <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
                      <polyline points="17 21 17 13 7 13 7 21"/>
                      <polyline points="7 3 7 8 15 8"/>
                    </svg>
                  </Button>
                  <Button aria-label="コピー" title="コピー" size={SizeKey.SM} color={CoreColorKey.Base} variant={VariantKey.Solid} onClick={()=> navigator.clipboard?.writeText(show(G))}>
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                      <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                      <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                    </svg>
                  </Button>
                </div>
              </div>
              <RationalFunctionInput value={G} onChange={setG} />
            </div>
          )}
          result={(
            <div>
              {err && <div style={{ color:'crimson' }}>{err}</div>}
              {out && (
                <div style={{ display:'grid', gap:6 }}>
                  <div style={{ display:'flex', alignItems:'center', gap:8 }}>
                    <div>結果</div>
                    <div style={{ marginLeft:'auto', display:'flex', gap:6 }}>
                      <Button aria-label="保存" title="保存" size={SizeKey.SM} variant={VariantKey.Solid} onClick={()=>{ const name = window.prompt('保存する変数名')?.trim(); if(!name) return; upsert(name, { kind:'rational', numerator: out.numerator.coeffs.slice(), denominator: out.denominator.coeffs.slice() }) }}>
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                          <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
                          <polyline points="17 21 17 13 7 13 7 21"/>
                          <polyline points="7 3 7 8 15 8"/>
                        </svg>
                      </Button>
                      <Button aria-label="コピー" title="コピー" size={SizeKey.SM} color={CoreColorKey.Base} variant={VariantKey.Solid} onClick={()=> navigator.clipboard?.writeText(show(out))}>
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                          <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                          <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                        </svg>
                      </Button>
                    </div>
                  </div>
                  <MarkdownMath math={show(out)} />
                </div>
              )}
            </div>
          )}
        />
      </div>
    </PageContainer>
  )
}
