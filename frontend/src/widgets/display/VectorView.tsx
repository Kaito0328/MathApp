import React from 'react'
import { Vector } from '../dto/linalg'
import { formatVectorMarkdown } from '../../utils/format/markdown'
import MarkdownMath from './MarkdownMath'

type Props = {
  value?: Vector
  values?: ArrayLike<number>
  orientation?: 'row' | 'col'
  precision?: number
}

export function VectorView({ value, values, orientation, precision }: Props) {
  const data = value?.data ?? (values ? Array.from({ length: values.length }, (_, i) => Number((values as any)[i])) : [])
  const md = formatVectorMarkdown({ data }, { orientation, precision, paren: true })
  return <MarkdownMath math={md} />
}

export default VectorView
