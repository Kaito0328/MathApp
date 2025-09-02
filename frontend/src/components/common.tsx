import React from 'react'
import { BaseText } from '../design/base/BaseText'
import { BaseBox } from '../design/base/BaseBox'
import { CoreColorKey, SizeKey, RoundKey, ColorViewProperty, SizeTextProperty, SizeViewProperty, ColorTextProperty } from '../design/tokens'

export function Sparkline({ data, width = 240, height = 48, colorKey = CoreColorKey.Primary }: { data: number[]; width?: number; height?: number; colorKey?: CoreColorKey }) {
  const w = width
  const h = height
  if (!data || data.length === 0) return (
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.SM, apply: { default: [] } }, roundKey: RoundKey.Md }} style={{ width: w, height: h, borderWidth: 1 }} />
  )
  const max = Math.max(...data)
  const min = Math.min(...data)
  const range = max - min || 1
  const points = data
    .map((v, i) => {
      const x = (i / (data.length - 1)) * w
      const y = h - ((v - min) / range) * h
      return `${x.toFixed(2)},${y.toFixed(2)}`
    })
    .join(' ')
  return (
    <BaseText styleKit={{ color: { colorKey, apply: { default: [ColorTextProperty.Text] } }, size: { sizeKey: SizeKey.SM, apply: { default: [] } } }}>
      <svg width={w} height={h} viewBox={`0 0 ${w} ${h}`}>
        <polyline fill="none" stroke="currentColor" strokeWidth={1} points={points} />
      </svg>
    </BaseText>
  )
}

export function KV({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div style={{ display: 'flex', gap: 8 }}>
      <div style={{ width: 140 }}>
  <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } }, size: { sizeKey: SizeKey.SM, apply: { default: [SizeTextProperty.FontSize] } } }}>{label}</BaseText>
      </div>
      <div>{children}</div>
    </div>
  )
}

export function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1, margin: '12px 0' }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } } }}>
        {title}
      </BaseText>
      {children}
    </BaseBox>
  )
}
