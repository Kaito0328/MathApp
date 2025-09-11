"use client"
import React from 'react'
import PageContainer from '../../../../src/baseComponents/layout/PageContainer'
import { VariablePicker } from '../../../../src/components/variables/VariablePicker'
import { useVariableStore } from '../../../../src/state/VariableStore'
import Stack from '../../../../src/baseComponents/layout/Stack'
import Row from '../../../../src/baseComponents/layout/Row'
import Panel from '../../../../src/baseComponents/layout/Panel'
import RationalOperandPanel from '../../../../src/components/rationalFunction/RationalOperandPanel'
import RationalResultPanel from '../../../../src/components/rationalFunction/RationalResultPanel'
import { getWasm } from '../../../../src/wasm/loader'
import OperationSetting from '../../../../src/components/operations/OperationSetting'

export default function RationalBinaryPage() {
  const [F, setF] = React.useState({ numerator: { coeffs: [1,0] }, denominator: { coeffs: [1] } })
  const [G, setG] = React.useState({ numerator: { coeffs: [1] }, denominator: { coeffs: [1,1] } })
  const [op, setOp] = React.useState<'add'|'sub'|'mul'|'div'>('add')
  const operations: { label: string; value: 'add'|'sub'|'mul'|'div' }[] = [
    { label: '+', value: 'add' },
    { label: '-', value: 'sub' },
    { label: '×', value: 'mul' },
    { label: '÷', value: 'div' },
  ]
  const [precision, setPrecision] = React.useState<number>(6)
  const [out, setOut] = React.useState<typeof F | null>(null)
  const [err, setErr] = React.useState<string | null>(null)
  const { get } = useVariableStore()

  const compute = async () => {
    setErr(null); setOut(null)
    try {
      const wasm: any = await getWasm()
      const f = new wasm.RationalFunctionF64(Float64Array.from(F.numerator.coeffs), Float64Array.from(F.denominator.coeffs))
      const g = new wasm.RationalFunctionF64(Float64Array.from(G.numerator.coeffs), Float64Array.from(G.denominator.coeffs))
      let h: any
      if (op==='add') h = f.add(g)
      else if (op==='sub') h = f.sub(g)
      else if (op==='mul') h = f.mul(g)
      else if (op==='div') h = f.div(g)
      else h = f
      h.simplify?.()
      const num = Array.from(h.numeratorCoeffs() as Float64Array)
      const den = Array.from(h.denominatorCoeffs() as Float64Array)
      setOut({ numerator: { coeffs: num }, denominator: { coeffs: den } })
      f.free?.(); g.free?.(); h.free?.()
    } catch (e:any) { setErr(e?.message || String(e)) }
  }


  return (
    <PageContainer title="有理関数の二項演算" stickyHeader>
      <Stack gap={12}>
        <Panel header={null}>
          <Row
            left={<div style={{ display:'flex', gap:8, flexWrap:'wrap', alignItems:'center' }}>
              <span>左:</span>
              <VariablePicker placeholder="変数から代入" allowedKinds={['rational']} onPick={(n)=>{ const v:any=get(n); if(v?.kind==='rational') setF({ numerator:{ coeffs: v.numerator.slice() }, denominator:{ coeffs: v.denominator.slice() } }) }} />
            </div>}
            center={
              <OperationSetting
                operations={operations}
                operation={op}
                onOperationChange={(v)=> setOp(v as any)}
                accuracy={precision}
                onAccuracyChange={(n)=> setPrecision(Math.max(0, Math.min(12, Number(n)||0)))}
                accuracy_able
                onCalc={compute}
                calc_button_able
              />
            }
            right={<div style={{ display:'flex', gap:8, flexWrap:'wrap', alignItems:'center', justifyContent:'flex-end' }}>
              <span>右:</span>
              <VariablePicker placeholder="変数から代入" allowedKinds={['rational']} onPick={(n)=>{ const v:any=get(n); if(v?.kind==='rational') setG({ numerator:{ coeffs: v.numerator.slice() }, denominator:{ coeffs: v.denominator.slice() } }) }} />
            </div>}
          />
        </Panel>
        <div style={{ display:'grid', gap:12, gridTemplateColumns:'repeat(auto-fit, minmax(320px, 1fr))' }}>
          <RationalOperandPanel
            title="左オペランド"
            value={F}
            onChange={setF}
            precision={precision}
            buildSavePayload={()=> ({ kind:'rational', numerator: F.numerator.coeffs.slice(), denominator: F.denominator.coeffs.slice() })}
          />
          <RationalOperandPanel
            title="右オペランド"
            value={G}
            onChange={setG}
            precision={precision}
            buildSavePayload={()=> ({ kind:'rational', numerator: G.numerator.coeffs.slice(), denominator: G.denominator.coeffs.slice() })}
          />
        </div>
        <RationalResultPanel
          result={out}
          error={err}
          precision={precision}
          buildSavePayload={()=> out ? { kind:'rational', numerator: out.numerator.coeffs.slice(), denominator: out.denominator.coeffs.slice() } : null}
        />
      </Stack>
    </PageContainer>
  )
}
