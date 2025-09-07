"use client"
import React from 'react'
import { Spectrum } from '../dto/signal_processing'
import NumberCell from '../../baseComponents/inputs/NumberCell'
import MarkdownMath from '../../widgets/display/MarkdownMath'

export type SpectrumInputProps = {
  value: Spectrum
  onChange: (value: Spectrum) => void
  length?: number
}

export const SpectrumInput: React.FC<SpectrumInputProps> = ({ value, onChange, length }) => {
  const n = length ?? value.data.length
  const data = React.useMemo(() => {
    const d = value.data.slice(0, n)
    while (d.length < n) d.push({ re: 0, im: 0 })
    return d
  }, [value.data, n])
  const set = (i: number, part: 're' | 'im', v: number) => {
    const next = data.slice(); next[i] = { ...next[i], [part]: v }; onChange({ ...value, data: next })
  }
  return (
    <div style={{ display: 'grid', gap: 8 }}>
      <MarkdownMath math={`X[k] = a_k + b_k i`} />
  <div style={{ display: 'grid', gap: 8 }}>
        {data.map((z, i) => (
          <div key={i} style={{ display: 'inline-flex', alignItems: 'center', gap: 6 }}>
            <NumberCell value={z.re} onChange={(v) => set(i, 're', v)} width={80} />
            <MarkdownMath math={`+`} block={false} />
            <NumberCell value={z.im} onChange={(v) => set(i, 'im', v)} width={80} />
            <MarkdownMath math={`i`} block={false} />
          </div>
        ))}
      </div>
      <div style={{ display: 'inline-flex', alignItems: 'center', gap: 8 }}>
        <MarkdownMath math={`F_s`} block={false} />
        <NumberCell value={value.sample_rate} onChange={(sample_rate) => onChange({ ...value, sample_rate })} width={100} />
      </div>
    </div>
  )
}

export default SpectrumInput
