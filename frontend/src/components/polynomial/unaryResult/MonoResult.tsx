"use client"
import React from 'react'
import MarkdownMath from '../../../widgets/display/MarkdownMath'

export const MonoResult: React.FC<{ result?: number[] | null; md: string }> = ({ result, md }) => {
  if (!result) return null
  return (
      <MarkdownMath math={md} />
  )
}

export default MonoResult
