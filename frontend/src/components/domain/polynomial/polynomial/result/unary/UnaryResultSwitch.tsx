"use client"
import React from 'react'
import SinglePolynomialResult from '../SinglePolynomialResult'
import Stack from '../../../../../../baseComponents/layout/Stack'
import Row from '../../../../../../baseComponents/layout/Row'
import RootsListResult from './RootsListResult'

export type PolyUnaryOp = 'diff'|'int'|'deg'|'roots'

export type PolyUnaryResultDU =
  | { op: 'diff'|'int'; value?: number[] | null; error?: string | null }
  | { op: 'deg'; info?: string | null; error?: string | null }
  | { op: 'roots'; rootsList?: string | null; factorLines?: string[] | null; verifyText?: string | null; error?: string | null }

export interface UnaryResultSwitchProps {
  data: PolyUnaryResultDU | null
  coeffs?: number[]
  precomputedRoots?: number[]
  precision?: number
  buildSavePayload: (kind: 'result') => any
}

const UnaryResultSwitch: React.FC<UnaryResultSwitchProps> = ({ data, coeffs, precomputedRoots, precision, buildSavePayload }) => {
  if (!data) return null
  return (
    <Stack gap={12}>
      {'error' in data && data.error && <Row center={<span style={{ color:'crimson' }}>{data.error}</span>} />}
      {data.op==='deg' && data.info && <Row center={<span>{data.info}</span>} />}
      {(data.op==='diff' || data.op==='int') && (
        <SinglePolynomialResult coeffs={data.value} buildSavePayload={()=> buildSavePayload('result')} />
      )}
      {data.op==='roots' && (coeffs && coeffs.length>0) && (
        <RootsListResult coeffs={coeffs} precomputedRoots={precomputedRoots} precision={precision} />
      )}
    </Stack>
  )
}

export default UnaryResultSwitch
