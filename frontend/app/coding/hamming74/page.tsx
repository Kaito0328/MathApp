"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import SectionPanelWithTitle from '../../../src/components/composites/panels/SectionPanelWithTitle'

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
  const [mode, setMode] = React.useState<InputMode>('text')
  const [text, setText] = React.useState<string>('HELLO')
  const [binary, setBinary] = React.useState<string>('')
  const [encoded, setEncoded] = React.useState<Uint8Array | null>(null)
  const [noisy, setNoisy] = React.useState<Uint8Array | null>(null)
  const [decoded, setDecoded] = React.useState<Uint8Array | null>(null)
  const [err, setErr] = React.useState<string>('')

  const [bitFlipIdx, setBitFlipIdx] = React.useState<number | null>(null)

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

  const onInjectError = () => {
    if (!encoded) return
    const v = noisy ? new Uint8Array(noisy) : new Uint8Array(encoded)
    if (bitFlipIdx!=null && bitFlipIdx>=0 && bitFlipIdx < v.length) v[bitFlipIdx] ^= 1
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

  return (
    <PageContainer title="Hamming(7,4)" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        <SectionPanelWithTitle title="入力">
          <div style={{ display:'grid', gap:8 }}>
            <div style={{ display:'flex', gap:8, alignItems:'center' }}>
              <label><input type="radio" checked={mode==='text'} onChange={()=> setMode('text')} /> Text</label>
              <label><input type="radio" checked={mode==='binary'} onChange={()=> setMode('binary')} /> Binary</label>
            </div>
            <div style={{ opacity:0.8, fontSize:12 }}>
              Text は UTF-8 を 8bit ごとのビット列に展開し、4bit ずつ符号化します。不足ビットは 0 パディングします。Binary は 4bit 単位に分割します。
            </div>
            {mode==='text' ? (
              <textarea value={text} onChange={(e)=> setText(e.target.value)} rows={3} style={{ width: '100%' }} />
            ) : (
              <textarea value={binary} onChange={(e)=> setBinary(e.target.value)} rows={3} style={{ width: '100%' }} placeholder="例) 1011001110" />
            )}
            <div>
              <button onClick={onEncode}>Encode</button>
            </div>
            {err && <div style={{ color:'crimson' }}>{err}</div>}
          </div>
        </SectionPanelWithTitle>

        <SectionPanelWithTitle title="符号語と誤り注入">
          <div style={{ display:'grid', gap:8 }}>
            <div style={{ opacity:0.8, fontSize:12 }}>符号語は 7bit ブロックで表示されます。</div>
            <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>
              {encoded ? Array.from(encoded).map((b, i)=> ((i>0 && i%7===0) ? ' ' : '') + String(b)).join('') : '-'}
            </div>
            <div style={{ display:'flex', gap:8, alignItems:'center', flexWrap:'wrap' }}>
              <span>flip index:</span>
              <input type="number" value={bitFlipIdx ?? ''} onChange={(e)=> setBitFlipIdx(e.target.value === '' ? null : Number(e.target.value))} style={{ width: 120 }} />
              <button onClick={onInjectError} disabled={!encoded}>この位置を反転</button>
              <button onClick={()=> { if(encoded){ const idx = Math.floor(Math.random() * encoded.length); setBitFlipIdx(idx); const v = new Uint8Array(encoded); v[idx]^=1; setNoisy(v) } }} disabled={!encoded}>ランダム反転</button>
            </div>
            <div>受信語（注入後）:</div>
            <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>{noisy ? Array.from(noisy).map((b, i)=> ((i>0 && i%7===0) ? ' ' : '') + String(b)).join('') : '-'}</div>
            <div>
              <button onClick={onDecode} disabled={!noisy}>Decode</button>
            </div>
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
      </div>
    </PageContainer>
  )
}
