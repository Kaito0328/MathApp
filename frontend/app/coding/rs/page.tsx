"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import SectionPanelWithTitle from '../../../src/components/composites/panels/SectionPanelWithTitle'

type InputMode = 'text' | 'hex'

function textToBytes(s: string): Uint8Array { return new TextEncoder().encode(s) }
function bytesToText(b: Uint8Array): string { return new TextDecoder().decode(b) }
function hexToBytes(hex: string): Uint8Array {
  const clean = hex.replace(/[^0-9a-fA-F]/g, '')
  const out = new Uint8Array(Math.floor(clean.length/2))
  for (let i=0;i<out.length;i++) out[i] = parseInt(clean.slice(2*i,2*i+2),16)
  return out
}
function bytesToHex(b: Uint8Array): string { return Array.from(b).map(x=>x.toString(16).padStart(2,'0')).join('') }

export default function RSPage() {
  const [mode, setMode] = React.useState<InputMode>('text')
  const [text, setText] = React.useState('hello rs')
  const [hex, setHex] = React.useState('')
  const [k, setK] = React.useState<number>(8)
  const [alphasCsv, setAlphasCsv] = React.useState<string>('1,2,3,4,5,6,7,8,9,10,11,12,13,14,15')
  const [encoded, setEncoded] = React.useState<Uint8Array | null>(null)
  const [noisy, setNoisy] = React.useState<Uint8Array | null>(null)
  const [decoded, setDecoded] = React.useState<Uint8Array | null>(null)
  const [err, setErr] = React.useState<string>('')
  const [flipIndex, setFlipIndex] = React.useState<number | null>(null)
  const [nVal, setNVal] = React.useState<number | null>(null)
  const [tVal, setTVal] = React.useState<number | null>(null)

  const getMessage = React.useCallback((): Uint8Array => {
    return mode==='text' ? textToBytes(text) : hexToBytes(hex)
  }, [mode, text, hex])

  const onEncode = async () => {
    setErr(''); setDecoded(null)
    try {
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      const alphaList = alphasCsv.split(',').map(s=>Number(s.trim())).filter(x=>Number.isFinite(x))
      if (alphaList.some(x=>x<0 || x>255)) throw new Error('alphas は 0..255 の整数で指定してください')
      const uniq = new Set(alphaList)
      if (uniq.size !== alphaList.length) throw new Error('alphas に重複があります')
      if (alphaList.length < k) throw new Error(`alphas の個数 (${alphaList.length}) は k (${k}) 以上が必要です`)
      const alphas = new Uint8Array(alphaList)
      const rs = new wasm.ReedSolomon(k, alphas)
      setNVal(rs.n())
      setTVal(rs.t())
      const f = getMessage()
      const c: Uint8Array = rs.encode(f)
      setEncoded(c)
      setNoisy(c)
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  const onInjectError = () => {
    if (!encoded) return
    const v = noisy ? new Uint8Array(noisy) : new Uint8Array(encoded)
    if (flipIndex!=null && flipIndex>=0 && flipIndex<v.length) v[flipIndex] ^= 0xFF // 適当にビット反転
    setNoisy(v)
  }

  const onDecode = async () => {
    setErr('')
    try {
      if (!noisy) return
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      const alphaList = alphasCsv.split(',').map(s=>Number(s.trim())).filter(x=>Number.isFinite(x))
      if (alphaList.some(x=>x<0 || x>255)) throw new Error('alphas は 0..255 の整数で指定してください')
      const uniq = new Set(alphaList)
      if (uniq.size !== alphaList.length) throw new Error('alphas に重複があります')
      if (alphaList.length < k) throw new Error(`alphas の個数 (${alphaList.length}) は k (${k}) 以上が必要です`)
      const alphas = new Uint8Array(alphaList)
      const rs = new wasm.ReedSolomon(k, alphas)
      setNVal(rs.n())
      setTVal(rs.t())
      const f: Uint8Array = rs.decode(new Uint8Array(noisy))
      setDecoded(f)
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  return (
    <PageContainer title="Reed–Solomon" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        <SectionPanelWithTitle title="入力">
          <div style={{ display:'grid', gap:8 }}>
            <div style={{ display:'flex', gap:8, alignItems:'center' }}>
              <label><input type="radio" checked={mode==='text'} onChange={()=> setMode('text')} /> Text</label>
              <label><input type="radio" checked={mode==='hex'} onChange={()=> setMode('hex')} /> Hex</label>
            </div>
            <div style={{ opacity:0.8, fontSize:12 }}>
              Text は UTF-8 を 1バイトごとにシンボルとみなします。Hex は 2桁単位で 1バイトに変換します。
            </div>
            {mode==='text' ? (
              <textarea value={text} onChange={(e)=> setText(e.target.value)} rows={3} style={{ width: '100%' }} />
            ) : (
              <textarea value={hex} onChange={(e)=> setHex(e.target.value)} rows={3} style={{ width: '100%' }} placeholder="e38182..." />
            )}
            <div style={{ display:'flex', gap:12, alignItems:'center', flexWrap:'wrap' }}>
              <label>k: <input type="number" value={k} onChange={(e)=> setK(Math.max(1, Math.floor(Number(e.target.value)||1)))} style={{ width: 120 }} /></label>
              <label>alphas: <input type="text" value={alphasCsv} onChange={(e)=> setAlphasCsv(e.target.value)} style={{ width: 420 }} /></label>
            </div>
            <div style={{ display:'flex', gap:12, alignItems:'center', flexWrap:'wrap', opacity:0.8, fontSize:12 }}>
              <button onClick={()=> setAlphasCsv(Array.from({length:15},(_,i)=> String(i+1)).join(','))}>プリセット: 1..15</button>
              <button onClick={()=> setAlphasCsv(Array.from({length:31},(_,i)=> String(i+1)).join(','))}>プリセット: 1..31</button>
              {nVal!=null && tVal!=null && (
                <span>現在の n={nVal}, 訂正能力 t=⌊(n-k)/2⌋={tVal}</span>
              )}
            </div>
            <div>
              <button onClick={onEncode}>Encode</button>
            </div>
            {err && <div style={{ color:'crimson' }}>{err}</div>}
          </div>
        </SectionPanelWithTitle>

        <SectionPanelWithTitle title="符号語と誤り注入">
          <div style={{ display:'grid', gap:8 }}>
            <div>符号語:</div>
            <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>{encoded ? bytesToHex(encoded) : '-'}</div>
            <div style={{ display:'flex', gap:8, alignItems:'center', flexWrap:'wrap' }}>
              <span>flip index:</span>
              <input type="number" value={flipIndex ?? ''} onChange={(e)=> setFlipIndex(e.target.value === '' ? null : Number(e.target.value))} style={{ width: 120 }} />
              <button onClick={onInjectError} disabled={!encoded}>この位置を反転</button>
              <button onClick={()=> { if(encoded){ const idx = Math.floor(Math.random() * encoded.length); setFlipIndex(idx); const v = new Uint8Array(encoded); v[idx]^=0xFF; setNoisy(v) } }} disabled={!encoded}>ランダム反転</button>
            </div>
            <div>受信語（注入後）:</div>
            <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>{noisy ? bytesToHex(noisy) : '-'}</div>
            <div>
              <button onClick={onDecode} disabled={!noisy}>Decode</button>
            </div>
          </div>
        </SectionPanelWithTitle>

        <SectionPanelWithTitle title="復号結果">
          <div style={{ display:'grid', gap:8 }}>
            <div>復号（Binary/Hex 表示）:</div>
            <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>{decoded ? bytesToHex(decoded) : '-'}</div>
            {mode==='text' && (
              <>
                <div>復号（テキスト解釈）:</div>
                <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{decoded ? bytesToText(decoded) : '-'}</div>
              </>
            )}
            <div style={{ opacity:0.8, fontSize:12 }}>
              注) RS の復号は受信語長 n に依存します。評価点 α の選び方と k から n=|α| が決まり、最大訂正能力 t=⌊(n-k)/2⌋ です。過大な誤りを入れると復号できません。
            </div>
          </div>
        </SectionPanelWithTitle>
      </div>
    </PageContainer>
  )
}
