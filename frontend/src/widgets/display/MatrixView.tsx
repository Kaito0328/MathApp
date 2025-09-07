import React from 'react'
import { Matrix } from '../dto/linalg'
import { formatMatrixMarkdown } from '../../utils/format/markdown'
import MarkdownMath from './MarkdownMath'

type Props = {
  value?: Matrix
  rows?: number
  cols?: number
  data?: ArrayLike<number>
  precision?: number
  block?: boolean
}

export function MatrixView({ value, rows, cols, data, precision, block = true }: Props) {
  const md = value
  ? formatMatrixMarkdown(value, { precision, paren: true })
  : formatMatrixMarkdown(rows || 0, cols || 0, data || [], { precision, paren: true })
  return <MarkdownMath math={md} block={block} />
}

export default MatrixView
