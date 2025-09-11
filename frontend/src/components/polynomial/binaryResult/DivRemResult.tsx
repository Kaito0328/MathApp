"use client"
import React from 'react'
import MarkdownMath from '../../../widgets/display/MarkdownMath'
import WithActions from '../../operations/WithActions'
import { formatPolynomialMarkdown } from '../../../utils/format/markdown'
import { PolynomialView } from '../../../widgets/display/PolynomialView'

export const DivRemResult: React.FC<{
  quot?: number[] | null
  rem?: number[] | null
  buildSavePayload: (k:'quot'|'rem') => any
}> = ({ quot, rem, buildSavePayload }) => {
  if (!quot && !rem) return null
  return (
    <div style={{ display:'grid', gap:12 }}>
      {quot && (
        <div>
          <div style={{ display:'flex', alignItems:'center', gap:6, flexWrap:'wrap' }}>
            <MarkdownMath math={`q(x) =`} block={false} />
            <WithActions buildSavePayload={()=> buildSavePayload('quot')} copyContent={`${formatPolynomialMarkdown(quot)}`}>
              <PolynomialView coeffs={quot} />
            </WithActions>
          </div>
        </div>
      )}
      {rem && (
        <div>
          <div style={{ display:'flex', alignItems:'center', gap:6, flexWrap:'wrap' }}>
            <MarkdownMath math={`r(x) =`} block={false} />
            <WithActions buildSavePayload={()=> buildSavePayload('rem')} copyContent={`${formatPolynomialMarkdown(rem)}`}>
              <PolynomialView coeffs={rem} />
            </WithActions>
          </div>
        </div>
      )}
    </div>
  )
}

export default DivRemResult
