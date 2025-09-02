// PNG <-> RGBA(Uint8Array) helpers for browser frontend
// - decode: Blob/ArrayBuffer/URL.createObjectURL -> ImageBitmap -> RGBA Uint8Array
// - encode: RGBA Uint8Array -> PNG Blob (via canvas)

export interface ImageDataU8 {
  width: number;
  height: number;
  rgba: Uint8Array; // length = width*height*4
}

type Ctx2D = OffscreenCanvasRenderingContext2D | CanvasRenderingContext2D;
interface CtxWrap { ctx: Ctx2D; off?: OffscreenCanvas; el?: HTMLCanvasElement }
function get2D(width: number, height: number): CtxWrap {
  if (typeof OffscreenCanvas !== 'undefined') {
    const off = new OffscreenCanvas(width, height);
    const ctx = off.getContext('2d');
    if (!ctx) throw new Error('2d context not available');
    return { ctx, off };
  }
  const el = document.createElement('canvas');
  el.width = width; el.height = height;
  const ctx = el.getContext('2d');
  if (!ctx) throw new Error('2d context not available');
  return { ctx, el };
}

export async function decodeToRgba(input: Blob | ArrayBuffer | string): Promise<ImageDataU8> {
  let blob: Blob;
  if (input instanceof Blob) {
    blob = input;
  } else if (typeof input === 'string') {
    const res = await fetch(input);
    blob = await res.blob();
  } else {
    blob = new Blob([input]);
  }

  const bitmap = await createImageBitmap(blob);
  try {
    const { width, height } = bitmap;
    const { ctx } = get2D(width, height);
    ctx.drawImage(bitmap, 0, 0);
    const img = ctx.getImageData(0, 0, width, height);
    return { width, height, rgba: new Uint8Array(img.data) };
  } finally {
    bitmap.close();
  }
}

export async function encodeRgbaToPng({ width, height, rgba }: ImageDataU8): Promise<Blob> {
  const { ctx, off, el } = get2D(width, height);
  const imageData = new ImageData(new Uint8ClampedArray(rgba), width, height);
  ctx.putImageData(imageData, 0, 0);
  if (off && 'convertToBlob' in off) {
    return off.convertToBlob({ type: 'image/png' });
  }
  if (el) {
    return await new Promise<Blob>((resolve) => el.toBlob((b) => resolve(b!), 'image/png'));
  }
  throw new Error('No canvas backing for PNG conversion');
}

export async function svgStringToPng(svg: string, width: number, height: number): Promise<Blob> {
  const svgBlob = new Blob([svg], { type: 'image/svg+xml' });
  const url = URL.createObjectURL(svgBlob);
  try {
    const imgBitmap = await createImageBitmap(await (await fetch(url)).blob());
    const { ctx, off, el } = get2D(width, height);
    ctx.drawImage(imgBitmap, 0, 0, width, height);
    if (off && 'convertToBlob' in off) {
      return off.convertToBlob({ type: 'image/png' });
    }
    if (el) {
      return await new Promise<Blob>((resolve) => el.toBlob((b) => resolve(b!), 'image/png'));
    }
    throw new Error('No canvas backing for PNG conversion');
  } finally {
    URL.revokeObjectURL(url);
  }
}
