"use client"
import React from 'react'
import Panel from '../../../../baseComponents/layout/Panel'
import { Text } from '../../../../baseComponents/foundation/Text'
import RationalFunctionInput from '../../../../widgets/input/RationalFunctionInput'
import SaveCopyButton from '../../../features/saveCopy/SaveCopyButton'
import { formatRationalFunctionMarkdown } from '../../../../utils/format/markdown'

export interface RationalOperandPanelProps {
  title: string
  value: { numerator:{ coeffs:number[] }; denominator:{ coeffs:number[] } }
  onChange: (v:{ numerator:{ coeffs:number[] }; denominator:{ coeffs:number[] } }) => void
  buildSavePayload: () => any
  precision: number
}

export const RationalOperandPanel: React.FC<RationalOperandPanelProps> = ({ title, value, onChange, buildSavePayload, precision }) => {
  const formatted = formatRationalFunctionMarkdown({ numerator:{ coeffs: value.numerator.coeffs.map(x=> Number(x.toFixed(precision))) }, denominator:{ coeffs: value.denominator.coeffs.map(x=> Number(x.toFixed(precision))) } } as any)
  return (
    <Panel
      header={(
        <div style={{ display:'flex', alignItems:'center', gap:8 }}>
          <Text weight={600 as any}>{title}</Text>
          <div style={{ marginLeft:'auto' }}>
            <SaveCopyButton
              buildPayload={buildSavePayload}
              copyContent={formatted}
            />
          </div>
        </div>
      )}
    >
      <RationalFunctionInput value={value} onChange={onChange} />
    </Panel>
  )
}

export default RationalOperandPanel
