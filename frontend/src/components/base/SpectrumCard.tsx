"use client"
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { Sparkline } from '../common'
import { CoreColorKey, SizeKey, RoundKey, FontWeightKey, SizeTextProperty, SizeViewProperty, ColorViewProperty, ColorTextProperty } from '../../design/tokens'

export function SpectrumCard({ spectrum, sample_rate }: { spectrum: number[]; sample_rate: number }) {
  const bins = Math.floor(spectrum.length / 2)
  const magnitude = Array.from({ length: bins }, (_, i) => {
    const re = spectrum[2 * i] || 0
    const im = spectrum[2 * i + 1] || 0
    return Math.hypot(re, im)
  })
  return (
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>Spectrum (DFT)</BaseText>
      <div style={{ display: 'grid', gridTemplateColumns: '140px 1fr', rowGap: 4, columnGap: 8, marginTop: 8 }}>
        <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>sample_rate</BaseText>
        <BaseText>{sample_rate} Hz</BaseText>
        <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>bins</BaseText>
        <BaseText>{bins}</BaseText>
      </div>
      <div style={{ marginTop: 8 }}>
        <Sparkline data={magnitude.slice(0, 512)} />
      </div>
    </BaseBox>
  )
}
