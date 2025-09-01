// Centralized loader for generated wasm-pkg. Keeps a singleton instance.
// This file is hand-written and lives separately from generated `src/wasm-pkg/*`.
// Avoid enumerating all exports: reuse the generated module type.
export type WasmExports = typeof import('../../wasm-pkg/wasm.js')

let wasmPromise: Promise<WasmExports> | null = null
const fnCache = new Map<string, Promise<unknown>>()

export function getWasm(): Promise<WasmExports> {
  if (!wasmPromise) {
    wasmPromise = (async () => {
      const init = (await import('../../wasm-pkg/wasm.js')).default as (input?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module) => Promise<unknown>
      const mod = await import('../../wasm-pkg/wasm.js')
      await init()
  return mod as unknown as WasmExports
    })()
  }
  return wasmPromise
}

type WasmFuncKeys = Exclude<keyof WasmExports, 'default'>

// Lazily bind a specific export by name and cache the resolved function
export function bind<K extends WasmFuncKeys>(name: K): Promise<WasmExports[K]>
export function bind(name: string): Promise<unknown>
export function bind(name: string): Promise<unknown> {
  const key = String(name)
  if (!fnCache.has(key)) {
    fnCache.set(key, getWasm().then((w) => (w as any)[name] as unknown))
  }
  return fnCache.get(key)!
}
