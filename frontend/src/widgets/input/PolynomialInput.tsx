"use client"
import React from 'react'
import { Polynomial } from '../dto/polynomial'
import NumberCellInput from '../../baseComponents/input/NumberCellInput'
import MarkdownMath from '../../widgets/display/MarkdownMath'
import { VectorSizeControls } from './SizeControls'

export type PolynomialInputProps = {
  value: Polynomial
  onChange: (value: Polynomial) => void
  degree?: number
}

export const PolynomialInput: React.FC<PolynomialInputProps> = ({ value, onChange, degree }) => {
  const initLen = (degree ?? (value.coeffs.length - 1)) + 1
  const [len, setLen] = React.useState<number>(initLen)
  React.useEffect(() => { setLen(initLen) }, [initLen])
  const coeffs = React.useMemo(() => {
    const d = value.coeffs.slice(0, len)
    while (d.length < len) d.push(0)
    return d
  }, [value.coeffs, len])
  const setCoeff = (i: number, v: number) => {
    const next = coeffs.slice(); next[i] = v; onChange({ coeffs: next })
  }
  // render highest power first: a_{n-1} x^{n-1} + ... + a_0
  return (
    <div style={{ display: 'grid', gap: 8 }}>
      <VectorSizeControls length={len} onChange={(n)=> { setLen(n); onChange({ coeffs: coeffs.slice(0, n).concat(Array(Math.max(0, n - coeffs.length)).fill(0)) }) }} />
      <div style={{ display: 'inline-flex', alignItems: 'center', gap: 6, flexWrap: 'wrap' }}>
        {Array.from({ length: len }).map((_, idx) => {
          const power = len - 1 - idx
          const ci = power // map power to coeff index
          return (
            <div key={idx} style={{ display: 'inline-flex', alignItems: 'center', gap: 6 }}>
              <NumberCellInput value={coeffs[ci] ?? 0} onChange={(v) => setCoeff(ci, v)} width={72} />
              {power > 1 && <MarkdownMath math={`x^{${power}}`} block={false} />}
              {power === 1 && <MarkdownMath math={`x`} block={false} />}
              {idx < len - 1 && <MarkdownMath math={`+`} block={false} />}
            </div>
          )
        })}
      </div>
    </div>
  )
}

export default PolynomialInput
