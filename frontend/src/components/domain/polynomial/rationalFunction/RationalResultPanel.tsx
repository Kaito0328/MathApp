"use client"
import React from 'react'
import Panel from '../../../../baseComponents/layout/Panel'
import { Text } from '../../../../baseComponents/foundation/Text'
import MarkdownMath from '../../../../widgets/display/MarkdownMath'
import SaveCopyButton from '../../../features/saveCopy/SaveCopyButton'
import { formatRationalFunctionMarkdown } from '../../../../utils/format/markdown'

export interface RationalResultPanelProps {
  result?: { numerator:{ coeffs:number[] }; denominator:{ coeffs:number[] } } | null
  error?: string | null
  precision: number
  buildSavePayload: () => any
}

export const RationalResultPanel: React.FC<RationalResultPanelProps> = ({ result, error, precision, buildSavePayload }) => {
  const formatted = result ? formatRationalFunctionMarkdown({ numerator:{ coeffs: result.numerator.coeffs.map(x=> Number(x.toFixed(precision))) }, denominator:{ coeffs: result.denominator.coeffs.map(x=> Number(x.toFixed(precision))) } } as any) : ''
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
    </Panel>
  )
}

export default RationalResultPanel
