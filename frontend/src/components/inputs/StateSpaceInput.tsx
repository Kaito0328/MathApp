"use client"
import React, { useMemo, useState } from 'react'
import { BaseBox } from '../../design/base/BaseBox'
import { BaseText } from '../../design/base/BaseText'
import { CoreColorKey, SizeKey, RoundKey, ColorViewProperty, SizeViewProperty, SizeTextProperty, FontWeightKey } from '../../design/tokens'
import { ApplyRow, TextAreaNumbers, parseNumbers } from './shared'

function parseMatrix(text: string): number[][] {
  // rows separated by newlines; numbers separated by comma/space
  const rows = text
    .split(/\n+/)
    .map((r) => parseNumbers(r))
    .filter((r) => r.length > 0)
  if (rows.length === 0) return []
  const cols = rows[0].length
  if (!rows.every((r) => r.length === cols)) throw new Error('Non-rectangular matrix')
  return rows
}

export function StateSpaceInput({
  value,
  onChange,
  label = 'State Space Input',
}: {
  value?: { a: number[][]; b: number[][]; c: number[][]; d: number[][] }
  onChange: (ss: { a: number[][]; b: number[][]; c: number[][]; d: number[][] }) => void
  label?: string
}) {
  const [aTxt, setATxt] = useState(() => (value?.a ? value.a.map(r => r.join(', ')).join('\n') : ''))
  const [bTxt, setBTxt] = useState(() => (value?.b ? value.b.map(r => r.join(', ')).join('\n') : ''))
  const [cTxt, setCTxt] = useState(() => (value?.c ? value.c.map(r => r.join(', ')).join('\n') : ''))
  const [dTxt, setDTxt] = useState(() => (value?.d ? value.d.map(r => r.join(', ')).join('\n') : ''))

  const valid = useMemo(() => {
    try {
      parseMatrix(aTxt); parseMatrix(bTxt); parseMatrix(cTxt); parseMatrix(dTxt); return true
    } catch { return false }
  }, [aTxt, bTxt, cTxt, dTxt])

  const handleApply = () => {
    const a = parseMatrix(aTxt)
    const b = parseMatrix(bTxt)
    const c = parseMatrix(cTxt)
    const d = parseMatrix(dTxt)
    onChange({ a, b, c, d })
  }

  return (
  <BaseBox styleKit={{ color: { colorKey: CoreColorKey.Base, apply: { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } }, size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } }, roundKey: RoundKey.Md }} style={{ borderWidth: 1 }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
        <BaseText styleKit={{ size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } }, fontWeightKey: FontWeightKey.Medium }}>{label}</BaseText>
        <div style={{ marginLeft: 'auto' }}>
          <ApplyRow onApply={handleApply} disabled={!valid}>
            {!valid && <BaseText>Enter rectangular matrices (rows separated by newline)</BaseText>}
          </ApplyRow>
        </div>
      </div>
      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 8 }}>
        <TextAreaNumbers label="A" value={aTxt} onChange={setATxt} rows={3} placeholder={'e.g. 1,0\n0,1'} />
        <TextAreaNumbers label="B" value={bTxt} onChange={setBTxt} rows={3} placeholder={'e.g. 0\n1'} />
        <TextAreaNumbers label="C" value={cTxt} onChange={setCTxt} rows={3} placeholder={'e.g. 1,0'} />
        <TextAreaNumbers label="D" value={dTxt} onChange={setDTxt} rows={3} placeholder={'e.g. 0'} />
      </div>
    </BaseBox>
  )
}
