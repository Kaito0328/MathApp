"use client"
import React, { Suspense } from 'react'
import PageContainer from '../../../src/baseComponents/layout/PageContainer'
import SectionPanelWithTitle from '../../../src/components/composites/panels/SectionPanelWithTitle'
import OperationSelect from '../../../src/components/features/operations/OperationSelect'
import { TextInput } from '../../../src/baseComponents/input/TextInput'
import OperationBaseBlock from '../../../src/components/features/operations/OperationBaseBlock'
import { Button } from '../../../src/baseComponents/controls/Button'
import { useSearchParams, useRouter, usePathname } from 'next/navigation'
import {
  lz78Encode, lz78Decode,
  sourceHuffmanEncode, sourceHuffmanDecode,
  sourceArithmeticEncode, sourceArithmeticDecode,
  jonesEncode, jonesDecode,
  eliasGammaEncodeMany, eliasGammaDecodeAll,
  craftCodeBuild,
  blockHuffmanNew, blockHuffmanEncode, blockHuffmanDecode,
  markovBlockProbability,
  estimateProbsFromText, hexOfBytes
} from '../../../src/wasm/source'

type SourceAlgo = 'lz78' | 'huffman' | 'arithmetic' | 'jones' | 'elias' | 'craft' | 'blockHuffman' | 'markov'

function textToBytes(s: string): Uint8Array { return new TextEncoder().encode(s) }
function hexToBytes(hex: string): Uint8Array {
  const cleaned = hex.replace(/[^0-9a-fA-F]/g, '')
  const even = cleaned.length % 2 === 0 ? cleaned : ('0' + cleaned)
  const out = new Uint8Array(even.length / 2)
  for (let i = 0; i < out.length; i++) out[i] = parseInt(even.slice(2 * i, 2 * i + 2), 16)
  return out
}

function SourceUnifiedPageInner() {
  const searchParams = useSearchParams()
  const router = useRouter()
  const pathname = usePathname()
  const [algo, setAlgo] = React.useState<SourceAlgo>('lz78')
  const [text, setText] = React.useState('HELLO HELLO HELLO')
  const [alphabet, setAlphabet] = React.useState('ABCDEFGHIJKLMNOPQRSTUVWXYZ _')
  const [probs, setProbs] = React.useState<string>('')
  const [autoProb, setAutoProb] = React.useState(true)
  const [totalJones, setTotalJones] = React.useState(1000)
  const [blockSize, setBlockSize] = React.useState(2) // for BlockHuffman toy example
  const [digitsQ, setDigitsQ] = React.useState(4) // base-q alphabet for BlockHuffman digits
  const [compressed, setCompressed] = React.useState<Uint8Array | null>(null)
  const [restoredText, setRestoredText] = React.useState<string | null>(null)
  const [lzPairs, setLzPairs] = React.useState<any[] | null>(null)
  const [eliasNums, setEliasNums] = React.useState<string>('10, 3, 7, 15')
  const [craftLens, setCraftLens] = React.useState<string>('1,2,2,3')
  const [craftCodes, setCraftCodes] = React.useState<any[] | null>(null)
  const [blockDigits, setBlockDigits] = React.useState<Uint32Array | null>(null)
  const [blockDec, setBlockDec] = React.useState<any[] | null>(null)
  const [markovPr, setMarkovPr] = React.useState<number | null>(null)
  const [markovBits, setMarkovBits] = React.useState<{ log2: number; bits: number; bitsPerSymbol: number } | null>(null)
  const [err, setErr] = React.useState('')
  // 伸長用入力
  const [decodeInput, setDecodeInput] = React.useState<string>('')

  const parseProbs = React.useCallback(() => {
    if (autoProb) return estimateProbsFromText(alphabet, text)
    const parts = probs.split(/[ ,]+/).filter(Boolean).map(Number)
    const arr = new Float64Array(alphabet.length)
    for (let i = 0; i < alphabet.length; i++) arr[i] = parts[i] ?? (1 / alphabet.length)
    // normalize
    const s = arr.reduce((a,b)=>a+b,0)
    if (s > 0) for (let i=0;i<arr.length;i++) arr[i] /= s
    return arr
  }, [autoProb, probs, alphabet, text])

  const onEncode = async () => {
    setErr(''); setRestoredText(null); setLzPairs(null); setCraftCodes(null); setBlockDec(null); setMarkovPr(null); setMarkovBits(null)
    try {
      if (algo === 'huffman' || algo === 'arithmetic' || algo === 'jones' || algo === 'markov') {
        if (!alphabet || alphabet.length === 0) throw new Error('アルファベットが空です')
        for (const ch of text) if (!alphabet.includes(ch)) throw new Error(`アルファベットに含まれない文字: '${ch}'`)
      }
      if (algo === 'lz78') {
        const pairs = await lz78Encode(text)
        setLzPairs(pairs)
        // store as JSON for preview; also bytes as UTF-8 JSON
        const json = JSON.stringify(pairs)
        setCompressed(textToBytes(json))
        setDecodeInput(json)
      } else if (algo === 'huffman') {
        const pv = parseProbs()
        const bits = await sourceHuffmanEncode(alphabet, pv, text)
        setCompressed(bits)
        setDecodeInput(hexOfBytes(bits))
      } else if (algo === 'arithmetic') {
        const pv = parseProbs()
        const bits = await sourceArithmeticEncode(alphabet, pv, text)
        setCompressed(bits)
        setDecodeInput(hexOfBytes(bits))
      } else if (algo === 'jones') {
        const pv = parseProbs()
        const bits = await jonesEncode(alphabet, pv, totalJones, text)
        setCompressed(bits)
        setDecodeInput(hexOfBytes(bits))
      } else if (algo === 'elias') {
        const nums = eliasNums.split(/[ ,]+/).filter(Boolean).map((x)=>Number(x))
        const bits = await eliasGammaEncodeMany(nums)
        setCompressed(bits)
        setDecodeInput(hexOfBytes(bits))
      } else if (algo === 'craft') {
        const lens = craftLens.split(/[ ,]+/).filter(Boolean).map((x)=>parseInt(x,10))
        const codes = await craftCodeBuild(lens.length, Uint32Array.from(lens))
        setCraftCodes(codes)
        // store human-readable JSON into compressed for preview
        setCompressed(textToBytes(JSON.stringify(codes)))
        setDecodeInput('')
      } else if (algo === 'blockHuffman') {
        // build block list from text using fixed blockSize
        const blocks: string[] = []
        for (let i=0; i<text.length; i+=blockSize) blocks.push(text.slice(i, i+blockSize))
        const uniq = Array.from(new Set(blocks))
        const counts = new Map<string, number>()
        for (const b of blocks) counts.set(b, (counts.get(b) || 0) + 1)
        const probsArr = new Float64Array(uniq.length)
        for (let i=0;i<uniq.length;i++) probsArr[i] = (counts.get(uniq[i]) || 0) / blocks.length
        const inst = await blockHuffmanNew(digitsQ, uniq, probsArr)
        const digits = await blockHuffmanEncode(inst, blocks)
        setBlockDigits(digits)
        setCompressed(new Uint8Array(digits.buffer))
        setDecodeInput(Array.from(digits).join(','))
      } else if (algo === 'markov') {
        // Markovは確率評価のみ（エンコードなし）
        const pv = parseProbs() // use as initial prob vector (length should equal alphabet length)
        // Build simple conditional probs: identity (each char mostly follows itself)
        const cond: Array<any> = []
        for (let i=0;i<alphabet.length;i++) {
          const row = new Float64Array(alphabet.length)
          for (let j=0;j<alphabet.length;j++) row[j] = i===j ? 0.9 : (0.1/(alphabet.length-1))
          cond.push(row)
        }
        const pr = await markovBlockProbability(alphabet, pv, cond, text)
        setMarkovPr(pr)
        const log2 = Math.log2(Math.max(Number.EPSILON, pr))
        const bits = -log2
        const bitsPerSymbol = text.length > 0 ? (bits / text.length) : 0
        setMarkovBits({ log2, bits, bitsPerSymbol })
        setCompressed(null)
        setDecodeInput('')
      }
    } catch (e:any) {
      setErr(e?.message || String(e))
    }
  }

  const onDecode = async () => {
    setErr(''); setRestoredText(null)
    try {
      if (algo === 'lz78') {
        const payload = decodeInput?.trim() || ''
        if (!payload) return
        const pairs = JSON.parse(payload)
        const out = await lz78Decode(pairs)
        setRestoredText(out)
      } else if (algo === 'huffman') {
        const src = decodeInput?.trim() ? hexToBytes(decodeInput) : (compressed || undefined)
        if (!src) return
        const pv = parseProbs()
        const out = await sourceHuffmanDecode(alphabet, pv, text.length, src)
        setRestoredText(out)
      } else if (algo === 'arithmetic') {
        const src = decodeInput?.trim() ? hexToBytes(decodeInput) : (compressed || undefined)
        if (!src) return
        const pv = parseProbs()
        const out = await sourceArithmeticDecode(alphabet, pv, text.length, src)
        setRestoredText(out)
      } else if (algo === 'jones') {
        const src = decodeInput?.trim() ? hexToBytes(decodeInput) : (compressed || undefined)
        if (!src) return
        const pv = parseProbs()
        const out = await jonesDecode(alphabet, pv, totalJones, text.length, src)
        setRestoredText(out)
      } else if (algo === 'elias') {
        const src = decodeInput?.trim() ? hexToBytes(decodeInput) : (compressed || undefined)
        if (!src) return
        const nums = await eliasGammaDecodeAll(src)
        setRestoredText(nums.join(', '))
      } else if (algo === 'craft') {
        // decoding not provided by wasm; show codes only
        setRestoredText(null)
      } else if (algo === 'blockHuffman') {
        // allow decode from comma-separated digits input
        let digits: Uint32Array | null = null
        if (decodeInput?.trim()) {
          const parts = decodeInput.split(/[ ,]+/).filter(Boolean).map((x)=>parseInt(x,10))
          digits = new Uint32Array(parts)
        } else if (blockDigits) {
          digits = blockDigits
        }
        if (!digits) return
        // need original blocks length to decode
        const blocksLen = Math.ceil(text.length / blockSize)
        // Rebuild model as in encode path
        const blocks: string[] = []
        for (let i=0; i<text.length; i+=blockSize) blocks.push(text.slice(i, i+blockSize))
        const uniq = Array.from(new Set(blocks))
        const counts = new Map<string, number>()
        for (const b of blocks) counts.set(b, (counts.get(b) || 0) + 1)
        const probsArr = new Float64Array(uniq.length)
        for (let i=0;i<uniq.length;i++) probsArr[i] = (counts.get(uniq[i]) || 0) / blocks.length
        const inst = await blockHuffmanNew(digitsQ, uniq, probsArr)
        const rec = await blockHuffmanDecode(inst, blocksLen, digits)
        setBlockDec(rec)
        setRestoredText(rec.join(''))
      } else if (algo === 'markov') {
        // nothing to decode
      }
    } catch (e:any) {
      setErr(e?.message || String(e))
    }
  }

  const ratio = React.useMemo(() => {
    if (!compressed) return '-'
    const src = textToBytes(text).length
    const dst = compressed.length
    if (src === 0) return '-'
    return (dst / src).toFixed(3)
  }, [compressed, text])

  // init from ?algo
  React.useEffect(() => {
    const q = (searchParams?.get('algo') || '').toLowerCase()
    const vals: SourceAlgo[] = ['lz78','huffman','arithmetic','jones','elias','craft','blockHuffman','markov']
    if (vals.includes(q as SourceAlgo)) setAlgo(q as SourceAlgo)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])
  // update URL on change
  React.useEffect(() => {
    const usp = new URLSearchParams(searchParams?.toString())
    usp.set('algo', algo)
    router.replace(`${pathname}?${usp.toString()}`)
  }, [algo, pathname, router, searchParams])

  return (
    <PageContainer title="情報源符号（統合）" stickyHeader>
      <div style={{ display:'grid', gap:12 }}>
        {/* アルゴリズム選択 */}
        <SectionPanelWithTitle title="アルゴリズム選択">
          <OperationSelect
            operations={[
              { value:'lz78', label:'LZ78' },
              { value:'huffman', label:'Huffman' },
              { value:'arithmetic', label:'Arithmetic' },
              { value:'jones', label:'Jones' },
              { value:'elias', label:'Elias Gamma' },
              { value:'craft', label:"Craft's Code" },
              { value:'blockHuffman', label:'Block Huffman' },
              { value:'markov', label:'Markov' },
            ]}
            value={algo}
            onChange={(v)=> setAlgo(v as SourceAlgo)}
          />
        </SectionPanelWithTitle>

        {/* 設定ブロック（必要な場合のみ） */}
        {(algo==='huffman' || algo==='arithmetic' || algo==='jones' || algo==='markov') && (
          <SectionPanelWithTitle title="設定">
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
                  確率（空白区切り、アルファベット順）:
                  <TextInput value={probs} onChange={(e)=> setProbs((e.target as HTMLInputElement).value)} style={{ width:'100%' }} placeholder="0.1 0.2 0.3 ..." />
                </label>
              )}
              {algo==='jones' && (
                <label>
                  総符号長の目安（total）:
                  <input type="number" value={totalJones} onChange={(e)=> setTotalJones(parseInt(e.target.value||'0',10))} />
                </label>
              )}
            </div>
          </SectionPanelWithTitle>
        )}
        {algo==='elias' && (
          <SectionPanelWithTitle title="設定">
            <input value={eliasNums} onChange={(e)=> setEliasNums(e.target.value)} style={{ width:'100%' }} placeholder="10, 3, 7, 15" />
          </SectionPanelWithTitle>
        )}
        {algo==='craft' && (
          <SectionPanelWithTitle title="設定">
            <input value={craftLens} onChange={(e)=> setCraftLens(e.target.value)} style={{ width:'100%' }} placeholder="1,2,2,3" />
          </SectionPanelWithTitle>
        )}
        {algo==='blockHuffman' && (
          <SectionPanelWithTitle title="設定">
            <div style={{ display:'flex', gap:12, flexWrap:'wrap' }}>
              <label>ブロック長: <input type="number" value={blockSize} min={1} onChange={(e)=> setBlockSize(parseInt(e.target.value||'1',10))} /></label>
              <label>基数 q: <input type="number" value={digitsQ} min={2} onChange={(e)=> setDigitsQ(parseInt(e.target.value||'2',10))} /></label>
            </div>
          </SectionPanelWithTitle>
        )}

        {/* 圧縮ボタン */}
        <OperationBaseBlock left={null} center={<Button onClick={onEncode}>圧縮</Button>} right={null} />

        {/* 入力 */}
        <SectionPanelWithTitle title="入力">
          <textarea value={text} onChange={(e)=> setText(e.target.value)} rows={3} style={{ width:'100%', boxSizing:'border-box' }} />
          {err && <div style={{ color:'crimson' }}>{err}</div>}
        </SectionPanelWithTitle>

        {/* 圧縮結果 */}
        <SectionPanelWithTitle title="圧縮結果">
          <div style={{ fontFamily:'monospace', wordBreak:'break-all' }}>{hexOfBytes(compressed)}</div>
          {algo==='lz78' && lzPairs && (
            <div>
              <div>辞書対（index, char）:</div>
              <pre style={{ whiteSpace:'pre-wrap' }}>{JSON.stringify(lzPairs)}</pre>
            </div>
          )}
          {algo==='craft' && craftCodes && (
            <div>
              <div>割当コード:</div>
              <pre style={{ whiteSpace:'pre-wrap' }}>{JSON.stringify(craftCodes)}</pre>
            </div>
          )}
          {algo==='blockHuffman' && blockDigits && (
            <div>
              <div>符号化 digits（base-{digitsQ}）:</div>
              <div style={{ fontFamily:'monospace' }}>{Array.from(blockDigits).join(', ')}</div>
            </div>
          )}
          {algo==='markov' && markovPr!=null && (
            <div style={{ display:'grid', gap:4 }}>
              <div>系列確率（簡易）: {markovPr}</div>
              {markovBits && (
                <>
                  <div>−log2(Pr) = {markovBits.bits.toFixed(3)} bits</div>
                  <div>1 文字あたり ≈ {markovBits.bitsPerSymbol.toFixed(3)} bits/sym</div>
                </>
              )}
            </div>
          )}
        </SectionPanelWithTitle>

        {/* 伸長操作（圧縮結果の利用ボタンを右側に） */}
        <OperationBaseBlock
          left={null}
          center={<Button onClick={onDecode} disabled={algo==='craft' || algo==='markov'}>伸長</Button>}
          right={<Button onClick={() => {
            if (algo==='lz78' && lzPairs) setDecodeInput(JSON.stringify(lzPairs))
            else if (compressed) setDecodeInput(hexOfBytes(compressed))
            else if (algo==='blockHuffman' && blockDigits) setDecodeInput(Array.from(blockDigits).join(','))
          }}>圧縮結果を利用</Button>}
        />

        {/* 伸長する入力 */}
        {algo!=='markov' && (
          <SectionPanelWithTitle title="伸長する入力">
            {algo==='lz78' && (
              <textarea value={decodeInput} onChange={(e)=> setDecodeInput(e.target.value)} rows={4} style={{ width:'100%', boxSizing:'border-box' }} placeholder='[ [0,"H"], [1,"E"], ... ]' />
            )}
            {(algo==='huffman' || algo==='arithmetic' || algo==='jones' || algo==='elias') && (
              <textarea value={decodeInput} onChange={(e)=> setDecodeInput(e.target.value)} rows={3} style={{ width:'100%', boxSizing:'border-box' }} placeholder='0aff...' />
            )}
            {algo==='blockHuffman' && (
              <TextInput value={decodeInput} onChange={(e)=> setDecodeInput((e.target as HTMLInputElement).value)} style={{ width:'100%' }} placeholder='例: 3, 2, 5, 1, ...' />
            )}
            {algo==='craft' && (
              <div style={{ fontSize:12, opacity:0.75 }}>注: Craft は復号未対応です。</div>
            )}
          </SectionPanelWithTitle>
        )}

        {/* 伸長結果 */}
        <SectionPanelWithTitle title="伸長結果">
          <div style={{ fontFamily:'monospace', whiteSpace:'pre-wrap' }}>{restoredText ?? '-'}</div>
          {algo==='blockHuffman' && blockDec && (
            <div>
              <div>復号ブロック:</div>
              <pre style={{ whiteSpace:'pre-wrap' }}>{JSON.stringify(blockDec)}</pre>
            </div>
          )}
        </SectionPanelWithTitle>

        <SectionPanelWithTitle title="メトリクス">
          <div>圧縮率: {ratio}</div>
        </SectionPanelWithTitle>
      </div>
    </PageContainer>
  )
}

export default function SourceUnifiedPage() {
  return (
    <Suspense fallback={<div />}> 
      <SourceUnifiedPageInner />
    </Suspense>
  )
}
