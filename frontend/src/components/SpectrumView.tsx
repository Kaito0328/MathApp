"use client"
import { useMemo } from 'react'
import type { DftResult } from '../types'
import { Sparkline, KV, Section } from './common'

export function SpectrumView({ value }: { value: DftResult }) {
  const magnitude = useMemo(() => {
    const out: number[] = []
    for (let i = 0; i < value.spectrum.length; i += 2) {
      const re = value.spectrum[i]
      const im = value.spectrum[i + 1] ?? 0
      out.push(Math.hypot(re, im))
    }
    return out
  }, [value])

  return (
    <Section title="Spectrum (DFT)">
      <KV label="sample_rate">{value.sample_rate} Hz</KV>
      <KV label="bins">{magnitude.length}</KV>
      <Sparkline data={magnitude.slice(0, 512)} />
    </Section>
  )
}
