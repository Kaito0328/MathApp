// Loader for the wasm package built by wasm-pack into src/wasm-pkg
// We rely on Vite's native support for `--target web` output from wasm-pack (ESM glue).

export type { WasmModule, DftResult } from './wasm.d';

export async function initWasm() {
  // Import the init function and exported functions from the generated pkg
  // The out-name is `wasm`, so files are wasm.js and wasm_bg.wasm
  const init = (await import('./wasm-pkg/wasm.js')).default as (input?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module) => Promise<unknown>;
  const mod = await import('./wasm-pkg/wasm.js');
  await init();
  return mod as unknown as {
    add(a: number, b: number): number;
    dft_real_obj(input: number[], sample_rate: number): import('./types').DftResult;
    dft_real(input: number[]): number[];
  };
}
