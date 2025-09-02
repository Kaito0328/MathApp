"use client"
import React, { useEffect, useMemo, useState } from 'react'
import type { TransferFunction } from '../../types'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { discreteTfImpulse, discreteTfStep, discreteTfBodeSvg, discreteTfNyquistSvg } from '../../wasm/ops'
import { CoreColorKey, SizeKey, RoundKey, FontWeightKey, SizeTextProperty, SizeViewProperty, ColorViewProperty, ColorTextProperty } from '../../design/tokens'

function parseNumbers(src: string): number[] {
  return src
    .split(/[,\s]+/)
    .map(s => s.trim())
    .filter(s => s.length > 0)
    .map(Number)
    .filter(n => Number.isFinite(n))
}

export function TransferFunctionInput({ value, onChange }: { value?: TransferFunction; onChange: (tf: TransferFunction) => void }) {
  const [numTxt, setNumTxt] = useState<string>((value?.num ?? []).join(', '))
  const [denTxt, setDenTxt] = useState<string>((value?.den ?? []).join(', '))
  const [TsTxt, setTsTxt] = useState<string>(value?.sample_time == null ? '' : String(value.sample_time))
  const [impulsePreview, setImpulsePreview] = useState<number[]>([])
  const [stepPreview, setStepPreview] = useState<number[]>([])
  const [bodeSvg, setBodeSvg] = useState<string>('')
  const [nyquistSvg, setNyquistSvg] = useState<string>('')
  const [previewMode, setPreviewMode] = useState<'bode' | 'nyquist'>('bode')
  const [showMinusOne, setShowMinusOne] = useState<boolean>(true)
  const [appliedTf, setAppliedTf] = useState<{ num: number[]; den: number[]; sample_time: number } | null>(null)

  const num = useMemo(() => parseNumbers(numTxt), [numTxt])
  const den = useMemo(() => parseNumbers(denTxt), [denTxt])
  const Ts = useMemo(() => (TsTxt.trim() === '' ? null : Number(TsTxt)), [TsTxt])

  const valid = den.length > 0 && den.some(c => c !== 0)

  const handleApply = async () => {
    const tf: TransferFunction = { num, den, sample_time: (Ts == null || Number.isNaN(Ts)) ? null : Ts }
    onChange(tf)
    try {
      if (tf.sample_time != null) {
        const dtf = { num: tf.num, den: tf.den, sample_time: tf.sample_time }
        const [imp, step] = await Promise.all([
          discreteTfImpulse(dtf, 128),
          discreteTfStep(dtf, 128),
        ])
        setImpulsePreview(imp)
        setStepPreview(step)
        setAppliedTf(dtf)
        if (previewMode === 'bode') {
          const bode = await discreteTfBodeSvg(dtf, 360, 160, 256, true, false)
          setBodeSvg(bode)
          setNyquistSvg('')
        } else {
          const nyq = await discreteTfNyquistSvg(dtf, 220, 220, 256, showMinusOne, false)
          setNyquistSvg(nyq)
          setBodeSvg('')
        }
      } else {
        setImpulsePreview([]); setStepPreview([]); setBodeSvg(''); setNyquistSvg(''); setAppliedTf(null)
      }
    } catch {
      // ignore preview errors
    }
  }

  // Recompute preview SVG when toggling mode/options, if a TF has been applied
  useEffect(() => {
    const run = async () => {
      if (!appliedTf) return
      try {
        if (previewMode === 'bode') {
          const bode = await discreteTfBodeSvg(appliedTf, 360, 160, 256, true, false)
          setBodeSvg(bode)
          setNyquistSvg('')
        } else {
          const nyq = await discreteTfNyquistSvg(appliedTf, 220, 220, 256, showMinusOne, false)
          setNyquistSvg(nyq)
          setBodeSvg('')
        }
      } catch {
        // ignore
      }
    }
    run()
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [previewMode, showMinusOne])

  return (
  <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>Transfer Function Input</BaseText>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 8, marginTop: 8 }}>
        <label>
          <BaseText>numerator</BaseText>
          <textarea value={numTxt} onChange={(e) => setNumTxt(e.target.value)} rows={2} placeholder={'e.g. 1, 0.5'} style={{ width: '100%', fontFamily: 'ui-monospace, monospace' }} />
        </label>
        <label>
          <BaseText>denominator</BaseText>
          <textarea value={denTxt} onChange={(e) => setDenTxt(e.target.value)} rows={2} placeholder={'e.g. 1, -1, 0.25'} style={{ width: '100%', fontFamily: 'ui-monospace, monospace' }} />
        </label>
        <label>
          <BaseText>sample_time (empty = continuous)</BaseText>
          <input value={TsTxt} onChange={(e) => setTsTxt(e.target.value)} placeholder={'e.g. 0.01'} />
        </label>
        <div style={{ display: 'flex', gap: 8, alignItems: 'center', flexWrap: 'wrap' }}>
          <button onClick={handleApply} disabled={!valid}>Apply</button>
          {!valid && <BaseText>denominator must be non-zero</BaseText>}
          {valid && tfPreviewBadge(impulsePreview.length, stepPreview.length)}
          {valid && (
            <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginLeft: 8 }}>
              <BaseText styleKit={{ color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>preview:</BaseText>
              <label style={{ display: 'inline-flex', alignItems: 'center', gap: 4 }}>
                <input
                  type="radio"
                  name="tf-preview-mode"
                  checked={previewMode === 'bode'}
                  onChange={() => setPreviewMode('bode')}
                />
                <BaseText>Bode</BaseText>
              </label>
              <label style={{ display: 'inline-flex', alignItems: 'center', gap: 4 }}>
                <input
                  type="radio"
                  name="tf-preview-mode"
                  checked={previewMode === 'nyquist'}
                  onChange={() => setPreviewMode('nyquist')}
                />
                <BaseText>Nyquist</BaseText>
              </label>
              {previewMode === 'nyquist' && (
                <label style={{ display: 'inline-flex', alignItems: 'center', gap: 4 }}>
                  <input
                    type="checkbox"
                    checked={showMinusOne}
                    onChange={(e) => setShowMinusOne(e.target.checked)}
                  />
                  <BaseText>show -1</BaseText>
                </label>
              )}
            </div>
          )}
        </div>
        {bodeSvg && previewMode === 'bode' && (
          <div style={{ marginTop: 8 }} dangerouslySetInnerHTML={{ __html: bodeSvg }} />
        )}
        {nyquistSvg && previewMode === 'nyquist' && (
          <div style={{ marginTop: 8 }} dangerouslySetInnerHTML={{ __html: nyquistSvg }} />
        )}
      </div>
    </BaseBox>
  )
}

function tfPreviewBadge(impLen: number, stepLen: number) {
  return (
  <BaseText styleKit={{ size: { sizeKey: SizeKey.SM, apply: { default: [SizeTextProperty.FontSize] } }, color: { colorKey: CoreColorKey.Secondary, apply: { default: [ColorTextProperty.Text] } } }}>
      impulse: {impLen} / step: {stepLen}
    </BaseText>
  )
}
