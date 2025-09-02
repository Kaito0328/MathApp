// Image <-> Vector helpers using wasm grayscale converters

import { getWasm } from '../wasm/bridge'
import { decodeToRgba, encodeRgbaToPng } from './png'

export interface GrayVector {
  width: number
  height: number
  gray: Float64Array // length = width*height, 0..255 scale as f64
}

export async function imageToGrayVector(input: Blob | ArrayBuffer | string): Promise<GrayVector> {
  const { width, height, rgba } = await decodeToRgba(input)
  const wasm = await getWasm() as any
  const gray = wasm.rgba_u8_to_gray_f64(rgba, width, height)
  return { width, height, gray }
}

export async function grayVectorToPng(gray: Float64Array, width: number, height: number): Promise<Blob> {
  const wasm = await getWasm() as any
  const rgba = wasm.gray_f64_to_rgba_u8(gray, width, height)
  return encodeRgbaToPng({ width, height, rgba })
}
