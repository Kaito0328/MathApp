"use client"
import React from 'react'
import { Zpk } from '../dto/lti-systems'
import NumberCellInput from '../../baseComponents/input/NumberCellInput'
import MarkdownMath from '../../widgets/display/MarkdownMath'

function ensurePairs(a: number[], pairs: number) {
  const out = a.slice(0, pairs * 2)
  while (out.length < pairs * 2) out.push(0)
  return out
}

export type ZpkInputProps = {
  value: Zpk
  onChange: (value: Zpk) => void
  zeroPairs?: number
  polePairs?: number
}

export const ZpkInput: React.FC<ZpkInputProps> = ({ value, onChange, zeroPairs, polePairs }) => {
  const [zPairsLocal, setZPairsLocal] = React.useState<number>(zeroPairs ?? Math.ceil((value.zeros.length || 0) / 2))
  const [pPairsLocal, setPPairsLocal] = React.useState<number>(polePairs ?? Math.ceil((value.poles.length || 0) / 2))
  const zPairs = zPairsLocal
  const pPairs = pPairsLocal
  const zeros = ensurePairs(value.zeros, zPairs)
  const poles = ensurePairs(value.poles, pPairs)

  const setZero = (i: number, re: boolean, v: number) => {
    const arr = zeros.slice()
    arr[i * 2 + (re ? 0 : 1)] = v
    onChange({ ...value, zeros: arr })
  }
  const setPole = (i: number, re: boolean, v: number) => {
    const arr = poles.slice()
    arr[i * 2 + (re ? 0 : 1)] = v
    onChange({ ...value, poles: arr })
  }

  return (
    <div style={{ display: 'grid', gap: 12 }}>
      <MarkdownMath math={`G(s) = k \\cdot \\frac{ \\prod_i (s - z_i) }{ \\prod_j (s - p_j) }`} />
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', flexWrap: 'wrap' }}>
        <div style={{ display: 'inline-flex', alignItems: 'center', gap: 4 }}>
          <span style={{ fontSize: 12, opacity: 0.8 }}>Zeros pairs</span>
          <input type="number" min={0} value={zPairsLocal} onChange={(e) => setZPairsLocal(Math.max(0, Math.floor(Number(e.target.value)||0)))} style={{ width: 72 }} />
          <button onClick={() => setZPairsLocal((n) => n + 1)}>+1</button>
          <button onClick={() => setZPairsLocal((n) => Math.max(0, n - 1))}>-1</button>
        </div>
        <div style={{ display: 'inline-flex', alignItems: 'center', gap: 4 }}>
          <span style={{ fontSize: 12, opacity: 0.8 }}>Poles pairs</span>
          <input type="number" min={0} value={pPairsLocal} onChange={(e) => setPPairsLocal(Math.max(0, Math.floor(Number(e.target.value)||0)))} style={{ width: 72 }} />
          <button onClick={() => setPPairsLocal((n) => n + 1)}>+1</button>
          <button onClick={() => setPPairsLocal((n) => Math.max(0, n - 1))}>-1</button>
        </div>
        <button onClick={() => {
          const newZeros = ensurePairs(value.zeros, zPairsLocal)
          const newPoles = ensurePairs(value.poles, pPairsLocal)
          onChange({ ...value, zeros: newZeros, poles: newPoles })
        }}>適用</button>
      </div>
      <div>
        <div style={{ fontSize: 12, opacity: 0.8, marginBottom: 4 }}>Zeros</div>
        <div style={{ display: 'grid', gap: 6 }}>
          {Array.from({ length: zPairs }).map((_, i) => (
            <div key={i} style={{ display: 'inline-flex', alignItems: 'center', gap: 6 }}>
              <NumberCellInput value={zeros[i * 2] ?? 0} onChange={(v) => setZero(i, true, v)} width={80} />
              <MarkdownMath math={`+`} block={false} />
              <NumberCellInput value={(zeros[i * 2 + 1] ?? 0)} onChange={(v) => setZero(i, false, v)} width={80} />
              <MarkdownMath math={`i`} block={false} />
            </div>
          ))}
        </div>
      </div>
      <div>
        <div style={{ fontSize: 12, opacity: 0.8, marginBottom: 4 }}>Poles</div>
        <div style={{ display: 'grid', gap: 6 }}>
          {Array.from({ length: pPairs }).map((_, i) => (
            <div key={i} style={{ display: 'inline-flex', alignItems: 'center', gap: 6 }}>
              <NumberCellInput value={poles[i * 2] ?? 0} onChange={(v) => setPole(i, true, v)} width={80} />
              <MarkdownMath math={`+`} block={false} />
              <NumberCellInput value={(poles[i * 2 + 1] ?? 0)} onChange={(v) => setPole(i, false, v)} width={80} />
              <MarkdownMath math={`i`} block={false} />
            </div>
          ))}
        </div>
      </div>
      <div style={{ display: 'inline-flex', alignItems: 'center', gap: 8 }}>
        <MarkdownMath math={`k`} block={false} />
        <NumberCellInput value={value.gain} onChange={(gain) => onChange({ ...value, gain })} width={100} />
        <MarkdownMath math={`T_s`} block={false} />
        <NumberCellInput value={value.sample_time ?? 0} onChange={(sample_time) => onChange({ ...value, sample_time })} width={100} />
      </div>
    </div>
  )
}

export default ZpkInput
