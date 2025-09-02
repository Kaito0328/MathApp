"use client"
import React, { useMemo, useState } from 'react'
import type { PolynomialR } from '../../types'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { CoreColorKey, SizeKey, RoundKey, ColorViewProperty, SizeViewProperty, SizeTextProperty, FontWeightKey, ColorTextProperty } from '../../design/tokens'
import { parseNumbers } from './shared'

export function PolynomialInput({ value, onChange, placeholder }: { value?: PolynomialR; onChange: (p: PolynomialR) => void; placeholder?: string }) {
  const initial = useMemo(() => (value?.coeffs ?? []).join(', '), [value])
  const [text, setText] = useState<string>(initial)

  const coeffs = useMemo(() => parseNumbers(text), [text])
  const degree = Math.max(0, coeffs.length - 1)

  const handleApply = () => onChange({ coeffs })

  return (
  <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>Polynomial Input</BaseText>
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
          <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>degree: {degree}</BaseText>
        </div>
      </div>
    </BaseBox>
  )
}
