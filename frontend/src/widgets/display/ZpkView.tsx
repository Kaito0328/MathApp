import React from 'react'
import { Zpk } from '../dto/lti-systems'
import { formatZpkMarkdown } from '../../utils/format/markdown'
import MarkdownMath from './MarkdownMath'

type Props = { value: Zpk; varName?: string; imag?: 'i' | 'j'; precision?: number }

export function ZpkView({ value, varName, imag, precision }: Props) {
  const md = formatZpkMarkdown(value, varName, { precision, imag, hideUnitGain: true })
  return <MarkdownMath math={md} />
}

export default ZpkView
