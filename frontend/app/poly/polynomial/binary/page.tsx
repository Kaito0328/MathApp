"use client"
import React from 'react'
import PageContainer from '../../../../src/baseComponents/layout/PageContainer'
import { polyAdd, polySub, polyMul, polyDiv, polyDivRem, polyGcd, polyLcm } from '../../../../src/wasm/polynomial'
import { useVariableStore } from '../../../../src/state/VariableStore'
import { VariablePicker } from '../../../../src/components/variables/VariablePicker'
import Row from '../../../../src/baseComponents/layout/Row'
import Panel from '../../../../src/baseComponents/layout/Panel'
import PolynomialOperandPanel from '../../../../src/components/polynomial/PolynomialOperand'
import PolynomialResultPanel, { PolyBinaryResultDU } from '../../../../src/components/polynomial/PolynomialBinaryResult'
import OperationSetting from '../../../../src/components/operations/OperationSetting'

export default function PolyBinaryPage() {
  type PolyBinaryOp = 'add'|'sub'|'mul'|'div'|'divrem'|'gcd'|'lcm'
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
        <Panel header={null}>
          <Row
            left={<div style={{ display:'flex', gap:8, flexWrap:'wrap', alignItems:'center' }}>
              <span>左:</span>
              <VariablePicker placeholder="変数から代入" allowedKinds={['polynomial']} onPick={(n)=>{ const v:any=get(n); if(v?.kind==='polynomial') setA({ coeffs: v.coeffs.slice() }) }} />
            </div>}
            center={
              <OperationSetting
                operations={operations}
                operation={op}
                onOperationChange={(v) => { setOp(v as any); if (v !== 'noop') run(v as PolyBinaryOp) }}
                onAccuracyChange={()=>{}}
              />
            }
            right={<div style={{ display:'flex', gap:8, flexWrap:'wrap', alignItems:'center', justifyContent:'flex-end' }}>
              <span>右:</span>
              <VariablePicker placeholder="変数から代入" allowedKinds={['polynomial']} onPick={(n)=>{ const v:any=get(n); if(v?.kind==='polynomial') setB({ coeffs: v.coeffs.slice() }) }} />
            </div>}
          />
        </Panel>
        <div style={{ display:'grid', gap:12, gridTemplateColumns:'repeat(auto-fit, minmax(320px, 1fr))' }}>
          <PolynomialOperandPanel
            title="左オペランド"
            value={A}
            onChange={setA}
            buildSavePayload={()=> ({ kind:'polynomial', coeffs: A.coeffs.slice() })}
          />
          <PolynomialOperandPanel
            title="右オペランド"
            value={B}
            onChange={setB}
            buildSavePayload={()=> ({ kind:'polynomial', coeffs: B.coeffs.slice() })}
          />
        </div>
        <PolynomialResultPanel
          data={((): PolyBinaryResultDU | null => {
            if (err) return { op: (op as any), error: err } as any
            if (op === 'divrem') return { op: 'divrem', q: quot || undefined, r: rem || undefined }
            if (op === 'add' || op === 'sub' || op === 'mul' || op === 'div' || op === 'gcd' || op === 'lcm') return { op, value: out || undefined }
            return null
          })()}
          buildSavePayload={(k)=> {
            if(k==='result' && out) return { kind:'polynomial', coeffs: out.slice() }
            if(k==='quot' && quot) return { kind:'polynomial', coeffs: quot.slice() }
            if(k==='rem' && rem) return { kind:'polynomial', coeffs: rem.slice() }
            return null
          }}
        />
    </PageContainer>
  )
}
