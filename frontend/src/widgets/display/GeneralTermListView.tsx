"use client"
import React from 'react'
import MarkdownMath from './MarkdownMath'
import { formatGeneralTermsMarkdown } from '../../utils/format/markdown'

export type GeneralTermDTO = { poly: { coeffs: number[] } | number[]; base: number }

const GeneralTermListView: React.FC<{ terms: GeneralTermDTO[]; varName?: string; precision?: number; hideBaseOne?: boolean; block?: boolean }>
  = ({ terms, varName='n', precision, hideBaseOne=true, block=false }) => {
  const md = formatGeneralTermsMarkdown(terms.map(t=> ({ poly: (t.poly as any).coeffs ? (t.poly as any).coeffs : (t.poly as number[]), base: t.base })), { varName, precision, hideBaseOne })
  return <MarkdownMath math={md} block={block} />
}

export default GeneralTermListView
