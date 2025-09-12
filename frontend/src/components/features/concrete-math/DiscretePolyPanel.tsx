"use client"
import React from 'react'
import SectionPanelWithTitle from '../../composites/panels/SectionPanelWithTitle'
import NumberCellInput from '../../../baseComponents/input/NumberCellInput'
import PolynomialInput from '../../../widgets/input/PolynomialInput'
import { formatPolynomialMarkdown } from '../../../utils/format/markdown'
import LabeledMathResult from '../result/LabeledMathResult'
import { Button } from '../../../baseComponents/controls/Button'

type DiscreteOp = 'falling' | 'rising' | 'shift' | 'diff' | 'sum'

export const DiscretePolyPanel: React.FC = () => {
  const [op, setOp] = React.useState<DiscreteOp>('falling')
  const [m, setM] = React.useState<number>(3)
  // reserved for future ops (e.g., binomXPlusKChooseKPoly)
  const [h, setH] = React.useState<number>(1)
  const [P, setP] = React.useState<{ coeffs: number[] }>({ coeffs: [0,1,1] })
  const [out, setOut] = React.useState<number[] | null>(null)
  // Σ に関する ClosedForm 入力（一般項の加算）
  const [gtPolyCoeffs, setGtPolyCoeffs] = React.useState<number[]>([1]) // a_0 + a_1 n + ...
  const [gtBase, setGtBase] = React.useState<number>(2) // 2^n など
  const [err, setErr] = React.useState<string | null>(null)

  const run = async () => {
    setErr(null); setOut(null)
    try {
  const { cm_fallingFactorialPoly, cm_risingFactorialPoly, cm_shiftPolyXPlusH, cm_discreteDiff, cm_discreteSum } = await import('../../../wasm/concreteMath')
      if (op === 'falling') {
        setOut(await cm_fallingFactorialPoly(m))
      } else if (op === 'rising') {
        setOut(await cm_risingFactorialPoly(m))
      } else if (op === 'shift') {
        setOut(await cm_shiftPolyXPlusH(P.coeffs, h))
      } else if (op === 'diff') {
        setOut(await cm_discreteDiff(P.coeffs))
      } else if (op === 'sum') {
        // Σ(P) の多項式和に、必要なら一般項 poly(n)*base^n の和を加える
        const base = await cm_discreteSum(P.coeffs)
        // ClosedForm への拡張はページ側（sequence）で実施。ここでは多項式部分の和のみ返す。
        setOut(base)
      }
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  return (
    <SectionPanelWithTitle title="離散多項式" showSave={false} showCopy={false}>
      <div style={{ display:'grid', gap:12 }}>
        <div style={{ display:'flex', gap:12, alignItems:'center', flexWrap:'wrap' }}>
          <label><input type="radio" checked={op==='falling'} onChange={()=>setOp('falling')} /> (x)_m</label>
          <label><input type="radio" checked={op==='rising'} onChange={()=>setOp('rising')} /> (x)^{`{m}`}</label>
          <label><input type="radio" checked={op==='shift'} onChange={()=>setOp('shift')} /> P(x+h)</label>
          <label><input type="radio" checked={op==='diff'} onChange={()=>setOp('diff')} /> ΔP(x)</label>
          <label><input type="radio" checked={op==='sum'} onChange={()=>setOp('sum')} /> ΣP(x)</label>
          <div style={{ marginLeft:'auto' }}>
            <Button onClick={run}>計算</Button>
          </div>
        </div>

        {op==='falling' && (
          <div style={{ display:'flex', gap:8, alignItems:'center' }}>
            <span>m=</span>
            <NumberCellInput value={m} onChange={setM} width={96} />
          </div>
        )}
        {op==='rising' && (
          <div style={{ display:'flex', gap:8, alignItems:'center' }}>
            <span>m=</span>
            <NumberCellInput value={m} onChange={setM} width={96} />
          </div>
        )}
        {op==='shift' && (
          <div style={{ display:'flex', gap:16, alignItems:'center' }}>
            <PolynomialInput value={P} onChange={setP} />
            <div style={{ display:'flex', gap:8, alignItems:'center' }}>
              <span>h=</span>
              <NumberCellInput value={h} onChange={setH} width={96} />
            </div>
          </div>
        )}
        {(op==='diff' || op==='sum') && (
          <PolynomialInput value={P} onChange={setP} />
        )}

        {op==='sum' && (
          <div style={{ display:'grid', gap:8 }}>
            <div style={{ fontSize:12, opacity:0.8 }}>一般項（任意）: a(n) = Q(n) · r^n</div>
            <div style={{ display:'flex', gap:16, alignItems:'center', flexWrap:'wrap' }}>
              <div style={{ display:'flex', gap:6, alignItems:'center' }}>
                <span>Q(n)（係数）:</span>
                <PolynomialInput value={{ coeffs: gtPolyCoeffs }} onChange={(v)=> setGtPolyCoeffs(v.coeffs)} />
              </div>
              <div style={{ display:'flex', gap:6, alignItems:'center' }}>
                <span>r=</span>
                <NumberCellInput value={gtBase} onChange={setGtBase} width={96} />
              </div>
            </div>
            <div style={{ fontSize:12, opacity:0.7 }}>
              注: ここでは ΣP(x) の多項式部分のみを計算します。Q(n)·r^n の和は「数列（漸化式）」ページの ClosedForm で扱います。
            </div>
          </div>
        )}

        {err && <div style={{ color:'crimson' }}>{err}</div>}
        {out && (
          <LabeledMathResult label={"= "} body={formatPolynomialMarkdown(out)} buildSavePayload={()=> ({ kind:'polynomial', coeffs: out.slice() })} />
        )}
      </div>
    </SectionPanelWithTitle>
  )
}

export default DiscretePolyPanel
