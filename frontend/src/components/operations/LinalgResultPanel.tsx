"use client"
import React from 'react'
import ActionPanel from './ActionPanel'
import { Text } from '../../baseComponents/foundation/Text'
import { variableToMarkdown } from '../variables/parts/VariableUtils'
import MarkdownMath from '../../widgets/display/MarkdownMath'
import { MatrixView, VectorView } from '../../widgets/display'
import WithActions from './WithActions'

export type MatrixDTO = { rows:number; cols:number; data:number[] }
export type VectorDTO = { data:number[] }

export interface LinalgResultPanelProps {
  compute: any
  leftKind: 'matrix'|'vector'|'scalar'
  rightKind: 'matrix'|'vector'|'scalar'
  op: 'add'|'sub'|'mul'
  s1: number
  s2: number
}

export const LinalgResultPanel: React.FC<LinalgResultPanelProps> = ({ compute, leftKind, rightKind, op, s1, s2 }) => {
  const isError = 'error' in (compute || {})
  const buildPayload = () => {
    if (isError) return null
    const r:any = compute
    if (r?.rows != null && r?.cols != null && Array.isArray(r.data)) return { kind:'matrix', rows:r.rows, cols:r.cols, data:r.data }
    if (Array.isArray(r?.data)) return { kind:'vector', data:r.data }
    return null
  }
  const copyMarkdown = () => {
    const p = buildPayload(); return p ? variableToMarkdown(p as any) : ''
  }
  const opSymbol = op==='mul' ? 'times' : (op==='add' ? '+' : '-')
  return (
    <ActionPanel title="結果">
      <div>
        {isError ? (
          <Text>{compute.error}</Text>
        ) : compute?.rows != null ? (
          <div style={{ display:'flex', gap:6, alignItems:'center', flexWrap:'wrap' }}>
            <MarkdownMath math={`${leftKind==='matrix' ? 'A' : leftKind==='vector' ? 'x' : String(s1)} \\${opSymbol} ${rightKind==='matrix' ? 'B' : rightKind==='vector' ? 'y' : String(s2)} =`} block={false} />
            <WithActions buildSavePayload={buildPayload} copyContent={copyMarkdown()}>
              <MatrixView rows={(compute as MatrixDTO).rows} cols={(compute as MatrixDTO).cols} data={(compute as MatrixDTO).data} />
            </WithActions>
          </div>
        ) : compute?.data ? (
          <div style={{ display:'flex', gap:6, alignItems:'center', flexWrap:'wrap' }}>
            <MarkdownMath math={`${leftKind==='matrix' ? 'A' : leftKind==='vector' ? 'x' : String(s1)} \\${opSymbol} ${rightKind==='matrix' ? 'B' : rightKind==='vector' ? 'y' : String(s2)} =`} block={false} />
            <WithActions buildSavePayload={buildPayload} copyContent={copyMarkdown()}>
              <VectorView orientation="col" values={(compute as VectorDTO).data} />
            </WithActions>
          </div>
        ) : (
          <Text>スカラー（保存未対応）: {compute?.toString?.() ?? JSON.stringify(compute)}</Text>
        )}
      </div>
    </ActionPanel>
  )
}

export default LinalgResultPanel
