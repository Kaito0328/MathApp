// Centralized loader for generated wasm-pkg. Keeps a singleton instance.
// This file is hand-written and lives separately from generated `src/wasm-pkg/*`.
// Avoid enumerating all exports: reuse the generated module type.
export type WasmExports = typeof import('../wasm-pkg/wasm.js')

let wasmPromise: Promise<WasmExports> | null = null
const fnCache = new Map<string, Promise<unknown>>()

export function getWasm(): Promise<WasmExports> {
  if (!wasmPromise) {
    wasmPromise = (async () => {
      // Ensure Web Crypto API exists (wasm-bindgen glue may rely on it)
      if (typeof globalThis !== 'undefined') {
        const g: any = globalThis as any
        if (!g.crypto || typeof g.crypto.getRandomValues !== 'function') {
          g.crypto = g.crypto || {}
          g.crypto.getRandomValues = function (arr: ArrayBufferView) {
            const u8 = arr instanceof Uint8Array ? arr : new Uint8Array((arr as any).buffer || arr)
            for (let i = 0; i < u8.length; i++) u8[i] = Math.floor(Math.random() * 256)
            return arr
          }
        }
      }
      const mod = await import('../wasm-pkg/wasm.js')
      const maybeInit = (mod as any)?.default
      if (typeof maybeInit === 'function') {
  // Always initialize with explicit URL to avoid bundler/env resolution issues
  const wasmUrl = new URL('../wasm-pkg/wasm_bg.wasm', import.meta.url)
  await maybeInit({ module_or_path: wasmUrl })
      }
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
