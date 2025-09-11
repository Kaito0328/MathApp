"use client"
import React from 'react'
import ActionPanel from '../operations/ActionPanel'
import { Text } from '../../baseComponents/foundation/Text'
import PolynomialInput from '../../widgets/input/PolynomialInput'
import MarkdownMath from '../../widgets/display/MarkdownMath'
import { formatPolynomialMarkdown } from '../../utils/format/markdown'
import { MonoResult } from './unaryResult/MonoResult'
import { RootsResult } from './unaryResult/RootsResult'

export interface PolynomialUnaryInputPanelProps {
  value: { coeffs:number[] }
  onChange: (v:{ coeffs:number[] }) => void
  buildSavePayload: () => any
}

export const PolynomialUnaryInputPanel: React.FC<PolynomialUnaryInputPanelProps> = ({ value, onChange, buildSavePayload }) => {
  return (
    <ActionPanel title="入力" showSave showCopy buildSavePayload={buildSavePayload} copyContent={formatPolynomialMarkdown(value.coeffs)}>
      <PolynomialInput value={value} onChange={onChange} />
    </ActionPanel>
  )
}

export type PolyUnaryResultDU =
  | { op: 'diff'|'int'; value?: number[] | null; error?: string | null }
  | { op: 'deg'; info?: string | null; error?: string | null }
  | { op: 'roots'; rootsList?: string | null; factorLines?: string[] | null; verifyText?: string | null; error?: string | null }

export interface PolynomialUnaryResultPanelProps {
  data: PolyUnaryResultDU | null
}

export const PolynomialUnaryResultPanel: React.FC<PolynomialUnaryResultPanelProps> = ({ data }) => {
  const polyFormatted = (data && (data.op==='diff' || data.op==='int') && data.value) ? formatPolynomialMarkdown(data.value) : ''
  return (
  <ActionPanel title="結果">
      {data && 'error' in data && data.error && <div style={{ color:'crimson' }}>{data.error}</div>}
      {data && data.op==='deg' && data.info && <div>{data.info}</div>}
      {data && (data.op==='diff' || data.op==='int') && (
        <MonoResult result={data.value||undefined} md={polyFormatted} />
      )}
      {data && data.op==='roots' && (
        <div style={{ marginTop: 16 }}>
          <RootsResult rootsList={data.rootsList||undefined} />
        </div>
      )}
      {data && data.op==='roots' && data.factorLines && data.factorLines.length>0 && (
        <div style={{ display:'grid', gap:6, marginTop:16 }}>
          <Text weight={600 as any}>因数分解</Text>
          {data.factorLines.map((l,i)=>(
            <MarkdownMath key={i} math={l} />
          ))}
        </div>
      )}
      {data && data.op==='roots' && data.verifyText && (
        <div style={{ display:'grid', gap:6, marginTop:16 }}>
          <Text weight={600 as any}>検証</Text>
          <div>{data.verifyText}</div>
        </div>
      )}
  </ActionPanel>
  )
}

export default PolynomialUnaryInputPanel
