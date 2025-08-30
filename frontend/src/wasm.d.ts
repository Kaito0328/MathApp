// Type declaration for our wasm init and functions
export interface WasmModule {
  add(a: number, b: number): number;
  dft_real_obj(input: number[], sample_rate: number): unknown; // runtime returns JS object
  dft_real(input: number[]): number[];
}

export async function initWasm(): Promise<WasmModule>;

// Allow importing the generated wasm-pack JS before it's built
declare module './wasm-pkg/wasm.js' {
  export function add(a: number, b: number): number;
  export function dft_real_obj(input: number[], sample_rate: number): unknown;
  export function dft_real(input: number[]): number[];
  const init: (input?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module) => Promise<unknown>;
  export default init;
}

// TS-side mirror of Rust's DftResult
export interface DftResult {
  spectrum: number[]; // interleaved [re, im, ...]
  sample_rate: number;
}
