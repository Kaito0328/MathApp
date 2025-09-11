"use client"
import React from 'react'
import ActionPanel from '../operations/ActionPanel'
import PolynomialInput from '../../widgets/input/PolynomialInput'
import { formatPolynomialMarkdown } from '../../utils/format/markdown'

export interface PolynomialOperandPanelProps {
  title: string
  value: { coeffs: number[] }
  onChange: (v:{ coeffs:number[] }) => void
  buildSavePayload: () => any
}

export const PolynomialOperandPanel: React.FC<PolynomialOperandPanelProps> = ({ title, value, onChange, buildSavePayload }) => {
  return (
    <ActionPanel title={title} showSave showCopy buildSavePayload={buildSavePayload} copyContent={formatPolynomialMarkdown(value.coeffs)}>
      <div style={{ display:'grid' }}>
        <PolynomialInput value={value} onChange={onChange} />
      </div>
    </ActionPanel>
  )
}

export default PolynomialOperandPanel
