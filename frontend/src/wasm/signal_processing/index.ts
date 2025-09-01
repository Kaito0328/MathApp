// Signal processing safe wrappers around raw wasm exports
import { bind } from '../core/loader'
import type { DftResult } from '../../types'

// Domain types to avoid leaking raw Float64Array layout to the rest of the app
export class Complex64 {
  constructor(public re: number, public im: number) {}
}

export class Spectrum {
  constructor(public bins: Complex64[], public sample_rate: number) {}

  // Convert to interleaved Float64Array [re, im, ...] expected by wasm
  toInterleaved(): Float64Array {
    const out = new Float64Array(this.bins.length * 2)
    for (let i = 0; i < this.bins.length; i++) {
      out[2 * i] = this.bins[i].re
      out[2 * i + 1] = this.bins[i].im
    }
    return out
  }

  static fromInterleaved(pairs: Float64Array, sample_rate: number): Spectrum {
    const bins: Complex64[] = new Array(Math.floor(pairs.length / 2))
    for (let i = 0, j = 0; i < pairs.length; i += 2, j++) {
      bins[j] = new Complex64(pairs[i] ?? 0, pairs[i + 1] ?? 0)
    }
    return new Spectrum(bins, sample_rate)
  }
}

// DTO interfaces aligned with Rust-side serde structs
export interface SignalDto { data: number[]; sample_rate: number }
export interface ComplexPairDto { re: number; im: number }
export interface SpectrumDto { bins: ComplexPairDto[]; sample_rate: number }
export interface ConvDtoIn { x: number[]; h: number[] }
export interface ConvDtoOut { y: number[] }

// Lazily bind specific wasm functions once per module to avoid repeated
// module property lookups and make call sites cleaner.
const getDftReal = () => bind('dft_real') as Promise<(input: Float64Array) => Float64Array>
const getIftReal = () => bind('ift_real') as Promise<(pairs: Float64Array, sample_rate: number) => Float64Array>
const getConvReal = () => bind('conv_real') as Promise<(x: Float64Array, h: Float64Array) => Float64Array>

export async function dftReal(signal: { data: number[]; sample_rate: number }): Promise<DftResult> {
  const fn = await getDftReal()
  const out = fn(new Float64Array(signal.data))
  return { spectrum: Array.from(out), sample_rate: signal.sample_rate }
}

// Variant that returns the resulting Float64Array directly to avoid a copy.
// Useful for internal pipelines; UI can continue to use dftReal above.
export async function dftRealTA(signal: { data: number[]; sample_rate: number }): Promise<{ spectrum: Float64Array; sample_rate: number }> {
  const fn = await getDftReal()
  const out = fn(new Float64Array(signal.data))
  return { spectrum: out, sample_rate: signal.sample_rate }
}

export async function iftReal(result: DftResult | Spectrum): Promise<number[]> {
  const fn = await getIftReal()
  if (result instanceof Spectrum) {
    const y = fn(result.toInterleaved(), result.sample_rate)
    return Array.from(y)
  }
  const y = fn(new Float64Array(result.spectrum), result.sample_rate)
  return Array.from(y)
}

// DFT returning domain Spectrum directly
export async function dftRealSpectrum(signal: { data: number[]; sample_rate: number }): Promise<Spectrum> {
  const fn = await getDftReal()
  const out = fn(new Float64Array(signal.data))
  return Spectrum.fromInterleaved(out, signal.sample_rate)
}

// ===== Serde-based object APIs (if available), with fallback =====
export async function dftObj(signal: SignalDto): Promise<SpectrumDto> {
  // Try serde-based wasm export
  try {
  const dftAny = await (await import('../core/loader')).bind('dft') as unknown as ((s: unknown) => unknown)
    if (typeof dftAny === 'function') {
      const out = dftAny(signal) as SpectrumDto
      if (out && Array.isArray(out.bins)) return out
    }
  } catch { /* ignore and fallback */ }
  // Fallback to numeric API and adapt
  const { spectrum, sample_rate } = await dftReal(signal)
  const bins: ComplexPairDto[] = []
  for (let i = 0; i < spectrum.length; i += 2) {
    bins.push({ re: spectrum[i] ?? 0, im: spectrum[i + 1] ?? 0 })
  }
  return { bins, sample_rate }
}

export async function iftObj(spec: SpectrumDto): Promise<SignalDto> {
  try {
  const iftAny = await (await import('../core/loader')).bind('ift') as unknown as ((s: unknown) => unknown)
    if (typeof iftAny === 'function') {
      const out = iftAny(spec) as SignalDto
      if (out && Array.isArray(out.data)) return out
    }
  } catch { /* ignore and fallback */ }
  const inter = new Float64Array(spec.bins.length * 2)
  for (let i = 0; i < spec.bins.length; i++) { inter[2 * i] = spec.bins[i].re; inter[2 * i + 1] = spec.bins[i].im }
  // Use numeric ift and wrap
  const fn = await getIftReal()
  const y = Array.from(fn(inter, spec.sample_rate))
  return { data: y, sample_rate: spec.sample_rate }
}

export async function convObj(input: ConvDtoIn): Promise<ConvDtoOut> {
  try {
  const convAny = await (await import('../core/loader')).bind('conv') as unknown as ((c: unknown) => unknown)
    if (typeof convAny === 'function') {
      const out = convAny(input) as ConvDtoOut
      if (out && Array.isArray(out.y)) return out
    }
  } catch { /* ignore and fallback */ }
  const y = await convReal(input.x, input.h)
  return { y }
}

export async function convReal(x: number[], h: number[]): Promise<number[]> {
  const fn = await getConvReal()
  const y = fn(new Float64Array(x), new Float64Array(h))
  return Array.from(y)
}
