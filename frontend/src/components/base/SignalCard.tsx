"use client"
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { Sparkline } from '../common'
import { CoreColorKey, SizeKey, RoundKey, FontWeightKey, SizeTextProperty, SizeViewProperty, ColorViewProperty, ColorTextProperty } from '../../design/tokens'

export function SignalCard({ data, sample_rate }: { data: number[]; sample_rate: number }) {
  const dur = data.length && sample_rate ? data.length / sample_rate : 0
  const max = data.length ? Math.max(...data) : 0
  const min = data.length ? Math.min(...data) : 0
  const mean = data.length ? (data.reduce((a, b) => a + b, 0) / data.length) : 0
  return (
  <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>Signal</BaseText>
      <div style={{ display: 'grid', gridTemplateColumns: '140px 1fr', rowGap: 4, columnGap: 8, marginTop: 8 }}>
        <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>sample_rate</BaseText><div>{sample_rate} Hz</div>
        <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>length</BaseText><div>{data.length}</div>
        <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>duration</BaseText><div>{dur.toFixed(3)} s</div>
        <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>min/max</BaseText><div>{min.toFixed(3)} / {max.toFixed(3)}</div>
        <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>mean</BaseText><div>{mean.toFixed(3)}</div>
      </div>
      <div style={{ marginTop: 8 }}>
        <Sparkline data={data.slice(0, 512)} />
      </div>
    </BaseBox>
  )
}
