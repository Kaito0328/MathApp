"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/patterns/PageContainer'
import { View } from '../../../src/baseComponents/foundation/View'
import { Text } from '../../../src/baseComponents/foundation/Text'
import { Button } from '../../../src/baseComponents/patterns/Button'
import { CoreColorKey, SizeKey, FontWeightKey } from '../../../src/design/tokens'
import { MatrixInput, VectorInput, MatrixSizeControls, VectorSizeControls } from '../../../src/widgets/input'
import { MatrixView, VectorView } from '../../../src/widgets/display'
import MarkdownMath from '../../../src/widgets/display/MarkdownMath'
import { useVariableStore } from '../../../src/state/VariableStore'
import { VariablePicker } from '../../../src/components/variables/VariablePicker'
import { variableToMarkdown } from '../../../src/components/variables/parts/VariableUtils'

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

  const saveResult = () => {
    if ('error' in (compute as any)) return
    const name = window.prompt('保存する変数名')?.trim()
    if (!name) return
    const r: any = compute
    if ((r as MatrixDTO).rows != null && (r as MatrixDTO).cols != null) {
      upsert(name, { kind: 'matrix', rows: r.rows, cols: r.cols, data: r.data })
    } else if ((r as VectorDTO).data) {
      upsert(name, { kind: 'vector', length: r.data.length, data: r.data })
    } else {
      alert('スカラー結果の保存には未対応です')
    }
  }

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
      <div style={{ display: 'grid', gap: 12 }}>
        {/* Controls */}
        <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1 }}>
          <div style={{ display: 'grid', gap: 8 }}>
            <div style={{ display: 'grid', gridTemplateColumns: '1fr auto 1fr', gap: 8, alignItems: 'center' }}>
              <div style={{ display: 'flex', gap: 8, alignItems: 'center', flexWrap: 'wrap' }}>
                <label>左: 
                  <select value={leftKind} onChange={(e) => setLeftKind(e.target.value as Kind)}>
                    <option value="matrix">行列</option>
                    <option value="vector">ベクトル</option>
                    <option value="scalar">スカラー</option>
                  </select>
                </label>
                <VariablePicker placeholder="変数から代入" allowedKinds={leftKind==='matrix' ? ['matrix'] : leftKind==='vector' ? ['vector'] : undefined} onPick={(n) => assignFromVar('left', n)} />
              </div>
              <div style={{ justifySelf: 'center', display: 'flex', alignItems: 'center', gap: 8 }}>
                <label>演算
                  <select value={op} onChange={(e) => setOp(e.target.value as Op)}>
                    <option value="add">+</option>
                    <option value="sub">-</option>
                    <option value="mul">×</option>
                  </select>
                </label>
              </div>
              <div style={{ display: 'flex', gap: 8, alignItems: 'center', flexWrap: 'wrap', justifyContent: 'flex-end' }}>
                <label>右: 
                  <select value={rightKind} onChange={(e) => setRightKind(e.target.value as Kind)}>
                    <option value="matrix">行列</option>
                    <option value="vector">ベクトル</option>
                    <option value="scalar">スカラー</option>
                  </select>
                </label>
                <VariablePicker placeholder="変数から代入" allowedKinds={rightKind==='matrix' ? ['matrix'] : rightKind==='vector' ? ['vector'] : undefined} onPick={(n) => assignFromVar('right', n)} />
              </div>
            </div>
          </div>
        </View>

        {/* Operands */}
        <div style={{ display: 'grid', gap: 12, gridTemplateColumns: 'repeat(auto-fit, minmax(320px, 1fr))' }}>
          <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1 }}>
            <Text weight={FontWeightKey.Medium}>左オペランド</Text>
            <div style={{ marginTop: 8 }}>
              {leftKind === 'matrix' ? (
                <div style={{ display:'grid', gridTemplateColumns: '1fr auto', gap: 8, alignItems: 'start' }}>
                  <div>
                    <div style={{ display:'flex', alignItems:'center', gap: 8, marginBottom: 8 }}>
                      <MatrixSizeControls rows={A.rows} cols={A.cols} onChange={(r,c)=> {
                        const size = r*c
                        const next = A.data.slice(0, size)
                        if (next.length < size) next.push(...Array(size - next.length).fill(0))
                        setA({ rows: r, cols: c, data: next })
                      }} />
                    </div>
                    <MatrixInput value={A} onChange={setA} rows={A.rows} cols={A.cols} />
                  </div>
                  <div>
                    <div style={{ display:'flex', gap: 4 }}>
                      <Button size={SizeKey.SM} onClick={() => { const name = window.prompt('保存する変数名')?.trim(); if (!name) return; upsert(name, { kind:'matrix', rows: A.rows, cols: A.cols, data: A.data.slice() }) }} color={CoreColorKey.Primary} aria-label="保存" title="保存">
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
                      </Button>
                      <Button size={SizeKey.SM} color={CoreColorKey.Base} aria-label="Markdown コピー" title="Markdown コピー" onClick={() => { const md = variableToMarkdown({ kind:'matrix', rows: A.rows, cols: A.cols, data: A.data }); if (md) navigator.clipboard?.writeText(md) }}>
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
                      </Button>
                    </div>
                  </div>
                </div>
              ) : leftKind === 'vector' ? (
                <>
                  <div style={{ display:'flex', alignItems:'center', gap: 8, marginBottom: 8 }}>
                    <VectorSizeControls length={x.data.length} onChange={(n)=> setX({ data: x.data.slice(0, n).concat(Array(Math.max(0, n - x.data.length)).fill(0)) })} />
                    <div style={{ display:'flex', gap: 4 }}>
                      <Button size={SizeKey.SM} onClick={() => { const name = window.prompt('保存する変数名')?.trim(); if (!name) return; upsert(name, { kind:'vector', length: x.data.length, data: x.data.slice() }) }} color={CoreColorKey.Primary} aria-label="保存" title="保存">
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
                      </Button>
                      <Button size={SizeKey.SM} color={CoreColorKey.Base} aria-label="Markdown コピー" title="Markdown コピー" onClick={() => { const md = variableToMarkdown({ kind:'vector', data: x.data }); if (md) navigator.clipboard?.writeText(md) }}>
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
                      </Button>
                    </div>
                  </div>
                  <VectorInput value={x} onChange={setX} orientation="col" length={x.data.length} />
                </>
              ) : (
                <input type="number" value={s1} onChange={(e) => setS1(Number(e.target.value||0))} />
              )}
            </div>
          </View>
          <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1 }}>
            <Text weight={FontWeightKey.Medium}>右オペランド</Text>
            <div style={{ marginTop: 8 }}>
              {rightKind === 'matrix' ? (
                <div style={{ display:'grid', gridTemplateColumns: '1fr auto', gap: 8, alignItems: 'start' }}>
                  <div>
                    <div style={{ display:'flex', alignItems:'center', gap: 8, marginBottom: 8 }}>
                      <MatrixSizeControls rows={B.rows} cols={B.cols} onChange={(r,c)=> {
                        const size = r*c
                        const next = B.data.slice(0, size)
                        if (next.length < size) next.push(...Array(size - next.length).fill(0))
                        setB({ rows: r, cols: c, data: next })
                      }} />
                    </div>
                    <MatrixInput value={B} onChange={setB} rows={B.rows} cols={B.cols} />
                  </div>
                  <div>
                    <div style={{ display:'flex', gap: 4 }}>
                      <Button size={SizeKey.SM} onClick={() => { const name = window.prompt('保存する変数名')?.trim(); if (!name) return; upsert(name, { kind:'matrix', rows: B.rows, cols: B.cols, data: B.data.slice() }) }} color={CoreColorKey.Primary} aria-label="保存" title="保存">
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
                      </Button>
                      <Button size={SizeKey.SM} color={CoreColorKey.Base} aria-label="Markdown コピー" title="Markdown コピー" onClick={() => { const md = variableToMarkdown({ kind:'matrix', rows: B.rows, cols: B.cols, data: B.data }); if (md) navigator.clipboard?.writeText(md) }}>
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
                      </Button>
                    </div>
                  </div>
                </div>
              ) : rightKind === 'vector' ? (
                <>
                  <div style={{ display:'flex', alignItems:'center', gap: 8, marginBottom: 8 }}>
                    <VectorSizeControls length={y.data.length} onChange={(n)=> setY({ data: y.data.slice(0, n).concat(Array(Math.max(0, n - y.data.length)).fill(0)) })} />
                    <div style={{ display:'flex', gap: 4 }}>
                      <Button size={SizeKey.SM} onClick={() => { const name = window.prompt('保存する変数名')?.trim(); if (!name) return; upsert(name, { kind:'vector', length: y.data.length, data: y.data.slice() }) }} color={CoreColorKey.Primary} aria-label="保存" title="保存">
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
                      </Button>
                      <Button size={SizeKey.SM} color={CoreColorKey.Base} aria-label="Markdown コピー" title="Markdown コピー" onClick={() => { const md = variableToMarkdown({ kind:'vector', data: y.data }); if (md) navigator.clipboard?.writeText(md) }}>
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
                      </Button>
                    </div>
                  </div>
                  <VectorInput value={y} onChange={setY} orientation="col" length={y.data.length} />
                </>
              ) : (
                <input type="number" value={s2} onChange={(e) => setS2(Number(e.target.value||0))} />
              )}
            </div>
          </View>
        </div>

        {/* Result */}
        <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1 }}>
          <div style={{ display: 'flex', alignItems: 'center' }}>
            <Text weight={FontWeightKey.Medium}>結果</Text>
            <div style={{ marginLeft: 'auto' }}>
              <div style={{ display:'flex', gap: 6 }}>
                <Button onClick={saveResult} disabled={'error' in (compute as any)} color={CoreColorKey.Primary} aria-label="保存" title="保存">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
                </Button>
                {('error' in (compute as any)) ? null : (
                  <Button color={CoreColorKey.Base} aria-label="Markdown コピー" title="Markdown コピー" onClick={() => {
                    const r: any = compute as any
                    const payload = (r.rows != null && r.cols != null && Array.isArray(r.data)) ? { kind:'matrix', rows: r.rows, cols: r.cols, data: r.data } : (Array.isArray(r.data) ? { kind:'vector', data: r.data } : null)
                    if (!payload) return
                    const md = variableToMarkdown(payload as any)
                    if (md) navigator.clipboard?.writeText(md)
                  }}>
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
                  </Button>
                )}
              </div>
            </div>
          </div>
          <div style={{ marginTop: 8 }}>
            {'error' in (compute as any) ? (
              <Text>{(compute as any).error}</Text>
            ) : ((compute as any).rows != null ? (
              <div style={{ display:'flex', gap: 6, alignItems:'center', flexWrap:'wrap' }}>
                <MarkdownMath math={`${leftKind==='matrix' ? 'A' : leftKind==='vector' ? 'x' : String(s1)} \\${op==='mul' ? 'times' : op==='add' ? '+' : '-'} ${rightKind==='matrix' ? 'B' : rightKind==='vector' ? 'y' : String(s2)} =`} block={false} />
                <MatrixView rows={(compute as MatrixDTO).rows} cols={(compute as MatrixDTO).cols} data={(compute as MatrixDTO).data} />
              </div>
            ) : ((compute as any).data ? (
              <div style={{ display:'flex', gap: 6, alignItems:'center', flexWrap:'wrap' }}>
                <MarkdownMath math={`${leftKind==='matrix' ? 'A' : leftKind==='vector' ? 'x' : String(s1)} \\${op==='mul' ? 'times' : op==='add' ? '+' : '-'} ${rightKind==='matrix' ? 'B' : rightKind==='vector' ? 'y' : String(s2)} =`} block={false} />
                <VectorView orientation="col" values={(compute as VectorDTO).data} />
              </div>
            ) : (
              <Text>スカラー（保存未対応）: {(compute as any).toString?.() ?? JSON.stringify(compute)}</Text>
            )))}
          </div>
        </View>
      </div>
    </PageContainer>
  )
}
