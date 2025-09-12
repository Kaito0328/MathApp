"use client"
import React from 'react'
import SectionPanelWithTitle from '../../composites/panels/SectionPanelWithTitle'
import VectorInput from '../../../widgets/input/VectorInput'
import NumberCellInput from '../../../baseComponents/input/NumberCellInput'
import LabeledMathResult from '../result/LabeledMathResult'
import { Button } from '../../../baseComponents/controls/Button'

export const RecurrencePanel: React.FC = () => {
  // a(n) = c1 a(n-1) + ... + ck a(n-k)
  const [k, setK] = React.useState<number>(2)
  const [coeffs, setCoeffs] = React.useState<number[]>([1, -1])
  const [init, setInit] = React.useState<number[]>([1, 1])
  const [outStr, setOutStr] = React.useState<string>('')
  const [sumStr, setSumStr] = React.useState<string>('')
  const [err, setErr] = React.useState<string>('')
  const [varName, setVarName] = React.useState<string>('n')
  // 非同次項 GeneralTerm: Q(n)·r^n を複数管理
  type Term = { poly: number[]; baseRe: number; baseIm: number }
  const [terms, setTerms] = React.useState<Term[]>([])

  const addTerm = () => setTerms((ts) => [...ts, { poly: [1], baseRe: 2, baseIm: 0 }])
  const removeTerm = (i: number) => setTerms((ts) => ts.filter((_, j) => j !== i))
  const updateTerm = (i: number, patch: Partial<Term>) => setTerms((ts) => ts.map((t, j) => j===i ? { ...t, ...patch } : t))

  const ensureLen = (arr: number[], n: number) => {
    const a = arr.slice(0, n)
    while (a.length < n) a.push(0)
    return a
  }

  React.useEffect(() => {
    setCoeffs((c) => ensureLen(c, k))
    setInit((a) => ensureLen(a, k))
  }, [k])

  const run = async () => {
    setErr(''); setOutStr(''); setSumStr('')
    try {
      const { getWasm } = await import('../../../wasm/loader')
      const wasm: any = await getWasm()
      const gtObjects: any[] = terms.map(t => new wasm.GeneralTerm(Float64Array.from(t.poly), t.baseRe, t.baseIm))
      const rr = new wasm.RecurrenceRelation(
        Float64Array.from(coeffs),
        gtObjects,
        Float64Array.from(init)
      )
  const cf = rr.solve()
  const s = String(cf.toString?.(varName, true) ?? '')
      setOutStr(s)
      const sum = cf.partialSum()
  const sumStr = String(sum.toString?.(varName, true) ?? '')
      setSumStr(sumStr)
  try { cf.free?.(); sum.free?.(); rr.free?.(); gtObjects.forEach(g=>g.free?.()) } catch { /* noop */ }
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  // preview tex can be composed in LabeledMathResult

  return (
    <SectionPanelWithTitle title="漸化式 → ClosedForm" showSave={false} showCopy={false}>
      <div style={{ display:'grid', gap:12 }}>
        <div style={{ display:'flex', gap:12, alignItems:'center', flexWrap:'wrap' }}>
          <div style={{ display:'flex', gap:6, alignItems:'center' }}>
            <span>次数 k=</span>
            <NumberCellInput value={k} onChange={(v)=> setK(Math.max(0, Math.floor(v)))} width={80} />
          </div>
          <div style={{ display:'flex', gap:6, alignItems:'center' }}>
            <span>変数</span>
            <input value={varName} onChange={(e)=> setVarName(e.target.value || 'n')} style={{ width: 56 }} />
          </div>
          <div style={{ marginLeft:'auto' }}>
            <Button onClick={run}>解く</Button>
          </div>
        </div>
        <div style={{ display:'flex', gap:24, alignItems:'center', flexWrap:'wrap' }}>
          <div>
            <div style={{ fontSize:12, opacity:0.8 }}>係数 [c1..ck]</div>
            <VectorInput value={{ data: coeffs }} onChange={(v)=> setCoeffs(v.data)} orientation="row" length={k} />
          </div>
          <div>
            <div style={{ fontSize:12, opacity:0.8 }}>初期値 [a0..a_{k-1}]</div>
            <VectorInput value={{ data: init }} onChange={(v)=> setInit(v.data)} orientation="row" length={k} />
          </div>
        </div>

        <SectionPanelWithTitle title="非同次項（任意）" showSave={false} showCopy={false}>
          <div style={{ display:'grid', gap:10 }}>
            {terms.map((t, i) => (
              <div key={i} style={{ display:'flex', gap:16, alignItems:'center', flexWrap:'wrap' }}>
                <div style={{ fontSize:12, opacity:0.8 }}>Q(n) 係数</div>
                <VectorInput value={{ data: t.poly }} onChange={(v)=> updateTerm(i, { poly: v.data })} orientation="row" />
                <div style={{ display:'flex', gap:6, alignItems:'center' }}>
                  <span>基数 r（複素可）:</span>
                  <NumberCellInput value={t.baseRe} onChange={(v)=> updateTerm(i, { baseRe: v })} width={96} />
                  <span>+ i</span>
                  <NumberCellInput value={t.baseIm} onChange={(v)=> updateTerm(i, { baseIm: v })} width={96} />
                </div>
                <div style={{ marginLeft:'auto' }}>
                  <Button onClick={()=> removeTerm(i)}>削除</Button>
                </div>
              </div>
            ))}
            <div>
              <Button onClick={addTerm}>+ 一般項を追加（Q(n)·r^n）</Button>
            </div>
          </div>
        </SectionPanelWithTitle>
        {err && <div style={{ color:'crimson' }}>{err}</div>}
        {outStr && (
          <LabeledMathResult label={"a(n) ="} body={outStr} />
        )}
        {sumStr && (
          <LabeledMathResult label={"S(n) ="} body={sumStr} />
        )}
      </div>
    </SectionPanelWithTitle>
  )
}

export default RecurrencePanel
