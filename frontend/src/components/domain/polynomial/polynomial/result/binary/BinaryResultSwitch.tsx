"use client"
import React from 'react'
import SinglePolynomialResult from '../SinglePolynomialResult'
import { DivRemResult } from './DivRemResult'
import Stack from '../../../../../../baseComponents/layout/Stack'
import Row from '../../../../../../baseComponents/layout/Row'

export type PolyBinaryOp = 'add'|'sub'|'mul'|'div'|'divrem'|'gcd'|'lcm'

export type PolyBinaryResultDU =
  | { op: Exclude<PolyBinaryOp, 'divrem'>; value?: number[] | null; error?: string | null }
  | { op: 'divrem'; q?: number[] | null; r?: number[] | null; error?: string | null }

export interface BinaryResultSwitchProps {
  data: PolyBinaryResultDU | null
  buildSavePayload: (kind: 'result'|'quot'|'rem') => any
}

export const BinaryResultSwitch: React.FC<BinaryResultSwitchProps> = ({ data, buildSavePayload }) => {
  if (!data) return null
  return (
    <Stack gap={12}>
      {'error' in data && data.error && <Row center={<span style={{ color:'crimson' }}>{data.error}</span>} />}
      {data.op !== 'divrem' && (
        <SinglePolynomialResult coeffs={data.value} buildSavePayload={()=> buildSavePayload('result')} />
      )}
      {data.op === 'divrem' && (
        <DivRemResult quot={data.q} rem={data.r} buildSavePayload={(k)=> buildSavePayload(k)} />
      )}
    </Stack>
  )
}

export default BinaryResultSwitch
