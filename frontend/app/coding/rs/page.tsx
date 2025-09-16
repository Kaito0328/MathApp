"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import SectionPanelWithTitle from '../../../src/components/composites/panels/SectionPanelWithTitle'
import OperationBaseBlock from '../../../src/components/features/operations/OperationBaseBlock'
import { VariablePicker } from '../../../src/components/features/variables/VariablePicker'
import CodeCharacteristics from '../../../src/components/features/coding/CodeCharacteristics'
import { Button } from '../../../src/baseComponents/controls/Button'
import { PolynomialInput } from '../../../src/widgets/input/PolynomialInput'
import type { Polynomial } from '../../../src/widgets/dto/polynomial'
import { useVariableStore } from '../../../src/state/VariableStore'

type InputMode = 'text' | 'hex'

function textToBytes(s: string): Uint8Array { return new TextEncoder().encode(s) }
function bytesToText(b: Uint8Array): string { return new TextDecoder().decode(b) }
function hexToBytes(hex: string): Uint8Array {
  // 非16進文字は除去し、奇数桁の場合は先頭に 0 を付与して偶数桁にする
  const cleaned = hex.replace(/[^0-9a-fA-F]/g, '')
  const even = cleaned.length % 2 === 0 ? cleaned : ('0' + cleaned)
  const out = new Uint8Array(even.length / 2)
  for (let i = 0; i < out.length; i++) out[i] = parseInt(even.slice(2 * i, 2 * i + 2), 16)
  return out
}
function bytesToHex(b: Uint8Array): string { return Array.from(b).map(x=>x.toString(16).padStart(2,'0')).join('') }

export default function RSPage() {
  const { get } = useVariableStore()
  const [mode, setMode] = React.useState<InputMode>('text')
  const [text, setText] = React.useState('hello rs')
  const [hex, setHex] = React.useState('')
  const [m, setM] = React.useState<number>(4)
  const [k, setK] = React.useState<number>(8)
  const [alphasCsv, setAlphasCsv] = React.useState<string>('')
  const [primitive, setPrimitive] = React.useState<Polynomial>({ coeffs: [1,0,0,1,1] })
  const [encoded, setEncoded] = React.useState<Uint8Array | null>(null)
  const [noisy, setNoisy] = React.useState<Uint8Array | null>(null)
  const [decoded, setDecoded] = React.useState<Uint8Array | null>(null)
  const [err, setErr] = React.useState<string>('')
  const [nVal, setNVal] = React.useState<number | null>(null)
  const [tVal, setTVal] = React.useState<number | null>(null)
  const [origMsgLen, setOrigMsgLen] = React.useState<number | null>(null)
  const [randomErrors, setRandomErrors] = React.useState<number>(0)
  const [autoAdjustLen, setAutoAdjustLen] = React.useState<'none' | 'pad' | 'trim'>('none')
  const [advancedEnabled, setAdvancedEnabled] = React.useState<boolean>(false)

  const getMessage = React.useCallback((): Uint8Array => {
    return mode==='text' ? textToBytes(text) : hexToBytes(hex)
  }, [mode, text, hex])

  const onEncode = async () => {
    setErr(''); setDecoded(null)
    try {
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      // 自動/高度設定による有効パラメータ決定
      const msg = getMessage()
      const autoK = msg.length || 1
      const nFromM = (mm: number) => (1 << Math.max(2, Math.min(8, mm))) - 1
      const kEff = advancedEnabled ? k : autoK
      let alphasEff: Uint8Array
      if (advancedEnabled) {
        const alphaList = alphasCsv.split(',').map(s=>Number(s.trim())).filter(x=>Number.isFinite(x))
        if (alphaList.some(x=>x<0 || x>255)) throw new Error('alphas は 0..255 の整数で指定してください')
        const uniq = new Set(alphaList)
        if (uniq.size !== alphaList.length) throw new Error('alphas に重複があります')
        if (alphaList.length < kEff) throw new Error(`alphas の個数 (${alphaList.length}) は k (${kEff}) 以上が必要です`)
        alphasEff = new Uint8Array(alphaList)
      } else {
        // m に基づいて n=2^m-1 個の評価点 1..n を使用
        const n = nFromM(m)
        setK(autoK)
        const a = new Uint8Array(Array.from({ length: n }, (_, i) => i + 1))
        setAlphasCsv(Array.from(a).join(','))
        alphasEff = a
      }

  const rs = new wasm.ReedSolomon(kEff, alphasEff)
      setNVal(rs.n())
      setTVal(rs.t())
      let f = new Uint8Array(msg)
      // 入力長調整（高度な設定時のみ有効）
      if (advancedEnabled) {
        if (autoAdjustLen === 'pad' && f.length < kEff) {
          const nf = new Uint8Array(kEff); nf.set(f); f = nf
        } else if (autoAdjustLen === 'trim' && f.length > kEff) {
          f = f.slice(0, kEff)
        }
      }
      setOrigMsgLen(f.length)
      const c: Uint8Array = rs.encode(f)
      setEncoded(c)
      setNoisy(c)
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  const onInjectRandomErrors = () => {
    // 常に符号語（encoded）をベースに誤りを注入し、受信語（noisy）へ代入する
    if (!encoded) return
    const n = encoded.length
    const v = new Uint8Array(encoded)
    const picked = new Set<number>()
    const m = Math.max(0, Math.min(randomErrors, n))
    while (picked.size < m) {
      picked.add(Math.floor(Math.random() * n))
    }
    for (const i of picked) {
      // 0x00 は避けつつランダム符号語差分（単純化: 全反転よりも 1..255 のランダム値 XOR）
      let delta = 0
      while (delta === 0) delta = (Math.floor(Math.random() * 255) + 1) & 0xFF
      v[i] = v[i] ^ delta
    }
    setNoisy(v)
  }

  const onDecode = async () => {
    setErr('')
    try {
      if (!noisy) return
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
  // 復号時は現在の k/alphas を使用（自動エンコード後は自動反映済み）
      const alphaList = alphasCsv.split(',').map(s=>Number(s.trim())).filter(x=>Number.isFinite(x))
      if (alphaList.some(x=>x<0 || x>255)) throw new Error('alphas は 0..255 の整数で指定してください')
      const uniq = new Set(alphaList)
      if (uniq.size !== alphaList.length) throw new Error('alphas に重複があります')
      if (alphaList.length < k) throw new Error(`alphas の個数 (${alphaList.length}) は k (${k}) 以上が必要です`)
      const alphasEff = new Uint8Array(alphaList)
      const rs = new wasm.ReedSolomon(k, alphasEff)
      setNVal(rs.n())
      setTVal(rs.t())
      // 受信語の長さを n に正規化（不足分は 0 詰め、超過分は切り捨て）
      const n = rs.n()
      let cw = new Uint8Array(noisy)
      if (cw.length < n) {
        const tmp = new Uint8Array(n)
        tmp.set(cw)
        cw = tmp
      } else if (cw.length > n) {
        cw = cw.slice(0, n)
      }
  // 改良版 Berlekamp–Massey 復号器を使用
  const f: Uint8Array = rs.decodeBM(cw)
      console.log('Encoded message:', cw)
      console.log('Decoded message',f)
      setDecoded(f)
    } catch (e: any) {
      setErr(e?.message || String(e))
    }
  }

  // 表示ポリシー:
  // - 16進表示: トリムしない（復号結果をそのまま表示）
  // - テキスト表示: 末尾 0x00 を省略（origMsgLen があれば優先してその長さに切り詰め）
  const decodedForText = React.useMemo((): Uint8Array | null => {
    if (!decoded) return null
    if (origMsgLen != null) return decoded.slice(0, Math.min(decoded.length, origMsgLen))
    let end = decoded.length
    while (end > 0 && decoded[end - 1] === 0) end--
    return decoded.slice(0, end)
  }, [decoded, origMsgLen])

  return (
  <PageContainer title="Reed–Solomon (GF(2^m))" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        {/* ページ全体の表現トグル */}
        <div style={{ display:'flex', gap:12, alignItems:'center' }}>
          <label><input type="radio" checked={mode==='text'} onChange={()=> setMode('text')} /> テキスト</label>
          <label><input type="radio" checked={mode==='hex'} onChange={()=> setMode('hex')} /> 16進</label>
        </div>

        {/* 入力調整（モード行直下） */}
        <div style={{ display:'flex', gap:12, alignItems:'center', flexWrap:'wrap' }}>
          <label>m: <input type="number" value={m} onChange={(e)=> setM(Math.max(2, Math.min(8, Math.floor(Number(e.target.value)||2))))} style={{ width: 120 }} /></label>
          <label>入力調整:
            <select value={autoAdjustLen} onChange={(e)=> setAutoAdjustLen(e.target.value as any)}>
              <option value="none">自動（入力に追従）</option>
              <option value="pad">k にパディング</option>
              <option value="trim">k で切り詰め</option>
            </select>
          </label>
        </div>
        {/* 高度な設定（入力調整の下に配置） */}
        <details
          open={advancedEnabled}
          onToggle={(e)=> {
            const open = (e.target as HTMLDetailsElement).open
            setAdvancedEnabled(open)
            if (open && autoAdjustLen === 'none') setAutoAdjustLen('pad')
          }}
        >
          <summary style={{ cursor:'pointer', opacity:0.85 }}>高度な設定を開く</summary>
          <div style={{ display:'grid', gap:8, marginTop:8 }}>
            <label>k: <input type="number" value={k} onChange={(e)=> setK(Math.max(1, Math.floor(Number(e.target.value)||1)))} style={{ width: 120 }} /></label>
            <label>評価点 α（CSV）: <input type="text" value={alphasCsv} onChange={(e)=> setAlphasCsv(e.target.value)} style={{ width: '100%' }} /></label>
            <div>
              <div style={{ fontSize:12, opacity:0.8, marginBottom:6 }}>原始多項式（UI のみ・未適用）</div>
              <PolynomialInput value={primitive} onChange={setPrimitive} />
            </div>
            <div style={{ display:'flex', gap:8, flexWrap:'wrap' }}>
              <Button onClick={()=> setAlphasCsv(Array.from({length:15},(_,i)=> String(i+1)).join(','))}>プリセット（1..15）</Button>
              <Button onClick={()=> setAlphasCsv(Array.from({length:31},(_,i)=> String(i+1)).join(','))}>プリセット（1..31）</Button>
            </div>
          </div>
        </details>

        {/* Encode Operation */}
        <OperationBaseBlock
          left={
            <VariablePicker
              placeholder="変数から代入"
              allowedKinds={[ 'vector' ]}
              onPick={(n)=>{
                const v:any = get(n)
                if (v?.kind==='vector' && Array.isArray(v.data)) {
                  const arr = new Uint8Array(v.data.map((x:number)=> (x|0) & 0xFF))
                  setMode('hex')
                  setHex(bytesToHex(arr))
                }
              }}
            />
          }
          center={<Button onClick={onEncode}>符号化</Button>}
          right={null}
        />

        {/* Encode 入力 */}
        <SectionPanelWithTitle title="エンコード入力">
          <div style={{ display:'grid', gap:8 }}>
            {mode==='text' ? (
              <>
                <textarea value={text} onChange={(e)=> setText(e.target.value)} rows={3} style={{ width: '100%', boxSizing:'border-box' }} />
                <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all', opacity:0.8 }}>
                  {bytesToHex(textToBytes(text)) || '-'}
                </div>
              </>
            ) : (
              <textarea value={hex} onChange={(e)=> setHex(e.target.value)} rows={3} style={{ width: '100%', boxSizing:'border-box' }} placeholder="e38182..." />
            )}
            {err && <div style={{ color:'crimson' }}>{err}</div>}
          </div>
        </SectionPanelWithTitle>

        {/* 特性表示 */}
        {nVal!=null && tVal!=null && (
            <SectionPanelWithTitle title="符号の特性">
              <CodeCharacteristics lines={[
                `n = ${nVal}`,
                `k = ${k}`,
                `d_{\\min} = n - k + 1 = ${nVal - k + 1}`,
                `t = \\left\\lfloor \\tfrac{n-k}{2} \\right\\rfloor = ${tVal}`,
                `R = \\tfrac{k}{n} = ${(k && nVal)?(k/nVal).toFixed(3):'-'}`,
              ]} />
            </SectionPanelWithTitle>
        )}

        {/* 符号語表示 */}
        <SectionPanelWithTitle title="符号語（16進）">
          <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>{encoded ? bytesToHex(encoded) : '-'}</div>
        </SectionPanelWithTitle>

        {/* Decode Operation */}
        <OperationBaseBlock
          left={
            <VariablePicker
              placeholder="変数から代入"
              allowedKinds={[ 'vector' ]}
              onPick={(n)=>{
                const v:any = get(n)
                if (v?.kind==='vector' && Array.isArray(v.data)) {
                  setNoisy(new Uint8Array(v.data.map((x:number)=> (x|0) & 0xFF)))
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

        {/* 受信語入力 */}
        <SectionPanelWithTitle title="受信語 16進">
          <div style={{ display:'grid', gap:8 }}>
            <textarea value={noisy ? bytesToHex(noisy) : ''} onChange={(e)=> setNoisy(hexToBytes(e.target.value))} rows={3} style={{ width:'100%', boxSizing:'border-box' }} />
          </div>
        </SectionPanelWithTitle>

        {/* 復号結果 */}
        <SectionPanelWithTitle title="復号結果">
          <div style={{ display:'grid', gap:8 }}>
            <div>16進:</div>
            <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>{decoded ? bytesToHex(decoded) : '-'}</div>
            {mode==='text' && (
              <>
                <div>テキスト:</div>
                <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{decodedForText ? bytesToText(decodedForText) : '-'}</div>
              </>
            )}
          </div>
        </SectionPanelWithTitle>
      </div>
    </PageContainer>
  )
}
