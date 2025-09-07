import { describe, it, expect } from 'vitest'
import { getWasm } from '../wasm/loader'

// Skip when running under Vitest (Node) where ESM Wasm import isn't supported without plugins
const d = typeof window === 'undefined' ? describe.skip : describe

d('wasm core smoke', () => {
  it('initializes and exposes classes', async () => {
    const wasm = await getWasm() as any
    expect(typeof wasm).toBe('object')
    expect(typeof wasm.MatrixF64).toBe('function')
    expect(typeof wasm.WasmGF2).toBe('function')
  })
})

// removed DirectMatrix/GF interop helpers; tests now focus on direct wasm exports only
