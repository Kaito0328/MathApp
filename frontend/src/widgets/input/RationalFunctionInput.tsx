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
      <PolynomialInput value={value.numerator} onChange={(numerator) => onChange({ ...value, numerator })} degree={degreeNum} />
      <PolynomialInput value={value.denominator} onChange={(denominator) => onChange({ ...value, denominator })} degree={degreeDen} />
    </div>
  )
}

export default RationalFunctionInput
