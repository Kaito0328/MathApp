"use client"
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { CoreColorKey, SizeKey, RoundKey, FontWeightKey, SizeTextProperty, SizeViewProperty, ColorViewProperty, ColorTextProperty } from '../../design/tokens'
import { formatPolynomial, formatPolynomialMarkdown } from '../utils/polynomial'
import ReactMarkdown from 'react-markdown'
import remarkMath from 'remark-math'
import rehypeKatex from 'rehype-katex'

export function PolynomialCard({ coeffs, varName = 'x', showMarkdown = true }: { coeffs: number[]; varName?: string; showMarkdown?: boolean }) {
  const deg = Math.max(0, coeffs.length - 1)
  const text = formatPolynomial(coeffs, varName)
  const md = formatPolynomialMarkdown(coeffs, varName)
  return (
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>Polynomial</BaseText>
      <div style={{ display: 'grid', gridTemplateColumns: '140px 1fr', rowGap: 4, columnGap: 8, marginTop: 8 }}>
  <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>degree</BaseText><div>{deg}</div>
  <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>coeffs</BaseText><div>[{coeffs.join(', ')}]</div>
  <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>display</BaseText><div style={{ fontFamily: 'ui-monospace, monospace' }}>{text || '0'}</div>
        {showMarkdown && (
          <>
            <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>math</BaseText>
            <div>
              <ReactMarkdown remarkPlugins={[remarkMath]} rehypePlugins={[rehypeKatex]}>
                {`$${md}$`}
              </ReactMarkdown>
            </div>
          </>
        )}
      </div>
    </BaseBox>
  )
}
