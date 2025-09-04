"use client"
import React, { useEffect, useState } from 'react'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { NumberField } from './shared'
import { ComplexVectorInput } from './ComplexVectorInput'
import { initWasm } from '../../../app/lib/wasm'
import { CoreColorKey, SizeKey, RoundKey, FontWeightKey, SizeTextProperty, SizeViewProperty, ColorViewProperty, ColorTextProperty } from '../../design/tokens'


export function SpectrumInput({ value, onChange, label = 'Spectrum Input' }: {
  value?: { spectrum: number[]; sample_rate: number }
  onChange: (v: { spectrum: number[]; sample_rate: number }) => void
  label?: string
}) {
  const [sr, setSr] = useState<number | ''>(value?.sample_rate ?? 64)
  const [spectrum, setSpectrum] = useState<number[]>([])

  // Optional: when spectrum changes, compute an iDFT preview length for validation
  const [timePreview, setTimePreview] = useState<number[]>([])
  useEffect(() => {
    (async () => {
      if (spectrum.length > 0) {
        const wasm: any = await initWasm()
        const rec: Float64Array = wasm.iftComplexF64(new Float64Array(spectrum))
        setTimePreview(Array.from(rec))
      } else {
        setTimePreview([])
      }
    })().catch(() => void 0)
  }, [spectrum])
  const apply = () => onChange({ spectrum, sample_rate: Math.max(1, Number(sr) || 1) })

  return (
  <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
        <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto', display: 'flex', gap: 8, alignItems: 'center' }}>
          <NumberField label="sample_rate" value={sr} onChange={setSr} />
          <button onClick={apply}>Apply</button>
        </div>
      </div>
  <ComplexVectorInput value={spectrum} onChange={setSpectrum} label={"Spectrum (Re, Im)"} showIndex={false} />
      <div style={{ marginTop: 8 }}>
  <BaseText styleKit={{ size: { sizeKey: SizeKey.SM, apply: { default: [SizeTextProperty.FontSize] } }, color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>
          preview (iDFT) length: {timePreview.length}
        </BaseText>
      </div>
    </BaseBox>
  )
}
