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

// (旧) CSV 0/1 入力は廃止

export default function BCHCodePage() {
  const { get } = useVariableStore()
  const [mode, setMode] = React.useState<InputMode>('text')
  const [text, setText] = React.useState<string>('HELLO')
  const [binary, setBinary] = React.useState<string>('')
  // 新仕様: m, t を指定（n = 2^m - 1 は内部で決定）
  const [m, setM] = React.useState<number>(4)
  const [tDesign, setTDesign] = React.useState<number>(2)
  // 高度な設定: 原始多項式（現状 API 未対応のため UI のみ）
  const [px, setPx] = React.useState<Polynomial | null>(null)
  const [k, setK] = React.useState<number | null>(null)
  const [t, setT] = React.useState<number | null>(null)
  const [encoded, setEncoded] = React.useState<Uint8Array | null>(null)
  const [noisy, setNoisy] = React.useState<Uint8Array | null>(null)
  const [decoded, setDecoded] = React.useState<Uint8Array | null>(null)
  const [err, setErr] = React.useState<string>('')
  const [bitFlipIdx, setBitFlipIdx] = React.useState<number | null>(null)
  const [nEff, setNEff] = React.useState<number>(15)

  const getMsgBits = React.useCallback((): number[] => {
    return mode==='text' ? textToBits(text) : (binary||'').split('').filter(c=>c==='0'||c==='1').map(c=>Number(c))
  }, [mode, text, binary])

  const onEncode = async () => {
    setErr(''); setDecoded(null)
    try {
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
  // 原始多項式は現状 API で指定不可のため、標準の狭義BCHを自動生成
  const bch = wasm.BCH.newAuto(Math.max(2, Math.floor(m||2)), Math.max(1, Math.floor(tDesign||1)))
      const kEff: number = bch.k()
      const tEff: number = bch.t()
  const nAuto: number = bch.n()
  setK(kEff); setT(tEff); setNEff(nAuto)
      const bits = getMsgBits()
      const blocks = chunk(bits, kEff)
      if (blocks.length && blocks[blocks.length-1].length < kEff) {
        while (blocks[blocks.length-1].length < kEff) blocks[blocks.length-1].push(0)
      }
      const out: number[] = []
      for (const b of blocks) {
        const cw: Uint8Array = bch.encode(new Uint8Array(b))
        out.push(...Array.from(cw))
      }
      const code = new Uint8Array(out)
      setEncoded(code)
      setNoisy(code)
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  

  const onRandomFlip = () => {
    if (!encoded) return
    const idx = Math.floor(Math.random() * encoded.length)
    setBitFlipIdx(idx)
    const v = new Uint8Array(encoded); v[idx] ^= 1; setNoisy(v)
  }

  const onDecode = async () => {
    setErr('')
    try {
      if (!noisy) return
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      const bch = wasm.BCH.newAuto(Math.max(2, Math.floor(m||2)), Math.max(1, Math.floor(tDesign||1)))
      const nLocal: number = bch.n()
      setNEff(nLocal)
      // 受信語長は n の倍数である必要がある
      const r = new Uint8Array(noisy)
      if (r.length % nLocal !== 0) {
        throw new Error(`受信語の長さ (${r.length}) は n (${nLocal}) の倍数である必要があります`)
      }
      const out: number[] = []
      for (let i = 0; i < r.length; i += nLocal) {
        const block = r.subarray(i, i + nLocal)
        // BM + Chien の復号
        const corr: Uint8Array = bch.decodeBM(block)
        out.push(...Array.from(corr))
      }
      setDecoded(new Uint8Array(out))
    } catch (e:any) {
      setErr(e?.message || String(e))
    }
  }


  return (
    <PageContainer title="BCH Code (GF(2))" stickyHeader>
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
              <label>m: <input type="number" value={m} onChange={(e)=> setM(Math.max(2, Math.floor(Number(e.target.value)||2)))} style={{ width: 120 }} /></label>
              <label>t (設計誤り訂正能力): <input type="number" value={tDesign} onChange={(e)=> setTDesign(Math.max(1, Math.floor(Number(e.target.value)||1)))} style={{ width: 160 }} /></label>
            </div>
      <details>
              <summary style={{ cursor:'pointer', opacity:0.85 }}>高度な設定（原始多項式の指定）</summary>
              <div style={{ marginTop:8 }}>
                <PolynomialInput value={px ?? { coeffs: [] }} onChange={(p)=> setPx({ coeffs: p.coeffs.map((x)=> Math.round(x) & 1) })} />
                <div style={{ fontSize:12, opacity:0.75, marginTop:6 }}>注: 現状の WASM API では原始多項式の指定に未対応のため、この入力は未適用です。</div>
              </div>
            </details>
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
            {encoded ? Array.from(encoded).map((b, i)=> ((i>0 && k!=null && i% nEff===0) ? ' ' : '') + String(b)).join('') : '-'}
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
              <label>誤り個数 <input type="number" value={bitFlipIdx ?? 0} onChange={(e)=> setBitFlipIdx(Math.max(0, Math.floor(Number(e.target.value)||0)))} style={{ width: 100 }} /></label>
              <Button onClick={onRandomFlip} disabled={!encoded}>ランダム誤り</Button>
            </>
          }
        />

        <div style={{ opacity:0.75, fontSize:12 }}>
          注) 復号は BM + Chien（狭義 BCH）で行います。受信語は n の倍数長で入力してください。
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
                <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{decoded ? bitsToText(Array.from(decoded)) : '-'}</div>
              </>
            )}
          </div>
        </SectionPanelWithTitle>

        <SectionPanelWithTitle title="符号の特性">
          <CodeCharacteristics lines={[
            `n = ${nEff}`,
            `k = ${k ?? '-'}`,
            `d_{\\min} = -`,
            `t = ${t ?? '-'}`,
            `R = \\tfrac{k}{n} = ${k!=null ? (k/nEff).toFixed(3) : '-'}`,
          ]} />
        </SectionPanelWithTitle>
      </div>
    </PageContainer>
  )
}
