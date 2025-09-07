import React from 'react'
import { Polynomial } from '../dto/polynomial'
import { formatPolynomialMarkdown } from '../../utils/format/markdown'
import MarkdownMath from './MarkdownMath'

type Props = { value?: Polynomial; coeffs?: ArrayLike<number>; varName?: string }

export function PolynomialView({ value, coeffs, varName }: Props) {
  const md = value ? formatPolynomialMarkdown(value.coeffs, varName) : formatPolynomialMarkdown(coeffs || [], varName)
  return <MarkdownMath math={md} />
}

export default PolynomialView
