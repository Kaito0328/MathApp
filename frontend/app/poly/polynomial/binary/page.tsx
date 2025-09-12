"use client"
import React from 'react'
import PageContainer from '../../../../src/baseComponents/layout/PageContainer'
import { polyAdd, polySub, polyMul, polyDiv, polyDivRem, polyGcd, polyLcm } from '../../../../src/wasm/polynomial'
import { useVariableStore } from '../../../../src/state/VariableStore'
import { VariablePicker } from '../../../../src/components/features/variables/VariablePicker'
import BinaryResultSwitch, { type PolyBinaryOp } from '../../../../src/components/domain/polynomial/polynomial/result/binary/BinaryResultSwitch'
import PolynomialInput from '../../../../src/widgets/input/PolynomialInput'
import BinaryLayout from '../../../../src/components/features/layout/BinaryLayout'
import Row from '../../../../src/baseComponents/layout/Row'
import { formatPolynomialMarkdown } from '../../../../src/utils/format/markdown'
import Document from '../../../../src/components/features/document/Document'
import SourceBlock from '../../../../src/components/features/source/SourceBlock'

export default function PolyBinaryPage() {
  const operations: { label: string; value: PolyBinaryOp }[] = [
    { label: '+', value: 'add' },
    { label: '-', value: 'sub' },
    { label: '×', value: 'mul' },
    { label: '÷', value: 'div' },
    { label: '商/余り', value: 'divrem' },
    { label: '最大公約多項式', value: 'gcd' },
    { label: '最小公倍多項式', value: 'lcm' },
  ]
  const [A, setA] = React.useState<{ coeffs: number[] }>({ coeffs: [1,0,-1] })
  const [B, setB] = React.useState<{ coeffs: number[] }>({ coeffs: [1,1] })
  const [out, setOut] = React.useState<number[] | null>(null)
  const [quot, setQuot] = React.useState<number[] | null>(null)
  const [rem, setRem] = React.useState<number[] | null>(null)
  const [err, setErr] = React.useState<string | null>(null)
  const { get } = useVariableStore()
  const [op, setOp] = React.useState<PolyBinaryOp | 'noop'>('noop')


  const run = async (kind: PolyBinaryOp) => {
    setErr(null); setOut(null); setQuot(null); setRem(null)
    try {
      if (kind==='add') setOut((await polyAdd(A.coeffs,B.coeffs)).coeffs)
      else if (kind==='sub') setOut((await polySub(A.coeffs,B.coeffs)).coeffs)
      else if (kind==='mul') setOut((await polyMul(A.coeffs,B.coeffs)).coeffs)
      else if (kind==='div') setOut((await polyDiv(A.coeffs,B.coeffs)).coeffs)
      else if (kind==='gcd') setOut((await polyGcd(A.coeffs,B.coeffs)).coeffs)
      else if (kind==='lcm') setOut((await polyLcm(A.coeffs,B.coeffs)).coeffs)
      else if (kind==='divrem') { const r = await polyDivRem(A.coeffs,B.coeffs); setQuot(r.q.coeffs); setRem(r.r.coeffs) }
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  return (
    <PageContainer title="多項式の二項演算" stickyHeader>
      <BinaryLayout
        // Operation block
        operations={operations}
        operation={op}
        onOperationChange={(v) => { setOp(v as any); if (v !== 'noop') run(v as PolyBinaryOp) }}
        onAccuracyChange={()=>{}}
        operation_left={<Row left={<span>左:</span>} right={<VariablePicker placeholder="変数から代入" allowedKinds={['polynomial']} onPick={(n)=>{ const v:any=get(n); if(v?.kind==='polynomial') setA({ coeffs: v.coeffs.slice() }) }} />} />}
        operation_right={<Row left={<span>右:</span>} right={<VariablePicker placeholder="変数から代入" allowedKinds={['polynomial']} onPick={(n)=>{ const v:any=get(n); if(v?.kind==='polynomial') setB({ coeffs: v.coeffs.slice() }) }} />} />}

        // Operands
        operand_left={<PolynomialInput value={A} onChange={setA} />}
        operand_left_buildSavePayload={()=> ({ kind:'polynomial', coeffs: A.coeffs.slice() })}
        operand_left_afterSave={()=>{}}
        operand_left_copyContent={formatPolynomialMarkdown(A.coeffs)}
        operand_right={<PolynomialInput value={B} onChange={setB} />}
        operand_right_buildSavePayload={()=> ({ kind:'polynomial', coeffs: B.coeffs.slice() })}
        operand_right_afterSave={()=>{}}
        operand_right_copyContent={formatPolynomialMarkdown(B.coeffs)}

        // Result
        result={
          <BinaryResultSwitch
            data={op==='noop' ? null : (
              op==='divrem'
                ? { op:'divrem', q: quot ?? undefined, r: rem ?? undefined, error: err ?? undefined }
                : { op, value: out ?? undefined, error: err ?? undefined }
            )}
            buildSavePayload={(k)=> {
              if (k==='result') return out ? { kind:'polynomial', coeffs: out.slice() } : null
              if (k==='quot') return quot ? { kind:'polynomial', coeffs: quot.slice() } : null
              if (k==='rem') return rem ? { kind:'polynomial', coeffs: rem.slice() } : null
              return null
            }}
          />
        }

        // Verification block
        verification={null}
        document={
          <Document
            docPath={
              op==='gcd' || op==='lcm' ? 'notes/polynomial/polynomial_gcd_lcm.md' : 'notes/polynomial/polynomial_core.md'
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
