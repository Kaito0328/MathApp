"use client"
import React from 'react'
import WithActions from '../../operations/WithActions'
import { formatPolynomialMarkdown } from '../../../utils/format/markdown'
import { PolynomialView } from '../../../widgets/display/PolynomialView'

export const AddSubMulDivResult: React.FC<{
  result?: number[] | null
  buildSavePayload: () => any
}> = ({ result, buildSavePayload }) => {
  if (!result) return null
  const md = formatPolynomialMarkdown(result)
  return (
    <WithActions buildSavePayload={buildSavePayload} copyContent={md}>
      <PolynomialView coeffs={result} />
    </WithActions>
  )
}

export default AddSubMulDivResult
