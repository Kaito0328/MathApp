"use client"
import React, { useMemo, useState } from 'react'
import type { PolynomialR } from '../../types'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { VectorInput } from './VectorInputV2'
import { CoreColorKey, SizeKey, RoundKey, FontWeightKey, SizeViewProperty, SizeTextProperty, ColorViewProperty } from '../../design/tokens'

export function RationalFunctionInput({ value, onChange, label = 'Rational Function Input' }: {
  value?: { num: PolynomialR; den: PolynomialR }
  onChange: (rf: { num: PolynomialR; den: PolynomialR }) => void
  label?: string
}) {
  const [numVec, setNumVec] = useState<number[]>(value?.num.coeffs ?? [1])
  const [denVec, setDenVec] = useState<number[]>(value?.den.coeffs ?? [1, 0])
  const num = useMemo(() => ({ coeffs: numVec }), [numVec])
  const den = useMemo(() => ({ coeffs: denVec }), [denVec])
  const apply = () => onChange({ num, den })
  return (
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
        <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <button onClick={apply}>Apply</button>
        </div>
      </div>
      <div style={{ display: 'grid', gridTemplateColumns: '1fr', gap: 12 }}>
        <div>
          <BaseText>numerator</BaseText>
          <VectorInput value={numVec} onChange={setNumVec} placeholder={'0'} label={''} />
        </div>
        <div>
          <BaseText>denominator</BaseText>
          <VectorInput value={denVec} onChange={setDenVec} placeholder={'0'} label={''} />
        </div>
      </div>
    </BaseBox>
  )
}
