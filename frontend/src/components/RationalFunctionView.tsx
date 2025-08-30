"use client"
// no react import needed
import type { PartialFractionExpansion, RationalFunctionR } from '../types'
import { KV, Section } from './common'

export function RationalFunctionView({ value }: { value: RationalFunctionR }) {
  return (
    <Section title="Rational Function">
      <KV label="numerator coeffs">[{value.numerator.coeffs.join(', ')}]</KV>
      <KV label="denominator coeffs">[{value.denominator.coeffs.join(', ')}]</KV>
    </Section>
  )
}

export function PartialFractionView({ value }: { value: PartialFractionExpansion }) {
  return (
    <Section title="Partial Fraction Expansion">
      <KV label="poly part">[{value.polynomial_part.coeffs.join(', ')}]</KV>
      <div>
        {value.pole_terms.map((pt, idx) => (
          <div key={idx} style={{ padding: '6px 0', borderTop: '1px dashed #333' }}>
            <div>pole: {pt.pole.re.toFixed(4)} + {pt.pole.im.toFixed(4)}i</div>
            <div>
              coefficients: [
              {pt.coefficients.map((c) => `${c.re.toFixed(4)}+${c.im.toFixed(4)}i`).join(', ')}
              ]
            </div>
          </div>
        ))}
      </div>
    </Section>
  )
}
