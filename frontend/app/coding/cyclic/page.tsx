"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import SectionPanelWithTitle from '../../../src/components/composites/panels/SectionPanelWithTitle'
import OperationBaseBlock from '../../../src/components/features/operations/OperationBaseBlock'
import { VariablePicker } from '../../../src/components/features/variables/VariablePicker'
import { useVariableStore } from '../../../src/state/VariableStore'
import CodeCharacteristics from '../../../src/components/features/coding/CodeCharacteristics'
import { Button } from '../../../src/baseComponents/controls/Button'
import { PolynomialInput } from '../../../src/widgets/input/PolynomialInput'
import type { Polynomial } from '../../../src/widgets/dto/polynomial'

type InputMode = 'text' | 'binary'

function textToBits(s: string): number[] {
  const bytes = new TextEncoder().encode(s)
  const bits: number[] = []
  for (const b of bytes) for (let i = 7; i >= 0; i--) bits.push((b >> i) & 1)
  return bits
}
function bitsToText(bits: number[]): string {
  const out: number[] = []
  for (let i = 0; i + 7 < bits.length; i += 8) {
    let b = 0
    for (let j = 0; j < 8; j++) b = (b << 1) | (bits[i + j] & 1)
    out.push(b)
  }
  return new TextDecoder().decode(new Uint8Array(out))
}
function chunk<T>(arr: T[], size: number): T[][] {
  const out: T[][] = []
  for (let i = 0; i < arr.length; i += size) out.push(arr.slice(i, i + size))
  return out
}

function normalizeBinaryCoeffs(p: Polynomial): Uint8Array {
  // GF(2) 多項式: 係数は 0/1 に丸める
  const a = p.coeffs.map((x)=> (Math.round(x) & 1) >>> 0)
  // 末尾の余分な 0 は削除（ただし定数 0 多項式は長さ 1 を保持）
  let end = a.length
  while (end > 1 && a[end-1] === 0) end--
  return new Uint8Array(a.slice(0, end))
}

export default function CyclicCodePage() {
  const { get } = useVariableStore()
  const [mode, setMode] = React.useState<InputMode>('text')
  const [text, setText] = React.useState<string>('HELLO')
  const [binary, setBinary] = React.useState<string>('')
  const [n, setN] = React.useState<number>(7)
  const [gPoly, setGPoly] = React.useState<Polynomial>({ coeffs: [1,1,0,1] }) // 1 + x + x^3
  const [k, setK] = React.useState<number | null>(null)
  const [encoded, setEncoded] = React.useState<Uint8Array | null>(null)
  const [noisy, setNoisy] = React.useState<Uint8Array | null>(null)
  const [decoded, setDecoded] = React.useState<Uint8Array | null>(null)
  const [err, setErr] = React.useState<string>('')
  const [randomErrors, setRandomErrors] = React.useState<number>(0)

  const getMsgBits = React.useCallback((): number[] => {
    return mode==='text' ? textToBits(text) : (binary||'').split('').filter(c=>c==='0'||c==='1').map(c=>Number(c))
  }, [mode, text, binary])

  const onEncode = async () => {
    setErr(''); setDecoded(null)
    try {
  const g = Array.from(normalizeBinaryCoeffs(gPoly))
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      const cyc = new wasm.CyclicCode(n, new Uint8Array(g))
      const kEff: number = cyc.k()
      setK(kEff)
      const bits = getMsgBits()
      const blocks = chunk(bits, kEff)
      if (blocks.length && blocks[blocks.length-1].length < kEff) {
        while (blocks[blocks.length-1].length < kEff) blocks[blocks.length-1].push(0)
      }
      const out: number[] = []
      for (const b of blocks) {
        const cw: Uint8Array = cyc.encode(new Uint8Array(b))
        out.push(...Array.from(cw))
      }
      const code = new Uint8Array(out)
      setEncoded(code)
      setNoisy(code)
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  

  const onInjectRandomErrors = () => {
    if (!encoded) return
    const nlen = encoded.length
    const v = new Uint8Array(encoded)
    const m = Math.max(0, Math.min(randomErrors, nlen))
    const picked = new Set<number>()
    while (picked.size < m) picked.add(Math.floor(Math.random() * nlen))
    for (const i of picked) v[i] ^= 1
    setNoisy(v)
  }

  const onDecode = async () => {
    setErr('')
    try {
      if (!noisy) return
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
  const cyc = new wasm.CyclicCode(n, normalizeBinaryCoeffs(gPoly))
      // 受信語長は n の倍数である必要がある
      const r = new Uint8Array(noisy)
      if (r.length % n !== 0) {
        throw new Error(`受信語の長さ (${r.length}) は n (${n}) の倍数である必要があります`)
      }
      const out: number[] = []
      for (let i = 0; i < r.length; i += n) {
        const block = r.subarray(i, i + n)
        const corr: Uint8Array = cyc.decodeLUT(block)
        out.push(...Array.from(corr))
      }
      setDecoded(new Uint8Array(out))
    } catch (e:any) {
      setErr(e?.message || String(e))
    }
  }

  const decodedText = React.useMemo(() => decoded ? bitsToText(Array.from(decoded)) : '' , [decoded])

  return (
    <PageContainer title="Cyclic Code (GF(2))" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        {/* ページ全体の表現トグル */}
        <div style={{ display:'flex', gap:12, alignItems:'center' }}>
          <label><input type="radio" checked={mode==='text'} onChange={()=> setMode('text')} /> テキスト</label>
          <label><input type="radio" checked={mode==='binary'} onChange={()=> setMode('binary')} /> 2進</label>
        </div>

        <OperationBaseBlock
          left={
            <VariablePicker
              placeholder="変数から代入"
              allowedKinds={[ 'vector' ]}
              onPick={(name)=>{
                const v:any = get(name)
                if (v?.kind==='vector' && Array.isArray(v.data)) setBinary(v.data.map((x:number)=> String((x|0)&1)).join(''))
              }}
            />
          }
          center={<Button onClick={onEncode}>符号化</Button>}
          right={null}
        />

        <SectionPanelWithTitle title="エンコード入力">
          <div style={{ display:'grid', gap:8 }}>
            <div style={{ display:'flex', gap:12, alignItems:'center', flexWrap:'wrap' }}>
              <label>n: <input type="number" value={n} onChange={(e)=> setN(Math.max(1, Math.floor(Number(e.target.value)||1)))} style={{ width: 120 }} /></label>
              <div>
                <div style={{ fontSize:12, opacity:0.8, marginBottom:6 }}>生成多項式 g（GF(2) 係数, 低次→高次）</div>
                <PolynomialInput value={gPoly} onChange={setGPoly} />
              </div>
            </div>
            {mode==='text' ? (
              <textarea value={text} onChange={(e)=> setText(e.target.value)} rows={3} style={{ width: '100%', boxSizing:'border-box' }} />
            ) : (
              <textarea value={binary} onChange={(e)=> setBinary(e.target.value)} rows={3} style={{ width: '100%', boxSizing:'border-box' }} placeholder="例) 1011001110" />
            )}
            {err && <div style={{ color:'crimson' }}>{err}</div>}
          </div>
        </SectionPanelWithTitle>
        {/* 符号語表示 */}
        <SectionPanelWithTitle title="符号語（2進）">
          <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>
            {encoded ? Array.from(encoded).map((b, i)=> ((i>0 && k!=null && i% n===0) ? ' ' : '') + String(b)).join('') : '-'}
          </div>
        </SectionPanelWithTitle>

        <OperationBaseBlock
          left={
            <VariablePicker
              placeholder="変数から代入"
              allowedKinds={[ 'vector' ]}
              onPick={(name)=>{
                const v:any = get(name)
                if (v?.kind==='vector' && Array.isArray(v.data)) setNoisy(new Uint8Array(v.data.map((x:number)=> (x|0)&1)))
              }}
            />
          }
          center={<Button onClick={onDecode} disabled={!noisy}>復号</Button>}
          right={
            <>
              <label>誤り個数 <input type="number" value={randomErrors} onChange={(e)=> setRandomErrors(Math.max(0, Math.floor(Number(e.target.value)||0)))} style={{ width: 100 }} /></label>
              <Button onClick={onInjectRandomErrors} disabled={!encoded}>ランダム誤り</Button>
            </>
          }
        />

        <div style={{ opacity:0.75, fontSize:12 }}>
          注) 既定の復号は内部で H を構成し、シンドロームLUTで訂正します（H の手入力は不要）。
        </div>

        {/* 受信語入力（2進） */}
        <SectionPanelWithTitle title="受信語 2進">
          <div style={{ display:'grid', gap:8 }}>
            <textarea
              value={noisy ? Array.from(noisy).join('') : ''}
              onChange={(e)=> setNoisy(new Uint8Array((e.target.value||'').split('').filter(c=>c==='0'||c==='1').map(c=>Number(c))))}
              rows={3}
              style={{ width:'100%', boxSizing:'border-box' }}
            />
          </div>
        </SectionPanelWithTitle>

        {/* H 行列を使った復号は廃止（通常運用で不要なため） */}

        <SectionPanelWithTitle title="復号結果（訂正後コード語）">
          <div style={{ display:'grid', gap:8 }}>
            <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{decoded ? Array.from(decoded).join('') : '-'}</div>
            {mode==='text' && (
              <>
                <div>復号（テキスト解釈）:</div>
                <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{decoded ? decodedText : '-'}</div>
              </>
            )}
          </div>
        </SectionPanelWithTitle>

        <SectionPanelWithTitle title="符号の特性">
          <CodeCharacteristics lines={[
            `n = ${n}`,
            `k = ${k ?? '-'}`,
            `d_{\\min} = -`,
            `t = -`,
            `R = \\tfrac{k}{n} = ${k!=null ? (k/n).toFixed(3) : '-'}`,
          ]} />
        </SectionPanelWithTitle>
      </div>
    </PageContainer>
  )
}
