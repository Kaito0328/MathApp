"use client"
import React, { useState } from 'react'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { PolynomialTermInput } from './PolynomialTermInput'
import { SizeKey, SizeViewProperty, RoundKey, SizeTextProperty, FontWeightKey } from '../../design/tokens'

export function RationalFunctionInputSimple({
  value,
  onChange,
  label = 'Rational Function',
  varName = 'x',
}: {
  value?: { num: number[]; den: number[] }
  onChange: (v: { num: number[]; den: number[] }) => void
  label?: string
  varName?: string
}) {
  const [num, setNum] = useState<number[]>(value?.num ?? [1])
  const [den, setDen] = useState<number[]>(value?.den ?? [1])

  const apply = () => onChange({ num, den })

  return (
    <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 8 }}>
        <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <button onClick={apply}>Apply</button>
        </div>
      </div>
      <div style={{ display: 'grid', gridTemplateColumns: '1fr', gap: 12 }}>
        <PolynomialTermInput value={num} onChange={setNum} label={`Numerator (${varName})`} varName={varName} />
        <PolynomialTermInput value={den} onChange={setDen} label={`Denominator (${varName})`} varName={varName} />
      </div>
    </BaseBox>
  )
}
