import React from 'react'
import { RationalFunction } from '../dto/polynomial'
import { formatRationalFunctionMarkdown } from '../../utils/format/markdown'
import MarkdownMath from './MarkdownMath'

type Props = { value: RationalFunction; varName?: string }

export function RationalFunctionView({ value, varName }: Props) {
  const md = formatRationalFunctionMarkdown(value, varName)
  return <MarkdownMath math={md} />
}

export default RationalFunctionView
