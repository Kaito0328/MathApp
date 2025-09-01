"use client"
import React, { useEffect, useMemo, useState } from 'react'
import type { Matrix } from '../../types'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'

function parseMatrix(src: string): { rows: number; cols: number; data: number[] } | null {
  const lines = src
    .split(/\n+/)
    .map(l => l.trim())
    .filter(l => l.length > 0)
  if (lines.length === 0) return { rows: 0, cols: 0, data: [] }
  const parsed = lines.map(l => l.split(/[,\s]+/).filter(Boolean).map(Number).filter(n => Number.isFinite(n)))
  const cols = Math.max(0, ...parsed.map(r => r.length))
  if (cols === 0) return { rows: parsed.length, cols: 0, data: [] }
  // Normalize jagged rows by padding with 0
  const rows = parsed.length
  const data: number[] = []
  for (const r of parsed) {
    for (let i = 0; i < cols; i++) data.push(r[i] ?? 0)
  }
  return { rows, cols, data }
}

export function MatrixInput({ value, onChange, placeholder }: { value?: Matrix; onChange: (m: Matrix) => void; placeholder?: string }) {
  const initial = useMemo(() => {
    if (!value) return ''
    const { rows, cols, data } = value
    return Array.from({ length: rows })
      .map((_, r) => Array.from({ length: cols }, (_, c) => data[r * cols + c]).join(', '))
      .join('\n')
  }, [value])
  const [text, setText] = useState<string>(initial)

  useEffect(() => {
    // sync external value changes
    setText(initial)
  }, [initial])

  const parsed = useMemo(() => parseMatrix(text), [text])
  const valid = !!parsed && Number.isFinite(parsed.rows) && Number.isFinite(parsed.cols)

  const handleApply = () => {
    if (!parsed) return
    onChange({ rows: parsed.rows, cols: parsed.cols, data: parsed.data })
  }

  return (
    <BaseBox styleKit={{ size: { sizeKey: 'md' as any, apply: { default: ['padding'] as any } }, roundKey: 'md' as any }} className="border-base" style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: 'md' as any, apply: { default: ['fontSize'] as any } }, fontWeightKey: 'medium' as any }}>Matrix Input</BaseText>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 8, marginTop: 8 }}>
        <textarea
          value={text}
          onChange={(e) => setText(e.target.value)}
          rows={6}
          placeholder={placeholder ?? 'CSV-like rows\n1, 2, 3\n4, 5, 6'}
          style={{ width: '100%', fontFamily: 'ui-monospace, monospace' }}
        />
        <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
          <button onClick={handleApply} disabled={!valid}>Apply</button>
          <BaseText>shape: {parsed?.rows ?? 0} × {parsed?.cols ?? 0}</BaseText>
        </div>
      </div>
    </BaseBox>
  )
}
