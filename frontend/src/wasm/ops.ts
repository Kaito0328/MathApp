// Use browser-only loader to avoid bundling Node 'node:' imports in client
import { initWasm as getBrowserWasm } from '../../app/lib/wasm'

// 1D DFT/IDFT helpers
export async function dft1d(signal: number[]): Promise<number[]> {
  const wasm = await getBrowserWasm() as any
  const x = new Float64Array(signal)
  const out: Float64Array = wasm.dftComplexF64(x)
  return Array.from(out)
}

export async function idft1d(spectrumInterleaved: number[]): Promise<number[]> {
  const wasm = await getBrowserWasm() as any
  const spec = new Float64Array(spectrumInterleaved)
  const out: Float64Array = wasm.iftComplexF64(spec)
  return Array.from(out)
}

// Discrete Transfer Function operations
export async function discreteTfImpulse(tf: { num: number[]; den: number[]; sample_time: number }, len = 128): Promise<number[]> {
  const wasm = await getBrowserWasm() as any
  const inst = new wasm.WasmDiscreteTF(new Float64Array(tf.num), new Float64Array(tf.den), tf.sample_time)
  const y: Float64Array = inst.impulse_response(len)
  inst.free?.()
  return Array.from(y)
}

export async function discreteTfStep(tf: { num: number[]; den: number[]; sample_time: number }, len = 128): Promise<number[]> {
  const wasm = await getBrowserWasm() as any
  const inst = new wasm.WasmDiscreteTF(new Float64Array(tf.num), new Float64Array(tf.den), tf.sample_time)
  const y: Float64Array = inst.step_response(len)
  inst.free?.()
  return Array.from(y)
}

export async function discreteTfBodeSvg(tf: { num: number[]; den: number[]; sample_time: number }, width = 360, height = 160, nPoints = 256, hzAxis = true, legend = false): Promise<string> {
  const wasm = await getBrowserWasm() as any
  const inst = new wasm.WasmDiscreteTF(new Float64Array(tf.num), new Float64Array(tf.den), tf.sample_time)
  const svg: string = inst.bode_svg(width, height, nPoints, hzAxis, legend)
  inst.free?.()
  return svg
}

export async function discreteTfNyquistSvg(tf: { num: number[]; den: number[]; sample_time: number }, width = 200, height = 200, nPoints = 256, showMinusOne = true, legend = false): Promise<string> {
  const wasm = await getBrowserWasm() as any
  const inst = new wasm.WasmDiscreteTF(new Float64Array(tf.num), new Float64Array(tf.den), tf.sample_time)
  const svg: string = inst.nyquist_svg(width, height, nPoints, showMinusOne, legend)
  inst.free?.()
  return svg
}
