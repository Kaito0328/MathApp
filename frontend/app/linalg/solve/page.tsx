"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import { View } from '../../../src/baseComponents/foundation/View'
import { Text } from '../../../src/baseComponents/foundation/Text'
import { Button } from '../../../src/baseComponents/controls/Button'
import { CoreColorKey, SizeKey, FontWeightKey } from '../../../src/design/tokens'
import { MatrixInput, VectorInput, MatrixSizeControls, VectorSizeControls } from '../../../src/widgets/input'
//
import MarkdownMath from '../../../src/widgets/display/MarkdownMath'
import { formatVectorMarkdown } from '../../../src/utils/format/markdown'
import { formatNumberForMath } from '../../../src/utils/format/markdown'
import { useVariableStore } from '../../../src/state/VariableStore'
import { VariablePicker } from '../../../src/components/features/variables/VariablePicker'
import { solveWith } from '../../../src/wasm/linalg'
import { variableToMarkdown } from '../../../src/components/features/variables/parts/VariableUtils'
import OperationSetting from '../../../src/components/features/operations/OperationSetting'
import Row from '../../../src/baseComponents/layout/Row'
import SectionPanelWithTitle from '../../../src/components/composites/panels/SectionPanelWithTitle'
import DocPanel from '../../../src/components/features/document/Document'
import SourceBlock from '../../../src/components/features/source/SourceBlock'

type MatrixDTO = { rows: number; cols: number; data: number[] }
type VectorDTO = { data: number[] }
type Method = 'auto' | 'inverse' | 'pinv' | 'lu' | 'qr' | 'svd' | 'cholesky'

export default function SolveAxEqB() {
  const { get, upsert } = useVariableStore()
  const [A, setA] = React.useState<MatrixDTO>({ rows: 2, cols: 2, data: [1,0,0,1] })
  const [b, setb] = React.useState<VectorDTO>({ data: [1,2] })
  const [method, setMethod] = React.useState<Method>('auto')

  const [compute, setCompute] = React.useState<any>({})
  const [precision, setPrecision] = React.useState<number>(4)
  const [checks, setChecks] = React.useState<any>({})
  const [refresh, setRefresh] = React.useState<number>(0)

  // latest snapshots to avoid auto compute and satisfy hook deps
  const ARef = React.useRef(A)
  const bRef = React.useRef(b)
  const methodRef = React.useRef(method)
  React.useEffect(() => { ARef.current = A }, [A])
  React.useEffect(() => { bRef.current = b }, [b])
  React.useEffect(() => { methodRef.current = method }, [method])

  React.useEffect(() => {
    let cancelled = false
    const run = async () => {
      if (refresh === 0) return
      const A0 = ARef.current
      const b0 = bRef.current
      const method0 = methodRef.current
      const r = await solveWith(method0, A0, b0)
      if (!cancelled) {
        setCompute(r)
        if (!(r as any)?.error && Array.isArray((r as any).data)) {
          // 残差 ||Ax - b|| の確認
          const x = r as { data: number[] }
          // Ax
          const Ax: number[] = Array.from({ length: A0.rows }, (_, i) => {
            let s = 0
            for (let j = 0; j < A0.cols; j++) s += A0.data[i*A0.cols + j] * x.data[j]
            return s
          })
          const resid = Math.sqrt(Ax.reduce((acc, v, i) => acc + (v - b0.data[i])**2, 0))
          setChecks({ residual: resid })
        }
      }
    }
    run()
    return () => { cancelled = true }
  }, [refresh])

  const save = () => {
  if (compute.error) return
    const name = window.prompt('x を保存する変数名')?.trim()
    if (!name) return
    upsert(name, { kind: 'vector', length: compute.data.length, data: compute.data.slice() })
  }

  const assignFromVar = (target: 'A'|'b', varName: string) => {
    const v: any = get(varName)
    if (!v) return
    if (target === 'A' && v.kind === 'matrix') setA({ rows: v.rows, cols: v.cols, data: v.data.slice() })
    if (target === 'b' && v.kind === 'vector') setb({ data: v.data.slice() })
  }

  const operations: { label: string; value: Method }[] = [
    { label: 'Auto', value: 'auto' },
    { label: '逆行列', value: 'inverse' },
    { label: '疑似逆行列', value: 'pinv' },
    { label: 'LU', value: 'lu' },
    { label: 'QR', value: 'qr' },
    { label: 'SVD', value: 'svd' },
    { label: 'コレスキー', value: 'cholesky' },
  ]

  return (
  <PageContainer title="連立一次方程式 Ax = b の解法" stickyHeader>
      <div style={{ display: 'grid', gap: 12 }}>
        <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1 }}>
          <Row
            left={<div style={{ display:'flex', gap:8, flexWrap:'wrap', alignItems:'center' }}>
              <VariablePicker placeholder="A を変数から代入" allowedKinds={['matrix']} onPick={(n) => assignFromVar('A', n)} />
            </div>}
            center={
              <OperationSetting
                operations={operations}
                operation={method}
                onOperationChange={(v)=> setMethod(v as Method)}
                accuracy={precision}
                onAccuracyChange={(n)=> setPrecision(Math.max(0, Math.min(10, Math.floor(Number(n)||0))))}
                accuracy_able
                onCalc={() => setRefresh((n)=>n+1)}
                calc_button_able
                label={'手法'}
              />
            }
            right={<div style={{ display:'flex', gap:8, flexWrap:'wrap', alignItems:'center', justifyContent:'flex-end' }}>
              <VariablePicker placeholder="b を変数から代入" allowedKinds={['vector']} onPick={(n) => assignFromVar('b', n)} />
            </div>}
          />
  </View>

        <div style={{ display: 'grid', gap: 12, gridTemplateColumns: 'repeat(auto-fit, minmax(320px, 1fr))' }}>
          <SectionPanelWithTitle title="A" showSave showCopy buildSavePayload={()=> ({ kind:'matrix', rows: A.rows, cols: A.cols, data: A.data.slice() })} copyContent={variableToMarkdown({ kind:'matrix', rows: A.rows, cols: A.cols, data: A.data })}>
            <div style={{ display:'grid', gap:8 }}>
              <div style={{ display: 'flex', gap: 12, alignItems: 'center' }}>
                <MatrixSizeControls rows={A.rows} cols={A.cols} onChange={(r, c) => {
                  const size = r*c
                  const next = A.data.slice(0, size)
                  if (next.length < size) next.push(...Array(size - next.length).fill(0))
                  setA({ rows: r, cols: c, data: next })
                  setb((prev) => ({ data: prev.data.slice(0, r).concat(Array(Math.max(0, r - prev.data.length)).fill(0)) }))
                }} />
              </div>
              <MatrixInput value={A} onChange={setA} rows={A.rows} cols={A.cols} />
            </div>
          </SectionPanelWithTitle>
          <SectionPanelWithTitle title="b" showSave showCopy buildSavePayload={()=> ({ kind:'vector', length: b.data.length, data: b.data.slice() })} copyContent={variableToMarkdown({ kind:'vector', data: b.data })}>
            <div style={{ display:'grid', gap:8 }}>
              <div style={{ display:'flex', gap: 12, alignItems:'center' }}>
                <VectorSizeControls length={b.data.length} onChange={(n) => setb({ data: b.data.slice(0, n).concat(Array(Math.max(0, n - b.data.length)).fill(0)) })} />
              </div>
              <VectorInput value={b} onChange={setb} orientation="col" length={b.data.length} />
            </div>
          </SectionPanelWithTitle>
        </div>

        <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1 }}>
          <div style={{ display:'flex', alignItems:'center' }}>
            <Text weight={FontWeightKey.Medium}>結果</Text>
            <div style={{ marginLeft: 'auto' }}>
              <div style={{ display:'flex', gap: 6 }}>
                <Button onClick={save} disabled={!Array.isArray(compute.data)} color={CoreColorKey.Primary} aria-label="保存" title="保存">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
                </Button>
                {Array.isArray(compute.data) && (
                  <Button color={CoreColorKey.Base} aria-label="Markdown コピー" title="Markdown コピー" onClick={() => { const md = variableToMarkdown({ kind:'vector', data: compute.data }); if (md) navigator.clipboard?.writeText(md) }}>
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
                  </Button>
                )}
              </div>
            </div>
          </div>
          <div style={{ marginTop: 8 }}>
            {compute.error ? (
              <Text>{compute.error}</Text>
            ) : Array.isArray(compute.data) ? (
              <MarkdownMath math={`x = ${formatVectorMarkdown(compute.data, { orientation: 'col', precision, paren: true })}`} />
            ) : null}
          </div>
        </View>

        {/* 検証ブロック */}
        <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1 }}>
          <Text weight={FontWeightKey.Medium}>検証</Text>
          <div style={{ marginTop: 8, display: 'grid', gap: 8 }}>
            {'residual' in checks && (
              <div>
                <MarkdownMath math={`\\lVert A x - b \\rVert = ${formatNumberForMath(Number(checks.residual || 0), precision)}`} />
              </div>
            )}
          </div>
        </View>

        <SectionPanelWithTitle title="ドキュメント">
          <DocPanel
            docPath={
              method === 'inverse' ? 'notes/linalg/overview.md'
              : method === 'pinv' ? 'notes/linalg/matrix_pinv.md'
              : method === 'qr' ? 'notes/linalg/matrix_qr.md'
              : method === 'svd' ? 'notes/linalg/matrix_svd.md'
              : method === 'cholesky' ? 'notes/linalg/overview.md'
              : 'notes/linalg/overview.md'
            }
          />
        </SectionPanelWithTitle>
        <div style={{ marginTop: 12 }}>
          <SourceBlock title="ソースコード（linalg solve 関連）" path="crates/linalg/src/matrix/numerical/eigen/mod.rs" />
        </div>
      </div>
    </PageContainer>
  )
}
