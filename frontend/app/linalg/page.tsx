"use client"
import Link from 'next/link'
import { BaseBox } from '../../src/design/base/BaseBox'
import { BaseText } from '../../src/design/base/BaseText'
import { CoreColorKey, ColorViewProperty, SizeKey, SizeViewProperty, SizeTextProperty, FontWeightKey } from '../../src/design/tokens'
import { VariableManager } from '../../src/components/variables/VariableManager'

export default function LinalgHome() {
  return (
    <div style={{ display: 'grid', gap: 12 }}>
      <BaseText style={{ fontWeight: 600 }}>変数</BaseText>
      <VariableManager />

      <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } } }} style={{ borderWidth: 1 }}>
        <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>各種演算</BaseText>
        <div style={{ display: 'grid', gap: 8, marginTop: 8 }}>
          <Link href="/linalg/add">A + B（加算）</Link>
          <Link href="/linalg/mul">A × B（乗算）</Link>
          <Link href="/linalg/decomp">分解（LU/QR/SVD）</Link>
        </div>
      </BaseBox>
    </div>
  )
}
