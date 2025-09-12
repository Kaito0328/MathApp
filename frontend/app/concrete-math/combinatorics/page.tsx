"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import UnaryLayout from '../../../src/components/features/layout/UnaryLayout'
import NumberCellInput from '../../../src/baseComponents/input/NumberCellInput'
import LabeledMathResult from '../../../src/components/features/result/LabeledMathResult'
import Document from '../../../src/components/features/document/Document'

export default function CombinatoricsPage() {
  type Op = 'binom' | 'stirling2'
  const operations: { label: string; value: Op }[] = [
    { label: '二項係数 C(n,k)', value: 'binom' },
    { label: '第二種スターリング数 S(n,k)', value: 'stirling2' },
  ]
  const [op, setOp] = React.useState<Op | 'noop'>('noop')
  const [precision, setPrecision] = React.useState<number>(0)
  const [n, setN] = React.useState<number>(5)
  const [k, setK] = React.useState<number>(2)
  const [val, setVal] = React.useState<number | null>(null)
  const [err, setErr] = React.useState<string | null>(null)

  const run = async () => {
    setErr(null); setVal(null)
    try {
      const { cm_binom, cm_stirling2 } = await import('../../../src/wasm/concreteMath')
      const v = op==='binom' ? await cm_binom(n, k) : await cm_stirling2(n, k)
      setVal(v)
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  return (
    <PageContainer title="Concrete Math / 組合せ論" stickyHeader>
      <UnaryLayout
        operations={operations}
        operation={op}
        onOperationChange={(v)=> setOp(v as Op)}
        accuracy={precision}
        onAccuracyChange={(p)=> setPrecision(p)}
        onCalc={run}
        calc_button_able
        accuracy_able
        // operand
        operand={
          <div style={{ display:'flex', gap:16, alignItems:'center' }}>
            <div style={{ display:'flex', gap:6, alignItems:'center' }}>
              <span>n=</span>
              <NumberCellInput value={n} onChange={setN} width={96} />
            </div>
            <div style={{ display:'flex', gap:6, alignItems:'center' }}>
              <span>k=</span>
              <NumberCellInput value={k} onChange={setK} width={96} />
            </div>
          </div>
        }
        operand_copyContent={undefined}
        operand_buildSavePayload={()=> null}
        operand_afterSave={()=>{}}
        // result
        result={
          err ? <div style={{ color:'crimson' }}>{err}</div>
              : (val!=null ? <LabeledMathResult label={op==='binom'? 'C(n,k)=' : 'S(n,k)='} body={String(val.toFixed(precision))} /> : null)
        }
        verification={null}
        document={
          <Document docPath={
            op==='binom' ? 'notes/concrete-math/combinatorics_numbers.md'
            : op==='stirling2' ? 'notes/concrete-math/combinatorics_numbers.md'
            : 'notes/concrete-math/overview.md'
          } />
        }
        documentTitle="ドキュメント"
      />
    </PageContainer>
  )
}
