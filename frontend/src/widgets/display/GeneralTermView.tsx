"use client"
import React from 'react'
import MarkdownMath from './MarkdownMath'
import { formatGeneralTermMarkdown } from '../../utils/format/markdown'

export type GeneralTermDTO = { poly: { coeffs: number[] } | number[]; base: number }

const GeneralTermView: React.FC<{ term: GeneralTermDTO; varName?: string; precision?: number; hideBaseOne?: boolean; block?: boolean }>
  = ({ term, varName='n', precision, hideBaseOne=true, block=false }) => {
  const md = formatGeneralTermMarkdown({ poly: (term.poly as any).coeffs ? (term.poly as any).coeffs : (term.poly as number[]), base: term.base }, { varName, precision, hideBaseOne })
  return <MarkdownMath math={md} block={block} />
}

export default GeneralTermView
