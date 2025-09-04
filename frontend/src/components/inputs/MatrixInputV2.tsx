"use client"
import React, { useMemo, useState } from 'react'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { BaseNumberGrid } from '../base/BaseNumberGrid'
import { SizeKey, RoundKey, SizeViewProperty, SizeTextProperty, FontWeightKey, CoreColorKey, ColorTextProperty } from '../../design/tokens'

// MatrixInputV2: edits a 2D matrix using BaseNumberGrid (R x C)
export function MatrixInput({ value, onChange, placeholder, label = 'Matrix Input', controlledSize, hideSizeControls }: {
  value?: { rows: number, cols: number, data: number[] }
  onChange: (m: { rows: number, cols: number, data: number[] }) => void
  placeholder?: string
  label?: string
  controlledSize?: { rows: number, cols: number }
  hideSizeControls?: boolean
}) {
  // applied size used by grid
  const [rows, setRows] = useState<number>(value?.rows ?? 3)
  const [cols, setCols] = useState<number>(value?.cols ?? 3)
  // editable inputs (staged) to avoid mid-edit errors
  const [rowsInput, setRowsInput] = useState<number>(value?.rows ?? 3)
  const [colsInput, setColsInput] = useState<number>(value?.cols ?? 3)

  // sync from external size controller
  React.useEffect(() => {
    if (controlledSize) {
      const r = Math.max(1, Math.floor(controlledSize.rows || 1))
      const c = Math.max(1, Math.floor(controlledSize.cols || 1))
      setRows(r); setCols(c);
      setRowsInput(r); setColsInput(c);
    }
  }, [controlledSize])

  const grid = useMemo(() => {
    const vr = value?.rows ?? rows
    const vc = value?.cols ?? cols
    const arr = value?.data ?? []
    const g = Array.from({ length: rows }, (_, r) =>
      Array.from({ length: cols }, (_, c) => {
        if (!value) return 0
        if (r >= vr || c >= vc) return 0
        const idx = r * vc + c
        return arr[idx] ?? 0
      })
    )
    return g
  }, [value, rows, cols])

  const handleGrid = (m: number[][]) => {
    const data: number[] = []
    for (let r = 0; r < rows; r++) for (let c = 0; c < cols; c++) data.push(m[r]?.[c] ?? 0)
    onChange({ rows, cols, data })
  }

  return (
    <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
        <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        {!hideSizeControls && (
          <div style={{ marginLeft: 'auto', display: 'flex', gap: 6 }}>
            <label>
              <BaseText>rows</BaseText>{' '}
              <input type="number" min={1} value={rowsInput} onChange={(e) => {
                const v = e.target.value
                if (v === '') return setRowsInput(NaN as unknown as number)
                const n = Number(v)
                setRowsInput(Number.isFinite(n) ? Math.max(1, Math.floor(n)) : (NaN as unknown as number))
              }} />
            </label>
            <label>
              <BaseText>cols</BaseText>{' '}
              <input type="number" min={1} value={colsInput} onChange={(e) => {
                const v = e.target.value
                if (v === '') return setColsInput(NaN as unknown as number)
                const n = Number(v)
                setColsInput(Number.isFinite(n) ? Math.max(1, Math.floor(n)) : (NaN as unknown as number))
              }} />
            </label>
            <button type="button" onClick={() => {
              const r = Number.isFinite(rowsInput as unknown as number) ? rowsInput : rows
              const c = Number.isFinite(colsInput as unknown as number) ? colsInput : cols
              setRows(Math.max(1, r)); setCols(Math.max(1, c));
            }}>
              Apply size
            </button>
          </div>
        )}
      </div>
      <BaseNumberGrid rows={rows} cols={cols} value={grid} onChange={handleGrid} placeholder={placeholder ?? '0'} />
      <div style={{ display: 'flex', justifyContent: 'flex-end', marginTop: 6 }}>
        <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } }, size: { sizeKey: SizeKey.SM, apply: { default: [SizeTextProperty.FontSize] } } }}>{rows}×{cols}</BaseText>
      </div>
    </BaseBox>
  )
}
