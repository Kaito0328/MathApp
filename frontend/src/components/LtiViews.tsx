"use client"
import type { TransferFunction, Zpk } from '../types'
import { KV, Section } from './common'

export function TransferFunctionView({ value }: { value: TransferFunction }) {
  return (
    <Section title="Transfer Function">
      <KV label="num">[{value.num.join(', ')}]</KV>
      <KV label="den">[{value.den.join(', ')}]</KV>
      <KV label="sample_time">{value.sample_time ?? 'continuous'}</KV>
    </Section>
  )
}

export function ZpkView({ value }: { value: Zpk }) {
  return (
    <Section title="ZPK">
      <KV label="gain">{value.gain}</KV>
      <KV label="zeros (interleaved)">[{value.zeros.join(', ')}]</KV>
      <KV label="poles (interleaved)">[{value.poles.join(', ')}]</KV>
      <KV label="sample_time">{value.sample_time ?? 'continuous'}</KV>
    </Section>
  )
}
