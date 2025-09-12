"use client"
import React from 'react'
import WithSaveCopy from './WithSaveCopy'
import MarkdownMath from '../../../widgets/display/MarkdownMath'

export interface MathWithSaveCopyProps {
  tex: string
  inline?: boolean
  buildSavePayload?: () => any
}

const MathWithSaveCopy: React.FC<MathWithSaveCopyProps> = ({ tex, inline = false, buildSavePayload }) => {
  const copyContent = tex
  return (
    <WithSaveCopy buildSavePayload={buildSavePayload} copyContent={copyContent} showSave={!!buildSavePayload}>
      <MarkdownMath math={tex} block={!inline} />
    </WithSaveCopy>
  )
}

export default MathWithSaveCopy
