"use client"
import type { Vector } from '../types'
import { KV, Section, Sparkline } from './common'

export function VectorView({ value }: { value: Vector }) {
  return (
    <Section title="Vector">
      <KV label="length">{value.data.length}</KV>
      <Sparkline data={value.data.slice(0, 512)} />
    </Section>
  )
}
