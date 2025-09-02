"use client"
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { Sparkline } from '../common'
import { CoreColorKey, SizeKey, SizeViewProperty, ColorViewProperty, RoundKey, SizeTextProperty, FontWeightKey } from '../../design/tokens'

export function SignalCardSimple({ data, title = 'Signal', showPlot = true, showVector = false }: { data: number[]; title?: string; showPlot?: boolean; showVector?: boolean }) {
  return (
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{title}</BaseText>
      <div style={{ marginTop: 8 }}>
        {showPlot && <Sparkline data={data.slice(0, 512)} />}
        {showVector && <div style={{ fontFamily: 'ui-monospace, monospace', marginTop: 8 }}>[{data.join(', ')}]</div>}
      </div>
    </BaseBox>
  )
}
