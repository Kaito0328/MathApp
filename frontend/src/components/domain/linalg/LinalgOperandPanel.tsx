"use client"
import React from 'react'
import Panel from '../../../baseComponents/layout/Panel'
import { Text } from '../../../baseComponents/foundation/Text'
import { Button } from '../../../baseComponents/controls/Button'
import { SizeKey, CoreColorKey } from '../../../design/tokens'
import { MatrixInput, VectorInput, MatrixSizeControls, VectorSizeControls } from '../../../widgets/input'
import { variableToMarkdown } from '../../features/variables/parts/VariableUtils'

export type MatrixDTO = { rows: number; cols: number; data: number[] }
export type VectorDTO = { data: number[] }
export type LinalgKind = 'matrix' | 'vector' | 'scalar'

export interface LinalgOperandPanelProps {
  title: string
  kind: LinalgKind
  matrix: MatrixDTO
  vector: VectorDTO
  scalar: number
  onChangeMatrix: (m: MatrixDTO) => void
  onChangeVector: (v: VectorDTO) => void
  onChangeScalar: (n: number) => void
  onSave: () => void
  copyEnabled?: boolean
}

export const LinalgOperandPanel: React.FC<LinalgOperandPanelProps> = ({
  title,
  kind,
  matrix,
  vector,
  scalar,
  onChangeMatrix,
  onChangeVector,
  onChangeScalar,
  onSave,
  copyEnabled = true,
}) => {
  const copyPayload = () => {
    if (kind === 'matrix') return { kind:'matrix', rows: matrix.rows, cols: matrix.cols, data: matrix.data }
    if (kind === 'vector') return { kind:'vector', data: vector.data }
    return null
  }
  const copyMarkdown = () => {
    const payload = copyPayload()
    return payload ? variableToMarkdown(payload as any) : ''
  }
  return (
    <Panel
      header={<Text weight={600 as any}>{title}</Text>}
    >
      <div style={{ marginTop: 8 }}>
        {kind === 'matrix' ? (
          <div style={{ display:'grid', gridTemplateColumns:'1fr auto', gap:8, alignItems:'start' }}>
            <div>
              <div style={{ display:'flex', alignItems:'center', gap:8, marginBottom:8 }}>
                <MatrixSizeControls rows={matrix.rows} cols={matrix.cols} onChange={(r,c)=> {
                  const size = r*c
                  const next = matrix.data.slice(0, size)
                  if (next.length < size) next.push(...Array(size - next.length).fill(0))
                  onChangeMatrix({ rows:r, cols:c, data: next })
                }} />
              </div>
              <MatrixInput value={matrix} onChange={onChangeMatrix} rows={matrix.rows} cols={matrix.cols} />
            </div>
            <div>
              <div style={{ display:'flex', gap:4 }}>
                <Button size={SizeKey.SM} onClick={onSave} color={CoreColorKey.Primary} aria-label="保存" title="保存">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
                </Button>
                {copyEnabled && (
                  <Button size={SizeKey.SM} color={CoreColorKey.Base} aria-label="Markdown コピー" title="Markdown コピー" onClick={() => { const md = copyMarkdown(); if(md) navigator.clipboard?.writeText(md) }}>
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
                  </Button>
                )}
              </div>
            </div>
          </div>
        ) : kind === 'vector' ? (
          <>
            <div style={{ display:'flex', alignItems:'center', gap:8, marginBottom:8 }}>
              <VectorSizeControls length={vector.data.length} onChange={(n)=> onChangeVector({ data: vector.data.slice(0, n).concat(Array(Math.max(0, n - vector.data.length)).fill(0)) })} />
              <div style={{ display:'flex', gap:4 }}>
                <Button size={SizeKey.SM} onClick={onSave} color={CoreColorKey.Primary} aria-label="保存" title="保存">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
                </Button>
                {copyEnabled && (
                  <Button size={SizeKey.SM} color={CoreColorKey.Base} aria-label="Markdown コピー" title="Markdown コピー" onClick={() => { const md = copyMarkdown(); if(md) navigator.clipboard?.writeText(md) }}>
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
                  </Button>
                )}
              </div>
            </div>
            <VectorInput value={vector} onChange={onChangeVector} orientation="col" length={vector.data.length} />
          </>
        ) : (
          <input type="number" value={scalar} onChange={(e)=> onChangeScalar(Number(e.target.value||0))} />
        )}
      </div>
    </Panel>
  )
}

export default LinalgOperandPanel
