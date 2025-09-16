"use client"
import React from 'react'
import MarkdownMath from '../../../widgets/display/MarkdownMath'

type Props = { lines: string[] }

export default function CodeCharacteristics({ lines }: Props) {
  const body = lines.join(' \\\\')
  const math = `\\begin{aligned}${body}\\end{aligned}`
  return <MarkdownMath math={math} block={true} />
}
