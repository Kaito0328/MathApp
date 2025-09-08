"use client"
import React from 'react'
import PageContainer from '../../../../src/baseComponents/patterns/PageContainer'
import { Button } from '../../../../src/baseComponents/patterns/Button'
import MarkdownMath from '../../../../src/widgets/display/MarkdownMath'
import { formatPolynomialMarkdown } from '../../../../src/utils/format/markdown'
import { polyAdd, polySub, polyMul, polyDiv, polyDivRem, polyGcd, polyLcm } from '../../../../src/wasm/polynomial'
import PolynomialInput from '../../../../src/widgets/input/PolynomialInput'
import { CoreColorKey, SizeKey, VariantKey } from '../../../../src/design/tokens'
import { useVariableStore } from '../../../../src/state/VariableStore'
import { VariablePicker } from '../../../../src/components/variables/VariablePicker'
import { BinaryOperationLayout } from '../../../../src/baseComponents/patterns/OperationLayout'

export default function PolyBinaryPage() {
  const [A, setA] = React.useState<{ coeffs: number[] }>({ coeffs: [1,0,-1] })
  const [B, setB] = React.useState<{ coeffs: number[] }>({ coeffs: [1,1] })
  const [out, setOut] = React.useState<number[] | null>(null)
  const [quot, setQuot] = React.useState<number[] | null>(null)
  const [rem, setRem] = React.useState<number[] | null>(null)
  const [err, setErr] = React.useState<string | null>(null)
  const { upsert, get } = useVariableStore()

  const show = (coeffs: number[] | null) => coeffs ? formatPolynomialMarkdown(coeffs) : ''

  const run = async (kind: 'add'|'sub'|'mul'|'div'|'divrem'|'gcd'|'lcm') => {
    setErr(null); setOut(null); setQuot(null); setRem(null)
    try {
      if (kind==='add') setOut((await polyAdd(A.coeffs,B.coeffs)).coeffs)
      else if (kind==='sub') setOut((await polySub(A.coeffs,B.coeffs)).coeffs)
      else if (kind==='mul') setOut((await polyMul(A.coeffs,B.coeffs)).coeffs)
      else if (kind==='div') setOut((await polyDiv(A.coeffs,B.coeffs)).coeffs)
      else if (kind==='gcd') setOut((await polyGcd(A.coeffs,B.coeffs)).coeffs)
      else if (kind==='lcm') setOut((await polyLcm(A.coeffs,B.coeffs)).coeffs)
      else if (kind==='divrem') { const r = await polyDivRem(A.coeffs,B.coeffs); setQuot(r.q.coeffs); setRem(r.r.coeffs) }
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  return (
    <PageContainer title="多項式の二項演算" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        <BinaryOperationLayout
          header={(
            <div style={{ display:'grid', gridTemplateColumns:'1fr auto 1fr', gap:8, alignItems:'center' }}>
              <div style={{ display:'flex', gap:8, alignItems:'center', flexWrap:'wrap' }}>
                <span>左:</span>
                <VariablePicker placeholder="変数から代入" allowedKinds={['polynomial']} onPick={(n)=>{ const v:any=get(n); if(v?.kind==='polynomial') setA({ coeffs: v.coeffs.slice() }) }} />
              </div>
              <div style={{ justifySelf:'center', display:'flex', gap:8, alignItems:'center', flexWrap:'wrap' }}>
                <label>演算
                  <select onChange={(e)=> run(e.target.value as any)} defaultValue="noop">
                    <option value="noop" disabled>選択</option>
                    <option value="add">+</option>
                    <option value="sub">-</option>
                    <option value="mul">×</option>
                    <option value="div">÷</option>
                    <option value="divrem">商/余り</option>
                    <option value="gcd">最大公約多項式</option>
                    <option value="lcm">最小公倍多項式</option>
                  </select>
                </label>
              </div>
              <div style={{ display:'flex', gap:8, alignItems:'center', flexWrap:'wrap', justifyContent:'flex-end' }}>
                <span>右:</span>
                <VariablePicker placeholder="変数から代入" allowedKinds={['polynomial']} onPick={(n)=>{ const v:any=get(n); if(v?.kind==='polynomial') setB({ coeffs: v.coeffs.slice() }) }} />
              </div>
            </div>
          )}
          leftOperand={(
            <div>
              <div style={{ display:'flex', alignItems:'center', gap:8, marginBottom:6 }}>
                <div style={{ fontWeight:600 }}>左オペランド</div>
                <div style={{ marginLeft:'auto', display:'flex', gap:6 }}>
                  <Button aria-label="保存" title="保存" size={SizeKey.SM} variant={VariantKey.Solid} onClick={()=>{ const name = window.prompt('保存する変数名')?.trim(); if(!name) return; upsert(name, { kind:'polynomial', coeffs: A.coeffs.slice() }) }}>
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                      <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
                      <polyline points="17 21 17 13 7 13 7 21"/>
                      <polyline points="7 3 7 8 15 8"/>
                    </svg>
                  </Button>
                  <Button aria-label="コピー" title="コピー" size={SizeKey.SM} color={CoreColorKey.Base} variant={VariantKey.Solid} onClick={()=> navigator.clipboard?.writeText(show(A.coeffs))}>
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                      <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                      <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                    </svg>
                  </Button>
                </div>
              </div>
              <div style={{ display:'grid' }}>
                <PolynomialInput value={A} onChange={setA} />
              </div>
            </div>
          )}
          rightOperand={(
            <div>
              <div style={{ display:'flex', alignItems:'center', gap:8, marginBottom:6 }}>
                <div style={{ fontWeight:600 }}>右オペランド</div>
                <div style={{ marginLeft:'auto', display:'flex', gap:6 }}>
                  <Button aria-label="保存" title="保存" size={SizeKey.SM} variant={VariantKey.Solid} onClick={()=>{ const name = window.prompt('保存する変数名')?.trim(); if(!name) return; upsert(name, { kind:'polynomial', coeffs: B.coeffs.slice() }) }}>
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                      <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
                      <polyline points="17 21 17 13 7 13 7 21"/>
                      <polyline points="7 3 7 8 15 8"/>
                    </svg>
                  </Button>
                  <Button aria-label="コピー" title="コピー" size={SizeKey.SM} color={CoreColorKey.Base} variant={VariantKey.Solid} onClick={()=> navigator.clipboard?.writeText(show(B.coeffs))}>
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                      <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                      <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                    </svg>
                  </Button>
                </div>
              </div>
              <div style={{ display:'grid' }}>
                <PolynomialInput value={B} onChange={setB} />
              </div>
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
              {(quot||rem) && (
                <div style={{ display:'grid', gap:8 }}>
                  <div>商/余り</div>
                  {quot && (
                    <div style={{ display:'grid', gap:6 }}>
                      <div style={{ display:'flex', gap:6, alignItems:'center' }}>
                        <div>q(x)</div>
                        <div style={{ marginLeft:'auto', display:'flex', gap:6 }}>
                          <Button aria-label="保存" title="保存" size={SizeKey.SM} variant={VariantKey.Solid} onClick={()=>{ const name = window.prompt('q(x) を保存する変数名')?.trim(); if(!name) return; upsert(name, { kind:'polynomial', coeffs: quot.slice() }) }}>
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                              <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
                              <polyline points="17 21 17 13 7 13 7 21"/>
                              <polyline points="7 3 7 8 15 8"/>
                            </svg>
                          </Button>
                          <Button aria-label="コピー" title="コピー" size={SizeKey.SM} color={CoreColorKey.Base} variant={VariantKey.Solid} onClick={()=> navigator.clipboard?.writeText(`q(x) = ${show(quot)}`)}>
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                              <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                              <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                            </svg>
                          </Button>
                        </div>
                      </div>
                      <MarkdownMath math={`q(x) = ${show(quot)}`} />
                    </div>
                  )}
                  {rem && (
                    <div style={{ display:'grid', gap:6 }}>
                      <div style={{ display:'flex', gap:6, alignItems:'center' }}>
                        <div>r(x)</div>
                        <div style={{ marginLeft:'auto', display:'flex', gap:6 }}>
                          <Button aria-label="保存" title="保存" size={SizeKey.SM} variant={VariantKey.Solid} onClick={()=>{ const name = window.prompt('r(x) を保存する変数名')?.trim(); if(!name) return; upsert(name, { kind:'polynomial', coeffs: rem.slice() }) }}>
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                              <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
                              <polyline points="17 21 17 13 7 13 7 21"/>
                              <polyline points="7 3 7 8 15 8"/>
                            </svg>
                          </Button>
                          <Button aria-label="コピー" title="コピー" size={SizeKey.SM} color={CoreColorKey.Base} variant={VariantKey.Solid} onClick={()=> navigator.clipboard?.writeText(`r(x) = ${show(rem)}`)}>
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                              <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                              <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                            </svg>
                          </Button>
                        </div>
                      </div>
                      <MarkdownMath math={`r(x) = ${show(rem)}`} />
                    </div>
                  )}
                </div>
              )}
            </div>
          )}
        />
      </div>
    </PageContainer>
  )
}
