"use client"
import React, { useMemo, useState } from 'react'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { SizeKey, RoundKey, SizeViewProperty, SizeTextProperty, FontWeightKey } from '../../design/tokens'
import { TextAreaNumbers, parseNumbers } from './shared'

export function ComplexArrayInput({
  value,
  onChange,
  label = 'Complex Array Input',
}: {
  value?: { re: number[]; im: number[] } | { interleaved: number[] }
  onChange: (out: { interleaved: number[] }) => void
  label?: string
}) {
  const [reTxt, setReTxt] = useState('')
  const [imTxt, setImTxt] = useState('')

  const interleaved = useMemo(() => {
    let R: number[] = []
    let I: number[] = []
    if (value && 'interleaved' in value) {
      const v = value.interleaved
      const n = Math.floor(v.length / 2)
      R = new Array(n); I = new Array(n)
      for (let i = 0; i < n; i++) { R[i] = v[2 * i] || 0; I[i] = v[2 * i + 1] || 0 }
    } else if (value) {
      R = (value as any).re || []; I = (value as any).im || []
    }
    const R2 = parseNumbers(reTxt)
    const I2 = parseNumbers(imTxt)
    const N = Math.max(R2.length || R.length, I2.length || I.length)
    const out: number[] = []
    for (let i = 0; i < N; i++) out.push((R2[i] ?? R[i] ?? 0), (I2[i] ?? I[i] ?? 0))
    return out
  }, [value, reTxt, imTxt])

  return (
    <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
        <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <button onClick={() => onChange({ interleaved })}>Apply</button>
        </div>
      </div>
      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 8 }}>
        <TextAreaNumbers label="Re" value={reTxt} onChange={setReTxt} rows={3} placeholder={'e.g. 1, 0, -1'} />
        <TextAreaNumbers label="Im" value={imTxt} onChange={setImTxt} rows={3} placeholder={'e.g. 0, 1, 0'} />
      </div>
    </BaseBox>
  )
}
