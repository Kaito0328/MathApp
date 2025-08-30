"use client"
import { useMemo } from 'react'
import type { Signal } from '../types'
import { Sparkline, KV, Section } from './common'

export function SignalView({ value }: { value: Signal }) {
  const dur = useMemo(() => (value.data.length && value.sample_rate ? value.data.length / value.sample_rate : 0), [value])
  return (
    <Section title="Signal">
      <KV label="sample_rate">{value.sample_rate} Hz</KV>
      <KV label="length">{value.data.length}</KV>
      <KV label="duration">{dur.toFixed(3)} s</KV>
      <Sparkline data={value.data.slice(0, 512)} />
    </Section>
  )
}
