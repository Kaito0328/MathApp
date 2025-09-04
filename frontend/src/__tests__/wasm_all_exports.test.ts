import { describe, it, expect } from 'vitest'
import { getWasm } from '../wasm/loader'

const d = typeof window === 'undefined' ? describe.skip : describe

d('wasm exported classes/functions (current API)', () => {
  it('MatrixF64 transpose', async () => {
    const w: any = await getWasm()
    const m = new w.MatrixF64(2, 1, new Float64Array([1, 2]))
    const t = m.transpose()
    expect(t.rows()).toBe(1)
    expect(t.cols()).toBe(2)
    t.free(); m.free()
  })

  it('VectorF64 basics', async () => {
    const w: any = await getWasm()
    const v = new w.VectorF64(new Float64Array([1, 2, 3]))
    expect(v.len()).toBe(3)
    expect(v.sum()).toBeCloseTo(6)
    v.free()
  })

  it('GF2/GF3 arithmetic', async () => {
    const w: any = await getWasm()
    const a2 = new w.WasmGF2(1n)
    const c2 = a2.add(w.WasmGF2.one())
    expect(c2.isZero).toBeTruthy()

    const a3 = new w.WasmGF3(2n)
    const d3 = a3.sub(w.WasmGF3.one())
    expect(d3.value).toBe(1n)
  })

  it('GF256 ctor/fromCoeffs', async () => {
    const w: any = await getWasm()
    const x = new w.WasmGF256(0xab)
    expect(x.toU8()).toBe(0xab)
    const y = w.WasmGF256.fromCoeffs(new Uint8Array([1,0,1,1]))
    expect(y.coeffs instanceof Uint8Array).toBeTruthy()
  })

  it('GFExtGF2 fromBase', async () => {
    const w: any = await getWasm()
    const px = new Uint8Array([1,1]) // x+1 (toy)
    const e = w.WasmGFExtGF2.fromBase(px, 0)
    expect(typeof e.isZero === 'boolean').toBeTruthy()
  })
})
