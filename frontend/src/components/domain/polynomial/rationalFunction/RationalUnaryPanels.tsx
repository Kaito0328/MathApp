"use client"
import React from 'react'
import Panel from '../../../../baseComponents/layout/Panel'
import { Text } from '../../../../baseComponents/foundation/Text'
import RationalFunctionInput from '../../../../widgets/input/RationalFunctionInput'
import SaveCopyButton from '../../../features/saveCopy/SaveCopyButton'
import MarkdownMath from '../../../../widgets/display/MarkdownMath'
import { formatRationalFunctionMarkdown } from '../../../../utils/format/markdown'
// (duplicate Text import removed)

export interface RationalUnaryInputPanelProps {
  value: { numerator:{ coeffs:number[] }; denominator:{ coeffs:number[] } }
  onChange: (v:{ numerator:{ coeffs:number[] }; denominator:{ coeffs:number[] } }) => void
  buildSavePayload: () => any
}

export const RationalUnaryInputPanel: React.FC<RationalUnaryInputPanelProps> = ({ value, onChange, buildSavePayload }) => {
  const formatted = formatRationalFunctionMarkdown(value as any)
  return (
    <Panel
      header={( 
        <div style={{ display:'flex', alignItems:'center', gap:8 }}>
          <Text weight={600 as any}>入力</Text>
          <div style={{ marginLeft:'auto' }}>
            <SaveCopyButton buildPayload={buildSavePayload} copyContent={formatted} />
          </div>
        </div>
      )}
    >
      <RationalFunctionInput value={value} onChange={onChange} />
    </Panel>
  )
}

export interface RationalUnaryResultPanelProps {
  result?: { numerator:{ coeffs:number[] }; denominator:{ coeffs:number[] } } | null
  error?: string | null
  zerosList?: string | null
  zerosFactor?: string | null
  polesList?: string | null
  polesFactor?: string | null
  pfeLines?: string[] | null
  buildSavePayload: () => any
}

export const RationalUnaryResultPanel: React.FC<RationalUnaryResultPanelProps> = ({ result, error, zerosList, zerosFactor, polesList, polesFactor, pfeLines, buildSavePayload }) => {
  const formatted = result ? formatRationalFunctionMarkdown(result as any) : ''
  return (
    <Panel header={<Text weight={600 as any}>結果</Text>}>
      {error && <div style={{ color:'crimson' }}>{error}</div>}
      {result && (
        <div style={{ display:'grid', gap:6 }}>
          <div style={{ marginLeft:'auto' }}>
            <SaveCopyButton buildPayload={buildSavePayload} copyContent={formatted} />
          </div>
          <MarkdownMath math={formatted} />
        </div>
      )}
      {zerosList && (
        <div style={{ display:'grid', gap:6, marginTop:16 }}>
          <Text weight={600 as any}>ゼロ</Text>
          <MarkdownMath math={zerosList} />
          {zerosFactor && (
            <div style={{ display:'grid', gap:6 }}>
              <Text weight={600 as any}>因数分解 (分子)</Text>
              <MarkdownMath math={zerosFactor} />
            </div>
          )}
        </div>
      )}
      {polesList && (
        <div style={{ display:'grid', gap:6, marginTop:16 }}>
          <Text weight={600 as any}>極</Text>
          <MarkdownMath math={polesList} />
          {polesFactor && (
            <div style={{ display:'grid', gap:6 }}>
              <Text weight={600 as any}>因数分解 (分母)</Text>
              <MarkdownMath math={polesFactor} />
            </div>
          )}
        </div>
      )}
      {pfeLines && pfeLines.length>0 && (
        <div style={{ display:'grid', gap:6, marginTop:16 }}>
          <Text weight={600 as any}>部分分数分解</Text>
          {pfeLines.map((l,i)=> <MarkdownMath key={i} math={l} />)}
        </div>
      )}
    </Panel>
  )
}

export default RationalUnaryInputPanel
