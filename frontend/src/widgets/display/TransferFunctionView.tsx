import React from 'react'
import { TransferFunction } from '../dto/lti-systems'
import { formatTransferFunctionMarkdown } from '../../utils/format/markdown'
import MarkdownMath from './MarkdownMath'

type Props = { value: TransferFunction; varName?: string }

export function TransferFunctionView({ value, varName }: Props) {
  const md = formatTransferFunctionMarkdown(value, varName)
  return <MarkdownMath math={md} />
}

export default TransferFunctionView
