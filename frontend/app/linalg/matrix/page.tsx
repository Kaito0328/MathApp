"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import Row from '../../../src/baseComponents/layout/Row'
import UnaryLayout from '../../../src/components/features/layout/UnaryLayout'
import { MatrixInput } from '../../../src/widgets/input'
import { MatrixView } from '../../../src/widgets/display'
import MarkdownMath from '../../../src/widgets/display/MarkdownMath'
import { VariablePicker } from '../../../src/components/features/variables/VariablePicker'
import { useVariableStore } from '../../../src/state/VariableStore'
import { variableToMarkdown } from '../../../src/components/features/variables/parts/VariableUtils'
import { formatNumberForMath, formatVectorMarkdown } from '../../../src/utils/format/markdown'
import { matMul as jsMatMul, transpose as jsTranspose, diagFrom as jsDiagFrom } from '../../../src/wasm/linalg'
import { inverse, cholesky, pinv, qr as wasmQR, svd as wasmSVD, eigen as wasmEigen, determinant as wasmDet, rank as wasmRank, frobeniusNorm as wasmFrob, expm as wasmExpm } from '../../../src/wasm/linalg'
import Document from '../../../src/components/features/document/Document'
import Modal from '../../../src/components/ui/Modal'
import CodeViewer from '../../../src/components/features/source/CodeViewer'
import SourceBlock from '../../../src/components/features/source/SourceBlock'

type MatrixDTO = { rows: number; cols: number; data: number[] }
type Unary = 'inverse' | 'pinv' | 'cholesky' | 'qr' | 'svd' | 'eigen' | 'det' | 'rank' | 'normF' | 'expm'

export default function MatrixOps() {
  const { get } = useVariableStore()
  const [A, setA] = React.useState<MatrixDTO>({ rows: 3, cols: 3, data: [1,0,0, 0,1,0, 0,0,1] })
  const [op, setOp] = React.useState<Unary>('inverse')
  const operations: { label: string; value: Unary }[] = [
    { label: '逆行列', value: 'inverse' },
    { label: '疑似逆行列', value: 'pinv' },
    { label: 'コレスキー分解', value: 'cholesky' },
    { label: 'QR 分解', value: 'qr' },
    { label: 'SVD 分解', value: 'svd' },
    { label: '固有値分解', value: 'eigen' },
    { label: '行列式', value: 'det' },
    { label: 'ランク', value: 'rank' },
    { label: 'フロベニウスノルム', value: 'normF' },
    { label: '行列指数関数', value: 'expm' },
  ]

  const [compute, setCompute] = React.useState<any>({})
  const [checks, setChecks] = React.useState<any>({})
  const [precision, setPrecision] = React.useState<number>(4)
  const [refresh, setRefresh] = React.useState<number>(0)
  const [dirty, setDirty] = React.useState<boolean>(false)
  const [computedRefresh, setComputedRefresh] = React.useState<number>(0)
  const [openSrc, setOpenSrc] = React.useState(false)
  const sourcePath = React.useMemo(() => (
    op === 'cholesky' ? 'crates/linalg/src/matrix/numerical/cholesky/mod.rs'
    : op === 'eigen' ? 'crates/linalg/src/matrix/numerical/eigen/mod.rs'
    : op === 'expm' ? 'crates/linalg/src/matrix/numerical/exp.rs'
    : op === 'det' || op === 'rank' || op === 'qr' ? 'crates/linalg/src/matrix/algebra/lu.rs'
    : op === 'pinv' || op === 'svd' ? 'crates/linalg/src/matrix/numerical/eigen/mod.rs'
    : /* inverse など */ 'crates/linalg/src/matrix/algebra/field.rs'
  ), [op])

  const isHeavy = (o: Unary) => (['qr','svd','eigen','expm'] as Unary[]).includes(o)

  React.useEffect(() => {
    if (isHeavy(op)) setDirty(true)
  }, [A, op])
  React.useEffect(() => {
    if (isHeavy(op)) setDirty(true)
    else setDirty(false)
  }, [op])
  React.useEffect(() => {
    setCompute({})
    setChecks({})
  }, [op])

  React.useEffect(() => {
    let cancelled = false
    const run = async () => {
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
      if (!cancelled) setCompute({ error: '未実装（今後実装: LU / RREF）' })
    }
    run()
    return () => { cancelled = true }
  }, [A, op, refresh, computedRefresh])

  return (
    <PageContainer title="行列の単項演算・分解" stickyHeader>
      <UnaryLayout
        operations={operations}
        operation={op}
        onOperationChange={(v)=> setOp(v as Unary)}
        accuracy={precision}
        onAccuracyChange={(n)=> setPrecision(Math.max(0, Math.min(10, Math.floor(Number(n)||0))))}
        accuracy_able
        onCalc={(['qr','svd','eigen','expm'] as Unary[]).includes(op) ? (()=> setRefresh((n)=>n+1)) : undefined}
        calc_button_able={(['qr','svd','eigen','expm'] as Unary[]).includes(op) && dirty}
        operation_left={
          <VariablePicker placeholder="変数から代入" allowedKinds={['matrix']} onPick={(name) => {
            const v = get(name) as any
            if (v && v.kind === 'matrix') setA({ rows: v.rows, cols: v.cols, data: v.data.slice() })
          }} />
        }
        operation_right={<Row />}
        operand={<MatrixInput value={A} onChange={setA} rows={A.rows} cols={A.cols} />}
        operand_copyContent={variableToMarkdown({ kind:'matrix', rows: A.rows, cols: A.cols, data: A.data })}
        operand_buildSavePayload={()=> ({ kind:'matrix', rows: A.rows, cols: A.cols, data: A.data.slice() })}
        operand_afterSave={()=>{}}
        result={
          <div style={{ display: 'grid', gap: 12 }}>
            {compute && compute.error ? (
              <span style={{ color:'crimson' }}>{compute.error}</span>
            ) : (
              <>
                {typeof compute.det !== 'undefined' && (
                  <div>
                    {typeof compute.det === 'number' ? (
                      <MarkdownMath math={`\\det(A) = ${formatNumberForMath(Number(compute.det), precision)}`} />
                    ) : (
                      <span>{String((compute.det as any)?.error || compute.det)}</span>
                    )}
                  </div>
                )}
                {typeof compute.rank !== 'undefined' && (
                  <div>
                    {typeof compute.rank === 'number' ? (
                      <MarkdownMath math={`\\operatorname{rank}(A) = ${formatNumberForMath(Number(compute.rank), precision)}`} />
                    ) : (
                      <span>{String((compute.rank as any)?.error || compute.rank)}</span>
                    )}
                  </div>
                )}
                {typeof compute.frob !== 'undefined' && (
                  <div>
                    {typeof compute.frob === 'number' ? (
                      <MarkdownMath math={`\\lVert A \\rVert_F = ${formatNumberForMath(Number(compute.frob), precision)}`} />
                    ) : (
                      <span>{String((compute.frob as any)?.error || compute.frob)}</span>
                    )}
                  </div>
                )}
                {compute.P && (<div style={{display:'flex',gap:6,alignItems:'center'}}><MarkdownMath math={'P ='} block={false} /> <MatrixView rows={compute.P.rows} cols={compute.P.cols} data={compute.P.data} precision={precision} block={false} /></div>)}
                {compute.L && (<div style={{display:'flex',gap:6,alignItems:'center'}}><MarkdownMath math={'L ='} block={false} /> <MatrixView rows={compute.L.rows} cols={compute.L.cols} data={compute.L.data} precision={precision} block={false} /></div>)}
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
              </>
            )}
          </div>
        }
        verification={
          <div style={{ display: 'grid', gap: 12 }}>
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
        }
        document={
          <div style={{ display:'grid', gap:8, marginTop: 4 }}>
            <div style={{ textAlign:'right' }}>
              <button onClick={()=> setOpenSrc(true)}>ソースを表示</button>
            </div>
            <Document
              docPath={
                op === 'qr' ? 'notes/linalg/matrix_qr.md'
                : op === 'svd' ? 'notes/linalg/matrix_svd.md'
                : op === 'eigen' ? 'notes/linalg/matrix_eigendecomp.md'
                : op === 'pinv' ? 'notes/linalg/matrix_pinv.md'
                : op === 'expm' ? 'notes/linalg/matrix_expm.md'
                : 'notes/linalg/overview.md'
              }
              rustUrl={
                // Placeholder link; adjust to actual repo path when ready
                'https://github.com/your-org/your-repo/tree/main/backend/crates/linalg'
              }
            />
            <Modal open={openSrc} onClose={()=> setOpenSrc(false)} title="backend/crates/linalg ソース">
              <CodeViewer rootRelPath={''} initialPath={'crates/linalg/src/lib.rs'} />
            </Modal>
          </div>
        }
        documentTitle="ドキュメント"
      />
      <div style={{ marginTop: 12 }}>
        <SourceBlock title="ソースコード（linalg）" path={sourcePath} />
      </div>
    </PageContainer>
  )
}
