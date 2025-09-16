// High-level wrappers for source coding algorithms exposed by wasm-pkg
// This file provides ergonomic functions to call Jones, Lz78, Markov,
// SourceArithmetic, SourceHuffman, CraftCode, EliasGamma, BlockHuffman.
//
// Note: Shapes marked as `any` are forwarded from wasm-bindgen-generated JS.

import { getWasm } from './loader'

export type ProbVec = number[] | Float64Array

// ---------------- LZ78 ----------------
export async function lz78Encode(text: string) {
  const { Lz78 } = await getWasm()
  const lz = new Lz78()
  return lz.encodeInternal(text) as Array<any> // typically [index, char]
}

export async function lz78Decode(pairs: Array<any>) {
  const { Lz78 } = await getWasm()
  const lz = new Lz78()
  return lz.decodeInternal(pairs) as string
}

// ---------------- Source Huffman ----------------
export async function sourceHuffmanEncode(alphabet: string, probs: ProbVec, text: string) {
  const { SourceHuffman } = await getWasm()
  const probs64 = probs instanceof Float64Array ? probs : Float64Array.from(probs)
  const hf = new SourceHuffman(alphabet, probs64)
  return hf.encode(text) as Uint8Array
}

export async function sourceHuffmanDecode(alphabet: string, probs: ProbVec, length: number, bits: Uint8Array) {
  const { SourceHuffman } = await getWasm()
  const probs64 = probs instanceof Float64Array ? probs : Float64Array.from(probs)
  const hf = new SourceHuffman(alphabet, probs64)
  return hf.decode(length, bits) as string
}

// ---------------- Source Arithmetic ----------------
export async function sourceArithmeticEncode(alphabet: string, probs: ProbVec, text: string) {
  const { SourceArithmetic } = await getWasm()
  const probs64 = probs instanceof Float64Array ? probs : Float64Array.from(probs)
  const ar = new SourceArithmetic(alphabet, probs64)
  return ar.encode(text) as Uint8Array
}

export async function sourceArithmeticDecode(alphabet: string, probs: ProbVec, length: number, bits: Uint8Array) {
  const { SourceArithmetic } = await getWasm()
  const probs64 = probs instanceof Float64Array ? probs : Float64Array.from(probs)
  const ar = new SourceArithmetic(alphabet, probs64)
  return ar.decode(length, bits) as string
}

// ---------------- Jones code ----------------
export async function jonesEncode(alphabet: string, probs: ProbVec, total: number, text: string) {
  const { Jones } = await getWasm()
  const probs64 = probs instanceof Float64Array ? probs : Float64Array.from(probs)
  const j = new Jones(alphabet, probs64, total)
  return j.encode(text) as Uint8Array
}

export async function jonesDecode(alphabet: string, probs: ProbVec, total: number, length: number, bits: Uint8Array) {
  const { Jones } = await getWasm()
  const probs64 = probs instanceof Float64Array ? probs : Float64Array.from(probs)
  const j = new Jones(alphabet, probs64, total)
  return j.decode(length, bits) as string
}

// ---------------- Elias Gamma ----------------
export async function eliasGammaEncodeOne(n: number | bigint) {
  const { EliasGamma } = await getWasm()
  const big = typeof n === 'bigint' ? n : BigInt(n)
  return EliasGamma.encode(big) as Uint8Array
}

export async function eliasGammaEncodeMany(nums: Array<number | bigint>) {
  // Concatenate encodings
  const chunks = await Promise.all(nums.map((x) => eliasGammaEncodeOne(x)))
  let total = 0
  for (const c of chunks) total += c.length
  const out = new Uint8Array(total)
  let off = 0
  for (const c of chunks) { out.set(c, off); off += c.length }
  return out
}

export async function eliasGammaDecodeAll(bits: Uint8Array) {
  const { EliasGamma } = await getWasm()
  const results: number[] = []
  let start = 0
  // iterate until decode returns undefined or we pass end
  for (let guard = 0; guard < bits.length * 8 + 8; guard++) {
    const r = EliasGamma.decode(bits, start) as any
    if (!r) break
    // wasm.d.ts says any; expect [value:number|bigint, nextStart:number]
    const value = r[0]
    const next = r[1]
    if (typeof next !== 'number' || next <= start) break
    results.push(typeof value === 'bigint' ? Number(value) : (value as number))
    start = next
    if (start >= bits.length * 8) break
  }
  return results
}

// ---------------- Craft's inequality code assignment ----------------
export async function craftCodeBuild(alphabetSize: number, codeLengths: Uint32Array | number[]) {
  const { CraftCode } = await getWasm()
  const lens = codeLengths instanceof Uint32Array ? codeLengths : Uint32Array.from(codeLengths)
  return CraftCode.build(alphabetSize, lens) as Array<any>
}

// ---------------- Block Huffman ----------------
export async function blockHuffmanNew(q: number, blocksDistinct: any[], probs: ProbVec) {
  const { BlockHuffman } = await getWasm()
  const probs64 = probs instanceof Float64Array ? probs : Float64Array.from(probs)
  return new BlockHuffman(q, blocksDistinct, probs64)
}

export async function blockHuffmanEncode(inst: any, blocksSeq: any[]) {
  // Returns digit sequence (base-q) as Uint32Array
  return (inst.encode(blocksSeq) as Uint32Array)
}

export async function blockHuffmanDecode(inst: any, length: number, digits: Uint32Array) {
  return (inst.decode(length, digits) as any[])
}

// ---------------- Markov model ----------------
export async function markovBlockProbability(alphabet: string, initPr: ProbVec, condPr: Array<any>, symbols: string) {
  const { Markov } = await getWasm()
  const init64 = initPr instanceof Float64Array ? initPr : Float64Array.from(initPr)
  const m = new Markov(alphabet, init64, condPr)
  return m.blockPr(symbols) as number
}

// -------------- helpers --------------
export function estimateProbsFromText(alphabet: string, text: string): Float64Array {
  const counts = new Map<string, number>()
  for (const ch of text) {
    if (!alphabet.includes(ch)) continue
    counts.set(ch, (counts.get(ch) || 0) + 1)
  }
  const probs = new Float64Array(alphabet.length)
  const total = Array.from(counts.values()).reduce((a, b) => a + b, 0)
  for (let i = 0; i < alphabet.length; i++) {
    const c = counts.get(alphabet[i]) || 0
    probs[i] = total > 0 ? c / total : 1 / alphabet.length
  }
  return probs
}

export function hexOfBytes(bytes: Uint8Array | null | undefined) {
  if (!bytes) return '-'
  return Array.from(bytes).map((x) => x.toString(16).padStart(2, '0')).join('')
}
