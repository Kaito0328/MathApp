"use client"
import React from 'react'
import PolynomialDisplayWithSaveCopy from '../../display/PolynomialDisplayWithSaveCopy'

export interface AddSubMulDivResultProps {
  result?: number[] | null
  buildSavePayload: () => any
}

export const AddSubMulDivResult: React.FC<AddSubMulDivResultProps> = ({ result, buildSavePayload }) => {
  if (!result) return null
  return (
    <PolynomialDisplayWithSaveCopy coeffs={result} buildSavePayload={buildSavePayload} />
  )
}

export default AddSubMulDivResult
