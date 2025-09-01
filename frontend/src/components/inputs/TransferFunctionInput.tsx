"use client"
import React, { useMemo, useState } from 'react'
import type { TransferFunction } from '../../types'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'

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

  const num = useMemo(() => parseNumbers(numTxt), [numTxt])
  const den = useMemo(() => parseNumbers(denTxt), [denTxt])
  const Ts = useMemo(() => (TsTxt.trim() === '' ? null : Number(TsTxt)), [TsTxt])

  const valid = den.length > 0 && den.some(c => c !== 0)

  const handleApply = () => {
    onChange({ num, den, sample_time: (Ts == null || Number.isNaN(Ts)) ? null : Ts })
  }

  return (
    <BaseBox styleKit={{ size: { sizeKey: 'md' as any, apply: { default: ['padding'] as any } }, roundKey: 'md' as any }} className="border-base" style={{ borderWidth: 1 }}>
      <BaseText styleKit={{ size: { sizeKey: 'md' as any, apply: { default: ['fontSize'] as any } }, fontWeightKey: 'medium' as any }}>Transfer Function Input</BaseText>
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
        <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
          <button onClick={handleApply} disabled={!valid}>Apply</button>
          {!valid && <BaseText>denominator must be non-zero</BaseText>}
        </div>
      </div>
    </BaseBox>
  )
}
