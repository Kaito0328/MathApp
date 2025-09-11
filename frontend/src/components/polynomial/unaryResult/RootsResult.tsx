"use client"
import React from 'react'
import MarkdownMath from '../../../widgets/display/MarkdownMath'

export const RootsResult: React.FC<{ rootsList?: string | null }> = ({ rootsList }) => {
  if (!rootsList) return null
  return (
  <MarkdownMath math={rootsList} />
  )
}

export default RootsResult
