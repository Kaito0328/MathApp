"use client"
import React, { useMemo, useState } from 'react'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { SizeKey, SizeViewProperty, RoundKey, SizeTextProperty, FontWeightKey } from '../../design/tokens'

export function ComplexVectorInput({
  value,
  onChange,
  label = 'Complex Vector',
  showIndex = false,
}: {
  value?: number[] // interleaved [re0, im0, re1, im1, ...]
  onChange: (interleaved: number[]) => void
  label?: string
  showIndex?: boolean
}) {
  const initialPairs = useMemo(() => {
    const out: Array<{ re: number; im: number }> = []
    const v = value ?? []
    const N = Math.floor(v.length / 2)
    for (let i = 0; i < N; i++) out.push({ re: v[2 * i] || 0, im: v[2 * i + 1] || 0 })
    if (out.length === 0) out.push({ re: 0, im: 0 })
    return out
  }, [value])

  const [pairs, setPairs] = useState<Array<{ re: number; im: number }>>(initialPairs)

  const add = () => setPairs((p) => [...p, { re: 0, im: 0 }])
  const remove = (i: number) => setPairs((p) => p.filter((_, idx) => idx !== i))
  const setRe = (i: number, v: number) => setPairs((p) => p.map((e, idx) => (idx === i ? { ...e, re: v } : e)))
  const setIm = (i: number, v: number) => setPairs((p) => p.map((e, idx) => (idx === i ? { ...e, im: v } : e)))

  const interleaved = useMemo(() => {
    const out: number[] = []
    for (const { re, im } of pairs) { out.push(re || 0, im || 0) }
    return out
  }, [pairs])

  return (
    <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 8 }}>
        <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <button onClick={() => onChange(interleaved)}>Apply</button>
        </div>
      </div>
      <div style={{ display: 'grid', gridTemplateColumns: '1fr', gap: 8 }}>
        {pairs.map((p, i) => (
          <div key={i} style={{ display: 'flex', gap: 6, alignItems: 'center' }}>
            {showIndex && <BaseText>#{i}</BaseText>}
            <label>
              <BaseText>Re</BaseText>
              <input type="number" value={p.re} onChange={(e) => setRe(i, Number(e.target.value) || 0)} style={{ width: 100, marginLeft: 6 }} />
            </label>
            <label>
              <BaseText>Im</BaseText>
              <input type="number" value={p.im} onChange={(e) => setIm(i, Number(e.target.value) || 0)} style={{ width: 100, marginLeft: 6 }} />
            </label>
            <button onClick={() => remove(i)} style={{ marginLeft: 'auto' }}>Remove</button>
          </div>
        ))}
      </div>
      <div style={{ marginTop: 8 }}>
        <button onClick={add}>Add element</button>
      </div>
    </BaseBox>
  )
}
