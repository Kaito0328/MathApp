"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import SectionPanelWithTitle from '../../../src/components/composites/panels/SectionPanelWithTitle'
import CodeCharacteristics from '../../../src/components/features/coding/CodeCharacteristics'
import OperationBaseBlock from '../../../src/components/features/operations/OperationBaseBlock'
import { VariablePicker } from '../../../src/components/features/variables/VariablePicker'
import { useVariableStore } from '../../../src/state/VariableStore'
import { Button } from '../../../src/baseComponents/controls/Button'

type InputMode = 'text' | 'binary'

function textToBits(s: string): number[] {
  const bytes = new TextEncoder().encode(s)
  const bits: number[] = []
  for (const b of bytes) {
    for (let i = 7; i >= 0; i--) bits.push((b >> i) & 1)
  }
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

function chunk(arr: number[], size: number): number[][] {
  const out: number[][] = []
  for (let i = 0; i < arr.length; i += size) out.push(arr.slice(i, i + size))
  return out
}

export default function Hamming74Page() {
  const { get } = useVariableStore()
  const [mode, setMode] = React.useState<InputMode>('text')
  const [text, setText] = React.useState<string>('HELLO')
  const [binary, setBinary] = React.useState<string>('')
  const [encoded, setEncoded] = React.useState<Uint8Array | null>(null)
  const [noisy, setNoisy] = React.useState<Uint8Array | null>(null)
  const [decoded, setDecoded] = React.useState<Uint8Array | null>(null)
  const [err, setErr] = React.useState<string>('')

  const [randomErrors, setRandomErrors] = React.useState<number>(0)

  const get4bitBlocks = React.useCallback((): number[][] => {
    let bits: number[]
    if (mode === 'text') bits = textToBits(text)
    else bits = (binary || '').split('').filter(c=>c==='0'||c==='1').map(c=>Number(c))
    const blocks = chunk(bits, 4)
    if (blocks.length && blocks[blocks.length-1].length < 4) {
      while (blocks[blocks.length-1].length < 4) blocks[blocks.length-1].push(0)
    }
    return blocks
  }, [mode, text, binary])

  const onEncode = async () => {
    setErr(''); setDecoded(null)
    try {
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      const h = new wasm.Hamming74()
      const blocks = get4bitBlocks()
      const out: number[] = []
      for (const b of blocks) {
        const u = new Uint8Array(b)
        const c: Uint8Array = h.encode(u) // 7ビット
        out.push(...Array.from(c as Uint8Array))
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
    const n = encoded.length
    const v = new Uint8Array(encoded)
    const m = Math.max(0, Math.min(randomErrors, n))
    const picked = new Set<number>()
    while (picked.size < m) picked.add(Math.floor(Math.random() * n))
    for (const i of picked) v[i] ^= 1
    setNoisy(v)
  }

  const onDecode = async () => {
    setErr('')
    try {
      if (!noisy) return
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      const h = new wasm.Hamming74()
      // 7ビットごとに復号
      const blocks = chunk(Array.from(noisy), 7)
      const msgBits: number[] = []
      for (const r of blocks) {
        const dec: Uint8Array = h.decode(new Uint8Array(r)) // 7ビット訂正済み
        // Hamming(7,4) の情報ビット（G=[I|P]）: 先頭4ビットを抽出
        msgBits.push(dec[0], dec[1], dec[2], dec[3])
      }
      setDecoded(new Uint8Array(msgBits))
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  const decodedText = React.useMemo(() => decoded ? bitsToText(Array.from(decoded)) : '' , [decoded])
  // 定数: Hamming(7,4)
  const n = 7, k = 4
  const dmin = 3
  const t = 1

  return (
    <PageContainer title="Hamming(7,4)" stickyHeader>
      <div style={{ background:'#fffbe6', border:'1px solid #f0e6a6', padding:8, marginBottom:12, borderRadius:6, fontSize:13 }}>
        このページは統合版に移行しました。新しい <a href="/coding/channel">チャネル符号（統合）</a> をご利用ください。
      </div>
      <div style={{ display:'grid', gap:12 }}>
        {/* ページ全体の表現トグル */}
        <div style={{ display:'flex', gap:12, alignItems:'center' }}>
          <label><input type="radio" checked={mode==='text'} onChange={()=> setMode('text')} /> テキスト</label>
          <label><input type="radio" checked={mode==='binary'} onChange={()=> setMode('binary')} /> 2進</label>
        </div>

        {/* Encode Operation */}
        <OperationBaseBlock
          left={
            <VariablePicker
              placeholder="変数から代入"
              allowedKinds={[ 'vector' ]}
              onPick={(n)=>{
                const v:any = get(n)
                if (v?.kind==='vector' && Array.isArray(v.data)) setBinary(v.data.map((x:number)=> String((x|0)&1)).join(''))
              }}
            />
          }
          center={<Button onClick={onEncode}>符号化</Button>}
          right={null}
        />

        {/* 入力ブロック */}
        <SectionPanelWithTitle title="エンコード入力">
          <div style={{ display:'grid', gap:8 }}>
            <div style={{ opacity:0.8, fontSize:12 }}>
              テキストは UTF-8 を 8bit のビット列に展開し、4bit ごとに符号化します。不足ビットは 0 パディング。2進は 4bit 単位に分割します。
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
            {encoded ? Array.from(encoded).map((b, i)=> ((i>0 && i%7===0) ? ' ' : '') + String(b)).join('') : '-'}
          </div>
        </SectionPanelWithTitle>

        {/* 復号オペレーション（誤り注入を含む） */}
        <OperationBaseBlock
          left={
            <VariablePicker
              placeholder="変数から代入"
              allowedKinds={[ 'vector' ]}
              onPick={(n)=>{
                const v:any = get(n)
                if (v?.kind==='vector' && Array.isArray(v.data)) {
                  setNoisy(new Uint8Array(v.data.map((x:number)=> (x|0)&1)))
                }
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

        <SectionPanelWithTitle title="復号結果">
          <div style={{ display:'grid', gap:8 }}>
            <div>復号（情報ビット列 Binary）:</div>
            <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{decoded ? Array.from(decoded).join('') : '-'}</div>
            {mode==='text' && (
              <>
                <div>復号（テキスト解釈）:</div>
                <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{decoded ? decodedText : '-'}</div>
              </>
            )}
            <div style={{ opacity:0.8, fontSize:12 }}>注) Hamming(7,4) は各 4bit ブロックを 7bit に符号化します。テキスト長に制約はありませんが、8bit→4bit の分割と 0 パディングが入る点にご注意ください。</div>
          </div>
        </SectionPanelWithTitle>

        <SectionPanelWithTitle title="符号の特性">
          <CodeCharacteristics lines={[ `n = ${n}`, `k = ${k}`, `d_{\\min} = ${dmin}`, `t = ${t}`, `R = \\tfrac{k}{n} = ${(k/n).toFixed(3)}` ]} />
        </SectionPanelWithTitle>
      </div>
    </PageContainer>
  )
}
