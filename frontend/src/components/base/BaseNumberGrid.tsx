"use client"
import React, { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { CoreColorKey, SizeKey, RoundKey, FontWeightKey, SizeTextProperty, SizeViewProperty, ColorViewProperty } from '../../design/tokens'

// A reusable grid editor for numbers with keyboard navigation (Tab/Enter/Arrow)
// - Renders rows x cols inputs
// - onChange is called with a 2D number array
// - Navigation: Tab/Right → next col, Shift+Tab/Left → prev col, Enter/Down → next row, Shift+Enter/Up → prev row

export function BaseNumberGrid({
  rows,
  cols,
  value,
  onChange,
  label,
  min,
  max,
  step,
  placeholder,
  className,
}: {
  rows: number
  cols: number
  value?: number[][]
  onChange: (m: number[][]) => void
  label?: string
  min?: number
  max?: number
  step?: number
  placeholder?: string
  className?: string
}) {
  const [grid, setGrid] = useState<number[][]>(() => normalize(value, rows, cols))
  const inputsRef = useRef<Array<Array<HTMLInputElement | null>>>([])

  useEffect(() => {
    setGrid(normalize(value, rows, cols))
  }, [rows, cols, value])

  // keep one effect only (above), to avoid duplicate state sets

  // Notify parent only on user edits (inside setCell); avoid effect-based echo loops

  const setCell = useCallback((r: number, c: number, v: number, notify: boolean = true) => {
    setGrid(prev => {
      // normalize prev to current shape to avoid OOB when shape just changed
      const base = normalize(prev as any, rows, cols)
      const next = base.map(row => row.slice())
      next[r][c] = v
      if (notify) {
        try { onChange(next) } catch { /* ignore user onChange errors */ }
      }
      return next
    })
  }, [onChange, rows, cols])

  const focusCell = useCallback((r: number, c: number) => {
    const row = inputsRef.current[r]
    const el = row?.[c]
    el?.focus()
    el?.select()
  }, [])

  const onKeyDown = useCallback((e: React.KeyboardEvent<HTMLInputElement>, r: number, c: number) => {
    const lastRow = rows - 1
    const lastCol = cols - 1
    const isShift = e.shiftKey
    const key = e.key
    const prevent = () => { e.preventDefault(); e.stopPropagation() }

    const move = (nr: number, nc: number) => {
      prevent()
      const rr = Math.max(0, Math.min(lastRow, nr))
      const cc = Math.max(0, Math.min(lastCol, nc))
      focusCell(rr, cc)
    }

    if (key === 'Enter') return move(r + (isShift ? -1 : 1), c)
    if (key === 'Tab')   return move(r, c + (isShift ? -1 : 1))
    if (key === 'ArrowRight') return move(r, c + 1)
    if (key === 'ArrowLeft')  return move(r, c - 1)
    if (key === 'ArrowDown')  return move(r + 1, c)
    if (key === 'ArrowUp')    return move(r - 1, c)
  }, [rows, cols, focusCell])

  const gridStyle: React.CSSProperties = useMemo(() => ({
    display: 'grid',
  gridTemplateColumns: `repeat(${cols}, minmax(64px, 1fr))`,
  gap: 8,
  }), [cols])

  // View grid guarantees the shape matches rows x cols during render
  const viewGrid = useMemo(() => normalize(grid, rows, cols), [grid, rows, cols])

  return (
  <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} className={className} style={{ borderWidth: 1 }}>
      {label && (
        <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
      )}
      <div style={{ display: 'flex', gap: 8, marginTop: 8 }}>
  <BaseText styleKit={{ size: { sizeKey: SizeKey.SM, apply: { default: [SizeTextProperty.FontSize] } } }}>size: {rows} × {cols}</BaseText>
      </div>
      <div style={{ marginTop: 8, ...gridStyle }}>
        {Array.from({ length: rows }).map((_, r) => (
          <React.Fragment key={r}>
      {Array.from({ length: cols }).map((__, c) => (
              <input
                key={`${r}-${c}`}
                ref={el => {
                  inputsRef.current[r] ??= []
                  inputsRef.current[r][c] = el
                }}
                type="number"
                inputMode="decimal"
        value={toDisplay(viewGrid[r][c])}
                min={min}
                max={max}
                step={step}
                placeholder={placeholder}
                onKeyDown={(e) => onKeyDown(e, r, c)}
                onChange={(e) => {
                  const str = e.target.value
                  if (str === '') {
                    // allow blank while editing; don't notify parent yet
                    setCell(r, c, Number.NaN, false)
                  } else {
                    const n = Number(str)
                    const finite = Number.isFinite(n)
                    setCell(r, c, finite ? n : Number.NaN, finite)
                  }
                }}
  style={{ width: '100%', minHeight: 32, padding: '6px 8px', border: '1px solid var(--c-base-border)', borderRadius: 6, fontFamily: 'ui-monospace, monospace', boxSizing: 'border-box' }}
              />
            ))}
          </React.Fragment>
        ))}
      </div>
    </BaseBox>
  )
}

function normalize(value: number[][] | undefined, rows: number, cols: number): number[][] {
  const g = Array.from({ length: rows }, (_, r) => Array.from({ length: cols }, (_, c) => value?.[r]?.[c] ?? 0))
  return g
}

function toDisplay(n: number): string | number {
  return Number.isFinite(n) ? n : ''
}
