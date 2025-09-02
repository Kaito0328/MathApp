"use client"
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import React from 'react'
import ReactMarkdown from 'react-markdown'
import remarkMath from 'remark-math'
import rehypeKatex from 'rehype-katex'
import { CoreColorKey, SizeKey, RoundKey, SizeTextProperty, SizeViewProperty, ColorViewProperty, FontWeightKey } from '../../design/tokens'
import { formatPolynomialMarkdown } from '../utils/polynomial'

export function PolynomialCardSimple({ coeffs, varName = 'x', title = 'Polynomial' }: { coeffs: number[]; varName?: string; title?: string }) {
  const md = formatPolynomialMarkdown(coeffs, varName)
  return (
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{title}</BaseText>
      <div style={{ marginTop: 8 }}>
  <BaseText styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: ['text' as any] } } as any }}>
          <ReactMarkdown remarkPlugins={[remarkMath]} rehypePlugins={[rehypeKatex]}>
            {`$${md}$`}
          </ReactMarkdown>
        </BaseText>
      </div>
    </BaseBox>
  )
}
