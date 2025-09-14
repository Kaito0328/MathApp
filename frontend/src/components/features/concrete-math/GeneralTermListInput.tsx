"use client"
import React from 'react'
import MarkdownMath from '../../../widgets/display/MarkdownMath'
import NumberCellInput from '../../../baseComponents/input/NumberCellInput'
import PolynomialInputCompact from '../../../widgets/input/PolynomialInputCompact'
import PolynomialInput from '../../../widgets/input/PolynomialInput'

export type GeneralTerm = { poly: { coeffs: number[] }; base: number }

export type GeneralTermListInputProps = {
  terms: GeneralTerm[]
  onChange: (next: GeneralTerm[]) => void
  varName?: string
  hideBaseForFirst?: boolean
  addButtonLabel?: string
  allowRemove?: boolean
  fullFirstTerm?: boolean
}

const GeneralTermListInput: React.FC<GeneralTermListInputProps> = ({
  terms,
  onChange,
  varName = 'n',
  hideBaseForFirst = false,
  addButtonLabel = '+ 一般項を追加',
  allowRemove = true,
  fullFirstTerm = false,
}) => {
  // 先頭項の底を常に 1 に正規化（UI で非表示のときの一貫性確保）
  const normalizeFirstBase = (arr: GeneralTerm[]): GeneralTerm[] => {
    if (!hideBaseForFirst || arr.length === 0) return arr
    const next = arr.slice()
    if (next[0]?.base !== 1) next[0] = { ...next[0], base: 1 }
    return next
  }
  const emit = (arr: GeneralTerm[]) => onChange(normalizeFirstBase(arr))

  const updatePoly = (i: number, poly: { coeffs: number[] }) => {
    const next = terms.slice(); next[i] = { ...next[i], poly }; emit(next)
  }
  const updateBase = (i: number, base: number) => {
    const next = terms.slice(); next[i] = { ...next[i], base }; emit(next)
  }
  const addTerm = () => {
    const next = terms.slice();
    // 追加時、先頭が空 or hideBaseForFirst の場合でも emit 側で正規化される
    // デフォルトは第1項=1^n 相当（base は正規化で 1 に）、以降は 2^n
    const defaultBase = next.length === 0 ? 1 : 2
    next.push({ poly: { coeffs: [1] }, base: defaultBase }); emit(next)
  }
  const removeAt = (idx: number) => {
    const next = terms.filter((_, i)=> i!==idx); emit(next)
  }
  return (
    <div style={{ display:'grid', gap:10 }}>
      {terms.map((t, i) => {
        const showBase = !(hideBaseForFirst && i===0)
        return (
          <div key={i} style={{ display:'flex', alignItems:'center', gap:8, flexWrap:'wrap' }}>
            {i>0 && <MarkdownMath math={'+'} block={false} />}
            <div style={{ minWidth: 280 }}>
              {fullFirstTerm && i===0 ? (
                <PolynomialInput value={t.poly} onChange={(p)=> updatePoly(i, p)} varName={varName} />
              ) : (
                <PolynomialInputCompact value={t.poly} onChange={(p)=> updatePoly(i, p)} varName={varName} />
              )}
            </div>
            {showBase ? (
              <>
                <NumberCellInput value={t.base} onChange={(v)=> updateBase(i, v)} width={96} />
                <MarkdownMath math={'^{n}'} block={false} />
              </>
            ) : null}
            {allowRemove && terms.length>1 && (
              <button
                onClick={()=> removeAt(i)}
                style={{ marginLeft: 8 }}
                aria-label="この一般項を削除"
                title="この一般項（Q(k) · r^k の一塊）を削除"
              >この一般項を削除</button>
            )}
          </div>
        )
      })}
      <div>
        <button onClick={addTerm}>{addButtonLabel}</button>
      </div>
    </div>
  )
}

export default GeneralTermListInput
