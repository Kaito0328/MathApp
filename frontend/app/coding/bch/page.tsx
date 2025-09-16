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
  // 入力モード（自動/標準/高度）
  const [encoderMode, setEncoderMode] = React.useState<'auto' | 'standard' | 'advanced'>('auto')
  // 標準/高度用: m, t を指定（n = 2^m - 1）
  const [m, setM] = React.useState<number>(4)
  const [tDesign, setTDesign] = React.useState<number>(2)
  // 高度な設定: 原始多項式（現状 API 未対応のため UI のみ）
  const [px, setPx] = React.useState<Polynomial | null>(null)
  const [k, setK] = React.useState<number | null>(null)
  const [t, setT] = React.useState<number | null>(null)
  const [encoded, setEncoded] = React.useState<Uint8Array | null>(null)
  const [noisy, setNoisy] = React.useState<Uint8Array | null>(null)
  const [decoded, setDecoded] = React.useState<Uint8Array | null>(null)
  const [recovered, setRecovered] = React.useState<Uint8Array | null>(null)
  const [err, setErr] = React.useState<string>('')
  const [bitFlipIdx, setBitFlipIdx] = React.useState<number | null>(null)
  const [nEff, setNEff] = React.useState<number>(15)

  const getMsgBits = React.useCallback((): number[] => {
    return mode==='text' ? textToBits(text) : (binary||'').split('').filter(c=>c==='0'||c==='1').map(c=>Number(c))
  }, [mode, text, binary])
  const msgBitsLen = React.useMemo(() => getMsgBits().length, [getMsgBits])
  const pickMForLen = (len: number): { m: number, n: number } => {
    const target = Math.max(3, len)
    for (let mm = 2; mm <= 12; mm++) { // BCHはmをやや広めに
      const n = (1 << mm) - 1
      if (n >= target) return { m: mm, n }
    }
    const mm = 12; return { m: mm, n: (1 << mm) - 1 }
  }
  // 自動モードのパラメータ推定: m は最小、t は安全側に小さめ（例: 2 以上で n-k が確保される範囲）
  const liveParams = React.useMemo(() => {
    if (encoderMode === 'auto') {
      const { m: mm, n } = pickMForLen(msgBitsLen || 1)
      // newAuto(m,t) は設計距離由来。k は実装から取得するためここでは未知。t は保守的に2以上で n>k を確保する仮値。
      const tGuess = Math.max(2, Math.floor((n - (msgBitsLen || 1)) / 2))
      return { m: mm, n, t: Math.max(1, tGuess) }
    }
    const mm = Math.max(2, Math.floor(m || 2))
    const n = (1 << mm) - 1
    return { m: mm, n, t: Math.max(1, Math.floor(tDesign || 1)) }
  }, [encoderMode, msgBitsLen, m, tDesign])

  

  const onEncode = async () => {
    setErr(''); setDecoded(null); setRecovered(null)
    try {
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      let bch: any
      if (encoderMode === 'auto') {
        const { m: mm } = liveParams
        // t は仮に 2 以上で開始。実際の k は bch.k() から取得。
        bch = wasm.BCH.newAuto(mm, Math.max(1, liveParams.t))
      } else {
        bch = wasm.BCH.newAuto(Math.max(2, Math.floor(m||2)), Math.max(1, Math.floor(tDesign||1)))
      }
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
      // 復号時の n を推定
      let nLocal: number
      if (encoderMode === 'auto') {
        nLocal = liveParams.n
      } else {
        const mm = Math.max(2, Math.floor(m||2))
        nLocal = (1 << mm) - 1
      }
      const bch = wasm.BCH.newAuto(Math.max(2, Math.floor(liveParams.m||2)), Math.max(1, Math.floor(liveParams.t||1)))
      setNEff(nLocal)
      const kEff: number = bch.k(); setK(kEff)
      // 受信語長は n の倍数である必要がある
      const r = new Uint8Array(noisy)
      if (r.length % nLocal !== 0) {
        throw new Error(`受信語の長さ (${r.length}) は n (${nLocal}) の倍数である必要があります`)
      }
      const out: number[] = []
      const msgOut: number[] = []
      for (let i = 0; i < r.length; i += nLocal) {
        const block = r.subarray(i, i + nLocal)
        // BM + Chien の復号
        const corr: Uint8Array = bch.decodeBM(block)
        out.push(...Array.from(corr))
        // systematic エンコードのため、各ブロック末尾 k ビットがメッセージ
        const msgBits = Array.from(corr.slice(nLocal - kEff, nLocal))
        msgOut.push(...msgBits)
      }
      setDecoded(new Uint8Array(out))
      setRecovered(new Uint8Array(msgOut))
    } catch (e:any) {
      setErr(e?.message || String(e))
    }
  }

  const recoveredText = React.useMemo(() => recovered ? bitsToText(Array.from(recovered)) : '' , [recovered])


  return (
    <PageContainer title="BCH Code (GF(2))" stickyHeader>
      <div style={{ background:'#fffbe6', border:'1px solid #f0e6a6', padding:8, marginBottom:12, borderRadius:6, fontSize:13 }}>
        このページは統合版に移行しました。新しい <a href="/coding/channel">チャネル符号（統合）</a> をご利用ください。
      </div>
      <div style={{ display:'grid', gap:12 }}>
        {/* ページ全体の表現トグル */}
        <div style={{ display:'flex', gap:12, alignItems:'center' }}>
          <label><input type="radio" checked={mode==='text'} onChange={()=> setMode('text')} /> テキスト</label>
          <label><input type="radio" checked={mode==='binary'} onChange={()=> setMode('binary')} /> 2進</label>
        </div>

        {/* 符号器設定ブロック（入力モード対応） */}
        <SectionPanelWithTitle title="符号器設定">
          <div style={{ display:'grid', gap:8 }}>
            <div style={{ display:'flex', gap:12, alignItems:'center', flexWrap:'wrap' }}>
              <label>入力モード:
                <select value={encoderMode} onChange={(e)=> setEncoderMode(e.target.value as any)} style={{ marginLeft:8 }}>
                  <option value="auto">自動</option>
                  <option value="standard">標準設定</option>
                  <option value="advanced">高度な設定</option>
                </select>
              </label>
            </div>
            {encoderMode !== 'auto' && (
              <div style={{ display:'flex', gap:12, alignItems:'center', flexWrap:'wrap' }}>
                <label>m: <input type="number" value={m} onChange={(e)=> setM(Math.max(2, Math.floor(Number(e.target.value)||2)))} style={{ width: 120 }} /></label>
                <label>t (設計誤り訂正能力): <input type="number" value={tDesign} onChange={(e)=> setTDesign(Math.max(1, Math.floor(Number(e.target.value)||1)))} style={{ width: 160 }} /></label>
              </div>
            )}
            {encoderMode === 'advanced' && (
              <div style={{ marginTop:8 }}>
                <div style={{ fontSize:12, opacity:0.85, marginBottom:6 }}>原始多項式（UIのみ・未適用）</div>
                <PolynomialInput value={px ?? { coeffs: [] }} onChange={(p)=> setPx({ coeffs: p.coeffs.map((x)=> Math.round(x) & 1) })} />
                <div style={{ fontSize:12, opacity:0.75, marginTop:6 }}>注: 現状の WASM API では原始多項式の指定に未対応のため、この入力は未適用です。</div>
              </div>
            )}
            <div style={{ fontSize:12, opacity:0.8 }}>現在の有効パラメータ（推定）: m={liveParams.m}, t={liveParams.t}</div>
          </div>
        </SectionPanelWithTitle>

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
          注) 復号は BM + Chien（狭義 BCH）で「訂正後コード語」を返します。BCH はsystematicエンコードのため、各ブロック末尾 k ビットをメッセージとして復元します。受信語は n の倍数長で入力してください。
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
            {decoded && noisy && (
              <div style={{ fontSize:12, opacity:0.8 }}>
                訂正ビット数（総和）: {Array.from(decoded).reduce((s, b, i)=> s + ((b ^ (noisy![i] ?? 0)) & 1), 0)}
              </div>
            )}
          </div>
        </SectionPanelWithTitle>

        {/* 復元メッセージ（systematic: 各ブロック末尾 k ビット） */}
        <SectionPanelWithTitle title="復元メッセージ（各ブロック末尾 k ビット）">
          <div style={{ display:'grid', gap:8 }}>
            <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>
              {recovered ? Array.from(recovered).map((b, i)=> ((i>0 && k!=null && i% (k||1)===0) ? ' ' : '') + String(b)).join('') : '-'}
            </div>
            {mode==='text' && (
              <>
                <div>メッセージ（テキスト解釈）:</div>
                <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{recovered ? recoveredText : '-'}</div>
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
