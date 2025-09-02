"use client"
import type { Matrix } from '../types'
import { KV, Section } from './common'

export function MatrixView({ value, previewRows = 6, previewCols = 6 }: { value: Matrix; previewRows?: number; previewCols?: number }) {
  const { rows, cols, data } = value
  const cell = (r: number, c: number) => data[r * cols + c]
  const pr = Math.min(rows, previewRows)
  const pc = Math.min(cols, previewCols)
  return (
    <Section title="Matrix">
      <KV label="shape">{rows} × {cols}</KV>
      <div style={{ display: 'grid', gridTemplateColumns: `repeat(${pc}, minmax(40px, 1fr))`, gap: 4 }}>
        {Array.from({ length: pr }).map((_, r) => (
          <div key={r} style={{ display: 'contents' }}>
            {Array.from({ length: pc }).map((__, c) => (
              <div key={c} style={{ padding: 6, textAlign: 'right', fontFamily: 'ui-monospace, monospace' }}>
                {cell(r, c)?.toFixed?.(3) ?? cell(r, c)}
              </div>
            ))}
          </div>
        ))}
      </div>
    </Section>
  )
}
