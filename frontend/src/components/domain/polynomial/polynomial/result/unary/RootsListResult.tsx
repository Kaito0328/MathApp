"use client"
import React from 'react'
import Row from '../../../../../../baseComponents/layout/Row'
import MarkdownMath from '../../../../../../widgets/display/MarkdownMath'
import { formatComplexMarkdown } from '../../../../../../utils/format/markdown'

export interface RootsListResultProps {
  coeffs: number[]
  // Interleaved roots from parent: [re0, im0, re1, im1, ...]
  precomputedRoots?: number[]
  precision?: number
}

const RootsListResult: React.FC<RootsListResultProps> = ({ coeffs, precomputedRoots, precision }) => {
  const [tex, setTex] = React.useState<string>('')
  const [error, setError] = React.useState<string | null>(null)

  React.useEffect(()=>{
    let cancelled = false
    const run = async ()=>{
      try {
        setError(null)
        // Zero polynomial special-case
        const nz = coeffs.some(v=> Math.abs(v) > 1e-12)
        if (!nz) { setTex('\\text{0 多項式（根は無数）}'); return }
        if (!precomputedRoots || precomputedRoots.length === 0) { setTex('\\text{計算を実行してください}'); return }
        const inter: ArrayLike<number> = precomputedRoots as ArrayLike<number>
        const rootsArr: { re:number; im:number }[] = []
        for (let i=0;i<inter.length;i+=2) {
          const re = inter[i]; const im = inter[i+1]
          if (!Number.isFinite(re) || !Number.isFinite(im)) continue
          rootsArr.push({ re, im })
        }
        const tol = typeof precision === 'number' ? 0.5 * Math.pow(10, -precision) : 1e-9
        // Build interleaved list with tiny parts snapped to 0 for clean formatting
        const display: number[] = []
        for (const r of rootsArr) {
          const re = Math.abs(r.re) < tol ? 0 : r.re
          const im = Math.abs(r.im) < tol ? 0 : r.im
          display.push(re, im)
        }
        if (cancelled) return
        const body = display.length>0 ? formatComplexMarkdown(display, { precision, orientation: 'row' }) : '\\varnothing'
        setTex(body)
      } catch (e:any) {
        if (!cancelled) setError(e?.message || String(e))
      }
    }
    run()
    return ()=>{ cancelled = true }
  }, [coeffs, precomputedRoots, precision])

  if (error) return <Row center={<span style={{ color:'crimson' }}>{error}</span>} />
  return tex ? <Row center={<MarkdownMath math={tex} />} /> : null
}

export default RootsListResult
