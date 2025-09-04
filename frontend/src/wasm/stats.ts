import { initWasm as getBrowserWasm } from '../../app/lib/wasm'

export async function createNormal(mu: number, sigma: number) {
  const wasm = await getBrowserWasm() as any
  const inst = new wasm.Normal(mu, sigma)
  return {
    mean: () => inst.mean(),
    variance: () => inst.variance(),
    stdDev: () => inst.std_dev(),
    pdf: (x: number) => inst.pdf(x),
    cdf: (x: number) => inst.cdf(x),
    quantile: (p: number) => inst.quantile(p),
    pdfSvg: (w: number, h: number, samples = 200) => inst.pdf_svg(w, h, samples),
  }
}

export async function createGamma(shape: number, rate: number) {
  const wasm = await getBrowserWasm() as any
  const inst = new wasm.Gamma(shape, rate)
  return {
    mean: () => inst.mean(),
    variance: () => inst.variance(),
    stdDev: () => inst.std_dev(),
    pdf: (x: number) => inst.pdf(x),
    cdf: (x: number) => inst.cdf(x),
    quantile: (p: number) => inst.quantile(p),
    pdfSvg: (w: number, h: number, samples = 200) => inst.pdf_svg(w, h, samples),
  }
}

export async function createBinomial(n: number, p: number) {
  const wasm = await getBrowserWasm() as any
  const inst = new wasm.Binomial(n, p)
  return {
    mean: () => inst.mean(),
    variance: () => inst.variance(),
    stdDev: () => inst.std_dev(),
    pmf: (k: number) => inst.pmf(k),
    cdf: (k: number) => inst.cdf(k),
    quantile: (q: number) => inst.quantile(q),
    pmfSvg: (w: number, h: number) => inst.pmf_svg(w, h),
  }
}

export async function createPoisson(lambda: number) {
  const wasm = await getBrowserWasm() as any
  const inst = new wasm.Poisson(lambda)
  return {
    mean: () => inst.mean(),
    variance: () => inst.variance(),
    stdDev: () => inst.std_dev(),
    pmf: (k: number) => inst.pmf(k),
    cdf: (k: number) => inst.cdf(k),
    quantile: (q: number) => inst.quantile(q),
    pmfSvg: (w: number, h: number) => inst.pmf_svg(w, h),
  }
}
