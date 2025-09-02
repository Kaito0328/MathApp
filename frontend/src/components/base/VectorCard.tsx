"use client"
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { Sparkline } from '../common'
import { CoreColorKey, SizeKey, RoundKey, ColorViewProperty, SizeViewProperty, SizeTextProperty, FontWeightKey, ColorTextProperty } from '../../design/tokens'

export function VectorCard({ data, title = 'Vector', showSizeBadge = false }: { data: number[]; title?: string; showSizeBadge?: boolean }) {
  return (
  <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1, position: 'relative' }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{title}</BaseText>
      <div style={{ marginTop: 8 }}>
        <Sparkline data={data.slice(0, 512)} />
      </div>
      {showSizeBadge && (
        <div style={{ position: 'absolute', right: 8, bottom: 8 }}>
          <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } }, size: { sizeKey: SizeKey.SM, apply: { default: [SizeTextProperty.FontSize] } } }}>{data.length}</BaseText>
        </div>
      )}
    </BaseBox>
  )
}
