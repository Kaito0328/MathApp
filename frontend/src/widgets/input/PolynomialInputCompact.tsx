"use client"
import React from 'react'
import { Polynomial } from '../dto/polynomial'
import NumberCellInput from '../../baseComponents/input/NumberCellInput'
import MarkdownMath from '../../widgets/display/MarkdownMath'

export type PolynomialInputCompactProps = {
  value: Polynomial
  onChange: (value: Polynomial) => void
  varName?: string // display variable symbol (default 'x')
}

export const PolynomialInputCompact: React.FC<PolynomialInputCompactProps> = ({ value, onChange, varName = 'x' }) => {
  const coeffs = value.coeffs.slice()
  const setCoeff = (i: number, v: number) => {
    const next = coeffs.slice(); next[i] = v; onChange({ coeffs: next })
  }
  const addTerm = () => {
    const next = coeffs.slice(); next.push(0); onChange({ coeffs: next })
  }
  const removeLeading = () => {
    if (coeffs.length > 1) {
      const next = coeffs.slice(0, coeffs.length - 1)
      onChange({ coeffs: next })
    }
  }
  return (
    <div style={{ display:'inline-flex', alignItems:'center', gap:6, flexWrap:'wrap' }}>
      {Array.from({ length: coeffs.length }).map((_, idx) => {
        const power = coeffs.length - 1 - idx
        const ci = power
        return (
          <div key={idx} style={{ display:'inline-flex', alignItems:'center', gap:6 }}>
            <NumberCellInput value={coeffs[ci] ?? 0} onChange={(v) => setCoeff(ci, v)} width={72} />
            {power > 1 && <MarkdownMath math={`${varName}^{${power}}`} block={false} />}
            {power === 1 && <MarkdownMath math={`${varName}`} block={false} />}
            {idx < coeffs.length - 1 && <MarkdownMath math={`+`} block={false} />}
          </div>
        )
      })}
      {/* +- controls at the end */}
      <button onClick={addTerm}>+</button>
      <button onClick={removeLeading} disabled={coeffs.length<=1}>-</button>
    </div>
  )
}

export default PolynomialInputCompact
