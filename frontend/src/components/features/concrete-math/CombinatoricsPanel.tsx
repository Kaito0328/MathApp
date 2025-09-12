"use client"
import React from 'react'
import LabeledMathResult from '../result/LabeledMathResult'
import { Button } from '../../../baseComponents/controls/Button'
import NumberCellInput from '../../../baseComponents/input/NumberCellInput'
import SectionPanelWithTitle from '../../composites/panels/SectionPanelWithTitle'
import { formatNumberForMath } from '../../../utils/format/markdown'

type BinomStirlingOp = 'binom' | 'stirling2'

export const CombinatoricsPanel: React.FC = () => {
  const [op, setOp] = React.useState<BinomStirlingOp>('binom')
  const [n, setN] = React.useState<number>(5)
  const [k, setK] = React.useState<number>(2)
  const [result, setResult] = React.useState<number | null>(null)
  const [err, setErr] = React.useState<string | null>(null)
  const [precision, setPrecision] = React.useState<number>(0)

  const run = async () => {
    setErr(null); setResult(null)
    try {
      const { cm_binom, cm_stirling2 } = await import('../../../wasm/concreteMath')
      const v = op === 'binom' ? await cm_binom(n, k) : await cm_stirling2(n, k)
      setResult(v)
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  const tex = result != null ? formatNumberForMath(result, precision) : ''

  return (
    <SectionPanelWithTitle title="組合せ論" showSave={false} showCopy={false}>
      <div style={{ display:'grid', gap:12 }}>
        <div style={{ display:'flex', gap:12, alignItems:'center' }}>
          <label>
            <input type="radio" checked={op==='binom'} onChange={()=>setOp('binom')} /> C(n,k)
          </label>
          <label>
            <input type="radio" checked={op==='stirling2'} onChange={()=>setOp('stirling2')} /> S(n,k)
          </label>
          <div style={{ marginLeft:'auto', display:'flex', alignItems:'center', gap:8 }}>
            <span>精度</span>
            <NumberCellInput value={precision} onChange={setPrecision} width={72} />
            <Button onClick={run}>計算</Button>
          </div>
        </div>
        <div style={{ display:'flex', gap:16, alignItems:'center' }}>
          <div style={{ display:'flex', alignItems:'center', gap:6 }}>
            <span>n=</span>
            <NumberCellInput value={n} onChange={setN} width={96} />
          </div>
          <div style={{ display:'flex', alignItems:'center', gap:6 }}>
            <span>k=</span>
            <NumberCellInput value={k} onChange={setK} width={96} />
          </div>
        </div>
        {err && <div style={{ color:'crimson' }}>{err}</div>}
        {result != null && (
          <LabeledMathResult label={op==='binom'? 'C(n,k)=' : 'S(n,k)='} body={tex} />
        )}
      </div>
    </SectionPanelWithTitle>
  )
}

export default CombinatoricsPanel
