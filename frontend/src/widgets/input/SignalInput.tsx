"use client"
import React from 'react'
import { Signal } from '../dto/signal_processing'
import NumberCell from '../../baseComponents/inputs/NumberCell'
import VectorInput from './VectorInput'
import MarkdownMath from '../../widgets/display/MarkdownMath'

export type SignalInputProps = {
  value: Signal
  onChange: (value: Signal) => void
  length?: number
}

export const SignalInput: React.FC<SignalInputProps> = ({ value, onChange, length }) => {
  const n = length ?? value.data.length
  const data = React.useMemo(() => {
    const d = value.data.slice(0, n)
    while (d.length < n) d.push(0)
    return d
  }, [value.data, n])
  return (
    <div style={{ display: 'grid', gap: 8 }}>
      <MarkdownMath math={`x[n]`} />
      <VectorInput value={{ data }} onChange={(v) => onChange({ ...value, data: v.data })} orientation={'row'} />
      <div style={{ display: 'inline-flex', alignItems: 'center', gap: 8 }}>
        <MarkdownMath math={`F_s`} block={false} />
        <NumberCell value={value.sample_rate} onChange={(sample_rate) => onChange({ ...value, sample_rate })} width={100} />
      </div>
    </div>
  )
}

export default SignalInput
