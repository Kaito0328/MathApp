"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import UnaryLayout from '../../../src/components/features/layout/UnaryLayout'
import NumberCellInput from '../../../src/baseComponents/input/NumberCellInput'
import PolynomialInput from '../../../src/widgets/input/PolynomialInput'
import GeneralTermListInput from '../../../src/components/features/concrete-math/GeneralTermListInput'
import LabeledMathResult from '../../../src/components/features/result/LabeledMathResult'
import Document from '../../../src/components/features/document/Document'
import MarkdownMath from '../../../src/widgets/display/MarkdownMath'
import { formatNumberForMath, formatPolynomialMarkdown, formatGeneralTermsMarkdown } from '../../../src/utils/format/markdown'
import { buildClosedFormTeX } from '../../../src/utils/format/closedForm'

type Term = { poly: { coeffs: number[] }, base: number }
type Op = 'rising' | 'falling' | 'sumGT' | 'diffPoly'

export default function SumPage() {
  const [precision, setPrecision] = React.useState<number>(4)
  const [op, setOp] = React.useState<Op>('rising')
  const [P, setP] = React.useState<{ coeffs: number[] }>({ coeffs: [0, 1] }) // P(n) = n
  const [terms, setTerms] = React.useState<Term[]>([{ poly: { coeffs: [1] }, base: 1 }])
  const [outTeX, setOutTeX] = React.useState<string>('')
  const [err, setErr] = React.useState<string>('')
  const [verifyN, setVerifyN] = React.useState<number>(10)
  const [verifyClosed, setVerifyClosed] = React.useState<number | null>(null)
  const [verifyDirect, setVerifyDirect] = React.useState<number | null>(null)
  const [m, setM] = React.useState<number>(3)

  // terms are managed via GeneralTermListInput

  // noop

  // Reset outputs when operation changes
  React.useEffect(()=>{
    setErr('')
    setOutTeX('')
    setVerifyClosed(null)
    setVerifyDirect(null)
  }, [op])

  const run = async () => {
    setErr(''); setOutTeX(''); setVerifyClosed(null); setVerifyDirect(null)
    try {
  const { cm_fallingFactorialPoly, cm_risingFactorialPoly, cm_discreteDiff } = await import('../../../src/wasm/concreteMath')
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      // 分岐実装
      if (op === 'rising') {
        const coeffs = await cm_risingFactorialPoly(Math.max(0, Math.floor(m)))
        // 変数は n で表示
        setOutTeX(formatPolynomialMarkdown(coeffs, 'n'))
        setVerifyClosed(null); setVerifyDirect(null)
        return
      }
      if (op === 'falling') {
        const coeffs = await cm_fallingFactorialPoly(Math.max(0, Math.floor(m)))
        // 変数は n で表示
        setOutTeX(formatPolynomialMarkdown(coeffs, 'n'))
        setVerifyClosed(null); setVerifyDirect(null)
        return
      }
      if (op === 'diffPoly') {
        // 提供関数を使用
        const coeffs = await cm_discreteDiff(P.coeffs)
        // 変数は n で表示
        setOutTeX(formatPolynomialMarkdown(coeffs, 'n'))
        setVerifyClosed(null); setVerifyDirect(null)
        return
      }
      // 一般項の和: Σ Q_i(n)·r_i^n
      if (op === 'sumGT') {
        if (terms.length === 0) { setOutTeX('0'); return }
        // UI 側とは独立に、計算安全性のため先頭 base を 1 に固定
        const safeTerms = terms.map((t, i)=> i===0 ? { ...t, base: 1 } : t)
        const polysFlat: number[] = []
        const offsets: number[] = [0]
        const bases: number[] = []
        for (const t of safeTerms) {
          for (const c of t.poly.coeffs) { polysFlat.push(Number(c || 0), 0) }
          offsets.push(polysFlat.length)
          bases.push(Number(t.base || 0), 0)
        }
        // 和のゼロ問題回避: k=0 は backend が ClosedForm::zero を返すため、
        // a_n = a_{n-1} + f(n), a0 = f(0) の一次漸化式 (k=1) として S(n) を直接解く。
        // ここで f(n) = Σ Q_i(n) r_i^n。
        // 下限が k=1 のため、初期値は a0 = 0 とする（a_n = a_{n-1} + f(n), a_0=0 => S(n)=∑_{k=1}^n f(k)）。
        const a0 = 0
        const cf = wasm.solveRecurrence(
          Float64Array.from([1]), // a_n = 1*a_{n-1} + f(n)
          Float64Array.from(polysFlat),
          Uint32Array.from(offsets),
          Float64Array.from(bases),
          Float64Array.from([a0])
        )
        const sumCf = cf
        const body = buildClosedFormTeX(sumCf, precision)
        setOutTeX(body)
  // 検証: n=verifyN で ClosedForm と直接和を比較（表示上の変数は k）
        try {
          const n = Math.max(0, Math.floor(verifyN))
          const arr: Float64Array | undefined = sumCf.term?.(n)
          if (arr && (arr as any).length) setVerifyClosed(Number((arr as any)[0]))
          // 直接和
          let direct = 0
          for (let i = 1; i <= n; i++) {
            for (const t of safeTerms) {
              const poly = t.poly.coeffs
              // Horner で Q(i)
              let q = 0
              for (let p = poly.length - 1; p >= 0; p--) q = q * i + Number(poly[p] ?? 0)
              direct += q * Math.pow(Number(t.base ?? 0), i)
            }
          }
          setVerifyDirect(direct)
        } catch {
          setVerifyClosed(null); setVerifyDirect(null)
        }
        try { cf.free?.(); (sumCf as any)?.free?.() } catch { /* noop */ }
        return
      }
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  const operand = (
    <div style={{ display:'grid', gap:12 }}>
      {op === 'rising' && (
        <div style={{ display:'flex', alignItems:'center', gap:8 }}>
          {/* n^{\overline{m}} 表記（m は右の入力） */}
          <MarkdownMath math={`n^{\\overline{m}}`} block={false} />
          <MarkdownMath math={'m ='} block={false} />
          <NumberCellInput value={m} onChange={(v)=> setM(Math.max(0, Math.floor(v)))} width={96} />
        </div>
      )}
      {op === 'falling' && (
        <div style={{ display:'flex', alignItems:'center', gap:8 }}>
          {/* n^{\underline{m}} 表記（m は右の入力） */}
          <MarkdownMath math={`n^{\\underline{m}}`} block={false} />
          <MarkdownMath math={'m ='} block={false} />
          <NumberCellInput value={m} onChange={(v)=> setM(Math.max(0, Math.floor(v)))} width={96} />
        </div>
      )}
      {op === 'diffPoly' && (
        <div style={{ display:'flex', alignItems:'center', gap:8, flexWrap:'wrap' }}>
          <MarkdownMath math={'P(n) ='} block={false} />
          <div style={{ minWidth: 300 }}>
            <PolynomialInput value={P} onChange={setP} varName="n" />
          </div>
        </div>
      )}
      {op === 'sumGT' && (
        <GeneralTermListInput
          terms={terms}
          onChange={setTerms}
          varName="k"
          hideBaseForFirst
          fullFirstTerm
          addButtonLabel={'+ 一般項を追加'}
          allowRemove={false}
        />
      )}
    </div>
  )

  const result = (
    <div style={{ display:'grid', gap:8 }}>
      {err && <div style={{ color:'crimson' }}>{err}</div>}
      {outTeX && (
        (()=>{
          let label = ''
          if (op === 'sumGT') {
            const safeTerms = terms.map((t, i)=> i===0 ? { ...t, base: 1 } : t)
            const parts = formatGeneralTermsMarkdown(safeTerms as any, { varName: 'k', precision, hideBaseOne: true, productSymbol: '\\cdot', parenPoly: 'auto' })
            label = `\\sum_{k=1}^{n} ${parts} =`
          } else if (op === 'diffPoly') {
            label = `\\Delta P(n) =`
          } else if (op === 'rising' || op === 'falling') {
            const mv = Math.max(0, Math.floor(m))
            label = op === 'rising' ? `n^{\\overline{${mv}}} =` : `n^{\\underline{${mv}}} =`
          }
          return <LabeledMathResult label={label} body={outTeX} />
        })()
      )}
    </div>
  )

  const verification = (
    <div style={{ display:'grid', gap:10 }}>
      <div style={{ display:'flex', alignItems:'center', gap:8, flexWrap:'wrap' }}>
        <span>n =</span>
        <NumberCellInput value={verifyN} onChange={(v)=> setVerifyN(Math.max(0, Math.floor(v)))} width={96} />
      </div>
      {op === 'sumGT' && (
        <>
          <div>
            <div style={{ opacity:0.8, fontSize:12 }}>ClosedForm での評価</div>
            <MarkdownMath math={verifyClosed==null? '-' : String(formatNumberForMath(verifyClosed, precision))} block={false} />
          </div>
          <div>
            <div style={{ opacity:0.8, fontSize:12 }}>直接和（i=1..n）での評価</div>
            <MarkdownMath math={verifyDirect==null? '-' : String(formatNumberForMath(verifyDirect, precision))} block={false} />
          </div>
        </>
      )}
    </div>
  )

  return (
    <PageContainer title="Concrete Math / 離散演算" stickyHeader>
      <UnaryLayout
        operations={[
          // 数式はラベルから削除（要望）
          { label: '上昇べき', value: 'rising' },
          { label: '下降べき', value: 'falling' },
          { label: '一般項の和', value: 'sumGT' },
          { label: '多項式の差分', value: 'diffPoly' },
        ]}
        operation={op}
        onOperationChange={(v: any) => setOp(v as Op)}
        accuracy={precision}
        onAccuracyChange={(v)=> setPrecision(Math.max(1, Math.floor(Number(v) || 1)))}
        onCalc={run}
        calc_button_able
        accuracy_able
        operand={operand}
        operand_buildSavePayload={()=> null}
        operand_afterSave={()=>{}}
        result={result}
  verification={op==='sumGT' ? verification : null}
        document={<Document docPath="notes/concrete-math/discrete_sum.md" />}
        documentTitle="ドキュメント"
      />
    </PageContainer>
  )
}
