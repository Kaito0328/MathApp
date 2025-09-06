"use client"
import React from 'react'
import { Complex } from '../dto/complex'
import NumberCell from '../../baseComponents/inputs/NumberCell'
import MarkdownMath from '../../widgets/display/MarkdownMath'

export type ComplexInputProps = {
  value: Complex
  onChange: (value: Complex) => void
}

export const ComplexInput: React.FC<ComplexInputProps> = ({ value, onChange }) => (
  <div style={{ display: 'grid', gap: 4 }}>
    <MarkdownMath math={`a + b i`} />
    <div style={{ display: 'inline-flex', alignItems: 'center', gap: 6 }}>
      <NumberCell value={value.re} onChange={(re) => onChange({ ...value, re })} width={80} />
      <MarkdownMath math={`+`} block={false} />
      <NumberCell value={value.im} onChange={(im) => onChange({ ...value, im })} width={80} />
      <MarkdownMath math={`i`} block={false} />
    </div>
  </div>
)

export default ComplexInput
