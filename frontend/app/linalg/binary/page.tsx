"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import Row from '../../../src/baseComponents/layout/Row'
import BinaryLayout from '../../../src/components/features/layout/BinaryLayout'
import { useVariableStore } from '../../../src/state/VariableStore'
import { VariablePicker } from '../../../src/components/features/variables/VariablePicker'
import { MatrixInput, VectorInput, MatrixSizeControls, VectorSizeControls } from '../../../src/widgets/input'
import { MatrixView, VectorView } from '../../../src/widgets/display'
import MarkdownMath from '../../../src/widgets/display/MarkdownMath'
import Document from '../../../src/components/features/document/Document'
import WithSaveCopy from '../../../src/components/features/saveCopy/WithSaveCopy'
import { variableToMarkdown } from '../../../src/components/features/variables/parts/VariableUtils'
import SourceBlock from '../../../src/components/features/source/SourceBlock'

type MatrixDTO = { rows: number; cols: number; data: number[] }
type VectorDTO = { data: number[] }

type Kind = 'matrix' | 'vector' | 'scalar'
type Op = 'add' | 'sub' | 'mul'

function matAdd(A: MatrixDTO, B: MatrixDTO): MatrixDTO | { error: string } {
  if (A.rows !== B.rows || A.cols !== B.cols) return { error: 'サイズが一致しません（同型行列が必要）' }
  const data = A.data.map((a, i) => a + (B.data[i] ?? 0))
  return { rows: A.rows, cols: A.cols, data }
}
function vecAdd(a: VectorDTO, b: VectorDTO): VectorDTO | { error: string } {
  if (a.data.length !== b.data.length) return { error: '長さが一致しません' }
  return { data: a.data.map((v, i) => v + (b.data[i] ?? 0)) }
}
function matMul(A: MatrixDTO, B: MatrixDTO): MatrixDTO | { error: string } {
  if (A.cols !== B.rows) return { error: '次元不一致: A.cols と B.rows が一致する必要があります' }
  const out: number[] = Array(A.rows * B.cols).fill(0)
  for (let i = 0; i < A.rows; i++) {
    for (let k = 0; k < A.cols; k++) {
      const aik = A.data[i * A.cols + k]
      for (let j = 0; j < B.cols; j++) {
        out[i * B.cols + j] += aik * B.data[k * B.cols + j]
      }
    }
  }
  return { rows: A.rows, cols: B.cols, data: out }
}
function matVecMul(A: MatrixDTO, x: VectorDTO): VectorDTO | { error: string } {
  if (A.cols !== x.data.length) return { error: '次元不一致: A.cols と len(x) が一致する必要があります' }
  const out: number[] = Array(A.rows).fill(0)
  for (let i = 0; i < A.rows; i++) {
    let s = 0
    for (let k = 0; k < A.cols; k++) s += A.data[i * A.cols + k] * x.data[k]
    out[i] = s
  }
  return { data: out }
}
const scaleMat = (s: number, A: MatrixDTO): MatrixDTO => ({ rows: A.rows, cols: A.cols, data: A.data.map(v => s * v) })
const scaleVec = (s: number, x: VectorDTO): VectorDTO => ({ data: x.data.map(v => s * v) })

export default function BinaryOps() {
  const { get } = useVariableStore()

  const [leftKind, setLeftKind] = React.useState<Kind>('matrix')
  const [rightKind, setRightKind] = React.useState<Kind>('matrix')
  const [op, setOp] = React.useState<Op>('mul')
  const operations: { label: string; value: Op }[] = [
    { label: '+', value: 'add' },
    { label: '-', value: 'sub' },
    { label: '×', value: 'mul' },
  ]

  const [A, setA] = React.useState<MatrixDTO>({ rows: 2, cols: 2, data: [1, 0, 0, 1] })
  const [B, setB] = React.useState<MatrixDTO>({ rows: 2, cols: 2, data: [1, 2, 3, 4] })
  const [x, setX] = React.useState<VectorDTO>({ data: [1, 2] })
  const [y, setY] = React.useState<VectorDTO>({ data: [3, 4] })
  const [s1, setS1] = React.useState<number>(2)
  const [s2, setS2] = React.useState<number>(3)

  const compute = React.useMemo(() => {
    if (op === 'add' || op === 'sub') {
      if (leftKind === 'matrix' && rightKind === 'matrix') {
        const r = matAdd(A, { rows: B.rows, cols: B.cols, data: B.data.map(v => op === 'sub' ? -v : v) })
        return r
      }
      if (leftKind === 'vector' && rightKind === 'vector') {
        const r = vecAdd(x, { data: y.data.map(v => op === 'sub' ? -v : v) })
        return r
      }
      return { error: '加減算は同じ種類（行列同士、ベクトル同士）のみ対応しています' }
    }
    // multiply
    if (leftKind === 'matrix' && rightKind === 'matrix') return matMul(A, B)
    if (leftKind === 'matrix' && rightKind === 'vector') return matVecMul(A, y)
    if (leftKind === 'scalar' && rightKind === 'matrix') return scaleMat(s1, B)
    if (leftKind === 'scalar' && rightKind === 'vector') return scaleVec(s1, y)
    if (leftKind === 'matrix' && rightKind === 'scalar') return scaleMat(s2, A)
    if (leftKind === 'vector' && rightKind === 'scalar') return scaleVec(s2, x)
    return { error: '未対応の組合せです' }
  }, [op, leftKind, rightKind, A, B, x, y, s1, s2])

  const assignFromVar = (side: 'left'|'right', varName: string) => {
    const v: any = get(varName)
    if (!v) return
    if (v.kind === 'matrix') {
      if (side === 'left') { setLeftKind('matrix'); setA({ rows: v.rows, cols: v.cols, data: v.data.slice() }) }
      else { setRightKind('matrix'); setB({ rows: v.rows, cols: v.cols, data: v.data.slice() }) }
    } else if (v.kind === 'vector') {
      if (side === 'left') { setLeftKind('vector'); setX({ data: v.data.slice() }) }
      else { setRightKind('vector'); setY({ data: v.data.slice() }) }
    }
  }

  const operandLeftNode = (
    leftKind === 'matrix' ? (
      <div style={{ display:'grid', gap:8 }}>
        <MatrixSizeControls rows={A.rows} cols={A.cols} onChange={(r,c)=>{
          const size = r*c
          const next = A.data.slice(0, size)
          if (next.length < size) next.push(...Array(size-next.length).fill(0))
          setA({ rows:r, cols:c, data: next })
        }} />
        <MatrixInput value={A} onChange={setA} rows={A.rows} cols={A.cols} />
      </div>
    ) : leftKind === 'vector' ? (
      <div style={{ display:'grid', gap:8 }}>
        <VectorSizeControls length={x.data.length} onChange={(n)=> setX({ data: x.data.slice(0,n).concat(Array(Math.max(0, n - x.data.length)).fill(0)) })} />
        <VectorInput value={x} onChange={setX} orientation="col" length={x.data.length} />
      </div>
    ) : (
      <input type="number" value={s1} onChange={(e)=> setS1(Number(e.target.value||0))} />
    )
  )
  const operandRightNode = (
    rightKind === 'matrix' ? (
      <div style={{ display:'grid', gap:8 }}>
        <MatrixSizeControls rows={B.rows} cols={B.cols} onChange={(r,c)=>{
          const size = r*c
          const next = B.data.slice(0, size)
          if (next.length < size) next.push(...Array(size-next.length).fill(0))
          setB({ rows:r, cols:c, data: next })
        }} />
        <MatrixInput value={B} onChange={setB} rows={B.rows} cols={B.cols} />
      </div>
    ) : rightKind === 'vector' ? (
      <div style={{ display:'grid', gap:8 }}>
        <VectorSizeControls length={y.data.length} onChange={(n)=> setY({ data: y.data.slice(0,n).concat(Array(Math.max(0, n - y.data.length)).fill(0)) })} />
        <VectorInput value={y} onChange={setY} orientation="col" length={y.data.length} />
      </div>
    ) : (
      <input type="number" value={s2} onChange={(e)=> setS2(Number(e.target.value||0))} />
    )
  )

  const buildPayloadLeft = () => {
    if (leftKind === 'matrix') return { kind:'matrix', rows: A.rows, cols: A.cols, data: A.data }
    if (leftKind === 'vector') return { kind:'vector', data: x.data }
    return null
  }
  const buildPayloadRight = () => {
    if (rightKind === 'matrix') return { kind:'matrix', rows: B.rows, cols: B.cols, data: B.data }
    if (rightKind === 'vector') return { kind:'vector', data: y.data }
    return null
  }
  const copyLeft = () => {
    const p = buildPayloadLeft(); return p ? variableToMarkdown(p as any) : ''
  }
  const copyRight = () => {
    const p = buildPayloadRight(); return p ? variableToMarkdown(p as any) : ''
  }

  const resultNode = (
    compute && (compute as any).error ? (
      <span style={{ color:'crimson' }}>{(compute as any).error}</span>
    ) : (compute as any)?.rows != null ? (
      <div style={{ display:'flex', gap:6, alignItems:'center', flexWrap:'wrap' }}>
        <MarkdownMath math={`${leftKind==='matrix' ? 'A' : leftKind==='vector' ? 'x' : String(s1)} \\${op==='mul'?'times':op==='add'?'+':'-'} ${rightKind==='matrix' ? 'B' : rightKind==='vector' ? 'y' : String(s2)} =`} block={false} />
        <WithSaveCopy buildSavePayload={() => ({ kind:'matrix', rows:(compute as MatrixDTO).rows, cols:(compute as MatrixDTO).cols, data:(compute as MatrixDTO).data })} copyContent={variableToMarkdown({ kind:'matrix', rows:(compute as MatrixDTO).rows, cols:(compute as MatrixDTO).cols, data:(compute as MatrixDTO).data })}>
          <MatrixView rows={(compute as MatrixDTO).rows} cols={(compute as MatrixDTO).cols} data={(compute as MatrixDTO).data} />
        </WithSaveCopy>
      </div>
    ) : (compute as any)?.data ? (
      <div style={{ display:'flex', gap:6, alignItems:'center', flexWrap:'wrap' }}>
        <MarkdownMath math={`${leftKind==='matrix' ? 'A' : leftKind==='vector' ? 'x' : String(s1)} \\${op==='mul'?'times':op==='add'?'+':'-'} ${rightKind==='matrix' ? 'B' : rightKind==='vector' ? 'y' : String(s2)} =`} block={false} />
        <WithSaveCopy buildSavePayload={() => ({ kind:'vector', data:(compute as VectorDTO).data })} copyContent={variableToMarkdown({ kind:'vector', data:(compute as VectorDTO).data })}>
          <VectorView orientation="col" values={(compute as VectorDTO).data} />
        </WithSaveCopy>
      </div>
    ) : null
  )

  return (
    <PageContainer title="行列・ベクトルの二項演算" stickyHeader>
      <BinaryLayout
        operations={operations}
        operation={op}
        onOperationChange={(v)=> setOp(v as Op)}
        onAccuracyChange={()=>{}}
        operation_left={
          <Row left={
            <label>左:
              <select value={leftKind} onChange={(e)=> setLeftKind(e.target.value as Kind)}>
                <option value="matrix">行列</option>
                <option value="vector">ベクトル</option>
                <option value="scalar">スカラー</option>
              </select>
            </label>
          } right={
            <VariablePicker placeholder="変数から代入" allowedKinds={leftKind==='matrix'?['matrix']:leftKind==='vector'?['vector']:undefined} onPick={(n)=> assignFromVar('left', n)} />
          }/>
        }
        operation_right={
          <Row left={
            <label>右:
              <select value={rightKind} onChange={(e)=> setRightKind(e.target.value as Kind)}>
                <option value="matrix">行列</option>
                <option value="vector">ベクトル</option>
                <option value="scalar">スカラー</option>
              </select>
            </label>
          } right={
            <VariablePicker placeholder="変数から代入" allowedKinds={rightKind==='matrix'?['matrix']:rightKind==='vector'?['vector']:undefined} onPick={(n)=> assignFromVar('right', n)} />
          }/>
        }
        operand_left={operandLeftNode}
        operand_left_buildSavePayload={buildPayloadLeft}
        operand_left_afterSave={()=>{}}
        operand_left_copyContent={copyLeft()}
        operand_right={operandRightNode}
        operand_right_buildSavePayload={buildPayloadRight}
        operand_right_afterSave={()=>{}}
        operand_right_copyContent={copyRight()}
        result={resultNode}
        document={<Document docPath={'notes/linalg/overview.md'} />}
        documentTitle="ドキュメント"
      />
      <div style={{ marginTop: 12 }}>
        <SourceBlock title="ソースコード（linalg 二項演算）" path="crates/linalg/src/matrix/algebra/field.rs" />
      </div>
    </PageContainer>
  )
}
