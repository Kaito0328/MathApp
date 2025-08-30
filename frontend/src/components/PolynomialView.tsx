"use client"
// no react import needed
import type { PolynomialR } from '../types'
import { KV, Section } from './common'

export function PolynomialView({ value, varName = 'x' }: { value: PolynomialR; varName?: string }) {
  const deg = value.coeffs.length - 1
  const terms = value.coeffs
    .map((c, i) => ({ c, i }))
    .filter(t => !(deg === 0 && t.i === 0 && t.c === 0))
    .map(({ c, i }) => {
      if (i === 0) return c.toString()
      const coeff = c === 1 ? '' : c === -1 ? '-' : `${c}`
      const pow = i === 1 ? varName : `${varName}^${i}`
      return `${coeff}${pow}`
    })
    .filter(s => s !== '0')
    .reverse()
    .join(' + ')
    .replace(/\+ -/g, '- ')

  return (
    <Section title="Polynomial">
      <KV label="degree">{deg}</KV>
      <KV label="coeffs">[{value.coeffs.join(', ')}]</KV>
      <div style={{ fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace' }}>{terms || '0'}</div>
    </Section>
  )
}
