"use client"
import React, { useEffect, useMemo, useState } from 'react'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { SizeKey, SizeTextProperty, FontWeightKey, SizeViewProperty, RoundKey } from '../../design/tokens'
import ReactMarkdown from 'react-markdown'
import remarkMath from 'remark-math'
import rehypeKatex from 'rehype-katex'

export function PolynomialTermInput({
  value,
  onChange,
  label = 'Polynomial',
  varName = 'x',
  initialDegree,
}: {
  value?: number[] // coeffs low -> high
  onChange: (coeffs: number[]) => void
  label?: string
  varName?: string
  initialDegree?: number
}) {
  const initDeg = useMemo(() => {
    if (typeof initialDegree === 'number') return Math.max(0, initialDegree | 0)
    const fromValue = Math.max(0, (value?.length ?? 0) - 1)
    return fromValue || 2
  }, [initialDegree, value])

  const [degree, setDegree] = useState<number>(initDeg)
  const [coeffs, setCoeffs] = useState<number[]>(() => {
    const d = initDeg
    const arr = Array.from({ length: d + 1 }, (_, i) => value?.[i] ?? 0)
    return arr
  })

  useEffect(() => {
    // keep external in sync
    onChange(coeffs)
  }, [coeffs, onChange])

  const setCoeff = (k: number, v: number) => {
    setCoeffs((prev) => {
      const next = prev.slice()
      next[k] = v
      return next
    })
  }

  const handleDegreeChange = (n: number) => {
    const d = Math.max(0, Math.floor(n))
    setDegree(d)
    setCoeffs((prev) => {
      const next = Array.from({ length: d + 1 }, (_, i) => prev[i] ?? 0)
      return next
    })
  }

  // Render terms high -> low for readability, but store low -> high
  const terms = [] as React.ReactNode[]
  for (let p = degree; p >= 0; p--) {
    const idx = p
    const powLabel = p === 0 ? '' : p === 1 ? varName : `${varName}^{${p}}`
    terms.push(
      <div key={p} style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
        <input
          type="number"
          value={coeffs[idx] ?? 0}
          onChange={(e) => setCoeff(idx, Number(e.target.value) || 0)}
          style={{ width: 80 }}
        />
        {powLabel &&
            <ReactMarkdown remarkPlugins={[remarkMath]} rehypePlugins={[rehypeKatex]}>
                {`$${powLabel}$`}
            </ReactMarkdown>
        }
      </div>
    )
    if (p > 0) terms.push(<BaseText key={`plus-${p}`}>+</BaseText>)
  }

  return (
    <BaseBox styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 8 }}>
        <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto', display: 'flex', gap: 6, alignItems: 'center' }}>
          <BaseText>degree</BaseText>
          <input type="number" min={0} value={degree} onChange={(e) => handleDegreeChange(Number(e.target.value) || 0)} style={{ width: 80 }} />
        </div>
      </div>
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: 8, alignItems: 'center' }}>
        {terms}
      </div>
    </BaseBox>
  )
}
