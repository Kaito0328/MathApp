"use client"
import React from 'react'
import Stack from '../../../../../../baseComponents/layout/Stack'
import LabeledMathResult from '../../../../../features/result/LabeledMathResult'
import { formatPolynomialMarkdown } from '../../../../../../utils/format/markdown'

export interface DivRemResultProps {
  quot?: number[] | null
  rem?: number[] | null
  buildSavePayload: (k:'quot'|'rem') => any
}

export const DivRemResult: React.FC<DivRemResultProps> = ({ quot, rem, buildSavePayload }) => {
  if (!quot && !rem) return null
  return (
    <Stack gap={12}>
      {quot && (
        <LabeledMathResult label={"q(x) ="} body={formatPolynomialMarkdown(quot)} buildSavePayload={()=> buildSavePayload('quot')} />
      )}
      {rem && (
        <LabeledMathResult label={"r(x) ="} body={formatPolynomialMarkdown(rem)} buildSavePayload={()=> buildSavePayload('rem')} />
      )}
    </Stack>
  )
}

export default DivRemResult
