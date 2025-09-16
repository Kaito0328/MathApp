"use client"
import React, { Suspense } from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import SectionPanelWithTitle from '../../../src/components/composites/panels/SectionPanelWithTitle'
import OperationSelect from '../../../src/components/features/operations/OperationSelect'
import { TextInput } from '../../../src/baseComponents/input/TextInput'
import ErrorRateControl from '../../../src/components/features/coding/common/ErrorRateControl'
import { lz78Encode, lz78Decode } from '../../../src/wasm/source'
import { sourceHuffmanEncode, sourceHuffmanDecode, sourceArithmeticEncode, sourceArithmeticDecode, estimateProbsFromText } from '../../../src/wasm/source'
import { getWasm } from '../../../src/wasm/loader'
import { useSearchParams, useRouter, usePathname } from 'next/navigation'

type SourceAlgo = 'lz78' | 'huffman' | 'arithmetic'
type ChannelAlgo = 'rs' | 'bch' | 'cyclic' | 'hamming74'

function CommPipelinePageInner() {
  const searchParams = useSearchParams()
  const router = useRouter()
  const pathname = usePathname()
  const [srcAlgo, setSrcAlgo] = React.useState<SourceAlgo>('lz78')
  const [chAlgo, setChAlgo] = React.useState<ChannelAlgo>('rs')
  // Source settings
  const [alphabet, setAlphabet] = React.useState('ABCDEFGHIJKLMNOPQRSTUVWXYZ _')
  const [autoProb, setAutoProb] = React.useState(true)
  const [probs, setProbs] = React.useState<string>('')
  // Channel settings (optional)
  const [bchM, setBchM] = React.useState(6)
  const [bchT, setBchT] = React.useState(2)
  const [cycN, setCycN] = React.useState(7)
  const [cycG, setCycG] = React.useState('1101')
  const [p, setP] = React.useState(0.01)
  const [seed, setSeed] = React.useState('')
  const [text, setText] = React.useState('HELLO COMM')
  // パイプライン表示用ステート
  // legacy plain text resultは廃止（パイプラインUIへ）
  const [inputBytesLen, setInputBytesLen] = React.useState<number>(0)
  const [srcOutBytesLen, setSrcOutBytesLen] = React.useState<number>(0)
  // 表示は recvLen と unit で十分のため codeLen は保持しない
  const [codeUnit, setCodeUnit] = React.useState<'bytes'|'bits'>('bytes')
  const [recvLen, setRecvLen] = React.useState<number>(0)
  const [flips, setFlips] = React.useState<number>(0)
  const [ber, setBer] = React.useState<number>(0)
  const [invalidJson, setInvalidJson] = React.useState<boolean>(false)
  const [recoveredBytesLast, setRecoveredBytesLast] = React.useState<Uint8Array | null>(null)
  const [restoredText, setRestoredText] = React.useState<string>('')
  const [isSuccess, setIsSuccess] = React.useState<boolean | null>(null)
  // 可視化用にバイナリも保持
  const [srcPayloadPreview, setSrcPayloadPreview] = React.useState<string>('')
  const [codewordPreview, setCodewordPreview] = React.useState<string>('')
  const [receivedPreview, setReceivedPreview] = React.useState<string>('')
  const [decodedPreview, setDecodedPreview] = React.useState<string>('')
  const [err, setErr] = React.useState<string>('')

  // init from ?src & ?ch
  React.useEffect(() => {
    const src = (searchParams?.get('src') || '').toLowerCase()
    const ch = (searchParams?.get('ch') || '').toLowerCase()
    if (src === 'lz78' || src === 'huffman' || src === 'arithmetic') setSrcAlgo(src as SourceAlgo)
    if (ch === 'rs' || ch === 'bch' || ch === 'cyclic' || ch === 'hamming74') setChAlgo(ch as ChannelAlgo)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])
  // update URL on change
  React.useEffect(() => {
    const usp = new URLSearchParams(searchParams?.toString())
    usp.set('src', srcAlgo)
    usp.set('ch', chAlgo)
    router.replace(`${pathname}?${usp.toString()}`)
  }, [srcAlgo, chAlgo, pathname, router, searchParams])

  // helpers
  const textToBytes = (s: string) => new TextEncoder().encode(s)
  const bytesToHex = (b: Uint8Array) => Array.from(b).map(x=>x.toString(16).padStart(2,'0')).join('')
  const bytesToText = (b: Uint8Array) => new TextDecoder().decode(b)
  // const bytesToHex = (b: Uint8Array) => Array.from(b).map(x=>x.toString(16).padStart(2,'0')).join('')
  const bytesToBits = (u8: Uint8Array) => {
    const bits: number[] = []
    for (const by of u8) for (let i=7;i>=0;i--) bits.push((by>>i)&1)
    return bits
  }
  const bitsToBytes = (bits: number[]) => {
    const nBytes = Math.ceil(bits.length/8)
    const out = new Uint8Array(nBytes)
    for (let i=0;i<nBytes;i++) {
      let v = 0
      for (let j=0;j<8;j++) {
        const idx = i*8 + j
        v = (v<<1) | (idx < bits.length ? (bits[idx]&1) : 0)
      }
      out[i] = v
    }
    return out
  }
  const parseProbs = React.useCallback(() => {
    if (autoProb) return estimateProbsFromText(alphabet, text)
    const parts = probs.split(/[ ,]+/).filter(Boolean).map(Number)
    const arr = new Float64Array(alphabet.length)
    for (let i = 0; i < alphabet.length; i++) arr[i] = parts[i] ?? (1 / alphabet.length)
    const s = arr.reduce((a,b)=>a+b,0)
    if (s > 0) for (let i=0;i<arr.length;i++) arr[i] /= s
    return arr
  }, [autoProb, probs, alphabet, text])
  const printable = (s: string, max = 512) => {
    const cut = s.length > max ? s.slice(0, max) + ' …' : s
    return cut.replace(/[^\x20-\x7E\n\r\t]/g, '·')
  }
  function rngFromSeed(str: string) {
    let h = 2166136261 >>> 0
    for (let i=0;i<str.length;i++) { h ^= str.charCodeAt(i); h = Math.imul(h, 16777619) >>> 0 }
    let state = (h || 0x9e3779b9) >>> 0
    return function() {
      state += 0x6D2B79F5
      let t = state
      t = Math.imul(t ^ (t >>> 15), 1 | t)
      t ^= t + Math.imul(t ^ (t >>> 7), 61 | t)
      return ((t ^ (t >>> 14)) >>> 0) / 4294967296
    }
  }
  const bscFlipBits = (bits: number[], prob: number, seedStr: string) => {
    const rand = rngFromSeed(seedStr)
    const out = bits.slice()
    let flips = 0
    for (let i=0;i<out.length;i++) if (rand() < prob) { out[i] ^= 1; flips++ }
    return { bits: out, flips }
  }
  const bscFlipBytes = (bytes: Uint8Array, prob: number, seedStr: string) => {
    const { bits, flips } = bscFlipBits(bytesToBits(bytes), prob, seedStr)
    return { bytes: bitsToBytes(bits), flips }
  }

  const onRun = async () => {
    setInvalidJson(false)
    setErr('')
    try {
      setIsSuccess(null)
      setRestoredText('')
      setInputBytesLen(textToBytes(text).length)
      // 1) Source encode (LZ78)
      let srcPayloadBytes: Uint8Array
      if (srcAlgo === 'lz78') {
        const lzPairs = await lz78Encode(text)
        const json = JSON.stringify(lzPairs)
        srcPayloadBytes = textToBytes(json)
        setSrcPayloadPreview(json)
      } else if (srcAlgo === 'huffman') {
        const pv = parseProbs()
        const bits: Uint8Array = await sourceHuffmanEncode(alphabet, pv, text)
        srcPayloadBytes = bitsToBytes(Array.from(bits))
        setSrcPayloadPreview(`hex:${bytesToHex(srcPayloadBytes)}\n(bits:${bits.length})`)
      } else if (srcAlgo === 'arithmetic') {
        const pv = parseProbs()
        const bits: Uint8Array = await sourceArithmeticEncode(alphabet, pv, text)
        srcPayloadBytes = bitsToBytes(Array.from(bits))
        setSrcPayloadPreview(`hex:${bytesToHex(srcPayloadBytes)}\n(bits:${bits.length})`)
      } else {
        srcPayloadBytes = textToBytes(text)
        setSrcPayloadPreview(Array.from(srcPayloadBytes).map(x=>x.toString(16).padStart(2,'0')).join(''))
      }
      setSrcOutBytesLen(srcPayloadBytes.length)

      // 2) Channel encode -> 3) BSC -> 4) Channel decode
      const wasm: any = await getWasm()
    let flipsLocal = 0
      let recoveredBytes: Uint8Array

      if (chAlgo === 'rs') {
        // RS over bytes
        const pickMForLen = (len: number) => {
          const target = Math.max(3, len)
          for (let mm=2; mm<=8; mm++) { const n=(1<<mm)-1; if (n>=target) return { m:mm, n } }
          return { m:8, n:(1<<8)-1 }
        }
    const { n } = pickMForLen(srcPayloadBytes.length || 1)
        const kEff = Math.min(srcPayloadBytes.length || 1, n)
        // encode in k-sized blocks
        const blocks: Uint8Array[] = []
        for (let i=0;i<srcPayloadBytes.length;i+=kEff) {
          const chunk = srcPayloadBytes.subarray(i, Math.min(i+kEff, srcPayloadBytes.length))
          let u = chunk
          if (chunk.length < kEff) { const tmp = new Uint8Array(kEff); tmp.set(chunk); u = tmp }
          const rs = new wasm.ReedSolomon(kEff, n)
          const c: Uint8Array = rs.encode(u)
          blocks.push(c)
        }
    const code = new Uint8Array(blocks.reduce((s,b)=>s+b.length,0))
        { let off=0; for (const b of blocks) { code.set(b, off); off+=b.length } }
        // BSC on bytes
    const noised = bscFlipBytes(code, p, seed)
    flipsLocal = noised.flips
    const r = noised.bytes
    setCodeUnit('bytes'); setRecvLen(r.length)
    setCodewordPreview(Array.from(code).map(x=>x.toString(16).padStart(2,'0')).join(''))
    setReceivedPreview(Array.from(r).map(x=>x.toString(16).padStart(2,'0')).join(''))
    setFlips(flipsLocal); setBer(code.length>0 ? flipsLocal/(code.length*8) : 0)
        if (r.length % n !== 0) throw new Error('内部エラー: RS 受信長整合性')
        const out: number[] = []
        for (let i=0;i<r.length;i+=n) {
          const block = r.subarray(i, i+n)
          const rs = new wasm.ReedSolomon(kEff, n)
          const u: Uint8Array = rs.decodeBM(block)
          out.push(...Array.from(u))
        }
        recoveredBytes = new Uint8Array(out)
      } else if (chAlgo === 'bch') {
        // BCH over bits (GF(2))
        const bits = bytesToBits(srcPayloadBytes)
  const mm = Math.max(2, Math.floor(bchM||2)); const tDesign = Math.max(1, Math.floor(bchT||1))
        const bch = wasm.BCH.newAuto(mm, tDesign)
        const nLocal: number = bch.n(); const kEff: number = bch.k()
        const blocks: number[][] = []
        for (let i=0;i<bits.length;i+=kEff) {
          const part = bits.slice(i, i+kEff)
          while (part.length < kEff) part.push(0)
          blocks.push(part)
        }
    const codeBits: number[] = []
    for (const b of blocks) { const cw: Uint8Array = bch.encode(new Uint8Array(b)); codeBits.push(...Array.from(cw)) }
    const noised = bscFlipBits(codeBits, p, seed); flipsLocal = noised.flips
    const rBits = noised.bits
    setCodeUnit('bits'); setRecvLen(rBits.length)
    setCodewordPreview(codeBits.join(''))
    setReceivedPreview(rBits.join(''))
    setFlips(flipsLocal); setBer(codeBits.length>0 ? flipsLocal/(codeBits.length) : 0)
        if (rBits.length % nLocal !== 0) throw new Error('内部エラー: BCH 受信長整合性')
        const msgBits: number[] = []
        for (let i=0;i<rBits.length;i+=nLocal) {
          const corr: Uint8Array = bch.decodeBM(new Uint8Array(rBits.slice(i, i+nLocal)))
          const tail = Array.from(corr.slice(nLocal - kEff, nLocal))
          msgBits.push(...tail)
        }
        // trim to original bits length
        recoveredBytes = bitsToBytes(msgBits.slice(0, bits.length))
      } else if (chAlgo === 'cyclic') {
        // Cyclic (n=7, g=1+x+x^3 as default)
  const bits = bytesToBits(srcPayloadBytes)
  const n = Math.max(3, Math.floor(cycN||7))
  const gCoeffs = new Uint8Array((cycG||'1101').split('').map(c=> c==='1'?1:0))
        const cyc = new wasm.CyclicCode(n, gCoeffs)
        const kEff: number = cyc.k()
        const blocks: number[][] = []
        for (let i=0;i<bits.length;i+=kEff) {
          const part = bits.slice(i, i+kEff); while (part.length < kEff) part.push(0); blocks.push(part)
        }
    const codeBits: number[] = []
    for (const b of blocks) { const cw: Uint8Array = cyc.encode(new Uint8Array(b)); codeBits.push(...Array.from(cw)) }
    const noised = bscFlipBits(codeBits, p, seed); flipsLocal = noised.flips
    const rBits = noised.bits
    setCodeUnit('bits'); setRecvLen(rBits.length)
    setCodewordPreview(codeBits.join(''))
    setReceivedPreview(rBits.join(''))
    setFlips(flipsLocal); setBer(codeBits.length>0 ? flipsLocal/(codeBits.length) : 0)
        if (rBits.length % n !== 0) throw new Error('内部エラー: Cyclic 受信長整合性')
        const msgBits: number[] = []
        for (let i=0;i<rBits.length;i+=n) {
          const corr: Uint8Array = cyc.decodeLUT(new Uint8Array(rBits.slice(i, i+n)))
          const cwPoly = new wasm.PolynomialGF2(corr)
          const gPoly = new wasm.PolynomialGF2(gCoeffs)
          const [quot/*, rem*/] = cwPoly.divRem(gPoly)
          // ignore rem here; assume correction succeeded
          const qArr = Array.from((quot.coeffs() as Uint8Array))
          while (qArr.length < kEff) qArr.push(0)
          msgBits.push(...qArr.slice(0, kEff))
        }
        recoveredBytes = bitsToBytes(msgBits.slice(0, bits.length))
      } else {
        // Hamming(7,4)
        const bits = bytesToBits(srcPayloadBytes)
        const h = new wasm.Hamming74()
        const blocks: number[][] = []
        for (let i=0;i<bits.length;i+=4) { const part = bits.slice(i, i+4); while (part.length < 4) part.push(0); blocks.push(part) }
    const codeBits: number[] = []
    for (const b of blocks) { const cw: Uint8Array = h.encode(new Uint8Array(b)); codeBits.push(...Array.from(cw)) }
    const noised = bscFlipBits(codeBits, p, seed); flipsLocal = noised.flips
    const rBits = noised.bits
    setCodeUnit('bits'); setRecvLen(rBits.length)
    setCodewordPreview(codeBits.join(''))
    setReceivedPreview(rBits.join(''))
    setFlips(flipsLocal); setBer(codeBits.length>0 ? flipsLocal/(codeBits.length) : 0)
        if (rBits.length % 7 !== 0) throw new Error('内部エラー: Hamming 受信長整合性')
        const msgBits: number[] = []
        for (let i=0;i<rBits.length;i+=7) { const dec: Uint8Array = h.decode(new Uint8Array(rBits.slice(i, i+7))); msgBits.push(dec[0], dec[1], dec[2], dec[3]) }
        recoveredBytes = bitsToBytes(msgBits.slice(0, bits.length))
      }

      // 5) Source decode
      let restoredTextLocal: string
      if (srcAlgo === 'lz78') {
        const jsonText = bytesToText(recoveredBytes)
        try {
          const pairs = JSON.parse(jsonText)
          restoredTextLocal = await lz78Decode(pairs)
          setDecodedPreview(JSON.stringify(pairs))
        } catch {
          // JSONが壊れている（通信失敗時）場合は、そのままテキストとして表示
          restoredTextLocal = jsonText
          setDecodedPreview('(invalid JSON)')
          setInvalidJson(true)
        }
      } else if (srcAlgo === 'huffman') {
        const pv = parseProbs()
        const bits = bytesToBits(recoveredBytes)
        const out = await sourceHuffmanDecode(alphabet, pv, text.length, new Uint8Array(bits))
        restoredTextLocal = out
        setDecodedPreview(`bits:${bits.length}`)
      } else if (srcAlgo === 'arithmetic') {
        const pv = parseProbs()
        const bits = bytesToBits(recoveredBytes)
        const out = await sourceArithmeticDecode(alphabet, pv, text.length, new Uint8Array(bits))
        restoredTextLocal = out
        setDecodedPreview(`bits:${bits.length}`)
      } else {
        restoredTextLocal = bytesToText(recoveredBytes)
        setDecodedPreview(Array.from(recoveredBytes).map(x=>x.toString(16).padStart(2,'0')).join(''))
      }
      setRestoredText(restoredTextLocal)
      setIsSuccess(restoredTextLocal === text)
      setRecoveredBytesLast(recoveredBytes)

  // legacy report は廃止
    } catch (e:any) {
      setErr(e?.message || String(e))
    }
  }

  return (
    <PageContainer title="通信体験（情報源→チャネル→誤り→復号）" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        <SectionPanelWithTitle title="アルゴリズム選択">
          <div style={{ display:'grid', gap:8 }}>
            <div>
              <div style={{ fontSize:12, opacity:0.8 }}>情報源符号</div>
              <OperationSelect
                operations={[
                  { value:'lz78', label:'LZ78' },
                  { value:'huffman', label:'Huffman' },
                  { value:'arithmetic', label:'Arithmetic' },
                ]}
                value={srcAlgo}
                onChange={(v)=> setSrcAlgo(v as SourceAlgo)}
                label="情報源"
              />
            </div>
            <div>
              <div style={{ fontSize:12, opacity:0.8 }}>チャネル符号</div>
              <OperationSelect
                operations={[
                  { value:'rs', label:'Reed–Solomon' },
                  { value:'bch', label:'BCH' },
                  { value:'cyclic', label:'Cyclic' },
                  { value:'hamming74', label:'Hamming(7,4)' },
                ]}
                value={chAlgo}
                onChange={(v)=> setChAlgo(v as ChannelAlgo)}
                label="チャネル"
              />
            </div>
          </div>
        </SectionPanelWithTitle>

        {(srcAlgo==='huffman' || srcAlgo==='arithmetic') && (
          <SectionPanelWithTitle title="情報源の設定">
            <div style={{ display:'grid', gap:8 }}>
              <label>
                アルファベット:
                <TextInput value={alphabet} onChange={(e)=> setAlphabet((e.target as HTMLInputElement).value)} style={{ width:'100%' }} />
              </label>
              <label style={{ display:'flex', gap:8, alignItems:'center' }}>
                <input type="checkbox" checked={autoProb} onChange={(e)=> setAutoProb(e.target.checked)} />
                テキストから頻度推定（自動）
              </label>
              {!autoProb && (
                <label>
                  確率（空白区切り）:
                  <TextInput value={probs} onChange={(e)=> setProbs((e.target as HTMLInputElement).value)} style={{ width:'100%' }} placeholder="0.1 0.2 0.3 ..." />
                </label>
              )}
            </div>
          </SectionPanelWithTitle>
        )}

        <SectionPanelWithTitle title="チャネルの設定">
          {chAlgo==='rs' && <div style={{ fontSize:12, opacity:0.8 }}>RS: 自動設定（入力長に合わせて m/k を選択）</div>}
          {chAlgo==='bch' && (
            <div style={{ display:'flex', gap:12, flexWrap:'wrap' }}>
              <label>m: <input type="number" value={bchM} onChange={(e)=> setBchM(parseInt(e.target.value||'6',10))} /></label>
              <label>t: <input type="number" value={bchT} onChange={(e)=> setBchT(parseInt(e.target.value||'2',10))} /></label>
            </div>
          )}
          {chAlgo==='cyclic' && (
            <div style={{ display:'flex', gap:12, flexWrap:'wrap', alignItems:'center' }}>
              <label>n: <input type="number" value={cycN} onChange={(e)=> setCycN(parseInt(e.target.value||'7',10))} /></label>
              <label>g: <TextInput value={cycG} onChange={(e)=> setCycG((e.target as HTMLInputElement).value)} style={{ width:200 }} placeholder="1101" /></label>
              <div style={{ fontSize:12, opacity:0.7 }}>g は 0/1 の列（低次→高次）</div>
            </div>
          )}
          {chAlgo==='hamming74' && <div style={{ fontSize:12, opacity:0.8 }}>Hamming(7,4): 追加設定なし</div>}
        </SectionPanelWithTitle>

        <SectionPanelWithTitle title="通信路（BSC）">
          <ErrorRateControl p={p} seed={seed} onChange={({p:pp, seed:s})=> { if (pp!=null) setP(pp); if (s!=null) setSeed(s) }} />
        </SectionPanelWithTitle>

        <SectionPanelWithTitle title="入力（テキスト）">
          <textarea value={text} onChange={(e)=> setText(e.target.value)} rows={3} style={{ width:'100%', boxSizing:'border-box' }} />
        </SectionPanelWithTitle>

        <div>
          <button onClick={onRun}>通信する</button>
        </div>

        <SectionPanelWithTitle title="結果">
          {err && <div style={{ color:'crimson', marginBottom:8 }}>{err}</div>}
          <div style={{ display:'grid', gap:14 }}>
            {/* 入力 */}
            <div style={{ border:'1px solid #ddd', borderRadius:8, padding:10 }}>
              <div style={{ fontSize:12, opacity:0.7 }}>① 入力テキスト（{inputBytesLen} bytes）</div>
              <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{printable(text, 800)}</div>
            </div>
            {/* 情報源 */}
            <div style={{ display:'grid', gap:8 }}>
              <div style={{ display:'flex', alignItems:'center', gap:8 }}>
                <span>➡️</span>
                <span style={{ fontWeight:600 }}>情報源符号: {srcAlgo.toUpperCase()}</span>
              </div>
              <div style={{ border:'1px solid #ddd', borderRadius:8, padding:10 }}>
                <div style={{ fontSize:12, opacity:0.7 }}>圧縮結果（{srcOutBytesLen} bytes）</div>
                <pre style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>{srcPayloadPreview || '-'}</pre>
              </div>
            </div>
            {/* チャネル＋BSC */}
            <div style={{ display:'grid', gap:8 }}>
              <div style={{ display:'flex', alignItems:'center', gap:8 }}>
                <span>➡️</span>
                <span style={{ fontWeight:600 }}>チャネル符号: {chAlgo.toUpperCase()}</span>
              </div>
              <div style={{ display:'grid', gap:8 }}>
                <div style={{ border:'1px solid #ddd', borderRadius:8, padding:10 }}>
                  <div style={{ fontSize:12, opacity:0.7 }}>符号語（{codeUnit==='bytes' ? `${recvLen} bytes` : `${recvLen} bits`}）</div>
                  <pre style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>{codewordPreview || '-'}</pre>
                </div>
                <div style={{ border:'1px solid #ddd', borderRadius:8, padding:10 }}>
                  <div style={{ fontSize:12, opacity:0.7 }}>BSC: p={p}, flips={flips}, BER≈{ber.toFixed(5)}</div>
                </div>
                <div style={{ border:'1px solid #ddd', borderRadius:8, padding:10 }}>
                  <div style={{ fontSize:12, opacity:0.7 }}>受信語</div>
                  <pre style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>{receivedPreview || '-'}</pre>
                </div>
              </div>
            </div>
            {/* 復号 */}
            <div style={{ display:'grid', gap:8 }}>
              <div style={{ display:'flex', alignItems:'center', gap:8 }}>
                <span>➡️</span>
                <span style={{ fontWeight:600 }}>復号結果</span>
              </div>
              <div style={{ border:'1px solid #ddd', borderRadius:8, padding:10 }}>
                <div style={{ fontSize:12, opacity:0.7 }}>復号データ</div>
                <pre style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>{decodedPreview || '-'}</pre>
                <div style={{ fontSize:12, opacity:0.7, marginTop:8 }}>テキスト</div>
                {!invalidJson && (
                  <pre style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{restoredText || '-'}</pre>
                )}
                {invalidJson && (
                  <div style={{ display:'grid', gap:6 }}>
                    <div style={{ color:'crimson' }}>注意: 受信データは有効な JSON ではありません。</div>
                    <div>
                      <div style={{ fontSize:12, opacity:0.7 }}>バイナリ（先頭プレビュー・hex）</div>
                      <pre style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>{recoveredBytesLast ? bytesToHex(recoveredBytesLast).slice(0,800) : '-'}</pre>
                    </div>
                    <div>
                      <div style={{ fontSize:12, opacity:0.7 }}>テキスト（不可視文字は·で表示）</div>
                      <pre style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{recoveredBytesLast ? printable(bytesToText(recoveredBytesLast), 800) : '-'}</pre>
                    </div>
                  </div>
                )}
              </div>
            </div>
            <div style={{ fontWeight:700, fontSize:16, color: isSuccess==null ? 'inherit' : (isSuccess ? 'seagreen' : 'crimson') }}>
              通信{isSuccess==null ? '' : (isSuccess ? '成功' : '失敗')}
            </div>
          </div>
        </SectionPanelWithTitle>
      </div>
    </PageContainer>
  )
}

export default function CommPipelinePage() {
  return (
    <Suspense fallback={<div />}> 
      <CommPipelinePageInner />
    </Suspense>
  )
}
