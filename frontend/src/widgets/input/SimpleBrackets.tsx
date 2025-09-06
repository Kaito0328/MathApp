"use client"
import React from 'react'
import { MarkdownMath } from '../display/MarkdownMath'

type Props = {
  side: 'left' | 'right'
  rows: number
  gapPx?: number
  base?: number // C の基準（1行あたりの換算係数）
  perGap?: number // ギャップ1pxあたりの加算係数
}

// gap を加味した定数 C を使い、行数×C に応じた vphantom 行数で括弧高さを近似
export const SimpleBrackets: React.FC<Props> = ({ side, rows, gapPx = 0, base = 1.7, perGap = 0.02 }) => {
  const N = Math.max(1, Math.floor(rows * (base + perGap * gapPx)))
  const body = React.useMemo(() => Array.from({ length: N }).map(() => '0').join(' \\\\ '), [N])
  const phantom = `\\vphantom{\\begin{array}{c} ${body} \\end{array}}`
  const math = side === 'left' ? `\\left[ ${phantom} \\right.` : `\\left. ${phantom} \\right]`
  return (
    <span className="katex-bracket">
      <MarkdownMath math={math} block={false} />
    </span>
  )
}

export default SimpleBrackets
