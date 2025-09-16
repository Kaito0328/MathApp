"use client"
import React, { Suspense } from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import SectionPanelWithTitle from '../../../src/components/composites/panels/SectionPanelWithTitle'
import OperationSelect from '../../../src/components/features/operations/OperationSelect'
import OperationBaseBlock from '../../../src/components/features/operations/OperationBaseBlock'
import CodeCharacteristics from '../../../src/components/features/coding/CodeCharacteristics'
import { Button } from '../../../src/baseComponents/controls/Button'
import { PolynomialInput } from '../../../src/widgets/input/PolynomialInput'
import type { Polynomial } from '../../../src/widgets/dto/polynomial'
import ErrorRateControl from '../../../src/components/features/coding/common/ErrorRateControl'
import { useSearchParams, useRouter, usePathname } from 'next/navigation'

type InputMode = 'text' | 'hex' | 'binary'

type ChannelAlgo = 'rs' | 'bch' | 'cyclic' | 'hamming74'

function textToBytes(s: string): Uint8Array { return new TextEncoder().encode(s) }
function bytesToText(b: Uint8Array): string { return new TextDecoder().decode(b) }
function hexToBytes(hex: string): Uint8Array {
  const cleaned = hex.replace(/[^0-9a-fA-F]/g, '')
  const even = cleaned.length % 2 === 0 ? cleaned : ('0' + cleaned)
  const out = new Uint8Array(even.length / 2)
  for (let i = 0; i < out.length; i++) out[i] = parseInt(even.slice(2 * i, 2 * i + 2), 16)
  return out
}
function bytesToHex(b: Uint8Array): string { return Array.from(b).map(x=>x.toString(16).padStart(2,'0')).join('') }

// GF(2) 用
function textToBits(s: string): number[] {
  const bytes = new TextEncoder().encode(s)
  const bits: number[] = []
  for (const by of bytes) for (let i = 7; i >= 0; i--) bits.push((by >> i) & 1)
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
  const a = p.coeffs.map((x)=> (Math.round(x) & 1) >>> 0)
  let end = a.length
  while (end > 1 && a[end-1] === 0) end--
  return new Uint8Array(a.slice(0, end))
}

function ChannelUnifiedPageInner() {
  const searchParams = useSearchParams()
  const router = useRouter()
  const pathname = usePathname()
  const [algo, setAlgo] = React.useState<ChannelAlgo>('rs')
  const [mode, setMode] = React.useState<InputMode>('text')

  // RS state
  const [encoderMode, setEncoderMode] = React.useState<'auto' | 'standard' | 'advanced'>('auto')
  const [m, setM] = React.useState<number>(4)
  const [k, setK] = React.useState<number>(8)
  const [rsPx, setRsPx] = React.useState<Polynomial | null>(null)
  const [text, setText] = React.useState('hello rs')
  const [hex, setHex] = React.useState('')
  const [binary, setBinary] = React.useState('')
  const [encoded, setEncoded] = React.useState<Uint8Array | null>(null)
  const [noisy, setNoisy] = React.useState<Uint8Array | null>(null)
  const [decoded, setDecoded] = React.useState<Uint8Array | null>(null)
  const [err, setErr] = React.useState<string>('')
  const [origMsgLen, setOrigMsgLen] = React.useState<number | null>(null)
  const [autoAdjustLen, setAutoAdjustLen] = React.useState<'none' | 'pad' | 'trim'>('none')

  // BCH state
  const [bchEncoderMode, setBchEncoderMode] = React.useState<'auto' | 'standard' | 'advanced'>('auto')
  const [bchM, setBchM] = React.useState<number>(4)
  const [bchTDesign, setBchTDesign] = React.useState<number>(2)
  const [bchPx, setBchPx] = React.useState<Polynomial | null>(null)
  const [bchK, setBchK] = React.useState<number | null>(null)
  const [bchT, setBchT] = React.useState<number | null>(null)
  const [bchN, setBchN] = React.useState<number>(15)
  const [bchRecovered, setBchRecovered] = React.useState<Uint8Array | null>(null)

  // Cyclic state
  const [cycN, setCycN] = React.useState<number>(7)
  const [gPoly, setGPoly] = React.useState<Polynomial>({ coeffs: [1,1,0,1] })
  const [cycK, setCycK] = React.useState<number | null>(null)
  const [cycRecovered, setCycRecovered] = React.useState<Uint8Array | null>(null)
  const [cycNonZeroRemBlocks, setCycNonZeroRemBlocks] = React.useState<number>(0)
  const [gValid, setGValid] = React.useState<boolean | null>(null)

  // Hamming state: 特に追加不要（n=7,k=4 固定）
  // BSC (誤り注入) state
  const [p, setP] = React.useState(0.01)
  const [seed, setSeed] = React.useState('')
  const [flipCount, setFlipCount] = React.useState<number>(0)

  // Algo切替時に入力モードを安全側に調整
  React.useEffect(()=>{
    if (algo==='rs' && mode==='binary') setMode('text')
    if ((algo==='bch' || algo==='cyclic' || algo==='hamming74') && mode==='hex') setMode('text')
    // 共有の表示状態をクリア
    setEncoded(null); setNoisy(null); setDecoded(null); setErr(''); setOrigMsgLen(null)
    setBchRecovered(null); setCycRecovered(null); setCycNonZeroRemBlocks(0)
  }, [algo, mode])

  // 初期 URL の ?algo= を反映
  React.useEffect(() => {
    const q = (searchParams?.get('algo') || '').toLowerCase()
    if (q === 'rs' || q === 'bch' || q === 'cyclic' || q === 'hamming74') setAlgo(q as ChannelAlgo)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  // algo 変更時に URL を更新
  React.useEffect(() => {
    const usp = new URLSearchParams(searchParams?.toString())
    usp.set('algo', algo)
    router.replace(`${pathname}?${usp.toString()}`)
  }, [algo, pathname, router, searchParams])

  // BSC helpers
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
    const pos: number[] = []
    for (let i=0;i<out.length;i++) if (rand() < prob) { out[i] ^= 1; flips++; pos.push(i) }
    return { bits: out, flips, positions: pos }
  }
  const bscFlipBytes = (bytes: Uint8Array, prob: number, seedStr: string) => {
    const bits: number[] = []
    for (const by of bytes) for (let i=7;i>=0;i--) bits.push((by>>i)&1)
    const { bits: flipped, flips, positions } = bscFlipBits(bits, prob, seedStr)
    const nBytes = Math.ceil(flipped.length/8)
    const out = new Uint8Array(nBytes)
    for (let i=0;i<nBytes;i++) { let v=0; for (let j=0;j<8;j++){ const idx=i*8+j; v=(v<<1)| (idx<flipped.length ? (flipped[idx]&1) : 0)} out[i]=v }
    return { bytes: out, flips, positions }
  }

  // ===== RS =====
  const getMessage = React.useCallback((): Uint8Array => {
    return mode==='text' ? textToBytes(text) : hexToBytes(hex)
  }, [mode, text, hex])

  const pickMForLen = (len: number): { m: number, n: number } => {
    const target = Math.max(3, len)
    for (let mm = 2; mm <= 8; mm++) { const n = (1 << mm) - 1; if (n >= target) return { m: mm, n } }
    return { m: 8, n: (1 << 8) - 1 }
  }

  const msgLen = React.useMemo(() => (mode==='text' ? textToBytes(text).length : hexToBytes(hex).length), [mode, text, hex])
  const liveParams = React.useMemo(() => {
    if (encoderMode === 'auto') {
      const { m: mm, n } = pickMForLen(msgLen || 1)
      const kEff = Math.min(msgLen || 1, n)
      const tEff = Math.floor((n - kEff) / 2)
      return { m: mm, n, k: kEff, t: tEff }
    }
    const mm = Math.max(2, Math.min(8, Math.floor(m || 2)))
    const n = (1 << mm) - 1
    const kEff = Math.max(1, Math.min(n, Math.floor(k || 1)))
    const tEff = Math.floor((n - kEff) / 2)
    return { m: mm, n, k: kEff, t: tEff }
  }, [encoderMode, msgLen, m, k])

  const onEncodeRS = async () => {
    setErr(''); setDecoded(null)
    try {
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      const msg = getMessage()
      let nEff: number; let kEff: number; let f = new Uint8Array(msg)
      if (encoderMode !== 'auto') {
        const mm = Math.max(2, Math.min(8, Math.floor(m || 2)))
        nEff = (1 << mm) - 1
        kEff = Math.max(1, Math.min(nEff, Math.floor(k || 1)))
        if (autoAdjustLen === 'pad' && f.length < kEff) { const nf = new Uint8Array(kEff); nf.set(f); f = nf }
        else if (autoAdjustLen === 'trim' && f.length > kEff) { f = f.slice(0, kEff) }
      } else {
        const autoK = f.length || 1
        const picked = pickMForLen(autoK)
        nEff = picked.n; kEff = Math.min(autoK, nEff)
      }
      const blocks: Uint8Array[] = []
      for (let i = 0; i < f.length; i += kEff) {
        const chunk = f.subarray(i, Math.min(i + kEff, f.length))
        let u = chunk
        if (chunk.length < kEff) { const tmp = new Uint8Array(kEff); tmp.set(chunk); u = tmp }
        const rs = new wasm.ReedSolomon(kEff, nEff)
        const c: Uint8Array = rs.encode(u)
        blocks.push(c)
      }
      setK(kEff); setOrigMsgLen(f.length)
      const all = new Uint8Array(blocks.reduce((s, b)=> s + b.length, 0))
      let off = 0; for (const b of blocks) { all.set(b, off); off += b.length }
      setEncoded(all); setNoisy(all)
    } catch (e:any) { setErr(e?.message || String(e)) }
  }

  const onDecodeRS = async () => {
    setErr('')
    try {
      if (!noisy) return
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      let nEff: number; let kEff: number
      if (encoderMode === 'auto') { nEff = liveParams.n; kEff = liveParams.k }
      else { const mm = Math.max(2, Math.min(8, Math.floor(m || 2))); nEff = (1 << mm) - 1; kEff = Math.max(1, Math.min(nEff, Math.floor(k || 1))) }
      const r = new Uint8Array(noisy)
      if (r.length % nEff !== 0) throw new Error(`受信語の長さ (${r.length}) は n (${nEff}) の倍数である必要があります`)
      const out: number[] = []
      for (let i = 0; i < r.length; i += nEff) {
        const block = r.subarray(i, i + nEff)
        const rs = new wasm.ReedSolomon(kEff, nEff)
        const u: Uint8Array = rs.decodeBM(block)
        out.push(...Array.from(u))
      }
      setDecoded(new Uint8Array(out))
    } catch (e:any) { setErr(e?.message || String(e)) }
  }

  // ===== BCH =====
  const getBchMsgBits = React.useCallback((): number[] => {
    return mode==='text' ? textToBits(text) : (binary||'').split('').filter(c=>c==='0'||c==='1').map(c=>Number(c))
  }, [mode, text, binary])
  const bchMsgLen = React.useMemo(()=> getBchMsgBits().length, [getBchMsgBits])
  const pickBchMForLen = (len: number): { m: number, n: number } => {
    const target = Math.max(3, len)
    for (let mm = 2; mm <= 12; mm++) { const n = (1 << mm) - 1; if (n >= target) return { m: mm, n } }
    return { m: 12, n: (1 << 12) - 1 }
  }
  const bchLive = React.useMemo(()=>{
    if (bchEncoderMode==='auto') {
      const { m: mm, n } = pickBchMForLen(bchMsgLen || 1)
      const tGuess = Math.max(2, Math.floor((n - (bchMsgLen || 1)) / 2))
      return { m: mm, n, t: Math.max(1, tGuess) }
    }
    const mm = Math.max(2, Math.floor(bchM||2)); const n = (1<<mm) - 1
    return { m: mm, n, t: Math.max(1, Math.floor(bchTDesign||1)) }
  }, [bchEncoderMode, bchMsgLen, bchM, bchTDesign])

  const onEncodeBCH = async () => {
    setErr(''); setDecoded(null); setBchRecovered(null)
    try {
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      let bch: any
      if (bchEncoderMode==='auto') bch = wasm.BCH.newAuto(bchLive.m, Math.max(1, bchLive.t))
      else bch = wasm.BCH.newAuto(Math.max(2, Math.floor(bchM||2)), Math.max(1, Math.floor(bchTDesign||1)))
      const kEff: number = bch.k(); const tEff: number = bch.t(); const nAuto: number = bch.n()
      setBchK(kEff); setBchT(tEff); setBchN(nAuto)
      const bits = getBchMsgBits()
      const blocks = chunk(bits, kEff)
      if (blocks.length && blocks[blocks.length-1].length < kEff) { while (blocks[blocks.length-1].length < kEff) blocks[blocks.length-1].push(0) }
      const out: number[] = []
      for (const b of blocks) { const cw: Uint8Array = bch.encode(new Uint8Array(b)); out.push(...Array.from(cw)) }
      const code = new Uint8Array(out)
      setEncoded(code); setNoisy(code)
    } catch (e:any) { setErr(e?.message || String(e)) }
  }

  const onDecodeBCH = async () => {
    setErr('')
    try {
      if (!noisy) return
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      const bch = wasm.BCH.newAuto(Math.max(2, Math.floor(bchLive.m||2)), Math.max(1, Math.floor(bchLive.t||1)))
      const nLocal: number = bch.n(); const kEff: number = bch.k(); setBchK(kEff); setBchN(nLocal)
      const r = new Uint8Array(noisy)
      if (r.length % nLocal !== 0) throw new Error(`受信語の長さ (${r.length}) は n (${nLocal}) の倍数である必要があります`)
      const out: number[] = []; const msgOut: number[] = []
      for (let i = 0; i < r.length; i += nLocal) {
        const block = r.subarray(i, i + nLocal)
        const corr: Uint8Array = bch.decodeBM(block)
        out.push(...Array.from(corr))
        const tail = Array.from(corr.slice(nLocal - kEff, nLocal))
        msgOut.push(...tail)
      }
      setDecoded(new Uint8Array(out)); setBchRecovered(new Uint8Array(msgOut))
    } catch (e:any) { setErr(e?.message || String(e)) }
  }

  // ===== Cyclic =====
  const getCycMsgBits = React.useCallback((): number[] => {
    return mode==='text' ? textToBits(text) : (binary||'').split('').filter(c=>c==='0'||c==='1').map(c=>Number(c))
  }, [mode, text, binary])

  const onEncodeCyc = async () => {
    setErr(''); setDecoded(null); setCycRecovered(null); setCycNonZeroRemBlocks(0)
    try {
      const gCoeffs = normalizeBinaryCoeffs(gPoly)
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      // g | (x^n - 1) 検証
      const xPowN = new wasm.PolynomialGF2(new Uint8Array(Array.from({ length: cycN + 1 }, (_, i)=> i===cycN ? 1 : 0)))
      const one = new wasm.PolynomialGF2(new Uint8Array([1]))
      const xnMinus1 = xPowN.sub(one)
      const gPolyGF2 = new wasm.PolynomialGF2(gCoeffs)
      const remPair = xnMinus1.divRem(gPolyGF2)
      const remCoeffs: Uint8Array = remPair[1].coeffs()
      const isZero = Array.from(remCoeffs).every((v)=> v===0)
      setGValid(isZero)
      if (!isZero) throw new Error('生成多項式 g(x) は x^n - 1 を割り切りません')

      const cyc = new wasm.CyclicCode(cycN, gCoeffs)
      const kEff: number = cyc.k(); setCycK(kEff)
      const bits = getCycMsgBits()
      const blocks = chunk(bits, kEff)
      if (blocks.length && blocks[blocks.length-1].length < kEff) { while (blocks[blocks.length-1].length < kEff) blocks[blocks.length-1].push(0) }
      const out: number[] = []
      for (const b of blocks) { const cw: Uint8Array = cyc.encode(new Uint8Array(b)); out.push(...Array.from(cw)) }
      const code = new Uint8Array(out)
      setEncoded(code); setNoisy(code)
    } catch (e:any) { setErr(e?.message || String(e)) }
  }

  const onDecodeCyc = async () => {
    setErr('')
    try {
      if (!noisy) return
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      const gCoeffs = normalizeBinaryCoeffs(gPoly)
      const cyc = new wasm.CyclicCode(cycN, gCoeffs)
      const kEff: number = cyc.k(); setCycK(kEff)
      const r = new Uint8Array(noisy)
      if (r.length % cycN !== 0) throw new Error(`受信語の長さ (${r.length}) は n (${cycN}) の倍数である必要があります`)
      const out: number[] = []; const msgOut: number[] = []
      let nzRemBlocks = 0
      for (let i = 0; i < r.length; i += cycN) {
        const block = r.subarray(i, i + cycN)
        const corr: Uint8Array = cyc.decodeLUT(block)
        out.push(...Array.from(corr))
        const cwPoly = new wasm.PolynomialGF2(corr)
        const gPolyGF2 = new wasm.PolynomialGF2(gCoeffs)
        const [quot, rem] = cwPoly.divRem(gPolyGF2)
        const remCoeffs: Uint8Array = rem.coeffs(); if (Array.from(remCoeffs).some((v)=> v!==0)) nzRemBlocks++
        const quotCoeffs: Uint8Array = quot.coeffs(); const qArr = Array.from(quotCoeffs)
        while (qArr.length < kEff) qArr.push(0)
        msgOut.push(...qArr.slice(0, kEff))
      }
      setDecoded(new Uint8Array(out)); setCycRecovered(new Uint8Array(msgOut)); setCycNonZeroRemBlocks(nzRemBlocks)
    } catch (e:any) { setErr(e?.message || String(e)) }
  }

  // ===== Hamming(7,4) =====
  const getHamMsgBits = React.useCallback((): number[] => {
    return mode==='text' ? textToBits(text) : (binary||'').split('').filter(c=>c==='0'||c==='1').map(c=>Number(c))
  }, [mode, text, binary])

  const onEncodeHam = async () => {
    setErr(''); setDecoded(null)
    try {
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      const h = new wasm.Hamming74()
      const blocks = chunk(getHamMsgBits(), 4)
      if (blocks.length && blocks[blocks.length-1].length < 4) { while (blocks[blocks.length-1].length < 4) blocks[blocks.length-1].push(0) }
      const out: number[] = []
      for (const b of blocks) { const u = new Uint8Array(b); const c: Uint8Array = h.encode(u); out.push(...Array.from(c)) }
      const code = new Uint8Array(out); setEncoded(code); setNoisy(code)
    } catch (e:any) { setErr(e?.message || String(e)) }
  }

  const onDecodeHam = async () => {
    setErr('')
    try {
      if (!noisy) return
      const { getWasm } = await import('../../../src/wasm/loader')
      const wasm: any = await getWasm()
      const h = new wasm.Hamming74()
      const blocks = chunk(Array.from(noisy), 7)
      const msgBits: number[] = []
      for (const r of blocks) { const dec: Uint8Array = h.decode(new Uint8Array(r)); msgBits.push(dec[0], dec[1], dec[2], dec[3]) }
      setDecoded(new Uint8Array(msgBits))
    } catch (e:any) { setErr(e?.message || String(e)) }
  }

  return (
    <PageContainer title="チャネル符号（統合）" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        <SectionPanelWithTitle title="アルゴリズム選択">
          <OperationSelect
            operations={[
              { value: 'rs', label: 'Reed–Solomon' },
              { value: 'bch', label: 'BCH (GF(2))' },
              { value: 'cyclic', label: 'Cyclic (GF(2))' },
              { value: 'hamming74', label: 'Hamming(7,4)' },
            ]}
            value={algo}
            onChange={(v)=> setAlgo(v as ChannelAlgo)}
            label="アルゴリズム"
          />
        </SectionPanelWithTitle>

        <SectionPanelWithTitle title="デコード仕様（概要）">
          <div style={{ fontSize:12, opacity:0.85, lineHeight:1.5 }}>
            <div>・Reed–Solomon: 復号はメッセージ（長さ k）を返します。</div>
            <div>・BCH / Cyclic: 復号は訂正後コード語（長さ n）を返します（UIでメッセージ復元を表示）。</div>
            <div>・Hamming(7,4): 訂正後7bitから先頭4bitを情報ビットとして扱います。</div>
          </div>
        </SectionPanelWithTitle>

        {/* RS */}
        {algo==='rs' && (
          <SectionPanelWithTitle title="符号器設定 (Reed–Solomon)">
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
                  <div style={{ fontSize:12, opacity:0.85, marginBottom:6 }}>原始多項式（GF(2) 係数, 低次→高次）</div>
                  <PolynomialInput value={rsPx ?? { coeffs: [] }} onChange={(p)=> setRsPx({ coeffs: p.coeffs.map((x)=> Math.round(x) & 1) })} />
                  <div style={{ fontSize:12, opacity:0.7, marginTop:6 }}>注: 現在の WASM API は RS の原始多項式指定に未対応のため未適用です。</div>
                </div>
              )}
              {encoderMode !== 'auto' && (
                <div style={{ fontSize:12, opacity:0.8 }}>
                  現在の有効パラメータ: n={liveParams.n}, k={liveParams.k}, t={liveParams.t}
                </div>
              )}
            </div>
          </SectionPanelWithTitle>
        )}

        {algo==='rs' && (
          <SectionPanelWithTitle title="エンコード入力">
            <div style={{ display:'grid', gap:8 }}>
              <div style={{ display:'flex', gap:12, alignItems:'center' }}>
                <label><input type="radio" checked={mode==='text'} onChange={()=> setMode('text')} /> テキスト</label>
                <label><input type="radio" checked={mode==='hex'} onChange={()=> setMode('hex')} /> 16進</label>
              </div>
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
        )}

        {algo==='rs' && (
          <OperationBaseBlock left={null} center={<Button onClick={onEncodeRS}>符号化</Button>} right={null} />
        )}

        {algo==='rs' && (
          <SectionPanelWithTitle title="符号語（16進）">
            <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>{encoded ? bytesToHex(encoded) : '-'}</div>
          </SectionPanelWithTitle>
        )}

        {algo==='rs' && encoded && (
          <SectionPanelWithTitle title="誤り注入（BSC）">
            <div style={{ display:'grid', gap:8 }}>
              <ErrorRateControl p={p} seed={seed} onChange={({p:pp, seed:s})=> { if (pp!=null) setP(pp); if (s!=null) setSeed(s) }} />
              <div>
                <Button onClick={() => {
                  if (!encoded) return
                  const { bytes, flips } = bscFlipBytes(encoded, p, seed)
                  setNoisy(bytes); setFlipCount(flips)
                }}>エラー注入</Button>
              </div>
              <div style={{ fontSize:12, opacity:0.85 }}>
                反転数: {flipCount}（BER ≈ {encoded ? (flipCount / (encoded.length*8)).toFixed(5) : '-'}）
              </div>
            </div>
          </SectionPanelWithTitle>
        )}

        {algo==='rs' && (
          <>
            <OperationBaseBlock left={null} center={<Button onClick={onDecodeRS} disabled={!noisy}>復号</Button>} right={null} />
            <SectionPanelWithTitle title="受信語 16進">
              <div style={{ display:'grid', gap:8 }}>
                <textarea value={noisy ? bytesToHex(noisy) : ''} onChange={(e)=> setNoisy(hexToBytes(e.target.value))} rows={3} style={{ width:'100%', boxSizing:'border-box' }} />
              </div>
            </SectionPanelWithTitle>
            <SectionPanelWithTitle title="復号結果">
              <div style={{ display:'grid', gap:8 }}>
                <div>16進:</div>
                <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>{decoded ? bytesToHex(decoded) : '-'}</div>
                {mode==='text' && (
                  <>
                    <div>テキスト:</div>
                    <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{decoded ? bytesToText((()=>{ const d = decoded!; let end = d.length; if (origMsgLen!=null) end = Math.min(d.length, origMsgLen); else { while (end>0 && d[end-1]===0) end--; } return d.slice(0,end) })()) : '-'}</div>
                  </>
                )}
              </div>
            </SectionPanelWithTitle>
            <div style={{ opacity:0.75, fontSize:12 }}>注) RS の復号はメッセージを返します。入力長に応じて m/k を自動選択可能です。</div>
          </>
        )}

        {algo==='rs' && (
          <SectionPanelWithTitle title="符号の特性">
            <CodeCharacteristics lines={[
              `n = ${liveParams.n}`,
              `k = ${liveParams.k}`,
              `d_{\\min} = n - k + 1 = ${liveParams.n - liveParams.k + 1}`,
              `t = \\left\\lfloor \\tfrac{n-k}{2} \\right\\rfloor = ${liveParams.t}`,
              `R = \\tfrac{k}{n} = ${(liveParams.k && liveParams.n)?(liveParams.k/liveParams.n).toFixed(3):'-'}`,
            ]} />
          </SectionPanelWithTitle>
        )}
        {algo==='rs' && (
          <SectionPanelWithTitle title="メトリクス">
            <div style={{ display:'grid', gap:4 }}>
              <div>入力長（bytes）: {(() => mode==='text' ? textToBytes(text).length : hexToBytes(hex).length)()}</div>
              <div>符号語長（bytes）: {encoded?.length ?? '-'}</div>
              <div>反転数: {flipCount}</div>
              <div>BER（概算）: {encoded ? (flipCount / (encoded.length*8)).toFixed(5) : '-'}</div>
            </div>
          </SectionPanelWithTitle>
        )}

        {/* BCH */}
        {algo==='bch' && (
          <SectionPanelWithTitle title="符号器設定 (BCH)">
            <div style={{ display:'grid', gap:8 }}>
              <div style={{ display:'flex', gap:12, alignItems:'center', flexWrap:'wrap' }}>
                <label>入力モード:
                  <select value={bchEncoderMode} onChange={(e)=> setBchEncoderMode(e.target.value as any)} style={{ marginLeft:8 }}>
                    <option value="auto">自動</option>
                    <option value="standard">標準設定</option>
                    <option value="advanced">高度な設定</option>
                  </select>
                </label>
              </div>
              {bchEncoderMode !== 'auto' && (
                <div style={{ display:'flex', gap:12, alignItems:'center', flexWrap:'wrap' }}>
                  <label>m: <input type="number" value={bchM} onChange={(e)=> setBchM(Math.max(2, Math.floor(Number(e.target.value)||2)))} style={{ width: 120 }} /></label>
                  <label>t: <input type="number" value={bchTDesign} onChange={(e)=> setBchTDesign(Math.max(1, Math.floor(Number(e.target.value)||1)))} style={{ width: 120 }} /></label>
                </div>
              )}
              {bchEncoderMode === 'advanced' && (
                <div style={{ marginTop:8 }}>
                  <div style={{ fontSize:12, opacity:0.85, marginBottom:6 }}>原始多項式（UIのみ・未適用）</div>
                  <PolynomialInput value={bchPx ?? { coeffs: [] }} onChange={(p)=> setBchPx({ coeffs: p.coeffs.map((x)=> Math.round(x) & 1) })} />
                  <div style={{ fontSize:12, opacity:0.7, marginTop:6 }}>注: 現在の WASM API は多項式指定に未対応のため未適用です。</div>
                </div>
              )}
              <div style={{ fontSize:12, opacity:0.8 }}>現在の有効パラメータ（推定）: m={bchLive.m}, t={bchLive.t}</div>
            </div>
          </SectionPanelWithTitle>
        )}

        {algo==='bch' && (
          <SectionPanelWithTitle title="エンコード入力">
            <div style={{ display:'grid', gap:8 }}>
              <div style={{ display:'flex', gap:12, alignItems:'center' }}>
                <label><input type="radio" checked={mode==='text'} onChange={()=> setMode('text')} /> テキスト</label>
                <label><input type="radio" checked={mode==='binary'} onChange={()=> setMode('binary')} /> 2進</label>
              </div>
              {mode==='text' ? (
                <textarea value={text} onChange={(e)=> setText(e.target.value)} rows={3} style={{ width: '100%', boxSizing:'border-box' }} />
              ) : (
                <textarea value={binary} onChange={(e)=> setBinary(e.target.value)} rows={3} style={{ width: '100%', boxSizing:'border-box' }} placeholder="例) 1011001110" />
              )}
              {err && <div style={{ color:'crimson' }}>{err}</div>}
            </div>
          </SectionPanelWithTitle>
        )}
        {algo==='bch' && (<OperationBaseBlock left={null} center={<Button onClick={onEncodeBCH}>符号化</Button>} right={null} />)}
        {algo==='bch' && (
          <SectionPanelWithTitle title="符号語（2進）">
            <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>
              {encoded ? Array.from(encoded).map((b, i)=> ((i>0 && i% bchN===0) ? ' ' : '') + String(b)).join('') : '-'}
            </div>
          </SectionPanelWithTitle>
        )}
        {algo==='bch' && encoded && (
          <SectionPanelWithTitle title="誤り注入（BSC）">
            <div style={{ display:'grid', gap:8 }}>
              <ErrorRateControl p={p} seed={seed} onChange={({p:pp, seed:s})=> { if (pp!=null) setP(pp); if (s!=null) setSeed(s) }} />
              <div>
                <Button onClick={() => {
                  if (!encoded) return
                  const arr = Array.from(encoded)
                  const { bits, flips } = bscFlipBits(arr, p, seed)
                  setNoisy(new Uint8Array(bits)); setFlipCount(flips)
                }}>エラー注入</Button>
              </div>
              <div style={{ fontSize:12, opacity:0.85 }}>
                反転数: {flipCount}（BER ≈ {encoded ? (flipCount / (encoded.length)).toFixed(5) : '-'}）
              </div>
            </div>
          </SectionPanelWithTitle>
        )}
        {algo==='bch' && (
          <>
            <OperationBaseBlock left={null} center={<Button onClick={onDecodeBCH} disabled={!noisy}>復号</Button>} right={null} />
            <SectionPanelWithTitle title="受信語 2進">
              <div style={{ display:'grid', gap:8 }}>
                <textarea value={noisy ? Array.from(noisy).join('') : ''} onChange={(e)=> setNoisy(new Uint8Array((e.target.value||'').split('').filter(c=>c==='0'||c==='1').map(c=>Number(c))))} rows={3} style={{ width:'100%', boxSizing:'border-box' }} />
              </div>
            </SectionPanelWithTitle>
            <SectionPanelWithTitle title="復号結果（訂正後コード語）">
              <div style={{ display:'grid', gap:8 }}>
                <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{decoded ? Array.from(decoded).join('') : '-'}</div>
                <div style={{ opacity:0.8, fontSize:12 }}>注) BCH 復号は訂正後コード語を返します。systematic のため各ブロック末尾 k ビットがメッセージです。</div>
              </div>
            </SectionPanelWithTitle>
            <SectionPanelWithTitle title="復元メッセージ（各ブロック末尾 k ビット）">
              <div style={{ display:'grid', gap:8 }}>
                <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>
                  {bchRecovered ? Array.from(bchRecovered).map((b, i)=> ((i>0 && bchK!=null && i% (bchK||1)===0) ? ' ' : '') + String(b)).join('') : '-'}
                </div>
                {mode==='text' && (
                  <>
                    <div>メッセージ（テキスト解釈）:</div>
                    <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{bchRecovered ? bitsToText(Array.from(bchRecovered)) : '-'}</div>
                  </>
                )}
              </div>
            </SectionPanelWithTitle>
            <SectionPanelWithTitle title="符号の特性">
              <CodeCharacteristics lines={[ `n = ${bchN}`, `k = ${bchK ?? '-'}`, `d_{\\min} = -`, `t = ${bchT ?? '-'}`, `R = \\tfrac{k}{n} = ${bchK!=null ? (bchK/bchN).toFixed(3) : '-'}` ]} />
            </SectionPanelWithTitle>
            <SectionPanelWithTitle title="メトリクス">
              <div style={{ display:'grid', gap:4 }}>
                <div>入力長（bits）: {bchMsgLen}</div>
                <div>符号語長（bits）: {encoded?.length ?? '-'}</div>
                <div>反転数: {flipCount}</div>
                <div>BER（概算）: {encoded ? (flipCount / (encoded.length)).toFixed(5) : '-'}</div>
              </div>
            </SectionPanelWithTitle>
          </>
        )}

        {/* Cyclic */}
        {algo==='cyclic' && (
          <SectionPanelWithTitle title="符号器設定 (Cyclic)">
            <div style={{ display:'grid', gap:8 }}>
              <label>n: <input type="number" value={cycN} onChange={(e)=> setCycN(Math.max(1, Math.floor(Number(e.target.value)||1)))} style={{ width: 120, marginLeft:8 }} /></label>
              <div>
                <div style={{ fontSize:12, opacity:0.8, marginBottom:6 }}>生成多項式 g（GF(2) 係数, 低次→高次）</div>
                <PolynomialInput value={gPoly} onChange={setGPoly} />
                {gValid===false && <div style={{ color:'crimson', marginTop:6 }}>g(x) は x^n - 1 を割り切っていません</div>}
                {gValid===true && <div style={{ color:'seagreen', marginTop:6, opacity:0.8 }}>g(x) は x^n - 1 を割り切ります</div>}
              </div>
            </div>
          </SectionPanelWithTitle>
        )}

        {algo==='cyclic' && (
          <SectionPanelWithTitle title="入力">
            <div style={{ display:'grid', gap:8 }}>
              <div style={{ display:'flex', gap:12, alignItems:'center' }}>
                <label><input type="radio" checked={mode==='text'} onChange={()=> setMode('text')} /> テキスト</label>
                <label><input type="radio" checked={mode==='binary'} onChange={()=> setMode('binary')} /> 2進</label>
              </div>
              {mode==='text' ? (
                <textarea value={text} onChange={(e)=> setText(e.target.value)} rows={3} style={{ width: '100%', boxSizing:'border-box' }} />
              ) : (
                <textarea value={binary} onChange={(e)=> setBinary(e.target.value)} rows={3} style={{ width: '100%', boxSizing:'border-box' }} placeholder="例) 1011001110" />
              )}
              {err && <div style={{ color:'crimson' }}>{err}</div>}
            </div>
          </SectionPanelWithTitle>
        )}

        {algo==='cyclic' && (<OperationBaseBlock left={null} center={<Button onClick={onEncodeCyc}>符号化</Button>} right={null} />)}

        {algo==='cyclic' && (
          <SectionPanelWithTitle title="符号語（2進）">
            <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>
              {encoded ? Array.from(encoded).map((b, i)=> ((i>0 && cycK!=null && i% cycN===0) ? ' ' : '') + String(b)).join('') : '-'}
            </div>
          </SectionPanelWithTitle>
        )}
        {algo==='cyclic' && encoded && (
          <SectionPanelWithTitle title="誤り注入（BSC）">
            <div style={{ display:'grid', gap:8 }}>
              <ErrorRateControl p={p} seed={seed} onChange={({p:pp, seed:s})=> { if (pp!=null) setP(pp); if (s!=null) setSeed(s) }} />
              <div>
                <Button onClick={() => {
                  if (!encoded) return
                  const arr = Array.from(encoded)
                  const { bits, flips } = bscFlipBits(arr, p, seed)
                  setNoisy(new Uint8Array(bits)); setFlipCount(flips)
                }}>エラー注入</Button>
              </div>
              <div style={{ fontSize:12, opacity:0.85 }}>
                反転数: {flipCount}（BER ≈ {encoded ? (flipCount / (encoded.length)).toFixed(5) : '-'}）
              </div>
            </div>
          </SectionPanelWithTitle>
        )}

        {algo==='cyclic' && (
          <>
            <OperationBaseBlock left={null} center={<Button onClick={onDecodeCyc} disabled={!noisy}>復号</Button>} right={null} />
            <SectionPanelWithTitle title="受信語 2進">
              <div style={{ display:'grid', gap:8 }}>
                <textarea value={noisy ? Array.from(noisy).join('') : ''} onChange={(e)=> setNoisy(new Uint8Array((e.target.value||'').split('').filter(c=>c==='0'||c==='1').map(c=>Number(c))))} rows={3} style={{ width:'100%', boxSizing:'border-box' }} />
              </div>
            </SectionPanelWithTitle>
            <SectionPanelWithTitle title="復号結果（訂正後コード語）">
              <div style={{ display:'grid', gap:8 }}>
                <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{decoded ? Array.from(decoded).join('') : '-'}</div>
                {cycNonZeroRemBlocks>0 && (
                  <div style={{ fontSize:12, color:'crimson' }}>注意: g(x) で割った余りが 0 でないブロック数: {cycNonZeroRemBlocks}</div>
                )}
                <div style={{ opacity:0.8, fontSize:12 }}>注) 復号は訂正後コード語を返します。メッセージは各ブロックを g(x) で割った商として復元します。</div>
              </div>
            </SectionPanelWithTitle>
            <SectionPanelWithTitle title="復元メッセージ（g で割った商）">
              <div style={{ display:'grid', gap:8 }}>
                <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{cycRecovered ? Array.from(cycRecovered).map((b, i)=> ((i>0 && cycK!=null && i% (cycK||1)===0) ? ' ' : '') + String(b)).join('') : '-'}</div>
                {mode==='text' && (
                  <>
                    <div>メッセージ（テキスト解釈）:</div>
                    <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{cycRecovered ? bitsToText(Array.from(cycRecovered)) : '-'}</div>
                  </>
                )}
              </div>
            </SectionPanelWithTitle>
            <SectionPanelWithTitle title="符号の特性">
              <CodeCharacteristics lines={[ `n = ${cycN}`, `k = ${cycK ?? '-'}`, `d_{\\min} = -`, `t = -`, `R = \\tfrac{k}{n} = ${cycK!=null ? (cycK/cycN).toFixed(3) : '-'}` ]} />
            </SectionPanelWithTitle>
            <SectionPanelWithTitle title="メトリクス">
              <div style={{ display:'grid', gap:4 }}>
                <div>入力長（bits）: {getCycMsgBits().length}</div>
                <div>符号語長（bits）: {encoded?.length ?? '-'}</div>
                <div>反転数: {flipCount}</div>
                <div>BER（概算）: {encoded ? (flipCount / (encoded.length)).toFixed(5) : '-'}</div>
              </div>
            </SectionPanelWithTitle>
          </>
        )}

        {/* Hamming(7,4) */}
        {algo==='hamming74' && (
          <SectionPanelWithTitle title="入力">
            <div style={{ display:'grid', gap:8 }}>
              <div style={{ display:'flex', gap:12, alignItems:'center' }}>
                <label><input type="radio" checked={mode==='text'} onChange={()=> setMode('text')} /> テキスト</label>
                <label><input type="radio" checked={mode==='binary'} onChange={()=> setMode('binary')} /> 2進</label>
              </div>
              {mode==='text' ? (
                <textarea value={text} onChange={(e)=> setText(e.target.value)} rows={3} style={{ width: '100%', boxSizing:'border-box' }} />
              ) : (
                <textarea value={binary} onChange={(e)=> setBinary(e.target.value)} rows={3} style={{ width: '100%', boxSizing:'border-box' }} placeholder="例) 1011001110" />
              )}
              {err && <div style={{ color:'crimson' }}>{err}</div>}
            </div>
          </SectionPanelWithTitle>
        )}

        {algo==='hamming74' && (<OperationBaseBlock left={null} center={<Button onClick={onEncodeHam}>符号化</Button>} right={null} />)}

        {algo==='hamming74' && (
          <SectionPanelWithTitle title="符号語（2進）">
            <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap', wordBreak:'break-all' }}>
              {encoded ? Array.from(encoded).map((b, i)=> ((i>0 && i%7===0) ? ' ' : '') + String(b)).join('') : '-'}
            </div>
          </SectionPanelWithTitle>
        )}
        {algo==='hamming74' && encoded && (
          <SectionPanelWithTitle title="誤り注入（BSC）">
            <div style={{ display:'grid', gap:8 }}>
              <ErrorRateControl p={p} seed={seed} onChange={({p:pp, seed:s})=> { if (pp!=null) setP(pp); if (s!=null) setSeed(s) }} />
              <div>
                <Button onClick={() => {
                  if (!encoded) return
                  const arr = Array.from(encoded)
                  const { bits, flips } = bscFlipBits(arr, p, seed)
                  setNoisy(new Uint8Array(bits)); setFlipCount(flips)
                }}>エラー注入</Button>
              </div>
              <div style={{ fontSize:12, opacity:0.85 }}>
                反転数: {flipCount}（BER ≈ {encoded ? (flipCount / (encoded.length)).toFixed(5) : '-'}）
              </div>
            </div>
          </SectionPanelWithTitle>
        )}

        {algo==='hamming74' && (
          <>
            <OperationBaseBlock left={null} center={<Button onClick={onDecodeHam} disabled={!noisy}>復号</Button>} right={null} />
            <SectionPanelWithTitle title="受信語 2進">
              <div style={{ display:'grid', gap:8 }}>
                <textarea value={noisy ? Array.from(noisy).join('') : ''} onChange={(e)=> setNoisy(new Uint8Array((e.target.value||'').split('').filter(c=>c==='0'||c==='1').map(c=>Number(c))))} rows={3} style={{ width:'100%', boxSizing:'border-box' }} />
              </div>
            </SectionPanelWithTitle>
            <SectionPanelWithTitle title="復号結果">
              <div style={{ display:'grid', gap:8 }}>
                <div>復号（情報ビット列 Binary）:</div>
                <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{decoded ? Array.from(decoded).join('') : '-'}</div>
                {mode==='text' && (
                  <>
                    <div>復号（テキスト解釈）:</div>
                    <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{decoded ? bitsToText(Array.from(decoded)) : '-'}</div>
                  </>
                )}
                <div style={{ opacity:0.8, fontSize:12 }}>注) Hamming(7,4) は 4bit→7bit。不足分は 0 パディング。</div>
              </div>
            </SectionPanelWithTitle>
            <SectionPanelWithTitle title="符号の特性">
              <CodeCharacteristics lines={[ `n = 7`, `k = 4`, `d_{\\min} = 3`, `t = 1`, `R = \\tfrac{4}{7} = ${(4/7).toFixed(3)}` ]} />
            </SectionPanelWithTitle>
            <SectionPanelWithTitle title="メトリクス">
              <div style={{ display:'grid', gap:4 }}>
                <div>入力長（bits）: {getHamMsgBits().length}</div>
                <div>符号語長（bits）: {encoded?.length ?? '-'}</div>
                <div>反転数: {flipCount}</div>
                <div>BER（概算）: {encoded ? (flipCount / (encoded.length)).toFixed(5) : '-'}</div>
              </div>
            </SectionPanelWithTitle>
          </>
        )}
      </div>
    </PageContainer>
  )
}

export default function ChannelUnifiedPage() {
  return (
    <Suspense fallback={<div />}> 
      <ChannelUnifiedPageInner />
    </Suspense>
  )
}
