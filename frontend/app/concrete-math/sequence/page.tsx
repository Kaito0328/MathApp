"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import UnaryLayout from '../../../src/components/features/layout/UnaryLayout'
import NumberCellInput from '../../../src/baseComponents/input/NumberCellInput'
import GeneralTermListInput from '../../../src/components/features/concrete-math/GeneralTermListInput'
import MarkdownMath from '../../../src/widgets/display/MarkdownMath'
import LabeledMathResult from '../../../src/components/features/result/LabeledMathResult'
import Document from '../../../src/components/features/document/Document'
import { formatNumberForMath } from '../../../src/utils/format/markdown'
import { buildClosedFormTeX } from '../../../src/utils/format/closedForm'

type Term = { poly: { coeffs: number[] }, base: number }

export default function SequencePage() {
  // 状態
  const [k, setK] = React.useState<number>(2) // 同次項数
  const [m, setM] = React.useState<number>(1) // 非同次項数
  const [coeffs, setCoeffs] = React.useState<number[]>([1, -1]) // c1..ck
  const [init, setInit] = React.useState<number[]>([1, 1]) // a0..a_{k-1}
  const [terms, setTerms] = React.useState<Term[]>([{ poly: { coeffs: [1] }, base: 2 }])
  const [outStr, setOutStr] = React.useState<string>('')
  const [err, setErr] = React.useState<string>('')
  const [precision, setPrecision] = React.useState<number>(4)
  const [verifyN, setVerifyN] = React.useState<number>(10)
  const [verifyClosed, setVerifyClosed] = React.useState<number | null>(null)
  const [verifyRecurrence, setVerifyRecurrence] = React.useState<number | null>(null)

  // 配列長の整合
  const ensureLen = (arr: number[], n: number) => {
    const a = arr.slice(0, n); while (a.length < n) a.push(0); return a
  }
  React.useEffect(() => { setCoeffs((c)=> ensureLen(c, k)) }, [k])
  React.useEffect(() => { setInit((a)=> ensureLen(a, k)) }, [k])
  React.useEffect(() => {
    setTerms((ts) => {
      const next = ts.slice(0, m)
      while (next.length < m) next.push({ poly: { coeffs: [1] }, base: 2 })
      return next
    })
  }, [m])

  // terms are managed via GeneralTermListInput

  // 計算
  // 数値の丸めは buildClosedFormTeX 内部で処理

  // ClosedForm → TeX は共通ユーティリティを利用

  const run = async () => {
    setErr(''); setOutStr('')
    try {
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      // wasm.solveRecurrence(coeffs, nh_polys_flat, nh_offsets, nh_bases, initial_values)
      // nh_polys_flat: 各項の多項式係数（複素）をフラットに連結 [re0, im0, re1, im1, ... | 次項 ...]
      // nh_offsets: 各項の開始オフセット（複素係数フラット配列の index）。長さ=項数+1（終端含む）
      // nh_bases: 各項の基数（re, im の交互配列）
      const polysFlat: number[] = []
      const offsets: number[] = [0]
      const bases: number[] = []
      for (const t of terms) {
        for (const c of t.poly.coeffs) { polysFlat.push(Number(c||0), 0) }
        offsets.push(polysFlat.length)
        bases.push(Number(t.base||0), 0)
      }
      const cf = wasm.solveRecurrence(
        Float64Array.from(coeffs),
        Float64Array.from(polysFlat),
        Uint32Array.from(offsets),
        Float64Array.from(bases),
        Float64Array.from(init)
      )
  const tex = buildClosedFormTeX(cf, precision)
  setOutStr(tex)
      // 検証: n=verifyN の値を ClosedForm と再帰で両方計算
      try {
        const n = Math.max(0, Math.floor(verifyN))
        // ClosedForm 側
        const arr = cf.term?.(n)
        if (arr && (arr as any).length) setVerifyClosed(Number((arr as any)[0]))
        else setVerifyClosed(null)
        // Recurrence 前進
        const seq = init.slice()
        for (let i = seq.length; i <= n; i++) {
          let next = 0
          // 同次部分
          for (let j = 1; j <= k; j++) {
            const c = Number(coeffs[j-1] ?? 0)
            const ai = i - j >= 0 ? Number(seq[i - j] ?? 0) : 0
            next += c * ai
          }
          // 非同次項 Σ Q(n)·r^n
          for (const t of terms) {
            const poly = t.poly.coeffs
            // Horner で Q(i) を評価
            let q = 0
            for (let p = poly.length - 1; p >= 0; p--) q = q * i + Number(poly[p] ?? 0)
            next += q * Math.pow(Number(t.base ?? 0), i)
          }
          seq[i] = next
        }
        setVerifyRecurrence(Number(seq[n] ?? 0))
      } catch {
        setVerifyClosed(null); setVerifyRecurrence(null)
      }
  try { cf.free?.() } catch { /* noop */ }
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  // 入力（オペランド）UI
  const operand = (
    <div style={{ display:'grid', gap:12 }}>
      <div style={{ display:'flex', gap:16, alignItems:'center', flexWrap:'wrap' }}>
        <div style={{ display:'flex', gap:6, alignItems:'center' }}>
          <span>同次項数</span>
          <NumberCellInput value={k} onChange={(v)=> setK(Math.max(0, Math.floor(v)))} width={80} />
        </div>
        <div style={{ display:'flex', gap:6, alignItems:'center' }}>
          <span>非同次項数</span>
          <NumberCellInput value={m} onChange={(v)=> setM(Math.max(0, Math.floor(v)))} width={80} />
        </div>
      </div>

      {/* a_n = Σ c_j a_{n-j} の可視化 */}
      <div style={{ display:'flex', alignItems:'center', gap:8, flexWrap:'wrap' }}>
        <MarkdownMath math={'a_n'} block={false} />
        <MarkdownMath math={'='} block={false} />
        {Array.from({ length: k }).map((_, j) => (
          <div key={j} style={{ display:'inline-flex', alignItems:'center', gap:6 }}>
            {j>0 && <MarkdownMath math={'+'} block={false} />}
            <NumberCellInput value={coeffs[j] ?? 0} onChange={(v)=> { const c = coeffs.slice(); c[j] = v; setCoeffs(c) }} width={80} />
            <MarkdownMath math={`a_{n-${j+1}}`} block={false} />
          </div>
        ))}
      </div>

      {/* 非同次項: + Q_i(n) · r_i^n */}
      <GeneralTermListInput terms={terms as any} onChange={setTerms as any} varName="n" />

      {/* 初期値: a_0 =, a_1 =, ... */}
      <div style={{ display:'grid', gap:8 }}>
        {Array.from({ length: k }).map((_, i) => (
          <div key={i} style={{ display:'inline-flex', alignItems:'center', gap:8 }}>
            <MarkdownMath math={`a_${i} =`} block={false} />
            <NumberCellInput value={init[i] ?? 0} onChange={(v)=> { const a = init.slice(); a[i] = v; setInit(a) }} width={96} />
          </div>
        ))}
      </div>
    </div>
  )

  // 結果
  const result = (
    <div style={{ display:'grid', gap:8 }}>
      {err && <div style={{ color:'crimson' }}>{err}</div>}
      {outStr && <LabeledMathResult label={'a_n ='} body={outStr} />}
    </div>
  )

  const verification = (
    <div style={{ display:'grid', gap:10 }}>
      <div style={{ display:'flex', alignItems:'center', gap:8, flexWrap:'wrap' }}>
        <span>n =</span>
        <NumberCellInput value={verifyN} onChange={(v)=> setVerifyN(Math.max(0, Math.floor(v)))} width={96} />
        <div style={{ marginLeft:'auto' }} />
      </div>
      <div>
        <div style={{ opacity:0.8, fontSize:12 }}>ClosedForm での評価</div>
        <MarkdownMath math={verifyClosed==null? '-' : String(formatNumberForMath(verifyClosed, precision))} block={false} />
      </div>
      <div>
        <div style={{ opacity:0.8, fontSize:12 }}>漸化式を前進させた評価</div>
        <MarkdownMath math={verifyRecurrence==null? '-' : String(formatNumberForMath(verifyRecurrence, precision))} block={false} />
      </div>
    </div>
  )

  // 単項レイアウト
  return (
    <PageContainer title="Concrete Math / 数列（漸化式）" stickyHeader>
      <UnaryLayout
        operations={[{ label: '解く', value: 'solve' }]}
        operation={'solve'}
        onOperationChange={() => {}}
  accuracy={precision}
  onAccuracyChange={(v)=> setPrecision(Math.max(1, Math.floor(Number(v) || 1)))}
        onCalc={run}
        calc_button_able
        accuracy_able
        operand={operand}
        operand_copyContent={undefined}
        operand_buildSavePayload={()=> null}
        operand_afterSave={()=>{}}
        result={result}
        verification={verification}
        document={<Document docPath="notes/concrete-math/recurrence_closed_form.md" />}
        documentTitle="ドキュメント"
      />
    </PageContainer>
  )
}
