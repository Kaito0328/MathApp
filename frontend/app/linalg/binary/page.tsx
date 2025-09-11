"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import Stack from '../../../src/baseComponents/layout/Stack'
import Panel from '../../../src/baseComponents/layout/Panel'
import Row from '../../../src/baseComponents/layout/Row'
import { useVariableStore } from '../../../src/state/VariableStore'
import { VariablePicker } from '../../../src/components/variables/VariablePicker'
import LinalgOperandPanel from '../../../src/components/operations/LinalgOperandPanel'
import LinalgResultPanel from '../../../src/components/operations/LinalgResultPanel'
import OperationSetting from '../../../src/components/operations/OperationSetting'

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
  const { get, upsert } = useVariableStore()

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

  // save handled by ActionPanel in result

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

  return (
    <PageContainer title="行列・ベクトルの二項演算" stickyHeader>
      <Stack gap={12}>
        {/* Controls */}
        <Panel
          header={null}
          style={{}}
        >
          <Stack gap={8}>
            <Row gap={8} zoneGap={8} leftWrap centerWrap rightWrap
              left={
                <>
                  <label>左:
                    <select value={leftKind} onChange={(e) => setLeftKind(e.target.value as Kind)}>
                      <option value="matrix">行列</option>
                      <option value="vector">ベクトル</option>
                      <option value="scalar">スカラー</option>
                    </select>
                  </label>
                  <VariablePicker placeholder="変数から代入" allowedKinds={leftKind==='matrix' ? ['matrix'] : leftKind==='vector' ? ['vector'] : undefined} onPick={(n) => assignFromVar('left', n)} />
                </>
              }
              center={
                <OperationSetting
                  operations={operations}
                  operation={op}
                  onOperationChange={(v)=> setOp(v as Op)}
                  onAccuracyChange={()=>{}}
                />
              }
              right={
                <>
                  <label>右:
                    <select value={rightKind} onChange={(e) => setRightKind(e.target.value as Kind)}>
                      <option value="matrix">行列</option>
                      <option value="vector">ベクトル</option>
                      <option value="scalar">スカラー</option>
                    </select>
                  </label>
                  <VariablePicker placeholder="変数から代入" allowedKinds={rightKind==='matrix' ? ['matrix'] : rightKind==='vector' ? ['vector'] : undefined} onPick={(n) => assignFromVar('right', n)} />
                </>
              }
            />
          </Stack>
        </Panel>

        {/* Operands */}
        <div style={{ display:'grid', gap:12, gridTemplateColumns:'repeat(auto-fit, minmax(320px, 1fr))' }}>
          <LinalgOperandPanel
            title="左オペランド"
            kind={leftKind}
            matrix={A}
            vector={x}
            scalar={s1}
            onChangeMatrix={setA}
            onChangeVector={setX}
            onChangeScalar={setS1}
            onSave={()=> { const name = window.prompt('保存する変数名')?.trim(); if(!name) return; if(leftKind==='matrix') upsert(name,{ kind:'matrix', rows:A.rows, cols:A.cols, data:A.data.slice() }); else if(leftKind==='vector') upsert(name,{ kind:'vector', length:x.data.length, data:x.data.slice() }); }}
          />
          <LinalgOperandPanel
            title="右オペランド"
            kind={rightKind}
            matrix={B}
            vector={y}
            scalar={s2}
            onChangeMatrix={setB}
            onChangeVector={setY}
            onChangeScalar={setS2}
            onSave={()=> { const name = window.prompt('保存する変数名')?.trim(); if(!name) return; if(rightKind==='matrix') upsert(name,{ kind:'matrix', rows:B.rows, cols:B.cols, data:B.data.slice() }); else if(rightKind==='vector') upsert(name,{ kind:'vector', length:y.data.length, data:y.data.slice() }); }}
          />
        </div>
        {/* Result */}
    <LinalgResultPanel
          compute={compute}
            leftKind={leftKind}
            rightKind={rightKind}
            op={op}
            s1={s1}
      s2={s2}
        />
      </Stack>
    </PageContainer>
  )
}
