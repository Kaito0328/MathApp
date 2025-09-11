"use client"
import React from 'react'
import { Complex } from '../dto/complex'
import ComplexInput from './ComplexInput'
import { VectorSizeControls } from './SizeControls'

export type ComplexVectorInputProps = {
  value: Complex[]
  onChange: (value: Complex[]) => void
}

export const ComplexVectorInput: React.FC<ComplexVectorInputProps> = ({ value, onChange }) => {
  const [len, setLen] = React.useState<number>(value.length || 1)
  React.useEffect(() => { setLen(value.length || 1) }, [value.length])
  const items = React.useMemo(() => {
    const arr = value.slice(0, len)
    while (arr.length < len) arr.push({ re: 0, im: 0 })
    return arr
  }, [value, len])

  return (
    <div style={{ display: 'grid', gap: 8 }}>
      <VectorSizeControls length={len} onChange={(n)=> { setLen(n); onChange(items.slice(0,n).concat(Array(Math.max(0, n-items.length)).fill({ re:0, im:0 }))) }} />
      <div style={{ display: 'grid', gap: 8 }}>
        {items.map((c, i) => (
          <div key={i} style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <span style={{ width: 24, opacity: 0.7 }}>[{i}]</span>
            <ComplexInput value={c} onChange={(ci) => { const next = items.slice(); next[i] = ci; onChange(next) }} />
          </div>
        ))}
      </div>
    </div>
  )
}

export default ComplexVectorInput
