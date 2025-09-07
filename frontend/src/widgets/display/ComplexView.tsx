import React from 'react'
import { Complex } from '../dto/complex'
import { formatComplexMarkdown } from '../../utils/format/markdown'
import MarkdownMath from './MarkdownMath'

type Props = {
  value?: Complex
  values?: ReadonlyArray<Complex>
  interleaved?: ArrayLike<number>
  precision?: number
  imag?: 'i' | 'j'
  orientation?: 'row' | 'col'
}

export function ComplexView({ value, values, interleaved, precision, imag, orientation }: Props) {
  let md = ''
  if (value) {
    md = formatComplexMarkdown(value, { precision, imag })
  } else if (values) {
    md = formatComplexMarkdown(values, { precision, imag, orientation })
  } else {
    md = formatComplexMarkdown(interleaved || [], { precision, imag, orientation })
  }
  return <MarkdownMath math={md} />
}

export default ComplexView
