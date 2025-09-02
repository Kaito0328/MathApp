import { describe, it, expect } from 'vitest'
import { getWasm } from '../wasm'
import { DirectMatrix, GF, toBigInt } from '../wasm/interop'

describe('wasm core smoke', () => {
  it('initializes and exposes classes', async () => {
    const wasm = await getWasm() as any
    expect(typeof wasm).toBe('object')
    expect(typeof wasm.TestMatrixF64Direct).toBe('function')
    expect(typeof wasm.WasmGF2).toBe('function')
  })
})

describe('DirectMatrix wrapper', () => {
  it('transposes dimensions', async () => {
    const dm = await DirectMatrix.from2D([[1,2,3],[4,5,6]])
    const t = dm.transpose()
    expect(t.rows).toBe(3)
    expect(t.cols).toBe(2)
    dm.free(); t.free()
  })
})

describe('GF helpers', () => {
  it('creates GF2 elements and adds', async () => {
    const a = await GF.gf2(1)
    const b = await GF.gf2(1)
    const c = a.add(b)
    // in GF(2) 1+1=0
    expect(c.isZero).toBeTruthy()
  })

  it('toBigInt works with string and number', () => {
    expect(toBigInt(10)).toBe(10n)
    expect(toBigInt('0xff')).toBe(255n)
  })
})
