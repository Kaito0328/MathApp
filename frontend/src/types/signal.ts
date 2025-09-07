// TypeScript mirrors for Rust signal_processing crate structures used across WASM boundary

export interface Signal {
  data: number[];
  sample_rate: number;
}

export interface Complex64 {
  re: number;
  im: number;
}

export interface Spectrum {
  data: Complex64[]; // length N
  sample_rate: number;
}

export interface DftResult {
  // Interleaved [re, im, re, im, ...]
  spectrum: number[];
  sample_rate: number;
}
