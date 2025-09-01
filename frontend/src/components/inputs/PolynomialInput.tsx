"use client"
import React, { useMemo, useState } from 'react'
import type { PolynomialR } from '../../types'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'

function parseNumbers(src: string): number[] {
  return src
    .split(/[,\s]+/)
    .map(s => s.trim())
    .filter(s => s.length > 0)
    .map(Number)
    .filter(n => Number.isFinite(n))
}

export function PolynomialInput({ value, onChange, placeholder }: { value?: PolynomialR; onChange: (p: PolynomialR) => void; placeholder?: string }) {
  const initial = useMemo(() => (value?.coeffs ?? []).join(', '), [value])
  const [text, setText] = useState<string>(initial)

  const coeffs = useMemo(() => parseNumbers(text), [text])
  const degree = Math.max(0, coeffs.length - 1)

  const handleApply = () => onChange({ coeffs })

  return (
    <BaseBox styleKit={{ size: { sizeKey: 'md' as any, apply: { default: ['padding'] as any } }, roundKey: 'md' as any }} className="border-base" style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: 'md' as any, apply: { default: ['fontSize'] as any } }, fontWeightKey: 'medium' as any }}>Polynomial Input</BaseText>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 8, marginTop: 8 }}>
        <textarea
          value={text}
          onChange={(e) => setText(e.target.value)}
          rows={3}
          placeholder={placeholder ?? 'coeffs low→high, e.g. 1, -3, 2'}
          style={{ width: '100%', fontFamily: 'ui-monospace, monospace' }}
        />
        <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
          <button onClick={handleApply}>Apply</button>
          <BaseText>degree: {degree}</BaseText>
        </div>
      </div>
    </BaseBox>
  )
}
