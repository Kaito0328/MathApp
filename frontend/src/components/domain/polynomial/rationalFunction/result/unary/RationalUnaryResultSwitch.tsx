"use client"
import React from 'react'
import Stack from '../../../../../../baseComponents/layout/Stack'
import Row from '../../../../../../baseComponents/layout/Row'
import MathWithSaveCopy from '../../../../../features/saveCopy/MathWithSaveCopy'
import MarkdownMath from '../../../../../../widgets/display/MarkdownMath'
import { formatComplexMarkdown, formatRationalFunctionMarkdown, formatNumberForMath } from '../../../../../../utils/format/markdown'

export type RationalUnaryOp = 'diff'|'simplify'|'zeros'|'poles'|'pfe'

export type RationalUnaryResultDU =
  | { op: 'diff'|'simplify'; value?: { numerator:{ coeffs:number[] }; denominator:{ coeffs:number[] } } | null; error?: string | null }
  | { op: 'zeros'; zeros?: number[]; error?: string | null }
  | { op: 'poles'; poles?: number[]; error?: string | null }
  | { op: 'pfe'; lines?: string[]; error?: string | null }

export interface RationalUnaryResultSwitchProps {
  data: RationalUnaryResultDU | null
  precision?: number
}

const RationalUnaryResultSwitch: React.FC<RationalUnaryResultSwitchProps> = ({ data, precision }) => {
  if (!data) return null
  return (
    <Stack gap={12}>
      {'error' in data && data.error && <Row center={<span style={{ color:'crimson' }}>{data.error}</span>} />}
      {(data.op==='diff' || data.op==='simplify') && data.value && (
        <MathWithSaveCopy
          tex={`$${formatRationalFunctionMarkdown({ numerator:{ coeffs: data.value.numerator.coeffs.map(x=> Number(formatNumberForMath(x, precision))) }, denominator:{ coeffs: data.value.denominator.coeffs.map(x=> Number(formatNumberForMath(x, precision))) } } as any)}$`}
          buildSavePayload={()=> ({ kind:'rational', numerator: data.value!.numerator.coeffs.slice(), denominator: data.value!.denominator.coeffs.slice() })}
        />
      )}
      {data.op==='zeros' && (
        <Row center={<MarkdownMath math={data.zeros && data.zeros.length ? formatComplexMarkdown(data.zeros as any, { precision, orientation:'row' }) : '\\varnothing'} />} />
      )}
      {data.op==='poles' && (
        <Row center={<MarkdownMath math={data.poles && data.poles.length ? formatComplexMarkdown(data.poles as any, { precision, orientation:'row' }) : '\\varnothing'} />} />
      )}
      {data.op==='pfe' && data.lines && data.lines.map((l,i)=> <MarkdownMath key={i} math={l} />)}
    </Stack>
  )
}

export default RationalUnaryResultSwitch
