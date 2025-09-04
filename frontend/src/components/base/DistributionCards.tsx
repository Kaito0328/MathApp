"use client"
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { CoreColorKey, SizeKey, RoundKey, ColorViewProperty, SizeViewProperty, SizeTextProperty, FontWeightKey, ColorTextProperty } from '../../design/tokens'

export function ContinuousPdfCard({ title, svg }: { title: string; svg: string }) {
  return (
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{title}</BaseText>
      <div style={{ marginTop: 8 }}>
        <BaseText styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorTextProperty.Text] } } }}>
          <div dangerouslySetInnerHTML={{ __html: svg }} />
        </BaseText>
      </div>
    </BaseBox>
  )
}

export function DiscretePmfCard({ title, svg }: { title: string; svg: string }) {
  return (
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{title}</BaseText>
      <div style={{ marginTop: 8 }}>
        <BaseText styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorTextProperty.Text] } } }}>
          <div dangerouslySetInnerHTML={{ __html: svg }} />
        </BaseText>
      </div>
    </BaseBox>
  )
}
