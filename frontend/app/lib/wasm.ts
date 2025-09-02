// Browser-only WASM init for Next.js client components.
// Avoids importing Node modules (fs/url) so Webpack doesn't choke on "node:" scheme.
export async function initWasm() {
  const mod = await import('../../src/wasm-pkg/wasm.js')
  const init = mod.default as unknown as (options?: { module_or_path?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module }) => Promise<unknown>
  try {
    // Prefer letting wasm.js resolve wasm via its own import.meta.url
    await init()
  } catch {
    // Fallback: explicit URL (in case bundler didn't emit asset link)
    const wasmUrl = new URL('../../src/wasm-pkg/wasm_bg.wasm', import.meta.url)
    await init({ module_or_path: wasmUrl })
  }
  return mod as any
}
