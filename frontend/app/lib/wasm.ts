export async function initWasm() {
  // same pattern as Vite build: ESM glue default export initializes the wasm
  const init = (await import('../../src/wasm-pkg/wasm.js')).default as (input?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module) => Promise<unknown>
  const mod = await import('../../src/wasm-pkg/wasm.js')
  await init()
  return mod as unknown as {
    add(a: number, b: number): number
    dft_real(input: Float64Array): Float64Array
  }
}
