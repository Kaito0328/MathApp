import React from 'react'
import { Signal } from '../dto/signal_processing'
import { formatSignalMarkdown } from '../../utils/format/markdown'
import MarkdownMath from './MarkdownMath'

type Props = { value: Signal; orientation?: 'row' | 'col'; precision?: number }

export function SignalView({ value, orientation, precision }: Props) {
  const md = formatSignalMarkdown(value, { orientation, precision })
  return <MarkdownMath math={md} />
}

export default SignalView
