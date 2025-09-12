"use client"
import React from 'react'
import PolynomialDisplayWithSaveCopy from '../display/PolynomialDisplayWithSaveCopy'

export interface SinglePolynomialResultProps {
  coeffs?: number[] | null
  buildSavePayload: () => any
}

const SinglePolynomialResult: React.FC<SinglePolynomialResultProps> = ({ coeffs, buildSavePayload }) => {
  if (!coeffs) return null
  return <PolynomialDisplayWithSaveCopy coeffs={coeffs} buildSavePayload={buildSavePayload} />
}

export default SinglePolynomialResult
