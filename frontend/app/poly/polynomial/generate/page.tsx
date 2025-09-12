"use client"
import React from 'react'
import { Button } from '../../../../src/baseComponents/controls/Button'
import MarkdownMath from '../../../../src/widgets/display/MarkdownMath'
import { formatPolynomialMarkdown } from '../../../../src/utils/format/markdown'
import PageContainer from '../../../../src/baseComponents/layout/PageContainer'
import { View } from '../../../../src/baseComponents/foundation/View'
import { CoreColorKey, SizeKey } from '../../../../src/design/tokens'
import { useVariableStore } from '../../../../src/state/VariableStore'
import { Complex } from '../../../../src/widgets/dto/complex'
import { ComplexVectorInput } from '../../../../src/widgets/input'
import SourceBlock from '../../../../src/components/features/source/SourceBlock'

export default function PolyGeneratePage() {
  const [roots, setRoots] = React.useState<Complex[]>([{ re: 1, im: 0 }, { re: -1, im: 0 }])
  const [out, setOut] = React.useState<number[] | null>(null)
  const [err, setErr] = React.useState<string | null>(null)
  const { upsert } = useVariableStore()
  const show = (cs: number[] | null) => cs ? formatPolynomialMarkdown(cs) : ''

  const build = async () => {
    setErr(null); setOut(null)
    try {
      const eps = 1e-12
      const used = new Array(roots.length).fill(false)
      let coeffs: number[] = [1] // start with 1

      const mul = (a: number[], b: number[]) => {
        const res = new Array(a.length + b.length - 1).fill(0)
        for (let i = 0; i < a.length; i++) {
          const ai = a[i]
          for (let j = 0; j < b.length; j++) res[i + j] += ai * b[j]
        }
        return res
      }

      for (let i = 0; i < roots.length; i++) {
        if (used[i]) continue
        const { re, im } = roots[i] || { re: 0, im: 0 }
        if (!isFinite(re) || !isFinite(im)) throw new Error('無効な根が含まれています')
        if (Math.abs(im) <= eps) {
          // linear factor: (x - re) -> [-re, 1]
          coeffs = mul(coeffs, [-re, 1])
          used[i] = true
        } else {
          // try to find its conjugate if explicitly provided (avoid double counting)
          for (let j = i + 1; j < roots.length; j++) {
            if (used[j]) continue
            const rj = roots[j]
            if (Math.abs(rj.re - re) <= 1e-9 && Math.abs(rj.im + im) <= 1e-9) {
              used[j] = true
              break
            }
          }
          // quadratic from conjugate pair: x^2 - 2re x + (re^2 + im^2)
          const c0 = re * re + im * im
          const c1 = -2 * re
          coeffs = mul(coeffs, [c0, c1, 1])
          used[i] = true
        }
      }
      setOut(coeffs)
    } catch (e:any) { setErr(e?.message || String(e)) }
  }

  return (
    <>
    <PageContainer title="多項式の生成" stickyHeader>
      <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ display:'grid', gap:12, borderWidth:1, padding:12 }}>
      <div style={{ display:'grid', gap:6 }}>
        <label>根（複素数ベクトル）</label>
        <ComplexVectorInput value={roots} onChange={setRoots} />
      </div>
      <div>
        <Button onClick={build}>生成</Button>
      </div>
      {err && <div style={{ color:'crimson' }}>{err}</div>}
      {out && (
        <div style={{ display:'grid', gap:6 }}>
          <div style={{ display:'flex', alignItems:'center', gap:8 }}>
            <div>結果</div>
            <div style={{ marginLeft:'auto', display:'flex', gap:6 }}>
              <Button size={SizeKey.SM} onClick={()=>{ const name = window.prompt('保存する変数名')?.trim(); if(!name) return; upsert(name, { kind:'polynomial', coeffs: out.slice() }) }}>保存</Button>
              <Button size={SizeKey.SM} color={CoreColorKey.Base} onClick={()=> navigator.clipboard?.writeText(show(out))}>Markdown コピー</Button>
            </div>
          </div>
          <MarkdownMath math={show(out)} />
        </div>
      )}
      </View>
    </PageContainer>
    <div style={{ marginTop: 12 }}>
      <SourceBlock title="ソースコード（polynomial）" path="crates/polynomial/src/lib.rs" />
    </div>
    </>
  )
}
