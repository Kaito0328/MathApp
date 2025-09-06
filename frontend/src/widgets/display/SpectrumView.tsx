import React from 'react'
import { Spectrum } from '../dto/signal_processing'
import { formatSpectrumMarkdown } from '../../utils/format/markdown'
import MarkdownMath from './MarkdownMath'

type Props = { value?: Spectrum; interleaved?: ArrayLike<number>; precision?: number }

export function SpectrumView({ value, interleaved, precision }: Props) {
  const md = value ? formatSpectrumMarkdown(value, { precision }) : formatSpectrumMarkdown(interleaved || [], { precision })
  return <MarkdownMath math={md} />
}

export default SpectrumView
