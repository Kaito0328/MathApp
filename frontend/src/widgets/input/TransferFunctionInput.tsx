"use client"
import React from 'react'
import { TransferFunction } from '../dto/lti-systems'
import NumberCell from '../../baseComponents/inputs/NumberCell'
import MarkdownMath from '../../widgets/display/MarkdownMath'
import RationalFunctionInput from './RationalFunctionInput'

export type TransferFunctionInputProps = {
  value: TransferFunction
  onChange: (value: TransferFunction) => void
  degreeNum?: number
  degreeDen?: number
}

export const TransferFunctionInput: React.FC<TransferFunctionInputProps> = ({ value, onChange, degreeNum, degreeDen }) => {
  const rf = React.useMemo(() => ({ numerator: { coeffs: [...value.num] }, denominator: { coeffs: [...value.den] } }), [value.num, value.den])
  return (
    <div style={{ display: 'grid', gap: 8 }}>
  <MarkdownMath math={`G(z) = \\dfrac{\\sum b_k z^{-k}}{\\sum a_k z^{-k}}`} />
      <RationalFunctionInput
        value={rf}
        onChange={(rf2) => onChange({ ...value, num: rf2.numerator.coeffs, den: rf2.denominator.coeffs })}
        degreeNum={degreeNum}
        degreeDen={degreeDen}
      />
      <div style={{ display: 'inline-flex', alignItems: 'center', gap: 6 }}>
        <MarkdownMath math={`T_s`} block={false} />
        <NumberCell value={value.sample_time ?? 0} onChange={(sample_time) => onChange({ ...value, sample_time })} width={100} />
        <span style={{ opacity: 0.7 }}>(nullで連続)</span>
      </div>
    </div>
  )
}

export default TransferFunctionInput
