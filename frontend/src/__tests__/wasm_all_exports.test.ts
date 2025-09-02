import { describe, it, expect } from 'vitest'
import { getWasm } from '../wasm'

describe('wasm exported classes/functions smoke tests', () => {
  it('MatrixF32 basic', async () => {
    const w: any = await getWasm()
    const m = new w.MatrixF32(1, 1)
    expect(m.rows()).toBe(1)
    expect(m.cols()).toBe(1)
    expect(m.is_square()).toBeTruthy()
    m.free()
  })

  it('MatrixF64 basic', async () => {
    const w: any = await getWasm()
    const m = new w.MatrixF64(2, 1)
    const t = m.transpose()
    expect(t.rows()).toBe(1)
    expect(t.cols()).toBe(2)
    t.free(); m.free()
  })

  it('MatrixI32 basic', async () => {
    const w: any = await getWasm()
    const m = new w.MatrixI32(1, 2)
    expect(m.is_square()).toBeFalsy()
    m.free()
  })

  it('MatrixI64 basic', async () => {
    const w: any = await getWasm()
    const m = new w.MatrixI64(1, 1)
    const tr = m.trace()
    expect(typeof tr === 'bigint' || typeof (tr as any) === 'number').toBeTruthy()
    m.free()
  })

  it('NewMatrixF64.with_default', async () => {
    const w: any = await getWasm()
    const m = w.NewMatrixF64.with_default(1, 2)
    expect(m.rows()).toBe(1)
    expect(m.cols()).toBe(2)
    m.free()
  })

  it('TestMatrixF64Direct ctor/transpose', async () => {
    const w: any = await getWasm()
    const data = new Float64Array([1,2,3,4,5,6])
    const m = new w.TestMatrixF64Direct(2, 3, data)
    const t = m.transpose()
    expect(t.get_rows()).toBe(3)
    expect(t.get_cols()).toBe(2)
    t.free(); m.free()
  })

  it('TestMatrixF64Simple is_square', async () => {
    const w: any = await getWasm()
    const m = new w.TestMatrixF64Simple(2, 3)
    expect(m.is_square()).toBeFalsy()
    m.free()
  })

  it('TestMatrixMacro new/rows/cols', async () => {
    const w: any = await getWasm()
    const data = new Float64Array([1,2,3,4])
    const m = new w.TestMatrixMacro(2, 2, data)
    expect(m.rows()).toBe(2)
    expect(m.cols()).toBe(2)
    m.free()
  })

  it('TestMatrixF64DeclarationOnly presence', async () => {
    const w: any = await getWasm()
    expect('TestMatrixF64DeclarationOnly' in w).toBeTruthy()
  })

  it('TestVectorF64Direct ctor/len', async () => {
    const w: any = await getWasm()
    const v = new w.TestVectorF64Direct(new Float64Array([1,2,3]))
    expect(v.len).toBe(3)
    expect(v.is_empty()).toBeFalsy()
    v.free()
  })

  it('VectorF32/F64/I32/I64 presence', async () => {
    const w: any = await getWasm()
    expect('VectorF32' in w).toBeTruthy()
    expect('VectorF64' in w).toBeTruthy()
    expect('VectorI32' in w).toBeTruthy()
    expect('VectorI64' in w).toBeTruthy()
  })

  it('SimpleGF2/GF3/GF5 arithmetic', async () => {
    const w: any = await getWasm()
    const a2 = new w.SimpleGF2(1n)
    const b2 = w.SimpleGF2.one()
    expect(a2.add(b2).value).toBe(0n)

    const a3 = new w.SimpleGF3(1n)
    expect(a3.add(w.SimpleGF3.one()).value).toBe(2n)

    const a5 = new w.SimpleGF5(2n)
    expect(a5.mul(w.SimpleGF5.one()).value).toBe(2n)
  })

  it('WasmGF2/GF3 arithmetic', async () => {
    const w: any = await getWasm()
    const a = new w.WasmGF2(1n)
    const c = a.add(w.WasmGF2.one())
    expect(c.isZero).toBeTruthy()

    const a3 = new w.WasmGF3(2n)
    expect(a3.sub(w.WasmGF3.one()).value).toBe(1n)
  })

  it('WasmGF256 ctor/fromCoeffs', async () => {
    const w: any = await getWasm()
    const x = new w.WasmGF256(0xab)
    expect(x.toU8()).toBe(0xab)
    const y = w.WasmGF256.fromCoeffs(new Uint8Array([1,0,1,1]))
    expect(y.coeffs instanceof Uint8Array).toBeTruthy()
  })

  it('WasmGFExtGF2 fromBase', async () => {
    const w: any = await getWasm()
    const px = new Uint8Array([1,1]) // x+1 (toy)
    const e = w.WasmGFExtGF2.fromBase(px, 0)
    expect(typeof e.isZero === 'boolean').toBeTruthy()
  })

  it('DirectMatrixF64 presence', async () => {
    const w: any = await getWasm()
    expect('DirectMatrixF64' in w).toBeTruthy()
  })
})
