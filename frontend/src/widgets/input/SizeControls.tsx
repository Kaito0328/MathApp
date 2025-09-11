"use client"
import React from 'react'

export function VectorSizeControls({ length, onChange }: { length: number; onChange: (n: number) => void }) {
  return (
    <div style={{ display: 'inline-flex', alignItems: 'center', gap: 6, flexWrap: 'wrap' }}>
      <label style={{ display: 'inline-flex', alignItems: 'center', gap: 4 }}>長さ
        <input type="number" min={1} value={length} onChange={(e) => onChange(Math.max(1, Math.floor(Number(e.target.value) || 1)))} style={{ width: 72 }} />
      </label>
      <button onClick={() => onChange(length + 1)}>+</button>
      <button onClick={() => onChange(Math.max(1, length - 1))}>-</button>
    </div>
  )
}

export function MatrixSizeControls({ rows, cols, onChange }: { rows: number; cols: number; onChange: (r: number, c: number) => void }) {
  return (
    <div style={{ display: 'inline-flex', alignItems: 'center', gap: 12, flexWrap: 'wrap' }}>
      <div style={{ display: 'inline-flex', alignItems: 'center', gap: 6 }}>
        <label>行 <input type="number" min={1} value={rows} onChange={(e) => onChange(Math.max(1, Math.floor(Number(e.target.value) || 1)), cols)} style={{ width: 72 }} /></label>
        <button onClick={() => onChange(rows + 1, cols)}>+</button>
        <button onClick={() => onChange(Math.max(1, rows - 1), cols)}>-</button>
      </div>
      <div style={{ display: 'inline-flex', alignItems: 'center', gap: 6 }}>
        <label>列 <input type="number" min={1} value={cols} onChange={(e) => onChange(rows, Math.max(1, Math.floor(Number(e.target.value) || 1)))} style={{ width: 72 }} /></label>
        <button onClick={() => onChange(rows, cols + 1)}>+</button>
        <button onClick={() => onChange(rows, Math.max(1, cols - 1))}>-</button>
      </div>
    </div>
  )
}

// no default export
