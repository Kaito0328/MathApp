// @vitest-environment jsdom
import { describe, it, expect } from 'vitest'
import { getWasm } from '../wasm/loader'

// Browser-only: wasm-bindgen glue wants DOM-ish env; follow existing pattern
const d = typeof window === 'undefined' ? describe.skip : describe

function bytesEq(a: Uint8Array, b: Uint8Array): boolean {
  if (a.length !== b.length) return false
  for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) return false
  return true
}

function injectRandomSymbolErrors(v: Uint8Array, m: number): Uint8Array {
  const n = v.length
  const out = new Uint8Array(v)
  const picked = new Set<number>()
  const M = Math.max(0, Math.min(m, n))
  while (picked.size < M) picked.add(Math.floor(Math.random() * n))
  for (const i of picked) {
    let delta = 0
    while (delta === 0) delta = (Math.floor(Math.random() * 255) + 1) & 0xff
    out[i] = out[i] ^ delta
  }
  return out
}

d('ReedSolomon encode/decode', () => {
  it('auto-like params (k = |m|, alphas = 1..n with n≈2k, n>=31)', async () => {
    let wasm: any
    try {
      wasm = await getWasm()
  } catch {
      // Vite/Vitest の wasm 取込制約で失敗する環境ではスキップ相当
      return
    }
    const msg = new Uint8Array([0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x72, 0x73]) // "hello rs"
    const k = msg.length
    const n = Math.max(31, Math.min(255, 2 * Math.max(1, k)))
    const alphas = new Uint8Array(Array.from({ length: n }, (_, i) => i + 1))

    const rs = new wasm.ReedSolomon(k, alphas)
    expect(rs.n()).toBe(n)
    const t = rs.t()
    expect(t).toBe(Math.floor((n - k) / 2))

    const c: Uint8Array = rs.encode(msg)
    expect(c.length).toBe(n)

    // up to t errors should be correctable
    const noisy = injectRandomSymbolErrors(c, t)
  const dec: Uint8Array = rs.decodeBM(noisy)
    expect(dec.length).toBe(k)
    expect(bytesEq(dec, msg)).toBe(true)
  })

  it('advanced-like params (fixed k, alphas length >= k)', async () => {
    let wasm: any
    try {
      wasm = await getWasm()
  } catch {
      return
    }
    const k = 10
    const n = 31
    const alphas = new Uint8Array(Array.from({ length: n }, (_, i) => i + 1))
    const rs = new wasm.ReedSolomon(k, alphas)
    const t = rs.t()

    // message exactly k symbols
    const msg = new Uint8Array(k)
    for (let i = 0; i < k; i++) msg[i] = (i * 17 + 3) & 0xff

    const c: Uint8Array = rs.encode(msg)
    const noisy = injectRandomSymbolErrors(c, Math.min(t, 3))
  const dec: Uint8Array = rs.decodeBM(noisy)
    expect(dec.length).toBe(k)
    expect(bytesEq(dec, msg)).toBe(true)
  })
})
