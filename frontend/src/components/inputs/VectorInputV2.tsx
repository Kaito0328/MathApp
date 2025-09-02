"use client"
import React, { useMemo, useState } from 'react'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { BaseNumberGrid } from '../base/BaseNumberGrid'
import { SizeKey, RoundKey, SizeTextProperty, SizeViewProperty, CoreColorKey, ColorTextProperty } from '../../design/tokens'

// VectorInputV2: edits a 1D vector using BaseNumberGrid (1 x N)
export function VectorInput({ value, onChange, placeholder, label = 'Vector Input' }: {
  value?: number[]
  onChange: (v: number[]) => void
  placeholder?: string
  label?: string
}) {
  const [cols, setCols] = useState<number>(value?.length ?? 4)
  const [colsInput, setColsInput] = useState<number>(value?.length ?? 4)
  const grid = useMemo(() => [Array.from({ length: cols }, (_, i) => value?.[i] ?? 0)], [value, cols])

  const handleGrid = (m: number[][]) => {
    onChange(m[0] ?? [])
  }

  return (
  <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
    <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } } }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto', display: 'flex', gap: 6 }}>
          <label>
            <BaseText>length</BaseText>{' '}
            <input type="number" min={1} value={colsInput} onChange={(e) => {
              const v = e.target.value
              if (v === '') return setColsInput(NaN as unknown as number)
              const n = Number(v)
              setColsInput(Number.isFinite(n) ? Math.max(1, Math.floor(n)) : (NaN as unknown as number))
            }} />
          </label>
          <button type="button" onClick={() => {
            const c = Number.isFinite(colsInput as unknown as number) ? colsInput : cols
            setCols(Math.max(1, c))
          }}>Apply size</button>
        </div>
      </div>
      <BaseNumberGrid rows={1} cols={cols} value={grid} onChange={handleGrid} placeholder={placeholder ?? '0'} />
      <div style={{ display: 'flex', justifyContent: 'flex-end', marginTop: 6 }}>
        <span>
          <span style={{ marginRight: 8 }}>
            <span></span>
          </span>
          <span>
            <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } }, size: { sizeKey: SizeKey.SM, apply: { default: [SizeTextProperty.FontSize] } } }}>{cols}</BaseText>
          </span>
        </span>
      </div>
    </BaseBox>
  )
}
