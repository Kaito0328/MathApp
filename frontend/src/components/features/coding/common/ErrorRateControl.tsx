"use client"
import React from 'react'

type Props = {
  p: number
  seed: string
  onChange: (next: { p?: number, seed?: string }) => void
}

export default function ErrorRateControl({ p, seed, onChange }: Props) {
  return (
    <div style={{ display:'flex', gap:12, alignItems:'center', flexWrap:'wrap' }}>
      <label>
        反転率 p (0..1):
        <input
          type="number"
          min={0}
          max={1}
          step="0.001"
          value={p}
          onChange={(e)=> onChange({ p: Math.max(0, Math.min(1, Number(e.target.value) || 0)) })}
          style={{ width: 120, marginLeft:8 }}
        />
      </label>
      <label>
        シード:
        <input
          type="text"
          value={seed}
          onChange={(e)=> onChange({ seed: e.target.value })}
          style={{ width: 160, marginLeft:8 }}
          placeholder="任意"
        />
      </label>
    </div>
  )
}
