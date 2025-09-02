"use client"
import React, { useMemo, useState } from 'react'
import type { Signal } from '../../types'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { SizeKey, SizeTextProperty, SizeViewProperty, RoundKey, FontWeightKey, CoreColorKey, ColorViewProperty, ColorTextProperty } from '../../design/tokens'

function parseNumbers(src: string): number[] {
  return src
    .split(/[\s,]+/)
    .map((s) => s.trim())
    .filter(Boolean)
    .map(Number)
    .filter((n) => Number.isFinite(n))
}

export function SignalInputSimple({ value, onChange, label = 'Signal Input' }: { value?: Signal; onChange: (s: Signal) => void; label?: string }) {
  const [sr, setSr] = useState<number>(value?.sample_rate ?? 64)
  const [text, setText] = useState<string>((value?.data ?? []).join(', '))
  const data = useMemo(() => parseNumbers(text), [text])
  const apply = () => onChange({ data, sample_rate: Math.max(1, Number(sr) || 1) })

  return (
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 8, marginTop: 8 }}>
        <label style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <BaseText>sample_rate</BaseText>
          <input type="number" min={1} value={sr} onChange={(e) => setSr(Number(e.target.value) || 0)} />
        </label>
        <div>
          <BaseText>data</BaseText>
          <textarea rows={4} value={text} onChange={(e) => setText(e.target.value)} placeholder={'e.g. 0, 1, 0, -1, ...'} style={{ width: '100%', fontFamily: 'ui-monospace, monospace' }} />
        </div>
        <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
          <button onClick={apply}>Apply</button>
          <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>length: {data.length}</BaseText>
        </div>
      </div>
    </BaseBox>
  )
}
