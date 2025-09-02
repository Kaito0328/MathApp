"use client"
import React, { useMemo, useState } from 'react'
import type { TransferFunction, Zpk } from '../../types'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { NumberField } from './shared'
//
import { ComplexVectorInput } from './ComplexVectorInput'
import { SizeKey, RoundKey, SizeViewProperty, SizeTextProperty, FontWeightKey } from '../../design/tokens'
import { PolynomialTermInput } from './PolynomialTermInput'

export function TransferFunctionInput({ value, onChange, label = 'Transfer Function Input', varName = 'z' }: {
  value?: TransferFunction
  onChange: (tf: TransferFunction) => void
  label?: string
  varName?: string
}) {
  const [numVec, setNumVec] = useState<number[]>(value?.num ?? [1])
  const [denVec, setDenVec] = useState<number[]>(value?.den ?? [1])
  const [ts, setTs] = useState<number | ''>(value?.sample_time ?? '')
  const valid = useMemo(() => denVec.length > 0 && denVec.some((c) => c !== 0), [denVec])
  const apply = () => onChange({ num: numVec, den: denVec, sample_time: ts === '' ? undefined : Number(ts) })
  return (
    <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
        <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto', display: 'flex', gap: 8, alignItems: 'center' }}>
          {!valid && <BaseText>den must be non-zero</BaseText>}
          <button onClick={apply} disabled={!valid}>Apply</button>
        </div>
      </div>
      <div style={{ display: 'grid', gridTemplateColumns: '1fr', gap: 12 }}>
        <PolynomialTermInput value={numVec} onChange={setNumVec} label={`Numerator (${varName})`} varName={varName} />
        <PolynomialTermInput value={denVec} onChange={setDenVec} label={`Denominator (${varName})`} varName={varName} />
        <NumberField label="sample_time (optional)" value={ts} onChange={setTs} />
      </div>
    </BaseBox>
  )
}

export function ZpkInput({ value, onChange, label = 'ZPK Input' }: {
  value?: Zpk
  onChange: (z: Zpk) => void
  label?: string
}) {
  const [zerosVec, setZerosVec] = useState<number[]>(value?.zeros ?? [])
  const [polesVec, setPolesVec] = useState<number[]>(value?.poles ?? [])
  const [gain, setGain] = useState<number | ''>(value?.gain ?? 1)
  const [ts, setTs] = useState<number | ''>(value?.sample_time ?? '')
  const apply = () => onChange({ zeros: zerosVec, poles: polesVec, gain: Number(gain) || 0, sample_time: ts === '' ? undefined : Number(ts) })
  return (
    <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
        <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}><button onClick={apply}>Apply</button></div>
      </div>
      <div style={{ display: 'grid', gridTemplateColumns: '1fr', gap: 12 }}>
  <ComplexVectorInput value={zerosVec} onChange={setZerosVec} label={'Zeros (Re, Im)'} showIndex={false} />
  <ComplexVectorInput value={polesVec} onChange={setPolesVec} label={'Poles (Re, Im)'} showIndex={false} />
        <NumberField label="gain" value={gain} onChange={setGain} />
        <NumberField label="sample_time (optional)" value={ts} onChange={setTs} />
      </div>
    </BaseBox>
  )
}
