"use client"
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import React from 'react'
import ReactMarkdown from 'react-markdown'
import remarkMath from 'remark-math'
import rehypeKatex from 'rehype-katex'
import { CoreColorKey, SizeKey, RoundKey, SizeTextProperty, SizeViewProperty, ColorViewProperty, FontWeightKey } from '../../design/tokens'

export function RationalFunctionCard({ markdown, title = 'Rational Function' }: { markdown: string; title?: string }) {
  return (
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{title}</BaseText>
      <div style={{ marginTop: 8 }}>
        <ReactMarkdown remarkPlugins={[remarkMath]} rehypePlugins={[rehypeKatex]}>
          {`$${markdown}$`}
        </ReactMarkdown>
      </div>
    </BaseBox>
  )
}
