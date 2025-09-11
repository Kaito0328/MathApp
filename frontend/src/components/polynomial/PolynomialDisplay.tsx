"use client"
import React from 'react'
import WithActions from '../operations/WithActions'
import { PolynomialView } from '../../widgets/display/PolynomialView'
import { formatPolynomialMarkdown } from '../../utils/format/markdown'

type Props = {
  coeffs?: ArrayLike<number>
  value?: { coeffs: number[] }
  varName?: string
}

export const PolynomialDisplay: React.FC<Props> = ({ coeffs, value, varName }) => {
  const c = value?.coeffs ?? (coeffs ? Array.from({ length: coeffs.length }, (_, i) => Number((coeffs as any)[i])) : undefined)
  if (!c) return null

  const buildSavePayload = () => ({ kind: 'polynomial', coeffs: c })
  const copyContent = formatPolynomialMarkdown(c, varName)

  return (
    <WithActions buildSavePayload={buildSavePayload} copyContent={copyContent}>
      <PolynomialView coeffs={c} varName={varName} />
    </WithActions>
  )
}

export default PolynomialDisplay
