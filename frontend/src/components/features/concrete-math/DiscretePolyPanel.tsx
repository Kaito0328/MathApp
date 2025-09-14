"use client"
import React from 'react'
import SectionPanelWithTitle from '../../composites/panels/SectionPanelWithTitle'
import NumberCellInput from '../../../baseComponents/input/NumberCellInput'
import PolynomialInput from '../../../widgets/input/PolynomialInput'
import { formatPolynomialMarkdown } from '../../../utils/format/markdown'
import LabeledMathResult from '../result/LabeledMathResult'
import { buildClosedFormTeX } from '../../../utils/format/closedForm'
import { Button } from '../../../baseComponents/controls/Button'

type DiscreteOp = 'falling' | 'rising' | 'shift' | 'diff' | 'sum'

export const DiscretePolyPanel: React.FC = () => {
  const [op, setOp] = React.useState<DiscreteOp>('falling')
  const [m, setM] = React.useState<number>(3)
  // reserved for future ops (e.g., binomXPlusKChooseKPoly)
  const [h, setH] = React.useState<number>(1)
  const [P, setP] = React.useState<{ coeffs: number[] }>({ coeffs: [0,1,1] })
  const [out, setOut] = React.useState<number[] | null>(null)
  const [closedFormTex, setClosedFormTex] = React.useState<string>('')
  const [precision, setPrecision] = React.useState<number>(4)
  // Σ に関する ClosedForm 入力（一般項の加算）
  const [gtPolyCoeffs, setGtPolyCoeffs] = React.useState<number[]>([1]) // a_0 + a_1 n + ...
  const [gtBase, setGtBase] = React.useState<number>(2) // 2^n など
  const [err, setErr] = React.useState<string | null>(null)

  // Reset results when operation changes
  React.useEffect(()=>{
    setErr(null)
    setOut(null)
    setClosedFormTex('')
  }, [op])

  const run = async () => {
    setErr(null); setOut(null); setClosedFormTex('')
    try {
  const { cm_fallingFactorialPoly, cm_risingFactorialPoly, cm_shiftPolyXPlusH, cm_discreteDiff, cm_discreteSum } = await import('../../../wasm/concreteMath')
      const { getWasm } = await import('../../../wasm/loader')
      const wasm: any = await getWasm()
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
        const basePoly = await cm_discreteSum(P.coeffs)
        setOut(basePoly)

        // オプションの一般項 Q(n)·r^n を ClosedForm で評価し、その部分和を表示（多項式部分は係数列表示のまま）
        const polysFlat: number[] = []
        const offsets: number[] = [0]
        const bases: number[] = []
        // 1項だけ（ユーザの入力）
        for (const c of gtPolyCoeffs) { polysFlat.push(Number(c || 0), 0) }
        offsets.push(polysFlat.length)
        bases.push(Number(gtBase || 0), 0)

        // 同次係数は 0、初期値は a_0=0 のみで OK（部分和で意味を持つのは非同次の一般項）
        const coeffs = new Float64Array(0)
        const nh_polys_flat = Float64Array.from(polysFlat)
        const nh_offsets = Uint32Array.from(offsets)
        const nh_bases = Float64Array.from(bases)
        const initial = new Float64Array(1) // a0 = 0
        const cf = wasm.solveRecurrence(coeffs, nh_polys_flat, nh_offsets, nh_bases, initial)
        const sumCf = (typeof cf.partialSum === 'function') ? cf.partialSum() : wasm.partialSum(cf)
        const tex = buildClosedFormTeX(sumCf, precision)
  try { cf.free?.(); sumCf.free?.() } catch { /* noop */ }
        setClosedFormTex(tex)
      }
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  // Auto-run for light ops: falling, rising, shift, diff
  React.useEffect(()=>{
    if (op === 'sum') return
    // Trigger on relevant inputs
    run()
    // We intentionally depend on all inputs relevant to light ops
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [op, m, h, P])

  return (
    <SectionPanelWithTitle title="離散多項式" showSave={false} showCopy={false}>
      <div style={{ display:'grid', gap:12 }}>
        <div style={{ display:'flex', gap:12, alignItems:'center', flexWrap:'wrap' }}>
          <label><input type="radio" checked={op==='falling'} onChange={()=>setOp('falling')} /> (x)_m</label>
          <label><input type="radio" checked={op==='rising'} onChange={()=>setOp('rising')} /> (x)^{`{m}`}</label>
          <label><input type="radio" checked={op==='shift'} onChange={()=>setOp('shift')} /> P(x+h)</label>
          <label><input type="radio" checked={op==='diff'} onChange={()=>setOp('diff')} /> ΔP(x)</label>
          <label><input type="radio" checked={op==='sum'} onChange={()=>setOp('sum')} /> ΣP(x)</label>
          <div style={{ marginLeft:'auto', display:'flex', alignItems:'center', gap:8 }}>
            <span style={{ fontSize:12, opacity:0.8 }}>精度</span>
            <NumberCellInput value={precision} onChange={(v)=> setPrecision(Math.max(1, Math.floor(Number(v) || 1)))} width={72} />
            {op==='sum' && <Button onClick={run}>計算</Button>}
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
          <PolynomialInput value={P} onChange={setP} varName="n" />
        )}

        {op==='sum' && (
          <div style={{ display:'grid', gap:8 }}>
            <div style={{ fontSize:12, opacity:0.8 }}>一般項（任意）: a(n) = Q(n) · r^n</div>
            <div style={{ display:'flex', gap:16, alignItems:'center', flexWrap:'wrap' }}>
              <div style={{ display:'flex', gap:6, alignItems:'center' }}>
                <span>Q(n)（係数）:</span>
                <PolynomialInput value={{ coeffs: gtPolyCoeffs }} onChange={(v)=> setGtPolyCoeffs(v.coeffs)} varName="n" />
              </div>
              <div style={{ display:'flex', gap:6, alignItems:'center' }}>
                <span>r=</span>
                <NumberCellInput value={gtBase} onChange={setGtBase} width={96} />
              </div>
            </div>
            {closedFormTex && (
              <div style={{ display:'grid', gap:6 }}>
                <div style={{ fontSize:12, opacity:0.8 }}>Q(n)·r^n の部分和（ClosedForm）</div>
                <LabeledMathResult label={'S(n) ='} body={closedFormTex} />
              </div>
            )}
          </div>
        )}

        {err && <div style={{ color:'crimson' }}>{err}</div>}
        {out && (
          <LabeledMathResult label={"= "} body={formatPolynomialMarkdown(out, 'n')} buildSavePayload={()=> ({ kind:'polynomial', coeffs: out.slice() })} />
        )}
      </div>
    </SectionPanelWithTitle>
  )
}

export default DiscretePolyPanel
