"use client"
import React from 'react'
import { polyDiff, polyInt, polyNormalize } from '../../../../src/wasm/polynomial'
import PageContainer from '../../../../src/baseComponents/layout/PageContainer'
import Row from '../../../../src/baseComponents/layout/Row'
import { useVariableStore } from '../../../../src/state/VariableStore'
import { VariablePicker } from '../../../../src/components/features/variables/VariablePicker'
import UnaryLayout from '../../../../src/components/features/layout/UnaryLayout'
import { formatPolynomialMarkdown } from '../../../../src/utils/format/markdown'
import PolynomialInput from '../../../../src/widgets/input/PolynomialInput'
import UnaryResultSwitch, { type PolyUnaryResultDU } from '../../../../src/components/domain/polynomial/polynomial/result/unary/UnaryResultSwitch'
import RootsVerification from '../../../../src/components/domain/polynomial/polynomial/verfication/RootsVerification'
import Document from '../../../../src/components/features/document/Document'
import SourceBlock from '../../../../src/components/features/source/SourceBlock'

export default function PolyUnaryPage() {
  type PolyUnaryOp = 'diff'|'int'|'deg'|'roots'
  const operations: { label: string; value: PolyUnaryOp }[] = [
    { label: '微分', value: 'diff' },
    { label: '積分', value: 'int' },
    { label: '次数', value: 'deg' },
    { label: '根', value: 'roots' },
  ]
  const [P, setP] = React.useState<{ coeffs: number[] }>({ coeffs: [1,0,-1] })
  const [out, setOut] = React.useState<number[] | null>(null)
  const [info, setInfo] = React.useState<string | null>(null)
  const [err, setErr] = React.useState<string | null>(null)
  const [rootsInterleaved, setRootsInterleaved] = React.useState<number[] | null>(null)
  const { get } = useVariableStore()
  const [op, setOp] = React.useState<PolyUnaryOp | 'noop'>('noop')
  const [precision, setPrecision] = React.useState<number>(6)

  const reset = () => { setErr(null); setInfo(null); setOut(null); setRootsInterleaved(null) }
  const diff = async () => { reset(); try { setOut((await polyDiff(P.coeffs)).coeffs) } catch(e:any){ setErr(e?.message||String(e)) } }
  const integ = async () => { reset(); try { setOut((await polyInt(P.coeffs)).coeffs) } catch(e:any){ setErr(e?.message||String(e)) } }
  const degree = async () => { reset(); try { const p = await polyNormalize(P.coeffs); setInfo(`deg = ${Math.max(0, p.coeffs.length-1)}`) } catch(e:any){ setErr(e?.message||String(e)) } }
  const findRoots = async () => {
    reset()
    try {
      const { getWasm } = await import('../../../../src/wasm/loader')
      const wasm: any = await getWasm()
      const poly = new wasm.PolynomialF64(Float64Array.from(P.coeffs))
      const inter = poly.findRoots() as Float64Array
      poly.free?.()
      setRootsInterleaved(Array.from(inter))
    } catch (e:any) {
      setErr(e?.message || String(e))
    }
  }

  const runOp = () => {
    if (op==='diff') return diff()
    if (op==='int') return integ()
    if (op==='deg') return degree()
    if (op==='roots') return findRoots()
  }

  let data: PolyUnaryResultDU | null = null
  if (op==='diff' || op==='int') {
    data = { op, value: out ?? undefined, error: err ?? undefined } as any
  } else if (op==='deg') {
    data = { op: 'deg', info: info ?? undefined, error: err ?? undefined }
  } else if (op==='roots') {
    data = { op: 'roots', error: err ?? undefined }
  }

  return (
    <PageContainer title="多項式の単項演算" stickyHeader>
      <UnaryLayout
        // Operation block
        operations={operations}
        operation={op}
        onOperationChange={(v)=> setOp(v as any)}
  accuracy={precision}
  onAccuracyChange={(v)=> setPrecision(v)}
        onCalc={runOp}
  calc_button_able
  accuracy_able
        operation_left={
          <VariablePicker
            placeholder="変数から代入"
            allowedKinds={['polynomial']}
            onPick={(n)=>{ const v:any=get(n); if(v?.kind==='polynomial') setP({ coeffs: v.coeffs.slice() }) }}
          />
        }
  operation_right={<Row />}

  // Operand block (raw content only)
  operand={<PolynomialInput value={P} onChange={setP} />}
  operand_copyContent={`$${formatPolynomialMarkdown(P.coeffs)}$`}
        operand_buildSavePayload={()=> ({ kind:'polynomial', coeffs: P.coeffs.slice() })}
        operand_afterSave={()=>{}}

        // Result: roots list only; Verification: factorization equality
        result={<UnaryResultSwitch data={data} coeffs={P.coeffs} precomputedRoots={rootsInterleaved ?? undefined} precision={precision} buildSavePayload={()=> (data && (data as any).value) ? { kind:'polynomial', coeffs: (data as any).value.slice() } : null} />}
        verification={op==='roots' ? <RootsVerification coeffs={P.coeffs} precomputedRoots={rootsInterleaved ?? undefined} precision={precision} /> : null}
        document={
          <Document
            docPath={
              op==='diff' ? 'notes/polynomial/polynomial_core.md'
              : op==='int' ? 'notes/polynomial/polynomial_core.md'
              : op==='deg' ? 'notes/polynomial/polynomial_core.md'
              : op==='roots' ? 'notes/polynomial/polynomial_special.md'
              : 'notes/polynomial/overview.md'
            }
          />
        }
        documentTitle="ドキュメント"
      />
      <div style={{ marginTop: 12 }}>
        <SourceBlock title="ソースコード（polynomial）" path="crates/polynomial/src/lib.rs" />
      </div>
    </PageContainer>
  )
}
