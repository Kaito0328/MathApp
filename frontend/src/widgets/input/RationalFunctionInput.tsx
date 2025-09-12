"use client"
import React from 'react'
import { RationalFunction } from '../dto/polynomial'
import PolynomialInput from './PolynomialInput'

export type RationalFunctionInputProps = {
  value: RationalFunction
  onChange: (value: RationalFunction) => void
  degreeNum?: number
  degreeDen?: number
}

export const RationalFunctionInput: React.FC<RationalFunctionInputProps> = ({ value, onChange, degreeNum, degreeDen }) => {
  return (
    <div style={{ display: 'grid', gap: 8 }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: 8, flexWrap: 'wrap' }}>
        <div style={{ fontWeight: 600, whiteSpace: 'nowrap' }}>分子</div>
        <div style={{ flex: '1 1 auto' }}>
          <PolynomialInput value={value.numerator} onChange={(numerator) => onChange({ ...value, numerator })} degree={degreeNum} />
        </div>
        <div style={{ fontWeight: 600, whiteSpace: 'nowrap' }}>／ 分母</div>
        <div style={{ flex: '1 1 auto' }}>
          <PolynomialInput value={value.denominator} onChange={(denominator) => onChange({ ...value, denominator })} degree={degreeDen} />
        </div>
      </div>
    </div>
  )
}

export default RationalFunctionInput
