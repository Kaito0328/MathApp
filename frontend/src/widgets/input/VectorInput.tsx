"use client"
import React from 'react'
import { Vector } from '../dto/linalg'
import NumberCellInput from '../../baseComponents/input/NumberCellInput'
import SimpleBrackets from './SimpleBrackets'

export type VectorInputProps = {
  value: Vector
  onChange: (value: Vector) => void
  orientation?: 'row' | 'col'
  length?: number
  cellWidth?: number | string
  gap?: number
}

export const VectorInput: React.FC<VectorInputProps> = ({ value, onChange, orientation = 'col', length, cellWidth = 72, gap }) => {
  const n = length ?? value.data.length
  const data = React.useMemo(() => {
    const d = value.data.slice(0, n)
    while (d.length < n) d.push(0)
    return d
  }, [value.data, n])

  const update = (i: number, v: number) => {
    const next = data.slice()
    next[i] = v
    onChange({ data: next })
  }

  // ギャップはデフォルトで 0（括弧とセル、セル間の隙間をなくす）
  const g = typeof gap === 'number' ? gap : 10
  return (
    <div style={{ alignItems: 'center', display: 'inline-flex', gap: 0 }}>
  <SimpleBrackets side="left" rows={orientation === 'row' ? 1 : n} gapPx={g} />
  <div style={{ display: 'grid', gridAutoFlow: orientation === 'row' ? 'column' : 'row', gap: g }}>
        {data.map((x, i) => (
          <NumberCellInput key={i} value={x} onChange={(v) => update(i, v)} width={cellWidth} />
        ))}
      </div>
  <SimpleBrackets side="right" rows={orientation === 'row' ? 1 : n} gapPx={g} />
    </div>
  )
}

export default VectorInput
