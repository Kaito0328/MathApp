"use client"
import React from 'react'
import { BaseText } from '../../design/base/BaseText'

// number list parser used across inputs
export function parseNumbers(src: string): number[] {
  return src
    .split(/[\s,]+/)
    .map((s) => s.trim())
    .filter(Boolean)
    .map(Number)
    .filter((n) => Number.isFinite(n))
}

export function NumberField({
  label,
  value,
  onChange,
  min,
  step,
  placeholder,
}: {
  label?: string
  value: number | ''
  onChange: (v: number | '') => void
  min?: number
  step?: number
  placeholder?: string
}) {
  return (
    <label style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
      {label && <BaseText>{label}</BaseText>}
      <input
        type="number"
        value={value as any}
        onChange={(e) => {
          const v = e.target.value
          if (v === '') return onChange('')
          const n = Number(v)
          onChange(Number.isFinite(n) ? n : '')
        }}
        min={min as any}
        step={step as any}
        placeholder={placeholder}
      />
    </label>
  )
}

export function TextAreaNumbers({
  label,
  value,
  onChange,
  rows = 2,
  placeholder,
}: {
  label: string
  value: string
  onChange: (v: string) => void
  rows?: number
  placeholder?: string
}) {
  return (
    <div>
      <BaseText>{label}</BaseText>
      <textarea
        value={value}
        onChange={(e) => onChange(e.target.value)}
        rows={rows}
        placeholder={placeholder}
        style={{ width: '100%', fontFamily: 'ui-monospace, monospace' }}
      />
    </div>
  )
}

export function ApplyRow({ onApply, disabled, children }: { onApply: () => void; disabled?: boolean; children?: React.ReactNode }) {
  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
      <button onClick={onApply} disabled={disabled}>Apply</button>
      {children}
    </div>
  )
}
