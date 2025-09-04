"use client"
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { CoreColorKey, SizeKey, RoundKey, ColorViewProperty, SizeViewProperty, SizeTextProperty, FontWeightKey } from '../../design/tokens'
import ReactMarkdown from 'react-markdown'
import remarkMath from 'remark-math'
import rehypeKatex from 'rehype-katex'
import { formatPolynomialMarkdown } from '../utils/polynomial'

export function TransferFunctionCardSimple({ num, den, varName = 'z', title = 'Transfer Function' }: { num: number[]; den: number[]; varName?: 'z' | 's' | string; title?: string }) {
  // one-line KaTeX to avoid JS string escapes like \f (form feed)
  const md = `$$H(${varName}) = \\frac{${formatPolynomialMarkdown(num, varName)}}{${formatPolynomialMarkdown(den, varName)}}$$`
  return (
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{title}</BaseText>
      <div style={{ marginTop: 8 }}>
        <BaseText>
          <ReactMarkdown remarkPlugins={[remarkMath]} rehypePlugins={[rehypeKatex]}>
            {md}
          </ReactMarkdown>
        </BaseText>
      </div>
    </BaseBox>
  )
}

export function ZpkCardSimple({ zeros, poles, gain, varName = 'z', title = 'ZPK' }: { zeros: Array<[number, number]>; poles: Array<[number, number]>; gain: number; varName?: 'z' | 's' | string; title?: string }) {
  // 表示: gain * Π (var - (a+bi)) / Π (var - (c+di))。複素表示は (a+bi)
  const fmt = (a: number, b: number) => (b === 0 ? `${a}` : `${a}${b >= 0 ? '+' : ''}${b}i`)
  const prod = (roots: Array<[number, number]>) => (roots.length === 0 ? '1' : roots.map(([a, b]) => `(${varName}-${fmt(a, b)})`).join(''))
  const g = Math.abs(gain - 1) < 1e-12 ? '' : `${gain}`
  const num = prod(zeros)
  const den = prod(poles)
  const md = den === '1' ? `$$${g}${num}$$` : `$$\frac{${g}${num}}{${den}}$$`
  return (
    <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{title}</BaseText>
      <div style={{ marginTop: 8 }}>
        <BaseText>
          <ReactMarkdown remarkPlugins={[remarkMath]} rehypePlugins={[rehypeKatex]}>
            {`$$G(${varName}) = ${md}$$`}
          </ReactMarkdown>
        </BaseText>
      </div>
    </BaseBox>
  )
}
