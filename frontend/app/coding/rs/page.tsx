"use client"
import React from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import SectionPanelWithTitle from '../../../src/components/composites/panels/SectionPanelWithTitle'
import OperationBaseBlock from '../../../src/components/features/operations/OperationBaseBlock'
import { VariablePicker } from '../../../src/components/features/variables/VariablePicker'
import CodeCharacteristics from '../../../src/components/features/coding/CodeCharacteristics'
import { Button } from '../../../src/baseComponents/controls/Button'
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
  // 高度設定用の m, k 入力（未開の場合は自動）
  const [m, setM] = React.useState<number>(4)
  const [k, setK] = React.useState<number>(8)
  const [encoded, setEncoded] = React.useState<Uint8Array | null>(null)
  const [noisy, setNoisy] = React.useState<Uint8Array | null>(null)
  const [decoded, setDecoded] = React.useState<Uint8Array | null>(null)
  const [err, setErr] = React.useState<string>('')
  const [origMsgLen, setOrigMsgLen] = React.useState<number | null>(null)
  const [randomErrors, setRandomErrors] = React.useState<number>(0)
  const [autoAdjustLen, setAutoAdjustLen] = React.useState<'none' | 'pad' | 'trim'>('none')
  // 入力モード: 自動/標準/高度
  const [encoderMode, setEncoderMode] = React.useState<'auto' | 'standard' | 'advanced'>('auto')

  const getMessage = React.useCallback((): Uint8Array => {
    return mode==='text' ? textToBytes(text) : hexToBytes(hex)
  }, [mode, text, hex])

  // 自動モードでの最小 m 選択（2..8）
  const pickMForLen = (len: number): { m: number, n: number } => {
    const target = Math.max(3, len)
    for (let mm = 2; mm <= 8; mm++) {
      const n = (1 << mm) - 1
      if (n >= target) return { m: mm, n }
    }
    // 上限 m=8
    return { m: 8, n: (1 << 8) - 1 }
  }

  // 現在の設定に基づく有効パラメータのライブプレビュー
  const msgLen = React.useMemo(() => (mode==='text' ? textToBytes(text).length : hexToBytes(hex).length), [mode, text, hex])
  const liveParams = React.useMemo(() => {
    if (encoderMode === 'auto') {
      const { m: mm, n } = pickMForLen(msgLen || 1)
      const kEff = Math.min(msgLen || 1, n)
      const tEff = Math.floor((n - kEff) / 2)
      return { m: mm, n, k: kEff, t: tEff }
    }
    // standard/advanced: m,k を使用
    const mm = Math.max(2, Math.min(8, Math.floor(m || 2)))
    const n = (1 << mm) - 1
    const kEff = Math.max(1, Math.min(n, Math.floor(k || 1)))
    const tEff = Math.floor((n - kEff) / 2)
    return { m: mm, n, k: kEff, t: tEff }
  }, [encoderMode, msgLen, m, k])

  const onEncode = async () => {
    setErr(''); setDecoded(null)
    try {
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      // 自動/高度設定による有効パラメータ決定
      const msg = getMessage()
      let nEff: number
      let kEff: number
      let f = new Uint8Array(msg)
      if (encoderMode !== 'auto') {
        const mm = Math.max(2, Math.min(8, Math.floor(m || 2)))
        nEff = (1 << mm) - 1
        kEff = Math.max(1, Math.min(nEff, Math.floor(k || 1)))
        // 長さ調整
        if (autoAdjustLen === 'pad' && f.length < kEff) {
          const nf = new Uint8Array(kEff); nf.set(f); f = nf
        } else if (autoAdjustLen === 'trim' && f.length > kEff) {
          f = f.slice(0, kEff)
        }
      } else {
        const autoK = f.length || 1
        const picked = pickMForLen(autoK)
        nEff = picked.n
        kEff = Math.min(autoK, nEff)
      }

      // エンコード（kEff 長のブロックに分割）
      const blocks: Uint8Array[] = []
      for (let i = 0; i < f.length; i += kEff) {
        const chunk = f.subarray(i, Math.min(i + kEff, f.length))
        let u = chunk
        if (chunk.length < kEff) {
          const tmp = new Uint8Array(kEff); tmp.set(chunk); u = tmp
        }
        const rs = new wasm.ReedSolomon(kEff, nEff)
        const c: Uint8Array = rs.encode(u)
        blocks.push(c)
      }
  setK(kEff)
      setOrigMsgLen(f.length)
      const all = new Uint8Array(blocks.reduce((s, b)=> s + b.length, 0))
      let off = 0
      for (const b of blocks) { all.set(b, off); off += b.length }
      setEncoded(all)
      setNoisy(all)
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
      // 復号パラメータ（n, k）を決定
      let nEff: number | null = null
      let kEff: number | null = null
      if (encoderMode === 'auto') {
        // 直近の自動設定で推定（受信語の最初のブロック長からも導出可だが、簡易に liveParams を使用）
        nEff = liveParams.n
        kEff = liveParams.k
      } else {
        const mm = Math.max(2, Math.min(8, Math.floor(m || 2)))
        nEff = (1 << mm) - 1
        kEff = Math.max(1, Math.min(nEff, Math.floor(k || 1)))
      }
      if (nEff == null || kEff == null) {
        throw new Error('復号パラメータ n/k が不明です。先にこのページで符号化するか、高度な設定で m と k を指定してください。')
      }
      const r = new Uint8Array(noisy)
      if (r.length % nEff !== 0) {
        throw new Error(`受信語の長さ (${r.length}) は n (${nEff}) の倍数である必要があります`)
      }
      const out: number[] = []
      for (let i = 0; i < r.length; i += nEff) {
        const block = r.subarray(i, i + nEff)
  const rs = new wasm.ReedSolomon(kEff, nEff)
        const u: Uint8Array = rs.decodeBM(block)
        out.push(...Array.from(u))
      }
      setDecoded(new Uint8Array(out))
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
      <div style={{ background:'#fffbe6', border:'1px solid #f0e6a6', padding:8, marginBottom:12, borderRadius:6, fontSize:13 }}>
        このページは統合版に移行しました。新しい <a href="/coding/channel">チャネル符号（統合）</a> をご利用ください。
      </div>
      <div style={{ display:'grid', gap:12 }}>
        {/* ページ全体の表現トグル */}
        <div style={{ display:'flex', gap:12, alignItems:'center' }}>
          <label><input type="radio" checked={mode==='text'} onChange={()=> setMode('text')} /> テキスト</label>
          <label><input type="radio" checked={mode==='hex'} onChange={()=> setMode('hex')} /> 16進</label>
        </div>

        {/* 符号器設定 */}
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
              {encoderMode !== 'auto' && (
                <label>入力調整:
                  <select value={autoAdjustLen} onChange={(e)=> setAutoAdjustLen(e.target.value as any)}>
                    <option value="none">自動（入力に追従）</option>
                    <option value="pad">k にパディング</option>
                    <option value="trim">k で切り詰め</option>
                  </select>
                </label>
              )}
            </div>
            {encoderMode !== 'auto' && (
              <div style={{ display:'flex', gap:12, alignItems:'center', flexWrap:'wrap' }}>
                <label>m: <input type="number" value={m} onChange={(e)=> setM(Math.max(2, Math.min(8, Math.floor(Number(e.target.value)||2))))} style={{ width: 120 }} /></label>
                <label>k: <input type="number" value={k} onChange={(e)=> setK(Math.max(1, Math.floor(Number(e.target.value)||1)))} style={{ width: 120 }} /></label>
              </div>
            )}
            {encoderMode === 'advanced' && (
              <div style={{ marginTop:8 }}>
                <div style={{ fontSize:12, opacity:0.85, marginBottom:6 }}>原始多項式（UIのみ・未適用）</div>
                {/* NOTE: RS の原始多項式は現状API未対応のためUIのみ */}
                {/** PolynomialInput コンポーネントは既にページから除去していたため再導入不要（UI要件として表示のみならダミーにします） */}
              </div>
            )}
            {encoderMode !== 'auto' && (
              <div style={{ fontSize:12, opacity:0.8 }}>
                現在の有効パラメータ: n={liveParams.n}, k={liveParams.k}, t={liveParams.t}
              </div>
            )}
          </div>
        </SectionPanelWithTitle>

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

        {/* 特性表示（復号結果の下） */}
        <SectionPanelWithTitle title="符号の特性">
          <CodeCharacteristics lines={[
            `n = ${liveParams.n}`,
            `k = ${liveParams.k}`,
            `d_{\\min} = n - k + 1 = ${liveParams.n - liveParams.k + 1}`,
            `t = \\left\\lfloor \\tfrac{n-k}{2} \\right\\rfloor = ${liveParams.t}`,
            `R = \\tfrac{k}{n} = ${(liveParams.k && liveParams.n)?(liveParams.k/liveParams.n).toFixed(3):'-'}`,
          ]} />
        </SectionPanelWithTitle>
      </div>
    </PageContainer>
  )
}
