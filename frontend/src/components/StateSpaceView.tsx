"use client"
import { KV, Section } from './common'

export interface StateSpaceDTO {
  a: { rows: number; cols: number; data: number[] }
  b: { rows: number; cols: number; data: number[] }
  c: { rows: number; cols: number; data: number[] }
  d: { rows: number; cols: number; data: number[] }
}

function shape(m: { rows: number; cols: number }) {
  return `${m.rows} × ${m.cols}`
}

export function StateSpaceView({ value, title = 'State Space' }: { value: StateSpaceDTO; title?: string }) {
  return (
    <Section title={title}>
      <KV label="A shape">{shape(value.a)}</KV>
      <KV label="B shape">{shape(value.b)}</KV>
      <KV label="C shape">{shape(value.c)}</KV>
      <KV label="D shape">{shape(value.d)}</KV>
    </Section>
  )
}
