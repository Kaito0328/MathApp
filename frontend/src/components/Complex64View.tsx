"use client"
import type { Complex64 } from '../types'
import { KV, Section } from './common'

export function Complex64View({ value, label = 'Complex' }: { value: Complex64; label?: string }) {
  const sign = value.im >= 0 ? '+' : '-'
  const imAbs = Math.abs(value.im)
  return (
    <Section title={label}>
      <KV label="re">{value.re}</KV>
      <KV label="im">{value.im}</KV>
      <div style={{ fontFamily: 'ui-monospace, monospace' }}>{`${value.re} ${sign} ${imAbs}i`}</div>
    </Section>
  )
}
