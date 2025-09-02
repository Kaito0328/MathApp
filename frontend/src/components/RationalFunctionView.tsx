"use client"
// no react import needed
import type { PartialFractionExpansion, RationalFunctionR } from '../types'
import { KV, Section } from './common'
import { BaseBox } from '../design/base/BaseBox'
import { BaseText } from '../design/base/BaseText'
import ReactMarkdown from 'react-markdown'
import remarkMath from 'remark-math'
import rehypeKatex from 'rehype-katex'
import { formatPolynomialMarkdown } from './utils/polynomial'
import { CoreColorKey, SizeKey, RoundKey, SizeViewProperty, ColorViewProperty } from '../design/tokens'

export function RationalFunctionView({ value }: { value: RationalFunctionR }) {
  return (
    <Section title="Rational Function">
      <KV label="numerator coeffs">[{value.numerator.coeffs.join(', ')}]</KV>
      <KV label="denominator coeffs">[{value.denominator.coeffs.join(', ')}]</KV>
      <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderTopWidth: 1, marginTop: 8 }}>
        <BaseText>
          <ReactMarkdown remarkPlugins={[remarkMath]} rehypePlugins={[rehypeKatex]}>
            {`$$R(x) = \\frac{${formatPolynomialMarkdown(value.numerator.coeffs, 'x')}}{${formatPolynomialMarkdown(value.denominator.coeffs, 'x')}}$$`}
          </ReactMarkdown>
        </BaseText>
      </BaseBox>
    </Section>
  )
}

export function PartialFractionView({ value }: { value: PartialFractionExpansion }) {
  return (
    <Section title="Partial Fraction Expansion">
      <KV label="poly part">[{value.polynomial_part.coeffs.join(', ')}]</KV>
      <div>
        {value.pole_terms.map((pt, idx) => (
          <BaseBox key={idx} styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.SM, apply: { default: [] } } }} style={{ padding: '6px 0', borderTopWidth: 1, borderStyle: 'dashed' }}>
            <div>pole: {pt.pole.re.toFixed(4)} + {pt.pole.im.toFixed(4)}i</div>
            <div>
              coefficients: [
              {pt.coefficients.map((c) => `${c.re.toFixed(4)}+${c.im.toFixed(4)}i`).join(', ')}
              ]
            </div>
          </BaseBox>
        ))}
      </div>
    </Section>
  )
}
