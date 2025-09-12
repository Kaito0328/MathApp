"use client"
import React from 'react'
import { composeTeX } from '../../../utils/format/tex'
import MathWithSaveCopy from '../saveCopy/MathWithSaveCopy'

export interface LabeledMathResultProps {
  label?: string | null
  body: string
  inline?: boolean
  buildSavePayload?: () => any
}

const LabeledMathResult: React.FC<LabeledMathResultProps> = ({ label, body, inline = false, buildSavePayload }) => {
  const { tex, block } = composeTeX({ left: label ?? undefined, body, mode: inline ? 'inline' : 'block' })
  return <MathWithSaveCopy tex={tex} inline={!block ? true : false} buildSavePayload={buildSavePayload} />
}

export default LabeledMathResult
