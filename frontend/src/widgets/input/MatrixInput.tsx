"use client"
import React from 'react'
import { Matrix } from '../dto/linalg'
import NumberCellInput from '../../baseComponents/input/NumberCellInput'
import SimpleBrackets from './SimpleBrackets'

export type MatrixInputProps = {
  value: Matrix
  onChange: (value: Matrix) => void
  rows?: number
  cols?: number
  cellWidth?: number | string
  gap?: number
}

export const MatrixInput: React.FC<MatrixInputProps> = ({ value, onChange, rows, cols, cellWidth = 72, gap }) => {
  const r = rows ?? value.rows
  const c = cols ?? value.cols
  const size = r * c
  const data = React.useMemo(() => {
    const d = value.data.slice(0, size)
    while (d.length < size) d.push(0)
    return d
  }, [value.data, size])

  const update = (ri: number, ci: number, v: number) => {
    const idx = ri * c + ci
    const next = data.slice()
    next[idx] = v
    onChange({ rows: r, cols: c, data: next })
  }

  // ギャップはデフォルトで 0（括弧とセル、セル間の隙間をなくす）
  const g = typeof gap === 'number' ? gap : 10
  return (
    <div style={{ display: 'inline-flex', alignItems: 'center', gap: 0 }}>
      <SimpleBrackets side="left" rows={r} gapPx={g} />
      <div style={{ display: 'grid', gridTemplateColumns: `repeat(${c}, auto)`, gap: g }}>
        {Array.from({ length: r }).map((_, ri) =>
          Array.from({ length: c }).map((_, ci) => {
            const idx = ri * c + ci
            return <NumberCellInput key={`${ri}-${ci}`} value={data[idx] ?? 0} onChange={(v) => update(ri, ci, v)} width={cellWidth} />
          }),
        )}
      </div>
      <SimpleBrackets side="right" rows={r} gapPx={g} />
    </div>
  )
}

export default MatrixInput
