"use client"
import React from 'react'
import PageContainer from '../../../../src/baseComponents/layout/PageContainer'
import { getWasm } from '../../../../src/wasm/loader'
import { formatRationalFunctionMarkdown, formatPartialFractionExpansionMarkdown } from '../../../../src/utils/format/markdown'
import UnaryLayout from '../../../../src/components/features/layout/UnaryLayout'
import RationalFunctionInput from '../../../../src/widgets/input/RationalFunctionInput'
import RationalUnaryResultSwitch, { type RationalUnaryResultDU } from '../../../../src/components/domain/polynomial/rationalFunction/result/unary/RationalUnaryResultSwitch'
import ZerosPolesVerification from '../../../../src/components/domain/polynomial/rationalFunction/verfication/ZerosPolesVerification'
import { VariablePicker } from '../../../../src/components/features/variables/VariablePicker'
import Row from '../../../../src/baseComponents/layout/Row'
import { useVariableStore } from '../../../../src/state/VariableStore'
import DocPanel from '../../../../src/components/features/document/Document'
import SourceBlock from '../../../../src/components/features/source/SourceBlock'

export default function RationalUnaryPage() {
  type RationalUnaryOp = 'diff'|'simplify'|'zeros'|'poles'|'pfe'
  const operations: { label: string; value: RationalUnaryOp }[] = [
    { label: '微分', value: 'diff' },
    { label: '約分', value: 'simplify' },
    { label: '根', value: 'zeros' },
    { label: '極', value: 'poles' },
    { label: '部分分数分解', value: 'pfe' },
  ]
  const { get } = useVariableStore()
  const [F, setF] = React.useState({ numerator: { coeffs: [1,0] }, denominator: { coeffs: [1,1] } })
  const [op, setOp] = React.useState<RationalUnaryOp | 'noop'>('noop')
  const [precision, setPrecision] = React.useState<number>(6)
  // results
  const [out, setOut] = React.useState<typeof F | null>(null)
  const [err, setErr] = React.useState<string | null>(null)
  const [zerosInterleaved, setZerosInterleaved] = React.useState<number[] | null>(null)
  const [polesInterleaved, setPolesInterleaved] = React.useState<number[] | null>(null)
  const [pfeAligned, setPfeAligned] = React.useState<string[] | null>(null)

  const reset = () => { setErr(null); setOut(null); setZerosInterleaved(null); setPolesInterleaved(null); setPfeAligned(null) }

  const runOp = async () => {
    reset()
    try {
      const wasm: any = await getWasm()
      const f = new wasm.RationalFunctionF64(Float64Array.from(F.numerator.coeffs), Float64Array.from(F.denominator.coeffs))
      if (op==='diff') {
        const d = f.differentiate(); d.simplify?.()
        setOut({ numerator: { coeffs: Array.from(d.numeratorCoeffs() as Float64Array) }, denominator: { coeffs: Array.from(d.denominatorCoeffs() as Float64Array) } })
        d.free?.()
      } else if (op==='simplify') {
        f.simplify?.()
        setOut({ numerator: { coeffs: Array.from(f.numeratorCoeffs() as Float64Array) }, denominator: { coeffs: Array.from(f.denominatorCoeffs() as Float64Array) } })
      } else if (op==='zeros') {
        const num = new wasm.PolynomialF64(f.numeratorCoeffs())
        const inter = num.findRoots() as Float64Array
        num.free?.()
        setZerosInterleaved(Array.from(inter))
      } else if (op==='poles') {
        const den = new wasm.PolynomialF64(f.denominatorCoeffs())
        const inter = den.findRoots() as Float64Array
        den.free?.()
        setPolesInterleaved(Array.from(inter))
      } else if (op==='pfe') {
        const res = f.partialFractionExpansion()
        const rhs = formatPartialFractionExpansionMarkdown(res, 'x', { precision })
        const original = formatRationalFunctionMarkdown({ numerator:{ coeffs:Array.from(f.numeratorCoeffs() as Float64Array)}, denominator:{ coeffs:Array.from(f.denominatorCoeffs() as Float64Array)} } as any)
        const line = rhs ? `\\begin{aligned} ${original} &= ${rhs} \\end{aligned}` : `\\begin{aligned} ${original} &= 0 \\end{aligned}`
        setPfeAligned([line])
      }
      f.free?.()
    } catch (e:any) {
      setErr(e?.message || String(e))
    }
  }

  let data: RationalUnaryResultDU | null = null
  if (op==='diff' || op==='simplify') {
    data = { op, value: out ?? undefined, error: err ?? undefined } as any
  } else if (op==='zeros') {
    data = { op: 'zeros', zeros: zerosInterleaved ?? undefined, error: err ?? undefined }
  } else if (op==='poles') {
    data = { op: 'poles', poles: polesInterleaved ?? undefined, error: err ?? undefined }
  } else if (op==='pfe') {
    data = { op: 'pfe', lines: pfeAligned ?? undefined, error: err ?? undefined }
  }

  return (
    <PageContainer title="有理関数の単項演算" stickyHeader>
      <UnaryLayout
        operations={operations}
        operation={op}
        onOperationChange={(v)=> setOp(v as any)}
        accuracy={precision}
        onAccuracyChange={(n)=> setPrecision(Math.max(0, Math.min(12, Number(n)||0)))}
        onCalc={runOp}
        calc_button_able
        accuracy_able
        operation_left={
          <VariablePicker
            placeholder="変数から代入"
            allowedKinds={['rational']}
            onPick={(name)=>{ const v:any = get(name); if (v?.kind==='rational') setF({ numerator:{ coeffs: v.numerator.slice() }, denominator:{ coeffs: v.denominator.slice() } }) }}
          />
        }
        operation_right={<Row />}
        operand={<RationalFunctionInput value={F} onChange={setF} />}
        operand_copyContent={`$${formatRationalFunctionMarkdown(F as any)}$`}
        operand_buildSavePayload={()=> ({ kind:'rational', numerator: F.numerator.coeffs.slice(), denominator: F.denominator.coeffs.slice() })}
        operand_afterSave={()=>{}}
                result={<RationalUnaryResultSwitch data={data} precision={precision} />}
                document={<DocPanel docPath={'notes/polynomial/rational_function.md'} />}
        documentTitle="ドキュメント"
        verification={op==='zeros' ? (
          <ZerosPolesVerification kind="zeros" coeffs={F.numerator.coeffs} roots={zerosInterleaved ?? undefined} precision={precision} />
        ) : op==='poles' ? (
          <ZerosPolesVerification kind="poles" coeffs={F.denominator.coeffs} roots={polesInterleaved ?? undefined} precision={precision} />
        ) : null}
      />
      <div style={{ marginTop: 12 }}>
        <SourceBlock title="ソースコード（polynomial / rational）" path="crates/polynomial/src/lib.rs" />
      </div>
    </PageContainer>
  )
}
