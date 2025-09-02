"use client"
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { CoreColorKey, SizeKey, RoundKey, FontWeightKey, SizeTextProperty, SizeViewProperty, ColorViewProperty, ColorTextProperty } from '../../design/tokens'

export function TransferFunctionCard({ num, den, sample_time }: { num: number[]; den: number[]; sample_time?: number | null }) {
  return (
  <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>Transfer Function</BaseText>
      <div style={{ display: 'grid', gridTemplateColumns: '140px 1fr', rowGap: 4, columnGap: 8, marginTop: 8 }}>
    <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>num</BaseText><div>[{num.join(', ')}]</div>
    <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>den</BaseText><div>[{den.join(', ')}]</div>
    <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>sample_time</BaseText><div>{sample_time ?? 'continuous'}</div>
      </div>
    </BaseBox>
  )
}

export function ZpkCard({ zeros, poles, gain, sample_time }: { zeros: number[]; poles: number[]; gain: number; sample_time?: number | null }) {
  return (
  <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>ZPK</BaseText>
      <div style={{ display: 'grid', gridTemplateColumns: '140px 1fr', rowGap: 4, columnGap: 8, marginTop: 8 }}>
    <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>gain</BaseText><div>{gain}</div>
    <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>zeros</BaseText><div>[{zeros.join(', ')}]</div>
    <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>poles</BaseText><div>[{poles.join(', ')}]</div>
    <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>sample_time</BaseText><div>{sample_time ?? 'continuous'}</div>
      </div>
    </BaseBox>
  )
}
