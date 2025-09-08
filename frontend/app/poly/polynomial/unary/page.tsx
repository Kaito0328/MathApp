"use client"
import React from 'react'
import { Button } from '../../../../src/baseComponents/patterns/Button'
import MarkdownMath from '../../../../src/widgets/display/MarkdownMath'
import { formatPolynomialMarkdown } from '../../../../src/utils/format/markdown'
import { polyDiff, polyInt, polyNormalize } from '../../../../src/wasm/polynomial'
import { getWasm } from '../../../../src/wasm/loader'
import { formatComplexMarkdown } from '../../../../src/utils/format/markdown'
import PolynomialInput from '../../../../src/widgets/input/PolynomialInput'
import PageContainer from '../../../../src/baseComponents/patterns/PageContainer'
import { CoreColorKey, SizeKey, VariantKey } from '../../../../src/design/tokens'
import { useVariableStore } from '../../../../src/state/VariableStore'
import { VariablePicker } from '../../../../src/components/variables/VariablePicker'
import { UnaryOperationLayout } from '../../../../src/baseComponents/patterns/OperationLayout'

export default function PolyUnaryPage() {
  const [P, setP] = React.useState<{ coeffs: number[] }>({ coeffs: [1,0,-1] })
  const [out, setOut] = React.useState<number[] | null>(null)
  const [info, setInfo] = React.useState<string | null>(null)
  const [rootsMd, setRootsMd] = React.useState<string | null>(null)
  const [err, setErr] = React.useState<string | null>(null)
  const { upsert, get } = useVariableStore()
  const show = (cs: number[] | null) => cs ? formatPolynomialMarkdown(cs) : ''

  const diff = async () => { setErr(null); setInfo(null); setOut(null); setRootsMd(null); try { setOut((await polyDiff(P.coeffs)).coeffs) } catch(e:any){ setErr(e?.message||String(e)) } }
  const integ = async () => { setErr(null); setInfo(null); setOut(null); setRootsMd(null); try { setOut((await polyInt(P.coeffs)).coeffs) } catch(e:any){ setErr(e?.message||String(e)) } }
  const degree = async () => { setErr(null); setInfo(null); setOut(null); setRootsMd(null); try { const p = await polyNormalize(P.coeffs); setInfo(`deg = ${Math.max(0, p.coeffs.length-1)}`) } catch(e:any){ setErr(e?.message||String(e)) } }
  const findRoots = async () => {
    setErr(null); setInfo(null); setOut(null); setRootsMd(null)
    try {
      const wasm: any = await getWasm()
      const poly = new wasm.PolynomialF64(Float64Array.from(P.coeffs))
      const inter = poly.findRoots() as Float64Array
      const zs: { re: number; im: number }[] = []
      for (let i=0; i<inter.length; i+=2) zs.push({ re: inter[i], im: inter[i+1] })
      poly.free?.()
      const md = formatComplexMarkdown(zs, { orientation: 'row' })
      setRootsMd(`根 = ${md}`)
    } catch(e:any) { setErr(e?.message||String(e)) }
  }

  return (
    <PageContainer title="多項式の単項演算" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        <UnaryOperationLayout
          header={(
            <div style={{ display:'grid', gridTemplateColumns:'1fr 1fr', gap:8, alignItems:'center' }}>
              <div style={{ display:'flex', gap:8, alignItems:'center', flexWrap:'wrap' }}>
                <VariablePicker placeholder="変数から代入" allowedKinds={['polynomial']} onPick={(n)=>{ const v:any=get(n); if(v?.kind==='polynomial') setP({ coeffs: v.coeffs.slice() }) }} />
              </div>
              <div style={{ display:'flex', gap:8, alignItems:'center', justifyContent:'flex-end', flexWrap:'wrap' }}>
                <label>演算
                  <select onChange={(e)=> { const k = e.target.value as any; if (k==='diff') diff(); if (k==='int') integ(); if (k==='deg') degree(); if (k==='roots') findRoots() }} defaultValue="noop">
                    <option value="noop" disabled>選択</option>
                    <option value="diff">微分</option>
                    <option value="int">積分</option>
                    <option value="deg">次数</option>
                    <option value="roots">根を求める</option>
                  </select>
                </label>
              </div>
            </div>
          )}
          input={(
            <div>
              <div style={{ display:'flex', alignItems:'center', gap:8, marginBottom:6 }}>
                <div style={{ fontWeight:600 }}>入力</div>
                <div style={{ marginLeft:'auto', display:'flex', gap:6 }}>
                  <Button aria-label="保存" title="保存" size={SizeKey.SM} variant={VariantKey.Solid} onClick={()=>{ const name = window.prompt('保存する変数名')?.trim(); if(!name) return; upsert(name, { kind:'polynomial', coeffs: P.coeffs.slice() }) }}>
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                      <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
                      <polyline points="17 21 17 13 7 13 7 21"/>
                      <polyline points="7 3 7 8 15 8"/>
                    </svg>
                  </Button>
                  <Button aria-label="コピー" title="コピー" size={SizeKey.SM} color={CoreColorKey.Base} variant={VariantKey.Solid} onClick={()=> navigator.clipboard?.writeText(show(P.coeffs))}>
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                      <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                      <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                    </svg>
                  </Button>
                </div>
              </div>
              <PolynomialInput value={{ coeffs: P.coeffs }} onChange={setP} />
            </div>
          )}
          result={(
            <div>
              {err && <div style={{ color:'crimson' }}>{err}</div>}
              {info && <div>{info}</div>}
              {out && (
                <div style={{ display:'grid', gap:6 }}>
                  <div style={{ display:'flex', alignItems:'center', gap:8 }}>
                    <div>結果</div>
                    <div style={{ marginLeft:'auto', display:'flex', gap:6 }}>
                      <Button aria-label="保存" title="保存" size={SizeKey.SM} variant={VariantKey.Solid} onClick={()=>{ const name = window.prompt('保存する変数名')?.trim(); if(!name) return; upsert(name, { kind:'polynomial', coeffs: out.slice() }) }}>
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
              {rootsMd && (
                <div style={{ display:'grid', gap:6 }}>
                  <div style={{ display:'flex', alignItems:'center', gap:8 }}>
                    <div>根</div>
                    <div style={{ marginLeft:'auto', display:'flex', gap:6 }}>
                      <Button aria-label="コピー" title="コピー" size={SizeKey.SM} color={CoreColorKey.Base} variant={VariantKey.Solid} onClick={()=> navigator.clipboard?.writeText(rootsMd!)}>
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                          <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                          <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                        </svg>
                      </Button>
                    </div>
                  </div>
                  <MarkdownMath math={rootsMd} />
                </div>
              )}
            </div>
          )}
        />
      </div>
    </PageContainer>
  )
}
