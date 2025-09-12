"use client"
import React from 'react'
import { formatPolynomialMarkdown } from '../../../../../utils/format/markdown'
import MathWithSaveCopy from '../../../../features/saveCopy/MathWithSaveCopy'

export interface PolynomialDisplayWithSaveCopyProps {
  coeffs?: number[] | null
  buildSavePayload: () => any
}

export const PolynomialDisplayWithSaveCopy: React.FC<PolynomialDisplayWithSaveCopyProps> = ({ coeffs, buildSavePayload }) => {
  if (!coeffs) return null
  const md = formatPolynomialMarkdown(coeffs)
  return <MathWithSaveCopy tex={md} inline={false} buildSavePayload={buildSavePayload} />
}

export default PolynomialDisplayWithSaveCopy
