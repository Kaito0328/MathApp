"use client"
import React from 'react'
import ActionPanel from '../operations/ActionPanel'
import { AddSubMulDivResult } from './binaryResult/AddSubMulDivResult'
import { DivRemResult } from './binaryResult/DivRemResult'

export type PolyBinaryResultDU =
  | { op: 'add'|'sub'|'mul'|'div'|'gcd'|'lcm'; value?: number[] | null; error?: string | null }
  | { op: 'divrem'; q?: number[] | null; r?: number[] | null; error?: string | null }

export interface PolynomialResultPanelProps {
  data: PolyBinaryResultDU | null
  buildSavePayload: (kind: 'result'|'quot'|'rem') => any
}

export const PolynomialResultPanel: React.FC<PolynomialResultPanelProps> = ({ data, buildSavePayload }) => {
  const headerTitle = (data && 'op' in data && (data.op==='gcd' || data.op==='lcm'))
    ? (data.op==='gcd' ? '最大公約多項式' : '最小公倍多項式')
    : '結果'
  return (
    <ActionPanel title={headerTitle}>
      {data && 'error' in data && data.error && <div style={{ color:'crimson' }}>{data.error}</div>}
      <div style={{ display:'grid', gap:12 }}>
  {data && (data.op === 'add' || data.op === 'sub' || data.op === 'mul' || data.op === 'div' || data.op === 'gcd' || data.op === 'lcm') && (
    <AddSubMulDivResult result={data.value} buildSavePayload={()=> buildSavePayload('result')} />
        )}
        {data && data.op === 'divrem' && (
          <DivRemResult quot={data.q} rem={data.r} buildSavePayload={(k)=> buildSavePayload(k)} />
        )}
      </div>
    </ActionPanel>
  )
}

export default PolynomialResultPanel
