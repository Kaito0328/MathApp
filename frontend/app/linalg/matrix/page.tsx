"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/patterns/PageContainer'
import { View } from '../../../src/baseComponents/foundation/View'
import { Text } from '../../../src/baseComponents/foundation/Text'
import { Button } from '../../../src/baseComponents/patterns/Button'
import { CoreColorKey, SizeKey, FontWeightKey } from '../../../src/design/tokens'
import { MatrixInput } from '../../../src/widgets/input'
import { MatrixView } from '../../../src/widgets/display'
import { MatrixSizeControls } from '../../../src/widgets/input'
import { matMul as jsMatMul, transpose as jsTranspose, diagFrom as jsDiagFrom } from '../../../src/wasm/linalg'
import MarkdownMath from '../../../src/widgets/display/MarkdownMath'
import { formatNumberForMath, formatVectorMarkdown } from '../../../src/utils/format/markdown'
import { useVariableStore } from '../../../src/state/VariableStore'
import { VariablePicker } from '../../../src/components/variables/VariablePicker'
import { variableToMarkdown } from '../../../src/components/variables/parts/VariableUtils'
import { inverse, cholesky, pinv, qr as wasmQR, svd as wasmSVD, eigen as wasmEigen, determinant as wasmDet, rank as wasmRank, frobeniusNorm as wasmFrob, expm as wasmExpm } from '../../../src/wasm/linalg'

type MatrixDTO = { rows: number; cols: number; data: number[] }
type Unary = 'inverse' | 'pinv' | 'cholesky' | 'qr' | 'svd' | 'eigen' | 'det' | 'rank' | 'normF' | 'expm'

export default function MatrixOps() {
  const { get, upsert } = useVariableStore()
  const [A, setA] = React.useState<MatrixDTO>({ rows: 3, cols: 3, data: [1,0,0, 0,1,0, 0,0,1] })
  const [op, setOp] = React.useState<Unary>('inverse')

  const [compute, setCompute] = React.useState<any>({})
  const [checks, setChecks] = React.useState<any>({})
  const [precision, setPrecision] = React.useState<number>(4)
  const [refresh, setRefresh] = React.useState<number>(0)
  const [dirty, setDirty] = React.useState<boolean>(false)
  const [computedRefresh, setComputedRefresh] = React.useState<number>(0)

  const isHeavy = (o: Unary) => (['qr','svd','eigen','expm'] as Unary[]).includes(o)

  // Heavy op: mark dirty when input changes; Light op: no dirty tracking
  React.useEffect(() => {
    if (isHeavy(op)) setDirty(true)
  }, [A, op])
  React.useEffect(() => {
    if (isHeavy(op)) setDirty(true)
    else setDirty(false)
  }, [op])
  // 演算変更時に前結果をクリア
  React.useEffect(() => {
    setCompute({})
    setChecks({})
    // heavy は dirty=true のまま、light はこの直後の計算で上書き
  }, [op])

  React.useEffect(() => {
    let cancelled = false
    const run = async () => {
      // For heavy ops, only run when user clicks the button (refresh changes)
      if (isHeavy(op)) {
        if (computedRefresh === refresh) return
      }
      if (op === 'inverse') {
        const r = await inverse(A)
        if (!cancelled) {
          setCompute(r)
          if (!(r as any)?.error && (r as any)?.rows) {
            const prod = jsMatMul(A, r as any)
            setChecks('error' in prod ? {} : { identity: prod })
          } else setChecks({})
        }
        if (isHeavy(op)) { setDirty(false); setComputedRefresh(refresh) }
        return
      }
  if (op === 'pinv') {
        const r = await pinv(A)
        if (!cancelled) {
          setCompute(r)
          if (!(r as any)?.error && (r as any)?.rows) {
            const AAp = jsMatMul(A, r as any)
            const recon = 'error' in AAp ? AAp : jsMatMul(AAp, A)
            setChecks('error' in recon ? {} : { recon })
          } else setChecks({})
        }
        if (isHeavy(op)) { setDirty(false); setComputedRefresh(refresh) }
  return
      }
      if (op === 'cholesky') {
        const r = await cholesky(A)
        if (!cancelled) {
          const payload = (r as any) && 'error' in (r as any) ? r : { L: r }
          setCompute(payload)
          if ((payload as any).L) {
            const L = (payload as any).L as MatrixDTO
            const Lt = jsTranspose(L)
            const recon = jsMatMul(L, Lt)
            setChecks('error' in recon ? {} : { recon })
          } else setChecks({})
        }
        if (isHeavy(op)) { setDirty(false); setComputedRefresh(refresh) }
        return
      }
  if (op === 'qr') {
        const r = await wasmQR(A)
        if (!cancelled) {
          setCompute(r)
          if (!(r as any)?.error && (r as any)?.Q && (r as any)?.R) {
            const recon = jsMatMul((r as any).Q, (r as any).R)
            setChecks('error' in recon ? {} : { recon })
          } else setChecks({})
        }
        if (isHeavy(op)) { setDirty(false); setComputedRefresh(refresh) }
        return
      }
      if (op === 'svd') {
        const r = await wasmSVD(A)
        if (!cancelled) {
          setCompute(r)
          if (!(r as any)?.error && (r as any)?.U && (r as any)?.S && (r as any)?.V) {
            const D = jsDiagFrom(((r as any).S.data as number[]))
            const UD = jsMatMul((r as any).U, D)
            const recon = 'error' in UD ? UD : jsMatMul(UD, jsTranspose((r as any).V))
            setChecks('error' in recon ? {} : { recon })
          } else setChecks({})
        }
        if (isHeavy(op)) { setDirty(false); setComputedRefresh(refresh) }
        return
      }
  if (op === 'eigen') {
        const r = await wasmEigen(A)
        if (!cancelled) {
          setCompute(r)
          if (!(r as any)?.error && (r as any)?.Lambda && (r as any)?.V) {
            const D = jsDiagFrom(((r as any).Lambda.data as number[]))
            const AV = jsMatMul(A, (r as any).V)
            const VD = jsMatMul((r as any).V, D)
            setChecks(('error' in AV || 'error' in VD) ? {} : { AV, VD })
          } else setChecks({})
        }
        if (isHeavy(op)) { setDirty(false); setComputedRefresh(refresh) }
        return
      }
  if (op === 'det') { const r = await wasmDet(A); if (!cancelled) { setCompute({ det: r }); setChecks({}) } return }
  if (op === 'rank') { const r = await wasmRank(A); if (!cancelled) { setCompute({ rank: r }); setChecks({}) } return }
  if (op === 'normF') { const r = await wasmFrob(A); if (!cancelled) { setCompute({ frob: r }); setChecks({}) } return }
  if (op === 'expm') { const r = await wasmExpm(A); if (!cancelled) { setCompute(r); setChecks({}) } return }
      // TODO: LU, RREF は wasm 側 API 探索後に接続
      if (!cancelled) setCompute({ error: '未実装（今後実装: LU / RREF）' })
    }
    run()
    return () => { cancelled = true }
  }, [A, op, refresh, computedRefresh])

  // save logic moved into inline button handler below

  return (
    <PageContainer title="行列の単項演算・分解" stickyHeader>
      <div style={{ display: 'grid', gap: 12 }}>
        {/* Controls */}
        <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1 }}>
          <div style={{ display: 'grid', gap: 8 }}>
            <div style={{ display: 'flex', gap: 8, alignItems: 'center', flexWrap: 'wrap' }}>
              <VariablePicker placeholder="変数から代入" allowedKinds={['matrix']} onPick={(name) => {
                const v = get(name) as any
                if (v && v.kind === 'matrix') setA({ rows: v.rows, cols: v.cols, data: v.data.slice() })
              }} />
              <select value={op} onChange={(e) => setOp(e.target.value as Unary)}>
                <option value="inverse">逆行列</option>
                <option value="pinv">疑似逆行列</option>
                <option value="cholesky">コレスキー分解</option>
                <option value="qr">QR 分解</option>
                <option value="svd">SVD 分解</option>
                <option value="eigen">固有値分解</option>
                <option value="det">行列式</option>
                <option value="rank">ランク</option>
                <option value="normF">フロベニウスノルム</option>
                <option value="expm">行列指数関数</option>
              </select>
              {/* 再計算ボタンは重い演算のみ表示 */}
              {(['qr','svd','eigen','expm'] as Unary[]).includes(op) && (
                <Button onClick={() => setRefresh((n) => n + 1)} color={dirty ? CoreColorKey.Primary : CoreColorKey.Secondary} disabled={!dirty} aria-label="計算">
                  <span style={{ display:'inline-flex', gap:4, alignItems:'center' }}>
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><polygon points="5 3 19 12 5 21 5 3"/></svg>
                    計算
                  </span>
                </Button>
              )}
              <label>精度
                <input type="number" min={0} max={10} value={precision} onChange={(e) => setPrecision(Math.max(0, Math.min(10, Math.floor(Number(e.target.value)||0))))} style={{ width: 72 }} />
              </label>
              
            </div>
          </div>
        </View>

        {/* Input */}
        <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1 }}>
          <Text weight={FontWeightKey.Medium}>入力行列</Text>
          <div style={{ marginTop: 8, display:'grid', gridTemplateColumns:'1fr auto', gap: 8, alignItems: 'start' }}>
            <div>
              <div style={{ display: 'flex', gap: 12, alignItems: 'center', marginBottom: 8 }}>
                <MatrixSizeControls rows={A.rows} cols={A.cols} onChange={(r, c) => {
                  const size = r * c
                  const next = A.data.slice(0, size)
                  if (next.length < size) next.push(...Array(size - next.length).fill(0))
                  setA({ rows: r, cols: c, data: next })
                }} />
              </div>
              <MatrixInput value={A} onChange={setA} rows={A.rows} cols={A.cols} />
            </div>
            <div style={{ display:'flex', gap: 4 }}>
              <Button size={SizeKey.SM} onClick={() => { const name = window.prompt('保存する変数名')?.trim(); if (!name) return; upsert(name, { kind:'matrix', rows: A.rows, cols: A.cols, data: A.data.slice() }) }} color={CoreColorKey.Primary} aria-label="保存" title="保存">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
              </Button>
              <Button size={SizeKey.SM} color={CoreColorKey.Base} aria-label="Markdown コピー" title="Markdown コピー" onClick={() => { const md = variableToMarkdown({ kind:'matrix', rows: A.rows, cols: A.cols, data: A.data }); if (md) navigator.clipboard?.writeText(md) }}>
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
              </Button>
            </div>
          </div>
        </View>

        {/* Result */}
        <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1 }}>
          <Text weight={FontWeightKey.Medium}>結果</Text>
          <div style={{ marginTop: 8, display: 'grid', gap: 12 }}>
            {compute && compute.error ? (
              <Text>{compute.error}</Text>
            ) : (
              <>
                {/* スカラー系の結果表示（$...$ を二重にしない） */}
                {typeof compute.det !== 'undefined' && (
                  <div>
                    {typeof compute.det === 'number' ? (
                      <MarkdownMath math={`\\det(A) = ${formatNumberForMath(Number(compute.det), precision)}`} />
                    ) : (
                      <Text>{String((compute.det as any)?.error || compute.det)}</Text>
                    )}
                  </div>
                )}
                {typeof compute.rank !== 'undefined' && (
                  <div>
                    {typeof compute.rank === 'number' ? (
                      <MarkdownMath math={`\\operatorname{rank}(A) = ${formatNumberForMath(Number(compute.rank), precision)}`} />
                    ) : (
                      <Text>{String((compute.rank as any)?.error || compute.rank)}</Text>
                    )}
                  </div>
                )}
                {typeof compute.frob !== 'undefined' && (
                  <div>
                    {typeof compute.frob === 'number' ? (
                      <MarkdownMath math={`\\lVert A \\rVert_F = ${formatNumberForMath(Number(compute.frob), precision)}`} />
                    ) : (
                      <Text>{String((compute.frob as any)?.error || compute.frob)}</Text>
                    )}
                  </div>
                )}
                {compute.P && (<div style={{display:'flex',gap:6,alignItems:'center'}}><MarkdownMath math={'P ='} block={false} /> <MatrixView rows={compute.P.rows} cols={compute.P.cols} data={compute.P.data} precision={precision} block={false} /></div>)}
                {compute.L && (<div style={{display:'flex',gap:6,alignItems:'center'}}><MarkdownMath math={'L ='} block={false} /> <MatrixView rows={compute.L.rows} cols={compute.L.cols} data={compute.L.data} precision={precision} block={false} /></div>)}
                {/* SVDがある場合はUを二重表示しない */}
                {compute.U && !(compute.S && compute.V) && (
                  <div style={{display:'flex',gap:6,alignItems:'center'}}><MarkdownMath math={'U ='} block={false} /> <MatrixView rows={compute.U.rows} cols={compute.U.cols} data={compute.U.data} precision={precision} block={false} /></div>
                )}
                {compute.Q && (<div style={{display:'flex',gap:6,alignItems:'center'}}><MarkdownMath math={'Q ='} block={false} /> <MatrixView rows={compute.Q.rows} cols={compute.Q.cols} data={compute.Q.data} precision={precision} block={false} /></div>)}
                {compute.R && (<div style={{display:'flex',gap:6,alignItems:'center'}}><MarkdownMath math={'R ='} block={false} /> <MatrixView rows={compute.R.rows} cols={compute.R.cols} data={compute.R.data} precision={precision} block={false} /></div>)}
                {compute.U && compute.S && compute.V && (
                  <>
                    <div style={{display:'flex',gap:6,alignItems:'center'}}><MarkdownMath math={'U ='} block={false} /> <MatrixView rows={compute.U.rows} cols={compute.U.cols} data={compute.U.data} precision={precision} block={false} /></div>
                    <div style={{display:'flex',gap:6,alignItems:'center',flexWrap:'wrap'}}>
                      <MarkdownMath math={'\\Sigma ='} block={false} />
                      <MarkdownMath math={`\\begin{bmatrix} ${(compute.S.data as number[]).map((x:number)=>formatNumberForMath(Number(x), precision)).join(' \\\\ ')} \\end{bmatrix}`} block={false} />
                    </div>
                    <div style={{display:'flex',gap:6,alignItems:'center'}}><MarkdownMath math={'V ='} block={false} /> <MatrixView rows={compute.V.rows} cols={compute.V.cols} data={compute.V.data} precision={precision} block={false} /></div>
                  </>
                )}
                {compute.Lambda && (
                  <div style={{display:'flex',gap:6,alignItems:'center'}}>
                    <MarkdownMath math={'\\Lambda ='} block={false} />
                    <MarkdownMath math={formatVectorMarkdown({ data: compute.Lambda.data as number[] }, { orientation: 'col', precision, paren: true })} block={false} />
                  </div>
                )}
                {compute.V && !compute.U && !compute.R && (<div style={{display:'flex',gap:6,alignItems:'center'}}><MarkdownMath math={'V ='} block={false} /> <MatrixView rows={compute.V.rows} cols={compute.V.cols} data={compute.V.data} precision={precision} block={false} /></div>)}
                {/* op別のラベル付き表示 */}
                {op === 'inverse' && compute.rows && (
                  <div style={{display:'flex',gap:6,alignItems:'center'}}>
                    <MarkdownMath math={'A^{-1} ='} block={false} />
                    <MatrixView rows={compute.rows} cols={compute.cols} data={compute.data} precision={precision} block={false} />
                  </div>
                )}
                {op === 'pinv' && compute.rows && (
                  <div style={{display:'flex',gap:6,alignItems:'center'}}>
                    <MarkdownMath math={'A^{+} ='} block={false} />
                    <MatrixView rows={compute.rows} cols={compute.cols} data={compute.data} precision={precision} block={false} />
                  </div>
                )}
                {op === 'expm' && compute.rows && (
                  <div style={{display:'flex',gap:6,alignItems:'center'}}>
                    <MarkdownMath math={'e^{A} ='} block={false} />
                    <MatrixView rows={compute.rows} cols={compute.cols} data={compute.data} precision={precision} block={false} />
                  </div>
                )}
                {!(op === 'inverse' || op === 'pinv' || op === 'expm') && compute.rows && compute.cols && compute.data && (
                  <div style={{display:'flex',gap:6,alignItems:'center'}}>
                    <MatrixView rows={compute.rows} cols={compute.cols} data={compute.data} precision={precision} block={false} />
                  </div>
                )}
                {/* 単一の行列結果のみ保存ボタンを表示 */}
                {(() => {
                  let target: MatrixDTO | null = null
                  if (compute && compute.rows && compute.cols && compute.data) target = compute as MatrixDTO
                  else if (compute && compute.L && compute.L.rows) target = compute.L as MatrixDTO
                  // その他（Q/R/U/V等が単独の場合のみ保存したい時は条件追加）
                  if (!target) return null
                  return (
                    <div style={{ display:'flex', gap: 6 }}>
                      <Button onClick={() => {
                        const name = window.prompt('保存する変数名を入力')?.trim()
                        if (!name) return
                        const M = target as MatrixDTO
                        upsert(name, { kind: 'matrix', rows: M.rows, cols: M.cols, data: M.data })
                      }} color={CoreColorKey.Primary} aria-label="保存" title="保存">
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
                      </Button>
                      <Button color={CoreColorKey.Base} aria-label="Markdown コピー" title="Markdown コピー" onClick={() => {
                        const md = variableToMarkdown({ kind:'matrix', rows: (target as any).rows, cols: (target as any).cols, data: (target as any).data })
                        if (md) navigator.clipboard?.writeText(md)
                      }}>
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>
                      </Button>
                    </div>
                  )
                })()}
              </>
            )}
          </div>
        </View>

        {/* 検証ブロック */}
        <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1 }}>
          <Text weight={FontWeightKey.Medium}>検証</Text>
          <div style={{ marginTop: 8, display: 'grid', gap: 12 }}>
            {op === 'inverse' && checks.identity && !checks.identity.error && (
              <div>
                <MarkdownMath math={'A \\; A^{-1}'} />
                <MatrixView rows={checks.identity.rows} cols={checks.identity.cols} data={(checks.identity as any).data} precision={precision} />
              </div>
            )}
            {op === 'pinv' && checks.recon && !checks.recon.error && (
              <div>
                <MarkdownMath math={'A \\; A^{+} \\; A'} />
                <MatrixView rows={checks.recon.rows} cols={checks.recon.cols} data={(checks.recon as any).data} precision={precision} />
              </div>
            )}
            {op === 'cholesky' && checks.recon && !checks.recon.error && (
              <div style={{ display: 'grid', gap: 8 }}>
                <div style={{display:'flex',gap:6,alignItems:'center',flexWrap:'wrap'}}>
                  <MarkdownMath math={'L L^{T} ='} block={false} />
                  <MatrixView rows={checks.recon.rows} cols={checks.recon.cols} data={(checks.recon as any).data} precision={precision} block={false} />
                </div>
                <div style={{display:'flex',gap:6,alignItems:'center',flexWrap:'wrap'}}>
                  <MarkdownMath math={'A ='} block={false} />
                  <MatrixView rows={A.rows} cols={A.cols} data={A.data} precision={precision} block={false} />
                </div>
              </div>
            )}
            {op === 'qr' && checks.recon && !checks.recon.error && (
              <div style={{ display: 'grid', gap: 8 }}>
                <div style={{display:'flex',gap:6,alignItems:'center',flexWrap:'wrap'}}>
                  <MarkdownMath math={'Q R ='} block={false} />
                  <MatrixView rows={checks.recon.rows} cols={checks.recon.cols} data={(checks.recon as any).data} precision={precision} block={false} />
                </div>
                <div style={{display:'flex',gap:6,alignItems:'center',flexWrap:'wrap'}}>
                  <MarkdownMath math={'A ='} block={false} />
                  <MatrixView rows={A.rows} cols={A.cols} data={A.data} precision={precision} block={false} />
                </div>
              </div>
            )}
            {op === 'svd' && checks.recon && !checks.recon.error && (
              <div style={{ display: 'grid', gap: 8 }}>
                <div style={{display:'flex',gap:6,alignItems:'center',flexWrap:'wrap'}}>
                  <MarkdownMath math={'U \\Sigma V^{T} ='} block={false} />
                  <MatrixView rows={checks.recon.rows} cols={checks.recon.cols} data={(checks.recon as any).data} precision={precision} block={false} />
                </div>
                <div style={{display:'flex',gap:6,alignItems:'center',flexWrap:'wrap'}}>
                  <MarkdownMath math={'A ='} block={false} />
                  <MatrixView rows={A.rows} cols={A.cols} data={A.data} precision={precision} block={false} />
                </div>
              </div>
            )}
            {op === 'eigen' && checks.AV && checks.VD && !checks.AV.error && !checks.VD.error && (
              <div style={{ display: 'grid', gap: 8 }}>
                <div style={{display:'flex',gap:6,alignItems:'center',flexWrap:'wrap'}}>
                  <MarkdownMath math={'A V ='} block={false} />
                  <MatrixView rows={checks.AV.rows} cols={checks.AV.cols} data={(checks.AV as any).data} precision={precision} block={false} />
                </div>
                <div style={{display:'flex',gap:6,alignItems:'center',flexWrap:'wrap'}}>
                  <MarkdownMath math={'V \\Lambda ='} block={false} />
                  <MatrixView rows={checks.VD.rows} cols={checks.VD.cols} data={(checks.VD as any).data} precision={precision} block={false} />
                </div>
              </div>
            )}
          </div>
        </View>
      </div>
    </PageContainer>
  )
}
