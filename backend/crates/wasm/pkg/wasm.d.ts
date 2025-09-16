/* tslint:disable */
/* eslint-disable */
export function convolveNaiveF64(x: Float64Array, h: Float64Array): Float64Array;
export function convolveFftF64(x: Float64Array, h: Float64Array): Float64Array;
export function convolveAutoF64(x: Float64Array, h: Float64Array, threshold: number): Float64Array;
export function defaultConvolutionThreshold(): number;
export function binom(n: number, k: number): number;
export function stirling2(n: number, k: number): number;
export function fallingFactorialPoly(m: number): Float64Array;
export function risingFactorialPoly(m: number): Float64Array;
export function shiftPolyXPlusH(coeffs_flat: Float64Array, h: number): Float64Array;
export function binomXPlusKChooseKPoly(k: number): Float64Array;
export function discreteDiff(coeffs_flat: Float64Array): Float64Array;
export function discreteSum(coeffs_flat: Float64Array): Float64Array;
export function solveRecurrence(coeffs: Float64Array, nh_polys_flat: Float64Array, nh_offsets: Uint32Array, nh_bases: Float64Array, initial_values: Float64Array): ClosedForm;
/**
 * 部分和（S(n) = sum_{i=0..n} a(i)）を ClosedForm として返す（自由関数版）
 */
export function partialSum(cf: ClosedForm): ClosedForm;
export function hammingDistanceGF2(a: Uint8Array, b: Uint8Array): number;
export function weightDistributionGF2(codebook_flat: Uint8Array, n: number): Uint32Array;
export function parityCheckFromGeneratorGF2(k: number, n: number, g_flat: Uint8Array): Uint8Array;
export function computeSyndromeGF2(h_flat: Uint8Array, rows: number, n: number, r: Uint8Array): Uint8Array;
export function syndromeDecodeGF2(h_flat: Uint8Array, rows: number, n: number, r: Uint8Array, t: number): Uint8Array;
export function boundedDistanceDecodeGF2(h_flat: Uint8Array, rows: number, n: number, r: Uint8Array, t: number): Uint8Array;
export function hammingDMinGF2(codebook_flat: Uint8Array, n: number): number;
export function codingRateFromGeneratorGF2(k: number, n: number, g_flat: Uint8Array): number;
export function init(): void;
export function __probe(): number;
export function solveLinearSystem(rows: number, cols: number, a_data: Float64Array, b: Float64Array): Float64Array;
export function ridgeRegression(rows: number, cols: number, a_data: Float64Array, b: Float64Array, alpha: number): Float64Array;
export function lassoRegression(rows: number, cols: number, a_data: Float64Array, b: Float64Array, alpha: number, max_iter: number, tol: number): Float64Array;
export function logisticFit(rows: number, cols: number, x_data: Float64Array, y: Float64Array, lr: number, max_iter: number): Float64Array;
export function logisticPredictProba(cols: number, coeffs: Float64Array, x: Float64Array): number;
export function gmmFit(n_samples: number, n_features: number, data: Float64Array, k: number, max_iter: number, tol: number): Float64Array;
export function gmmPredictProba(n_features: number, params: Float64Array, x: Float64Array): Float64Array;
export function bayesianLinearPosterior(rows: number, cols: number, x_data: Float64Array, y: Float64Array, prior_mean: Float64Array, prior_cov: Float64Array, noise_cov: Float64Array): Float64Array;
export function kalmanPredict(n: number, f_flat: Float64Array, q_flat: Float64Array, x_flat: Float64Array, p_flat: Float64Array): Float64Array;
export function kalmanUpdate(n: number, h_flat: Float64Array, r_flat: Float64Array, z_flat: Float64Array, x_flat: Float64Array, p_flat: Float64Array): Float64Array;
export function nt_factor_u64(n: bigint): BigUint64Array;
export function nt_factor_bigint_str(n_str: string): string[];
export function window_hann(size: number): Float64Array;
export function window_hamming(size: number): Float64Array;
export function window_blackman(size: number): Float64Array;
export function window_rectangular(size: number): Float64Array;
export function window_kaiser(size: number, beta: number): Float64Array;
export function sp_design_fir_lowpass(num_taps: number, normalized_cutoff: number): Float64Array;
export function sp_design_fir_highpass(num_taps: number, normalized_cutoff: number): Float64Array;
export function sp_design_fir_bandpass(num_taps: number, f1: number, f2: number): Float64Array;
export function sp_design_fir_bandstop(num_taps: number, f1: number, f2: number): Float64Array;
export function sp_design_iir_butter_lowpass(order: number, fs: number, fc_hz: number): DiscreteTF;
export function sp_design_iir_butter_highpass(order: number, fs: number, fc_hz: number): DiscreteTF;
export function sp_design_iir_butter_bandpass(order: number, fs: number, f1_hz: number, f2_hz: number): DiscreteTF;
export function sp_design_iir_butter_bandstop(order: number, fs: number, f1_hz: number, f2_hz: number): DiscreteTF;
export function sp_design_iir_cheby1_lowpass(order: number, ripple_db: number, fs: number, fc_hz: number): DiscreteTF;
export function sp_design_iir_cheby2_lowpass(order: number, stop_atten_db: number, fs: number, fc_hz: number): DiscreteTF;
export function img_convolve2d_f32_simple(src: Float64Array, width: number, height: number, kernel: Float64Array, kw: number, kh: number, border: WasmBorder): Float64Array;
export function img_convolve2d_f32(src: Float64Array, width: number, height: number, kernel: Float64Array, kw: number, kh: number, border: WasmBorder): Float64Array;
export function img_convolve2d_u8(src: Uint8Array, width: number, height: number, kernel: Float64Array, kw: number, kh: number, border: WasmBorder): Uint8Array;
export function img_dft2d(src: Float64Array, width: number, height: number): Float64Array;
export function img_idft2d(spec_interleaved: Float64Array, width: number, height: number): Float64Array;
export function img_fftshift(spec_interleaved: Float64Array, width: number, height: number): Float64Array;
export function img_magnitude(spec_interleaved: Float64Array, width: number, height: number): Float64Array;
export function img_gaussian_blur_f32(src: Float64Array, width: number, height: number, sigma: number, radius: number, border: WasmBorder): Float64Array;
export function img_gaussian_blur_u8(src: Uint8Array, width: number, height: number, sigma: number, radius: number, border: WasmBorder): Uint8Array;
export function img_box_filter_f32(src: Float64Array, width: number, height: number, radius: number, border: WasmBorder): Float64Array;
export function img_box_filter_u8(src: Uint8Array, width: number, height: number, radius: number, border: WasmBorder): Uint8Array;
export function img_unsharp_mask_f32(src: Float64Array, width: number, height: number, sigma: number, radius: number, amount: number, border: WasmBorder): Float64Array;
export function img_unsharp_mask_u8(src: Uint8Array, width: number, height: number, sigma: number, radius: number, amount: number, border: WasmBorder): Uint8Array;
export function img_sobel_magnitude_f32(src: Float64Array, width: number, height: number, border: WasmBorder): Float64Array;
export function img_sobel_magnitude_u8(src: Uint8Array, width: number, height: number, border: WasmBorder): Uint8Array;
export function img_laplacian_f32(src: Float64Array, width: number, height: number, border: WasmBorder): Float64Array;
export function img_laplacian_u8(src: Uint8Array, width: number, height: number, border: WasmBorder): Uint8Array;
export function img_median_filter_f32(src: Float64Array, width: number, height: number, radius: number, border: WasmBorder): Float64Array;
export function img_median_filter_u8(src: Uint8Array, width: number, height: number, radius: number, border: WasmBorder): Uint8Array;
export function img_bilateral_filter_f32(src: Float64Array, width: number, height: number, radius: number, sigma_s: number, sigma_r: number, border: WasmBorder): Float64Array;
export function img_bilateral_filter_u8(src: Uint8Array, width: number, height: number, radius: number, sigma_s: number, sigma_r: number, border: WasmBorder): Uint8Array;
export function rgb_u8_to_gray_f64(rgb: Uint8Array, width: number, height: number): Float64Array;
export function rgba_u8_to_gray_f64(rgba: Uint8Array, width: number, height: number): Float64Array;
export function gray_f64_to_rgba_u8(gray: Float64Array, width: number, height: number): Uint8Array;
export function u8_to_gray_f64(pixels: Uint8Array): Float64Array;
export function gray_f64_to_u8_clamped(gray: Float64Array): Uint8Array;
export function img_convolve2d_f32_io(src: Float32Array, width: number, height: number, kernel: Float32Array, kw: number, kh: number, border: WasmBorder): Float32Array;
export function img_gaussian_blur_f32_io(src: Float32Array, width: number, height: number, sigma: number, radius: number, border: WasmBorder): Float32Array;
export function dftComplexF64(x_flat: Float64Array): Float64Array;
export function iftComplexF64(x_flat: Float64Array): Float64Array;
export function erf(x: number): number;
export function erfc(x: number): number;
export function erfInv(y: number): number;
export function gamma(x: number): number;
export function logGamma(x: number): number;
export function regularizedGamma(s: number, x: number): number;
export function beta(a: number, b: number): number;
export function logBeta(a: number, b: number): number;
export function regularizedBeta(a: number, b: number, x: number): number;
export enum WasmBorder {
  ConstantZero = 0,
  Replicate = 1,
  Reflect = 2,
}
export class BCH {
  free(): void;
  constructor(n: number, g: Uint8Array);
  /**
   * Construct BCH automatically from m (GF(2^m)) and designed t (n = 2^m - 1). Narrow-sense (b=1).
   */
  static newAuto(m: number, t: number): BCH;
  encode(u: Uint8Array): Uint8Array;
  k(): number;
  n(): number;
  t(): number;
  /**
   * 与えられた H を使って復号（(n-k)×n 行列を行優先、t は有界距離）
   */
  decodeWithH(h_flat: Uint8Array, rows: number, r: Uint8Array, t: number): Uint8Array;
  /**
   * 生成多項式から内部で標準的な巡回G/Hを構成し、GF(2)シンドロームLUTで復号
   */
  decodeLUT(r: Uint8Array): Uint8Array;
  /**
   * BM + Chien の復号（狭義BCH, b=1）
   */
  decodeBM(r: Uint8Array): Uint8Array;
}
export class Bernoulli {
  free(): void;
  constructor(p: number);
  mean(): number;
  variance(): number;
  std_dev(): number;
  pmf(k: number): number;
  cdf(k: number): number;
  quantile(p: number): number;
  pmf_svg(width: number, height: number): string;
  readonly p: number;
}
export class Binomial {
  free(): void;
  constructor(n: number, p: number);
  mean(): number;
  variance(): number;
  std_dev(): number;
  pmf(k: number): number;
  cdf(k: number): number;
  quantile(p: number): number;
  pmf_svg(width: number, height: number): string;
  readonly n: number;
  readonly p: number;
}
export class BlockHuffman {
  free(): void;
  constructor(q: number, blocks: Array<any>, probs: Float64Array);
  encode(blocks: Array<any>): Uint32Array;
  decode(length: number, digits: Uint32Array): Array<any>;
}
export class Categorical {
  free(): void;
  constructor(probs: Float64Array);
  pmf(k: number): number;
  log_pmf(k: number): number;
  cdf(k: number): number;
  quantile(p: number): number;
  pmf_svg(width: number, height: number): string;
  readonly probs: Float64Array;
}
export class ChiSquare {
  free(): void;
  constructor(k: number);
  mean(): number;
  variance(): number;
  std_dev(): number;
  pdf(x: number): number;
  cdf(x: number): number;
  quantile(p: number): number;
  pdf_svg(width: number, height: number, samples: number): string;
  readonly k_param: number;
}
export class ClosedForm {
  private constructor();
  free(): void;
  termsCount(): number;
  termPoly(i: number): Float64Array;
  termBase(i: number): Float64Array;
  term(n: number): Float64Array;
  /**
   * 人が読める文字列表現（既定 var="n"）。上付き指数はオプション。
   */
  toString(_var: string | null | undefined, unicode_superscript: boolean): string;
  /**
   * 部分和（S(n) = sum_{i=0..n} a(i)）を ClosedForm として返す
   */
  partialSum(): ClosedForm;
}
export class ContinuousSS {
  free(): void;
  constructor(a: Float64Array, na: number, ma: number, b: Float64Array, nb: number, mb: number, c: Float64Array, nc: number, mc: number, d: Float64Array, nd: number, md: number);
  static from_tf_siso(num: Float64Array, den: Float64Array): ContinuousSS;
  to_tf_siso(): Float64Array;
  c2d_zoh(fs: number): DiscreteSS;
  a_flat(): Float64Array;
  b_flat(): Float64Array;
  c_flat(): Float64Array;
  d_flat(): Float64Array;
  a_shape(): Uint32Array;
  b_shape(): Uint32Array;
  c_shape(): Uint32Array;
  d_shape(): Uint32Array;
}
export class ContinuousTF {
  free(): void;
  constructor(b: Float64Array, a: Float64Array);
  b_coeffs(): Float64Array;
  a_coeffs(): Float64Array;
  is_stable(): boolean;
  impulse_response(fs: number, len: number): Float64Array;
  step_response(fs: number, len: number): Float64Array;
  frequency_response_mag_phase(omega_max: number, n_freqs: number): Float64Array;
  to_discrete_bilinear(fs: number): DiscreteTF;
  to_discrete_bilinear_prewarp(fs: number, f_warp_hz: number): DiscreteTF;
  bode_svg(width: number, height: number, f_min_hz: number, f_max_hz: number, n_points: number, legend: boolean): string;
  nyquist_svg(width: number, height: number, f_min_hz: number, f_max_hz: number, n_points: number, log_freq: boolean, legend: boolean): string;
  block_feedback_svg(width: number, height: number, negative_feedback: boolean, feedback_label?: string | null): string;
}
export class ContinuousZpk {
  free(): void;
  constructor(zeros_interleaved: Float64Array, poles_interleaved: Float64Array, gain: number);
  static from_tf(tf: ContinuousTF): ContinuousZpk;
  to_tf(): ContinuousTF;
  zeros_interleaved(): Float64Array;
  poles_interleaved(): Float64Array;
  gain(): number;
}
export class CraftCode {
  private constructor();
  free(): void;
  static build(alphabet_size: number, code_lengths: Uint32Array): Array<any>;
}
export class CyclicCode {
  free(): void;
  constructor(n: number, g: Uint8Array);
  encode(u: Uint8Array): Uint8Array;
  k(): number;
  /**
   * 与えられた H を使って復号（(n-k)×n 行列を行優先、t は有界距離）
   */
  decodeWithH(h_flat: Uint8Array, rows: number, r: Uint8Array, t: number): Uint8Array;
  /**
   * 生成多項式から内部でHを構成し、GF(2)シンドロームLUTで復号（t=(n-k)/2）
   */
  decodeLUT(r: Uint8Array): Uint8Array;
}
export class DiscreteSS {
  free(): void;
  constructor(a: Float64Array, na: number, ma: number, b: Float64Array, nb: number, mb: number, c: Float64Array, nc: number, mc: number, d: Float64Array, nd: number, md: number);
  to_tf_siso(): Float64Array;
  a_flat(): Float64Array;
  b_flat(): Float64Array;
  c_flat(): Float64Array;
  d_flat(): Float64Array;
  a_shape(): Uint32Array;
  b_shape(): Uint32Array;
  c_shape(): Uint32Array;
  d_shape(): Uint32Array;
}
export class DiscreteTF {
  free(): void;
  constructor(b: Float64Array, a: Float64Array, sample_rate: number);
  sample_rate(): number;
  set_sample_rate(fs: number): void;
  b_coeffs(): Float64Array;
  a_coeffs(): Float64Array;
  is_stable(): boolean;
  impulse_response(len: number): Float64Array;
  step_response(len: number): Float64Array;
  frequency_response_mag_phase(n_freqs: number): Float64Array;
  bode_svg(width: number, height: number, n_points: number, hz_axis: boolean, legend: boolean): string;
  nyquist_svg(width: number, height: number, n_points: number, show_minus_one: boolean, legend: boolean): string;
  series(other: DiscreteTF): DiscreteTF;
  parallel(other: DiscreteTF): DiscreteTF;
  feedback_unity(): DiscreteTF;
  block_feedback_svg(width: number, height: number, negative_feedback: boolean, feedback_label?: string | null): string;
}
export class DiscreteZpk {
  free(): void;
  constructor(zeros_interleaved: Float64Array, poles_interleaved: Float64Array, gain: number, sample_rate: number);
  static from_tf(tf: DiscreteTF): DiscreteZpk;
  to_tf(): DiscreteTF;
  zeros_interleaved(): Float64Array;
  poles_interleaved(): Float64Array;
  gain(): number;
  sample_rate(): number;
}
export class EliasGamma {
  private constructor();
  free(): void;
  static encode(n: bigint): Uint8Array;
  static decode(bits: Uint8Array, start: number): Array<any> | undefined;
}
export class Exponential {
  free(): void;
  constructor(lambda: number);
  mean(): number;
  variance(): number;
  std_dev(): number;
  pdf(x: number): number;
  cdf(x: number): number;
  quantile(p: number): number;
  pdf_svg(width: number, height: number, samples: number): string;
  readonly lambda: number;
}
export class F {
  free(): void;
  constructor(d1: number, d2: number);
  mean(): number;
  variance(): number;
  std_dev(): number;
  pdf(x: number): number;
  cdf(x: number): number;
  quantile(p: number): number;
  pdf_svg(width: number, height: number, samples: number): string;
  readonly d1: number;
  readonly d2: number;
}
export class GF2 {
  free(): void;
  constructor(value: bigint);
  static modulus(): number;
  inv(): GF2;
  static zero(): GF2;
  static one(): GF2;
  add(rhs: GF2): GF2;
  sub(rhs: GF2): GF2;
  mul(rhs: GF2): GF2;
  div(other: GF2): GF2;
  neg(): GF2;
  readonly value: bigint;
  readonly isZero: boolean;
  readonly isOne: boolean;
}
export class GF3 {
  free(): void;
  constructor(value: bigint);
  static modulus(): number;
  inv(): GF3;
  static zero(): GF3;
  static one(): GF3;
  add(rhs: GF3): GF3;
  sub(rhs: GF3): GF3;
  mul(rhs: GF3): GF3;
  div(other: GF3): GF3;
  neg(): GF3;
  readonly value: bigint;
  readonly isZero: boolean;
  readonly isOne: boolean;
}
export class GFExtGF2 {
  free(): void;
  constructor(px_coeffs: Uint8Array, coeffs: Uint8Array);
  static fromBase(px_coeffs: Uint8Array, base_value: number): GFExtGF2;
  inv(): GFExtGF2;
  static zero(): GFExtGF2;
  static one(): GFExtGF2;
  add(rhs: GFExtGF2): GFExtGF2;
  sub(rhs: GFExtGF2): GFExtGF2;
  mul(rhs: GFExtGF2): GFExtGF2;
  div(other: GFExtGF2): GFExtGF2;
  neg(): GFExtGF2;
  readonly coeffs: Uint8Array;
  readonly px: Uint8Array;
}
export class Gamma {
  free(): void;
  constructor(shape: number, rate: number);
  mean(): number;
  variance(): number;
  std_dev(): number;
  pdf(x: number): number;
  cdf(x: number): number;
  quantile(p: number): number;
  pdf_svg(width: number, height: number, samples: number): string;
  readonly shape: number;
  readonly rate: number;
}
export class GeneralTerm {
  free(): void;
  /**
   * constructor from complex-coefficient polynomial (flat) and base (re, im)
   */
  constructor(poly_flat: Float64Array, base_re: number, base_im: number);
  polynomial(): Float64Array;
  base(): Float64Array;
}
export class Hamming74 {
  free(): void;
  constructor();
  encode(u: Uint8Array): Uint8Array;
  /**
   * H 行列（(n-k)×n）を行優先で返す
   */
  parityCheck(): Uint8Array;
  /**
   * 有界距離復号（t=1）で訂正したコード語を返す
   */
  decode(r: Uint8Array): Uint8Array;
}
export class Jones {
  free(): void;
  constructor(alphabet: string, probs: Float64Array, total: number);
  encode(symbols: string): Uint8Array;
  decode(length: number, bits: Uint8Array): string;
}
export class LinearCode {
  free(): void;
  constructor(k: number, n: number, g_data: Uint8Array);
  encode(u: Uint8Array): Uint8Array;
  /**
   * H行列（(n-k)×n）を返す（標準形への変換を内部で行う）
   */
  parityCheck(): Uint8Array;
  /**
   * シンドローム復号（内部で H を構成）
   */
  decodeSyndrome(r: Uint8Array, t: number): Uint8Array;
  /**
   * 与えられた H を使って復号（(n-k)×n 行列を行優先、t は有界距離）
   */
  decodeWithH(h_flat: Uint8Array, rows: number, r: Uint8Array, t: number): Uint8Array;
}
export class Lz78 {
  free(): void;
  constructor();
  encodeInternal(input: string): Array<any>;
  decodeInternal(pairs: Array<any>): string;
}
export class Markov {
  free(): void;
  constructor(alphabet: string, init_pr: Float64Array, cond_pr: Array<any>);
  blockPr(symbols: string): number;
}
export class Matrix {
  free(): void;
  constructor(rows: number, cols: number, data: Float64Array);
  static with_default(rows: number, cols: number): Matrix;
  static zeros(rows: number, cols: number): Matrix;
  static identity(size: number): Matrix;
  rows(): number;
  cols(): number;
  is_square(): boolean;
  transpose(): Matrix;
  trace(): number;
  determinant(): number;
  rank(): number;
  inverse(): Matrix | undefined;
  frobenius_norm(): number;
  expm(): Matrix;
  qr_decomposition(): any;
  svd(): any;
  eigen_decomposition(): any;
  cholesky(): Matrix;
  pinv(): Matrix;
  data(): Float64Array;
  columns(): number;
}
export class MatrixF32 {
  free(): void;
  add(rhs: MatrixF32): MatrixF32;
  sub(rhs: MatrixF32): MatrixF32;
  mul(rhs: MatrixF32): MatrixF32;
  constructor(rows: number, cols: number, data: Float32Array);
  static with_default(rows: number, cols: number): MatrixF32;
  static zeros(rows: number, cols: number): MatrixF32;
  static identity(size: number): MatrixF32;
  rows(): number;
  cols(): number;
  is_square(): boolean;
  transpose(): MatrixF32;
  trace(): number;
  determinant(): number;
  rank(): number;
  inverse(): MatrixF32 | undefined;
  data(): Float32Array;
  columns(): number;
  /**
   * 既約行基本形（RREF）を返す
   */
  rref(): MatrixF32;
  /**
   * 連立の拡大行列のRREFを返す（左：係数側のRREF，右：右辺側のRREF）
   */
  rrefWith(other: MatrixF32): any;
  /**
   * LU 分解（部分ピボット付き）。{ p, l, u } を返す
   */
  lu(): any;
}
export class MatrixF64 {
  free(): void;
  constructor(rows: number, cols: number, data: Float64Array);
  static with_default(rows: number, cols: number): MatrixF64;
  static zeros(rows: number, cols: number): MatrixF64;
  static identity(size: number): MatrixF64;
  rows(): number;
  cols(): number;
  is_square(): boolean;
  transpose(): MatrixF64;
  trace(): number;
  determinant(): number;
  rank(): number;
  inverse(): MatrixF64 | undefined;
  frobenius_norm(): number;
  expm(): MatrixF64;
  qr_decomposition(): any;
  svd(): any;
  eigen_decomposition(): any;
  cholesky(): MatrixF64;
  pinv(): MatrixF64;
  add(rhs: MatrixF64): MatrixF64;
  sub(rhs: MatrixF64): MatrixF64;
  mul(rhs: MatrixF64): MatrixF64;
  get(row: number, col: number): number;
  row(index: number): VectorF64 | undefined;
  col(index: number): VectorF64 | undefined;
  multiply_vector(vector: VectorF64): VectorF64 | undefined;
  diagonal(): VectorF64;
  solve(b: VectorF64): VectorF64 | undefined;
  data(): Float64Array;
  columns(): number;
  /**
   * 既約行基本形（RREF）を返す
   */
  rref(): MatrixF64;
  /**
   * 連立の拡大行列のRREFを返す（左：係数側のRREF，右：右辺側のRREF）
   */
  rrefWith(other: MatrixF64): any;
  /**
   * LU 分解（部分ピボット付き）。{ p, l, u } を返す
   */
  lu(): any;
}
export class MatrixI32 {
  free(): void;
  add(rhs: MatrixI32): MatrixI32;
  sub(rhs: MatrixI32): MatrixI32;
  mul(rhs: MatrixI32): MatrixI32;
  constructor(rows: number, cols: number, data: Int32Array);
  static with_default(rows: number, cols: number): MatrixI32;
  static zeros(rows: number, cols: number): MatrixI32;
  static identity(size: number): MatrixI32;
  rows(): number;
  cols(): number;
  is_square(): boolean;
  transpose(): MatrixI32;
  trace(): number;
  data(): Int32Array;
  columns(): number;
}
export class Normal {
  free(): void;
  constructor(mu: number, sigma: number);
  mean(): number;
  variance(): number;
  std_dev(): number;
  pdf(x: number): number;
  cdf(x: number): number;
  quantile(p: number): number;
  pdf_svg(width: number, height: number, samples: number): string;
  readonly mu: number;
  readonly sigma: number;
}
export class Poisson {
  free(): void;
  constructor(lambda: number);
  mean(): number;
  variance(): number;
  std_dev(): number;
  pmf(k: number): number;
  log_pmf(k: number): number;
  cdf(k: number): number;
  quantile(p: number): number;
  pmf_svg(width: number, height: number): string;
  readonly lambda: number;
}
export class PolynomialF64 {
  free(): void;
  add(rhs: PolynomialF64): PolynomialF64;
  sub(rhs: PolynomialF64): PolynomialF64;
  mul(rhs: PolynomialF64): PolynomialF64;
  div(rhs: PolynomialF64): PolynomialF64;
  divRem(other: PolynomialF64): PolynomialF64[];
  constructor(coeffs: Float64Array);
  deg(): number;
  get(i: number): number;
  eval(x: number): number;
  differentiate(): PolynomialF64;
  integrate(): PolynomialF64;
  /**
   * 係数ベクトル（低次→高次）を返す
   */
  coeffs(): Float64Array;
  /**
   * 実根列から多項式を生成（roots は実数）
   */
  static fromRoots(roots: Float64Array): PolynomialF64;
  /**
   * 多項式の最大公約多項式（静的メソッド）
   */
  static gcd(a: PolynomialF64, b: PolynomialF64): PolynomialF64;
  /**
   * 多項式の最小公倍多項式（静的メソッド）
   */
  static lcm(a: PolynomialF64, b: PolynomialF64): PolynomialF64;
  /**
   * 複素根を返す（re, im の交互並びのフラット配列: [re0, im0, re1, im1, ...]）
   */
  findRoots(): Float64Array;
  /**
   * 渡された複素根列をクラスタリングして重複度付き根情報を返す
   * inputs: roots_interleaved = [re0, im0, re1, im1, ...], tolerance
   */
  static groupRoots(roots_interleaved: Float64Array, tolerance: number): WasmRoot[];
  mulSimple(other: PolynomialF64): PolynomialF64;
  mulFft(other: PolynomialF64): PolynomialF64;
  mulAuto(other: PolynomialF64): PolynomialF64;
}
export class PolynomialGF2 {
  free(): void;
  add(rhs: PolynomialGF2): PolynomialGF2;
  sub(rhs: PolynomialGF2): PolynomialGF2;
  mul(rhs: PolynomialGF2): PolynomialGF2;
  div(rhs: PolynomialGF2): PolynomialGF2;
  divRem(other: PolynomialGF2): PolynomialGF2[];
  constructor(coeffs: Uint8Array);
  deg(): number;
  get(i: number): number;
  eval(x: number): number;
  /**
   * 係数ベクトル（低次→高次, 0/1）を返す
   */
  coeffs(): Uint8Array;
}
export class PolynomialGF256 {
  free(): void;
  add(rhs: PolynomialGF256): PolynomialGF256;
  sub(rhs: PolynomialGF256): PolynomialGF256;
  mul(rhs: PolynomialGF256): PolynomialGF256;
  div(rhs: PolynomialGF256): PolynomialGF256;
  divRem(other: PolynomialGF256): PolynomialGF256[];
  constructor(coeffs: Uint8Array);
  deg(): number;
  get(i: number): number;
  eval(x: number): number;
  /**
   * 係数ベクトル（低次→高次, u8）を返す
   */
  coeffs(): Uint8Array;
}
export class PolynomialGFExtGF2 {
  free(): void;
  add(rhs: PolynomialGFExtGF2): PolynomialGFExtGF2;
  sub(rhs: PolynomialGFExtGF2): PolynomialGFExtGF2;
  mul(rhs: PolynomialGFExtGF2): PolynomialGFExtGF2;
  div(rhs: PolynomialGFExtGF2): PolynomialGFExtGF2;
  divRem(other: PolynomialGFExtGF2): PolynomialGFExtGF2[];
  constructor(px: Uint8Array, coeffs: Uint8Array[]);
  deg(): number;
  get(i: number): Uint8Array;
  eval(x_coeffs: Uint8Array): Uint8Array;
  /**
   * 係数ベクトル（各係数は GFExt(GF2) の係数列を Uint8Array として返す）
   */
  coeffs(): Uint8Array[];
}
export class RationalFunctionF64 {
  free(): void;
  /**
   * constructor from numerator/denominator coeff arrays (low->high)
   */
  constructor(numerator: Float64Array, denominator: Float64Array);
  /**
   * numerator coefficients (low->high)
   */
  numeratorCoeffs(): Float64Array;
  /**
   * denominator coefficients (low->high)
   */
  denominatorCoeffs(): Float64Array;
  simplify(): void;
  is_zero(): boolean;
  inverse(): RationalFunctionF64;
  /**
   * evaluate at x (Some(y) or None if denominator(x)==0)
   */
  eval(x: number): number | undefined;
  /**
   * derivative using quotient rule from crate
   */
  differentiate(): RationalFunctionF64;
  /**
   * basic ops
   */
  add(rhs: RationalFunctionF64): RationalFunctionF64;
  sub(rhs: RationalFunctionF64): RationalFunctionF64;
  mul(rhs: RationalFunctionF64): RationalFunctionF64;
  div(rhs: RationalFunctionF64): RationalFunctionF64;
  /**
   * multiply/divide by a polynomial
   */
  mulPoly(poly_coeffs: Float64Array): RationalFunctionF64;
  divPoly(poly_coeffs: Float64Array): RationalFunctionF64;
  /**
   * find poles (value + multiplicity)
   */
  findPoles(): WasmRoot[];
  /**
   * partial fraction expansion result
   */
  partialFractionExpansion(): any;
}
export class RecurrenceRelation {
  free(): void;
  /**
   * constructor: coeffs (a1..ak), non-homogeneous terms, initial values
   */
  constructor(coeffs: Float64Array, terms: GeneralTerm[], initial_values: Float64Array);
  solve(): ClosedForm;
  coeffs(): Float64Array;
}
export class ReedSolomon {
  free(): void;
  constructor(k: number, n: number);
  encode(f: Uint8Array): Uint8Array;
  decode(r: Uint8Array): Uint8Array;
  n(): number;
  t(): number;
  /**
   * Berlekamp–Massey ベースの代替復号器
   */
  decodeBM(r: Uint8Array): Uint8Array;
}
export class SourceArithmetic {
  free(): void;
  constructor(alphabet: string, probs: Float64Array);
  encode(symbols: string): Uint8Array;
  decode(length: number, bits: Uint8Array): string;
}
export class SourceHuffman {
  free(): void;
  constructor(alphabet: string, probs: Float64Array);
  encode(symbols: string): Uint8Array;
  decode(length: number, bits: Uint8Array): string;
}
export class StudentT {
  free(): void;
  constructor(df: number);
  mean(): number;
  variance(): number;
  std_dev(): number;
  pdf(x: number): number;
  cdf(x: number): number;
  quantile(p: number): number;
  pdf_svg(width: number, height: number, samples: number): string;
  readonly df: number;
}
export class Uniform {
  free(): void;
  constructor(a: number, b: number);
  mean(): number;
  variance(): number;
  std_dev(): number;
  pdf(x: number): number;
  cdf(x: number): number;
  quantile(p: number): number;
  pdf_svg(width: number, height: number, samples: number): string;
  readonly a: number;
  readonly b: number;
}
export class Vector {
  free(): void;
  constructor(data: Float64Array);
  static zeros(dim: number): Vector;
  static ones(dim: number): Vector;
  dim(): number;
  len(): number;
  is_empty(): boolean;
  dot(other: Vector): number;
  argmax(): number | undefined;
  argmin(): number | undefined;
  max(): number | undefined;
  min(): number | undefined;
  norm(): number;
  normalize(): Vector;
  cosine_similarity(other: Vector): number;
  mean(): number | undefined;
  std(): number;
  static linspace(start: number, end: number, num: number): Vector;
  sum(): number;
  transpose(): MatrixF64;
  to_column_matrix(): MatrixF64;
  to_row_matrix(): MatrixF64;
  data(): Float64Array;
}
export class VectorF32 {
  free(): void;
  add(rhs: VectorF32): VectorF32;
  sub(rhs: VectorF32): VectorF32;
  mul(rhs: VectorF32): VectorF32;
  constructor(data: Float32Array);
  static zeros(dim: number): VectorF32;
  static ones(dim: number): VectorF32;
  dim(): number;
  len(): number;
  is_empty(): boolean;
  dot(other: VectorF32): number;
  argmax(): number | undefined;
  argmin(): number | undefined;
  max(): number | undefined;
  min(): number | undefined;
  data(): Float32Array;
}
export class VectorF64 {
  free(): void;
  constructor(data: Float64Array);
  static zeros(dim: number): VectorF64;
  static ones(dim: number): VectorF64;
  dim(): number;
  len(): number;
  is_empty(): boolean;
  dot(other: VectorF64): number;
  argmax(): number | undefined;
  argmin(): number | undefined;
  max(): number | undefined;
  min(): number | undefined;
  norm(): number;
  normalize(): VectorF64;
  cosine_similarity(other: VectorF64): number;
  mean(): number | undefined;
  std(): number;
  static linspace(start: number, end: number, num: number): VectorF64;
  add(rhs: VectorF64): VectorF64;
  sub(rhs: VectorF64): VectorF64;
  mul(rhs: VectorF64): VectorF64;
  sum(): number;
  multiply_matrix(matrix: MatrixF64): MatrixF64;
  transpose(): MatrixF64;
  to_column_matrix(): MatrixF64;
  to_row_matrix(): MatrixF64;
  data(): Float64Array;
}
export class VectorI32 {
  free(): void;
  add(rhs: VectorI32): VectorI32;
  sub(rhs: VectorI32): VectorI32;
  mul(rhs: VectorI32): VectorI32;
  constructor(data: Int32Array);
  static zeros(dim: number): VectorI32;
  static ones(dim: number): VectorI32;
  dim(): number;
  len(): number;
  is_empty(): boolean;
  dot(other: VectorI32): number;
  argmax(): number | undefined;
  argmin(): number | undefined;
  max(): number | undefined;
  min(): number | undefined;
  data(): Int32Array;
}
export class WasmGF2 {
  free(): void;
  constructor(value: bigint);
  static modulus(): number;
  inv(): WasmGF2;
  static zero(): WasmGF2;
  static one(): WasmGF2;
  add(rhs: WasmGF2): WasmGF2;
  sub(rhs: WasmGF2): WasmGF2;
  mul(rhs: WasmGF2): WasmGF2;
  div(other: WasmGF2): WasmGF2;
  neg(): WasmGF2;
  readonly value: bigint;
  readonly isZero: boolean;
  readonly isOne: boolean;
}
export class WasmGF256 {
  free(): void;
  toU8(): number;
  inv(): WasmGF256;
  static zero(): WasmGF256;
  static one(): WasmGF256;
  add(rhs: WasmGF256): WasmGF256;
  sub(rhs: WasmGF256): WasmGF256;
  mul(rhs: WasmGF256): WasmGF256;
  constructor(value: number);
  static fromCoeffs(coeffs: Uint8Array): WasmGF256;
  static modulus(): Uint8Array;
  div(other: WasmGF256): WasmGF256;
  neg(): WasmGF256;
  readonly isZero: boolean;
  readonly isOne: boolean;
  readonly value: number;
  readonly coeffs: Uint8Array;
}
export class WasmGF3 {
  free(): void;
  constructor(value: bigint);
  static modulus(): number;
  inv(): WasmGF3;
  static zero(): WasmGF3;
  static one(): WasmGF3;
  add(rhs: WasmGF3): WasmGF3;
  sub(rhs: WasmGF3): WasmGF3;
  mul(rhs: WasmGF3): WasmGF3;
  div(other: WasmGF3): WasmGF3;
  neg(): WasmGF3;
  readonly value: bigint;
  readonly isZero: boolean;
  readonly isOne: boolean;
}
export class WasmGFExtGF2 {
  free(): void;
  inv(): WasmGFExtGF2;
  static zero(): WasmGFExtGF2;
  static one(): WasmGFExtGF2;
  add(rhs: WasmGFExtGF2): WasmGFExtGF2;
  sub(rhs: WasmGFExtGF2): WasmGFExtGF2;
  mul(rhs: WasmGFExtGF2): WasmGFExtGF2;
  constructor(px_coeffs: Uint8Array, coeffs: Uint8Array);
  static fromBase(px_coeffs: Uint8Array, base_value: number): WasmGFExtGF2;
  div(other: WasmGFExtGF2): WasmGFExtGF2;
  neg(): WasmGFExtGF2;
  readonly isZero: boolean;
  readonly isOne: boolean;
  readonly coeffs: Uint8Array;
  readonly px: Uint8Array;
}
export class WasmLMS {
  free(): void;
  constructor(taps: number, step_size: number);
  process_sample(input: number, desired: number): Float64Array;
  process_series(input: Float64Array, desired: Float64Array): Float64Array;
}
export class WasmLinearModel {
  private constructor();
  free(): void;
  static solveLinearSystem(rows: number, cols: number, a_data: Float64Array, b: Float64Array): Float64Array;
  static ridgeRegression(rows: number, cols: number, a_data: Float64Array, b: Float64Array, alpha: number): Float64Array;
  static lassoRegression(rows: number, cols: number, a_data: Float64Array, b: Float64Array, alpha: number, max_iter: number, tol: number): Float64Array;
}
export class WasmNLMS {
  free(): void;
  constructor(taps: number, step_size: number, epsilon: number);
  process_sample(input: number, desired: number): Float64Array;
  process_series(input: Float64Array, desired: Float64Array): Float64Array;
}
/**
 * JS へ返す重複度付き根
 */
export class WasmRoot {
  private constructor();
  free(): void;
  readonly re: number;
  readonly im: number;
  readonly multiplicity: number;
}
export class WasmSignal {
  free(): void;
  constructor(data: Float64Array, sample_rate: number);
  data(): Float64Array;
  sample_rate(): number;
  len(): number;
  dft(): WasmSpectrum;
  convolve(h: WasmSignal): WasmSignal;
  apply_fir(taps: Float64Array): WasmSignal;
  downsample(factor: number, filter_taps: number): WasmSignal;
  upsample(factor: number, filter_taps: number): WasmSignal;
  resample(upsample_factor: number, downsample_factor: number, filter_taps: number): WasmSignal;
  decimate(factor: number): WasmSignal;
  expand(factor: number): WasmSignal;
  save_svg_simple(width: number, height: number): string;
  save_svg_with_axes(width: number, height: number, label?: string | null): string;
}
export class WasmSpectrum {
  free(): void;
  constructor(data_interleaved: Float64Array, sample_rate: number);
  data_interleaved(): Float64Array;
  sample_rate(): number;
  len(): number;
  ift(): WasmSignal;
  magnitude_db_svg(width: number, height: number, label?: string | null): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly convolveNaiveF64: (a: number, b: number, c: number, d: number) => [number, number];
  readonly convolveFftF64: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly convolveAutoF64: (a: number, b: number, c: number, d: number, e: number) => [number, number, number, number];
  readonly defaultConvolutionThreshold: () => number;
  readonly __wbg_sourcearithmetic_free: (a: number, b: number) => void;
  readonly sourcearithmetic_new: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly sourcearithmetic_encode: (a: number, b: number, c: number) => [number, number, number, number];
  readonly sourcearithmetic_decode: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly __wbg_sourcehuffman_free: (a: number, b: number) => void;
  readonly sourcehuffman_new: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly sourcehuffman_encode: (a: number, b: number, c: number) => [number, number];
  readonly sourcehuffman_decode: (a: number, b: number, c: number, d: number) => [number, number];
  readonly __wbg_eliasgamma_free: (a: number, b: number) => void;
  readonly eliasgamma_encode: (a: bigint) => [number, number];
  readonly eliasgamma_decode: (a: number, b: number, c: number) => any;
  readonly __wbg_craftcode_free: (a: number, b: number) => void;
  readonly craftcode_build: (a: number, b: number, c: number) => any;
  readonly __wbg_jones_free: (a: number, b: number) => void;
  readonly jones_new: (a: number, b: number, c: number, d: number, e: number) => [number, number, number];
  readonly jones_encode: (a: number, b: number, c: number) => [number, number];
  readonly jones_decode: (a: number, b: number, c: number, d: number) => [number, number];
  readonly __wbg_lz78_free: (a: number, b: number) => void;
  readonly lz78_new: () => number;
  readonly lz78_encodeInternal: (a: number, b: number, c: number) => any;
  readonly lz78_decodeInternal: (a: number, b: any) => [number, number];
  readonly __wbg_markov_free: (a: number, b: number) => void;
  readonly markov_new: (a: number, b: number, c: number, d: number, e: any) => number;
  readonly markov_blockPr: (a: number, b: number, c: number) => number;
  readonly __wbg_blockhuffman_free: (a: number, b: number) => void;
  readonly blockhuffman_new: (a: number, b: any, c: number, d: number) => number;
  readonly blockhuffman_encode: (a: number, b: any) => [number, number];
  readonly blockhuffman_decode: (a: number, b: number, c: number, d: number) => any;
  readonly __wbg_polynomialf64_free: (a: number, b: number) => void;
  readonly polynomialf64_add: (a: number, b: number) => number;
  readonly polynomialf64_sub: (a: number, b: number) => number;
  readonly polynomialf64_mul: (a: number, b: number) => number;
  readonly polynomialf64_div: (a: number, b: number) => number;
  readonly __wbg_polynomialgf2_free: (a: number, b: number) => void;
  readonly polynomialgf2_add: (a: number, b: number) => number;
  readonly polynomialgf2_sub: (a: number, b: number) => number;
  readonly polynomialgf2_mul: (a: number, b: number) => number;
  readonly polynomialgf2_div: (a: number, b: number) => number;
  readonly __wbg_polynomialgf256_free: (a: number, b: number) => void;
  readonly polynomialgf256_add: (a: number, b: number) => number;
  readonly polynomialgf256_sub: (a: number, b: number) => number;
  readonly polynomialgf256_mul: (a: number, b: number) => number;
  readonly polynomialgf256_div: (a: number, b: number) => number;
  readonly __wbg_polynomialgfextgf2_free: (a: number, b: number) => void;
  readonly polynomialgfextgf2_add: (a: number, b: number) => number;
  readonly polynomialgfextgf2_sub: (a: number, b: number) => number;
  readonly polynomialgfextgf2_mul: (a: number, b: number) => number;
  readonly polynomialgfextgf2_div: (a: number, b: number) => number;
  readonly polynomialf64_divRem: (a: number, b: number) => [number, number];
  readonly polynomialgf2_divRem: (a: number, b: number) => [number, number];
  readonly polynomialgf256_divRem: (a: number, b: number) => [number, number];
  readonly polynomialgfextgf2_divRem: (a: number, b: number) => [number, number];
  readonly polynomialf64_new: (a: number, b: number) => number;
  readonly polynomialf64_deg: (a: number) => number;
  readonly polynomialf64_get: (a: number, b: number) => number;
  readonly polynomialf64_eval: (a: number, b: number) => number;
  readonly polynomialf64_differentiate: (a: number) => number;
  readonly polynomialf64_integrate: (a: number) => number;
  readonly polynomialf64_coeffs: (a: number) => [number, number];
  readonly polynomialf64_fromRoots: (a: number, b: number) => number;
  readonly polynomialf64_gcd: (a: number, b: number) => number;
  readonly polynomialf64_lcm: (a: number, b: number) => number;
  readonly polynomialf64_findRoots: (a: number) => [number, number];
  readonly polynomialf64_groupRoots: (a: number, b: number, c: number) => [number, number];
  readonly __wbg_wasmroot_free: (a: number, b: number) => void;
  readonly wasmroot_re: (a: number) => number;
  readonly wasmroot_im: (a: number) => number;
  readonly wasmroot_multiplicity: (a: number) => number;
  readonly polynomialf64_mulSimple: (a: number, b: number) => number;
  readonly polynomialf64_mulFft: (a: number, b: number) => number;
  readonly polynomialf64_mulAuto: (a: number, b: number) => number;
  readonly __wbg_rationalfunctionf64_free: (a: number, b: number) => void;
  readonly rationalfunctionf64_new: (a: number, b: number, c: number, d: number) => number;
  readonly rationalfunctionf64_numeratorCoeffs: (a: number) => [number, number];
  readonly rationalfunctionf64_denominatorCoeffs: (a: number) => [number, number];
  readonly rationalfunctionf64_simplify: (a: number) => void;
  readonly rationalfunctionf64_is_zero: (a: number) => number;
  readonly rationalfunctionf64_inverse: (a: number) => number;
  readonly rationalfunctionf64_eval: (a: number, b: number) => [number, number];
  readonly rationalfunctionf64_differentiate: (a: number) => number;
  readonly rationalfunctionf64_add: (a: number, b: number) => number;
  readonly rationalfunctionf64_sub: (a: number, b: number) => number;
  readonly rationalfunctionf64_mul: (a: number, b: number) => number;
  readonly rationalfunctionf64_div: (a: number, b: number) => number;
  readonly rationalfunctionf64_mulPoly: (a: number, b: number, c: number) => number;
  readonly rationalfunctionf64_divPoly: (a: number, b: number, c: number) => number;
  readonly rationalfunctionf64_findPoles: (a: number) => [number, number];
  readonly rationalfunctionf64_partialFractionExpansion: (a: number) => any;
  readonly polynomialgf2_new: (a: number, b: number) => number;
  readonly polynomialgf2_deg: (a: number) => number;
  readonly polynomialgf2_get: (a: number, b: number) => number;
  readonly polynomialgf2_eval: (a: number, b: number) => number;
  readonly polynomialgf2_coeffs: (a: number) => [number, number];
  readonly polynomialgf256_new: (a: number, b: number) => number;
  readonly polynomialgf256_deg: (a: number) => number;
  readonly polynomialgf256_get: (a: number, b: number) => number;
  readonly polynomialgf256_eval: (a: number, b: number) => number;
  readonly polynomialgf256_coeffs: (a: number) => [number, number];
  readonly polynomialgfextgf2_new: (a: number, b: number, c: number, d: number) => number;
  readonly polynomialgfextgf2_get: (a: number, b: number) => [number, number];
  readonly polynomialgfextgf2_eval: (a: number, b: number, c: number) => [number, number];
  readonly polynomialgfextgf2_coeffs: (a: number) => [number, number];
  readonly binom: (a: number, b: number) => number;
  readonly stirling2: (a: number, b: number) => number;
  readonly fallingFactorialPoly: (a: number) => [number, number];
  readonly risingFactorialPoly: (a: number) => [number, number];
  readonly shiftPolyXPlusH: (a: number, b: number, c: number) => [number, number];
  readonly binomXPlusKChooseKPoly: (a: number) => [number, number];
  readonly discreteDiff: (a: number, b: number) => [number, number];
  readonly discreteSum: (a: number, b: number) => [number, number];
  readonly __wbg_closedform_free: (a: number, b: number) => void;
  readonly closedform_termsCount: (a: number) => number;
  readonly closedform_termPoly: (a: number, b: number) => [number, number, number, number];
  readonly closedform_termBase: (a: number, b: number) => [number, number, number, number];
  readonly closedform_term: (a: number, b: number) => [number, number];
  readonly closedform_toString: (a: number, b: number, c: number, d: number) => [number, number];
  readonly closedform_partialSum: (a: number) => number;
  readonly solveRecurrence: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => [number, number, number];
  readonly __wbg_generalterm_free: (a: number, b: number) => void;
  readonly generalterm_new: (a: number, b: number, c: number, d: number) => number;
  readonly generalterm_polynomial: (a: number) => [number, number];
  readonly generalterm_base: (a: number) => [number, number];
  readonly __wbg_recurrencerelation_free: (a: number, b: number) => void;
  readonly recurrencerelation_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly recurrencerelation_solve: (a: number) => number;
  readonly recurrencerelation_coeffs: (a: number) => [number, number];
  readonly partialSum: (a: number) => number;
  readonly polynomialgfextgf2_deg: (a: number) => number;
  readonly __wbg_hamming74_free: (a: number, b: number) => void;
  readonly hamming74_new: () => number;
  readonly hamming74_encode: (a: number, b: number, c: number) => [number, number, number, number];
  readonly hamming74_parityCheck: (a: number) => [number, number, number, number];
  readonly hamming74_decode: (a: number, b: number, c: number) => [number, number, number, number];
  readonly __wbg_linearcode_free: (a: number, b: number) => void;
  readonly linearcode_new: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly linearcode_encode: (a: number, b: number, c: number) => [number, number, number, number];
  readonly linearcode_parityCheck: (a: number) => [number, number, number, number];
  readonly linearcode_decodeSyndrome: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly linearcode_decodeWithH: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number, number];
  readonly __wbg_cycliccode_free: (a: number, b: number) => void;
  readonly cycliccode_new: (a: number, b: number, c: number) => number;
  readonly cycliccode_encode: (a: number, b: number, c: number) => [number, number, number, number];
  readonly cycliccode_k: (a: number) => number;
  readonly cycliccode_decodeWithH: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number, number];
  readonly cycliccode_decodeLUT: (a: number, b: number, c: number) => [number, number, number, number];
  readonly __wbg_reedsolomon_free: (a: number, b: number) => void;
  readonly reedsolomon_new: (a: number, b: number) => [number, number, number];
  readonly reedsolomon_encode: (a: number, b: number, c: number) => [number, number, number, number];
  readonly reedsolomon_decode: (a: number, b: number, c: number) => [number, number, number, number];
  readonly reedsolomon_decodeBM: (a: number, b: number, c: number) => [number, number, number, number];
  readonly __wbg_bch_free: (a: number, b: number) => void;
  readonly bch_new: (a: number, b: number, c: number) => number;
  readonly bch_newAuto: (a: number, b: number) => number;
  readonly bch_encode: (a: number, b: number, c: number) => [number, number, number, number];
  readonly bch_k: (a: number) => number;
  readonly bch_n: (a: number) => number;
  readonly bch_t: (a: number) => number;
  readonly bch_decodeWithH: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number, number];
  readonly bch_decodeLUT: (a: number, b: number, c: number) => [number, number, number, number];
  readonly bch_decodeBM: (a: number, b: number, c: number) => [number, number, number, number];
  readonly hammingDistanceGF2: (a: number, b: number, c: number, d: number) => number;
  readonly weightDistributionGF2: (a: number, b: number, c: number) => [number, number, number, number];
  readonly parityCheckFromGeneratorGF2: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly computeSyndromeGF2: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number, number];
  readonly syndromeDecodeGF2: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number, number];
  readonly boundedDistanceDecodeGF2: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number, number];
  readonly hammingDMinGF2: (a: number, b: number, c: number) => [number, number, number];
  readonly codingRateFromGeneratorGF2: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly __wbg_gf2_free: (a: number, b: number) => void;
  readonly gf2_new: (a: bigint) => number;
  readonly gf2_modulus: () => number;
  readonly gf2_inv: (a: number) => [number, number, number];
  readonly gf2_zero: () => number;
  readonly gf2_one: () => number;
  readonly gf2_add: (a: number, b: number) => number;
  readonly gf2_sub: (a: number, b: number) => number;
  readonly gf2_mul: (a: number, b: number) => number;
  readonly gf2_div: (a: number, b: number) => [number, number, number];
  readonly gf2_neg: (a: number) => number;
  readonly gf2_value: (a: number) => bigint;
  readonly gf2_is_zero: (a: number) => number;
  readonly gf2_is_one: (a: number) => number;
  readonly __wbg_gf3_free: (a: number, b: number) => void;
  readonly gf3_new: (a: bigint) => number;
  readonly gf3_modulus: () => number;
  readonly gf3_inv: (a: number) => [number, number, number];
  readonly gf3_add: (a: number, b: number) => number;
  readonly gf3_sub: (a: number, b: number) => number;
  readonly gf3_mul: (a: number, b: number) => number;
  readonly gf3_div: (a: number, b: number) => [number, number, number];
  readonly gf3_neg: (a: number) => number;
  readonly __wbg_gfextgf2_free: (a: number, b: number) => void;
  readonly gfextgf2_new: (a: number, b: number, c: number, d: number) => number;
  readonly gfextgf2_fromBase: (a: number, b: number, c: number) => number;
  readonly gfextgf2_inv: (a: number) => [number, number, number];
  readonly gfextgf2_zero: () => number;
  readonly gfextgf2_one: () => number;
  readonly gfextgf2_add: (a: number, b: number) => number;
  readonly gfextgf2_sub: (a: number, b: number) => number;
  readonly gfextgf2_mul: (a: number, b: number) => number;
  readonly gfextgf2_div: (a: number, b: number) => [number, number, number];
  readonly gfextgf2_neg: (a: number) => number;
  readonly gfextgf2_coeffs: (a: number) => [number, number];
  readonly gfextgf2_px: (a: number) => [number, number];
  readonly __probe: () => number;
  readonly __wbg_matrix_free: (a: number, b: number) => void;
  readonly matrix_new: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly matrix_with_default: (a: number, b: number) => number;
  readonly matrix_zeros: (a: number, b: number) => number;
  readonly matrix_identity: (a: number) => number;
  readonly matrix_rows: (a: number) => number;
  readonly matrix_cols: (a: number) => number;
  readonly matrix_is_square: (a: number) => number;
  readonly matrix_transpose: (a: number) => number;
  readonly matrix_trace: (a: number) => [number, number, number];
  readonly matrix_determinant: (a: number) => [number, number, number];
  readonly matrix_rank: (a: number) => [number, number, number];
  readonly matrix_inverse: (a: number) => number;
  readonly matrix_frobenius_norm: (a: number) => number;
  readonly matrix_expm: (a: number) => number;
  readonly matrix_qr_decomposition: (a: number) => [number, number, number];
  readonly matrix_svd: (a: number) => [number, number, number];
  readonly matrix_eigen_decomposition: (a: number) => [number, number, number];
  readonly matrix_cholesky: (a: number) => [number, number, number];
  readonly matrix_pinv: (a: number) => [number, number, number];
  readonly matrix_data: (a: number) => [number, number];
  readonly __wbg_vector_free: (a: number, b: number) => void;
  readonly vector_new: (a: number, b: number) => number;
  readonly vector_zeros: (a: number) => number;
  readonly vector_ones: (a: number) => number;
  readonly vector_dim: (a: number) => number;
  readonly vector_is_empty: (a: number) => number;
  readonly vector_dot: (a: number, b: number) => number;
  readonly vector_argmax: (a: number) => number;
  readonly vector_argmin: (a: number) => number;
  readonly vector_max: (a: number) => [number, number];
  readonly vector_min: (a: number) => [number, number];
  readonly vector_norm: (a: number) => number;
  readonly vector_normalize: (a: number) => number;
  readonly vector_cosine_similarity: (a: number, b: number) => number;
  readonly vector_mean: (a: number) => [number, number];
  readonly vector_std: (a: number) => number;
  readonly vector_linspace: (a: number, b: number, c: number) => [number, number, number];
  readonly vector_sum: (a: number) => number;
  readonly vector_transpose: (a: number) => number;
  readonly vector_to_column_matrix: (a: number) => number;
  readonly vector_to_row_matrix: (a: number) => number;
  readonly vector_data: (a: number) => [number, number];
  readonly solveLinearSystem: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number, number];
  readonly ridgeRegression: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number, number];
  readonly lassoRegression: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => [number, number, number, number];
  readonly logisticFit: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number, number, number];
  readonly logisticPredictProba: (a: number, b: number, c: number, d: number, e: number) => [number, number, number];
  readonly gmmFit: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number, number];
  readonly gmmPredictProba: (a: number, b: number, c: number, d: number, e: number) => [number, number, number, number];
  readonly bayesianLinearPosterior: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => [number, number, number, number];
  readonly kalmanPredict: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => [number, number, number, number];
  readonly kalmanUpdate: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => [number, number, number, number];
  readonly gf3_is_zero: (a: number) => number;
  readonly gf3_is_one: (a: number) => number;
  readonly init: () => void;
  readonly gf3_zero: () => number;
  readonly gf3_one: () => number;
  readonly reedsolomon_n: (a: number) => number;
  readonly reedsolomon_t: (a: number) => number;
  readonly matrix_columns: (a: number) => number;
  readonly vector_len: (a: number) => number;
  readonly gf3_value: (a: number) => bigint;
  readonly __wbg_wasmgf2_free: (a: number, b: number) => void;
  readonly wasmgf2_new: (a: bigint) => number;
  readonly wasmgf2_modulus: () => number;
  readonly wasmgf2_inv: (a: number) => [number, number, number];
  readonly wasmgf2_zero: () => number;
  readonly wasmgf2_one: () => number;
  readonly wasmgf2_add: (a: number, b: number) => number;
  readonly wasmgf2_sub: (a: number, b: number) => number;
  readonly wasmgf2_mul: (a: number, b: number) => number;
  readonly wasmgf2_value: (a: number) => bigint;
  readonly wasmgf2_div: (a: number, b: number) => [number, number, number];
  readonly wasmgf2_neg: (a: number) => number;
  readonly wasmgf2_is_zero: (a: number) => number;
  readonly wasmgf2_is_one: (a: number) => number;
  readonly __wbg_wasmgf3_free: (a: number, b: number) => void;
  readonly wasmgf3_new: (a: bigint) => number;
  readonly wasmgf3_modulus: () => number;
  readonly wasmgf3_inv: (a: number) => [number, number, number];
  readonly wasmgf3_add: (a: number, b: number) => number;
  readonly wasmgf3_sub: (a: number, b: number) => number;
  readonly wasmgf3_mul: (a: number, b: number) => number;
  readonly wasmgf3_div: (a: number, b: number) => [number, number, number];
  readonly wasmgf3_neg: (a: number) => number;
  readonly __wbg_wasmgf256_free: (a: number, b: number) => void;
  readonly wasmgf256_is_zero: (a: number) => number;
  readonly wasmgf256_is_one: (a: number) => number;
  readonly wasmgf256_toU8: (a: number) => number;
  readonly wasmgf256_inv: (a: number) => [number, number, number];
  readonly wasmgf256_zero: () => number;
  readonly wasmgf256_one: () => number;
  readonly wasmgf256_add: (a: number, b: number) => number;
  readonly wasmgf256_sub: (a: number, b: number) => number;
  readonly wasmgf256_mul: (a: number, b: number) => number;
  readonly wasmgf256_new: (a: number) => number;
  readonly wasmgf256_coeffs: (a: number) => [number, number];
  readonly wasmgf256_fromCoeffs: (a: number, b: number) => number;
  readonly wasmgf256_modulus: () => [number, number];
  readonly __wbg_wasmgfextgf2_free: (a: number, b: number) => void;
  readonly wasmgfextgf2_inv: (a: number) => [number, number, number];
  readonly wasmgfextgf2_add: (a: number, b: number) => number;
  readonly wasmgfextgf2_sub: (a: number, b: number) => number;
  readonly wasmgfextgf2_mul: (a: number, b: number) => number;
  readonly wasmgfextgf2_new: (a: number, b: number, c: number, d: number) => number;
  readonly wasmgfextgf2_fromBase: (a: number, b: number, c: number) => number;
  readonly wasmgfextgf2_coeffs: (a: number) => [number, number];
  readonly wasmgfextgf2_px: (a: number) => [number, number];
  readonly wasmgf256_div: (a: number, b: number) => [number, number, number];
  readonly wasmgf256_neg: (a: number) => number;
  readonly wasmgfextgf2_div: (a: number, b: number) => [number, number, number];
  readonly wasmgfextgf2_neg: (a: number) => number;
  readonly nt_factor_u64: (a: bigint) => [number, number];
  readonly nt_factor_bigint_str: (a: number, b: number) => [number, number];
  readonly __wbg_normal_free: (a: number, b: number) => void;
  readonly __wbg_uniform_free: (a: number, b: number) => void;
  readonly uniform_new: (a: number, b: number) => [number, number, number];
  readonly uniform_mean: (a: number) => number;
  readonly uniform_variance: (a: number) => number;
  readonly uniform_std_dev: (a: number) => number;
  readonly uniform_pdf: (a: number, b: number) => number;
  readonly uniform_cdf: (a: number, b: number) => number;
  readonly uniform_quantile: (a: number, b: number) => number;
  readonly uniform_pdf_svg: (a: number, b: number, c: number, d: number) => [number, number];
  readonly __wbg_studentt_free: (a: number, b: number) => void;
  readonly studentt_new: (a: number) => [number, number, number];
  readonly studentt_mean: (a: number) => number;
  readonly studentt_variance: (a: number) => number;
  readonly studentt_std_dev: (a: number) => number;
  readonly studentt_pdf: (a: number, b: number) => number;
  readonly studentt_cdf: (a: number, b: number) => number;
  readonly studentt_quantile: (a: number, b: number) => number;
  readonly studentt_pdf_svg: (a: number, b: number, c: number, d: number) => [number, number];
  readonly __wbg_chisquare_free: (a: number, b: number) => void;
  readonly chisquare_new: (a: number) => [number, number, number];
  readonly chisquare_k_param: (a: number) => number;
  readonly chisquare_mean: (a: number) => number;
  readonly chisquare_variance: (a: number) => number;
  readonly chisquare_std_dev: (a: number) => number;
  readonly chisquare_pdf: (a: number, b: number) => number;
  readonly chisquare_cdf: (a: number, b: number) => number;
  readonly chisquare_quantile: (a: number, b: number) => number;
  readonly chisquare_pdf_svg: (a: number, b: number, c: number, d: number) => [number, number];
  readonly __wbg_f_free: (a: number, b: number) => void;
  readonly f_new: (a: number, b: number) => [number, number, number];
  readonly f_d1: (a: number) => number;
  readonly f_d2: (a: number) => number;
  readonly f_mean: (a: number) => number;
  readonly f_variance: (a: number) => number;
  readonly f_std_dev: (a: number) => number;
  readonly f_pdf: (a: number, b: number) => number;
  readonly f_cdf: (a: number, b: number) => number;
  readonly f_quantile: (a: number, b: number) => number;
  readonly f_pdf_svg: (a: number, b: number, c: number, d: number) => [number, number];
  readonly __wbg_binomial_free: (a: number, b: number) => void;
  readonly binomial_new: (a: number, b: number) => [number, number, number];
  readonly binomial_n: (a: number) => number;
  readonly binomial_p: (a: number) => number;
  readonly binomial_mean: (a: number) => number;
  readonly binomial_variance: (a: number) => number;
  readonly binomial_std_dev: (a: number) => number;
  readonly binomial_pmf: (a: number, b: number) => number;
  readonly binomial_cdf: (a: number, b: number) => number;
  readonly binomial_quantile: (a: number, b: number) => number;
  readonly binomial_pmf_svg: (a: number, b: number, c: number) => [number, number];
  readonly __wbg_categorical_free: (a: number, b: number) => void;
  readonly categorical_new: (a: number, b: number) => [number, number, number];
  readonly categorical_probs: (a: number) => [number, number];
  readonly categorical_pmf: (a: number, b: number) => number;
  readonly categorical_log_pmf: (a: number, b: number) => number;
  readonly categorical_cdf: (a: number, b: number) => number;
  readonly categorical_quantile: (a: number, b: number) => number;
  readonly categorical_pmf_svg: (a: number, b: number, c: number) => [number, number];
  readonly normal_new: (a: number, b: number) => [number, number, number];
  readonly normal_mu: (a: number) => number;
  readonly normal_sigma: (a: number) => number;
  readonly normal_variance: (a: number) => number;
  readonly normal_std_dev: (a: number) => number;
  readonly normal_pdf: (a: number, b: number) => number;
  readonly normal_cdf: (a: number, b: number) => number;
  readonly normal_quantile: (a: number, b: number) => number;
  readonly normal_pdf_svg: (a: number, b: number, c: number, d: number) => [number, number];
  readonly __wbg_gamma_free: (a: number, b: number) => void;
  readonly gamma_new: (a: number, b: number) => [number, number, number];
  readonly gamma_rate: (a: number) => number;
  readonly gamma_mean: (a: number) => number;
  readonly gamma_variance: (a: number) => number;
  readonly gamma_std_dev: (a: number) => number;
  readonly gamma_pdf: (a: number, b: number) => number;
  readonly gamma_cdf: (a: number, b: number) => number;
  readonly gamma_quantile: (a: number, b: number) => number;
  readonly gamma_pdf_svg: (a: number, b: number, c: number, d: number) => [number, number];
  readonly __wbg_exponential_free: (a: number, b: number) => void;
  readonly exponential_new: (a: number) => [number, number, number];
  readonly exponential_mean: (a: number) => number;
  readonly exponential_variance: (a: number) => number;
  readonly exponential_std_dev: (a: number) => number;
  readonly exponential_pdf: (a: number, b: number) => number;
  readonly exponential_cdf: (a: number, b: number) => number;
  readonly exponential_quantile: (a: number, b: number) => number;
  readonly exponential_pdf_svg: (a: number, b: number, c: number, d: number) => [number, number];
  readonly __wbg_bernoulli_free: (a: number, b: number) => void;
  readonly bernoulli_new: (a: number) => [number, number, number];
  readonly bernoulli_p: (a: number) => number;
  readonly bernoulli_mean: (a: number) => number;
  readonly bernoulli_variance: (a: number) => number;
  readonly bernoulli_std_dev: (a: number) => number;
  readonly bernoulli_pmf: (a: number, b: number) => number;
  readonly bernoulli_cdf: (a: number, b: number) => number;
  readonly bernoulli_quantile: (a: number, b: number) => number;
  readonly bernoulli_pmf_svg: (a: number, b: number, c: number) => [number, number];
  readonly __wbg_poisson_free: (a: number, b: number) => void;
  readonly poisson_new: (a: number) => [number, number, number];
  readonly poisson_std_dev: (a: number) => number;
  readonly poisson_pmf: (a: number, b: number) => number;
  readonly poisson_log_pmf: (a: number, b: number) => number;
  readonly poisson_cdf: (a: number, b: number) => number;
  readonly poisson_quantile: (a: number, b: number) => number;
  readonly poisson_pmf_svg: (a: number, b: number, c: number) => [number, number];
  readonly wasmgf3_is_zero: (a: number) => number;
  readonly wasmgf3_is_one: (a: number) => number;
  readonly wasmgfextgf2_zero: () => number;
  readonly wasmgfextgf2_one: () => number;
  readonly wasmgf256_value: (a: number) => number;
  readonly wasmgf3_zero: () => number;
  readonly wasmgf3_one: () => number;
  readonly wasmgfextgf2_is_zero: (a: number) => number;
  readonly wasmgfextgf2_is_one: (a: number) => number;
  readonly studentt_df: (a: number) => number;
  readonly uniform_a: (a: number) => number;
  readonly normal_mean: (a: number) => number;
  readonly gamma_shape: (a: number) => number;
  readonly uniform_b: (a: number) => number;
  readonly exponential_lambda: (a: number) => number;
  readonly poisson_lambda: (a: number) => number;
  readonly poisson_mean: (a: number) => number;
  readonly poisson_variance: (a: number) => number;
  readonly wasmgf3_value: (a: number) => bigint;
  readonly __wbg_matrixf64_free: (a: number, b: number) => void;
  readonly matrixf64_new: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly matrixf64_with_default: (a: number, b: number) => number;
  readonly matrixf64_zeros: (a: number, b: number) => number;
  readonly matrixf64_identity: (a: number) => number;
  readonly matrixf64_transpose: (a: number) => number;
  readonly matrixf64_trace: (a: number) => [number, number, number];
  readonly matrixf64_determinant: (a: number) => [number, number, number];
  readonly matrixf64_rank: (a: number) => [number, number, number];
  readonly matrixf64_inverse: (a: number) => number;
  readonly matrixf64_frobenius_norm: (a: number) => number;
  readonly matrixf64_expm: (a: number) => number;
  readonly matrixf64_qr_decomposition: (a: number) => [number, number, number];
  readonly matrixf64_svd: (a: number) => [number, number, number];
  readonly matrixf64_eigen_decomposition: (a: number) => [number, number, number];
  readonly matrixf64_cholesky: (a: number) => [number, number, number];
  readonly matrixf64_pinv: (a: number) => [number, number, number];
  readonly matrixf64_add: (a: number, b: number) => number;
  readonly matrixf64_sub: (a: number, b: number) => number;
  readonly matrixf64_mul: (a: number, b: number) => number;
  readonly __wbg_vectorf64_free: (a: number, b: number) => void;
  readonly vectorf64_new: (a: number, b: number) => number;
  readonly vectorf64_zeros: (a: number) => number;
  readonly vectorf64_ones: (a: number) => number;
  readonly vectorf64_dim: (a: number) => number;
  readonly vectorf64_is_empty: (a: number) => number;
  readonly vectorf64_dot: (a: number, b: number) => number;
  readonly vectorf64_argmax: (a: number) => number;
  readonly vectorf64_argmin: (a: number) => number;
  readonly vectorf64_max: (a: number) => [number, number];
  readonly vectorf64_min: (a: number) => [number, number];
  readonly vectorf64_norm: (a: number) => number;
  readonly vectorf64_normalize: (a: number) => number;
  readonly vectorf64_cosine_similarity: (a: number, b: number) => number;
  readonly vectorf64_mean: (a: number) => [number, number];
  readonly vectorf64_std: (a: number) => number;
  readonly vectorf64_linspace: (a: number, b: number, c: number) => [number, number, number];
  readonly vectorf64_add: (a: number, b: number) => number;
  readonly vectorf64_sub: (a: number, b: number) => number;
  readonly vectorf64_mul: (a: number, b: number) => number;
  readonly vectorf64_sum: (a: number) => number;
  readonly vectorf64_multiply_matrix: (a: number, b: number) => [number, number, number];
  readonly matrixf64_get: (a: number, b: number, c: number) => number;
  readonly matrixf64_row: (a: number, b: number) => number;
  readonly matrixf64_col: (a: number, b: number) => number;
  readonly matrixf64_multiply_vector: (a: number, b: number) => number;
  readonly matrixf64_diagonal: (a: number) => number;
  readonly matrixf64_solve: (a: number, b: number) => number;
  readonly matrixf64_data: (a: number) => [number, number];
  readonly matrixf64_rref: (a: number) => [number, number, number];
  readonly matrixf64_rrefWith: (a: number, b: number) => [number, number, number];
  readonly matrixf64_lu: (a: number) => [number, number, number];
  readonly vectorf64_transpose: (a: number) => number;
  readonly vectorf64_to_column_matrix: (a: number) => number;
  readonly vectorf64_to_row_matrix: (a: number) => number;
  readonly vectorf64_data: (a: number) => [number, number];
  readonly __wbg_matrixf32_free: (a: number, b: number) => void;
  readonly matrixf32_add: (a: number, b: number) => number;
  readonly matrixf32_sub: (a: number, b: number) => number;
  readonly matrixf32_mul: (a: number, b: number) => number;
  readonly matrixf32_new: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly matrixf32_with_default: (a: number, b: number) => number;
  readonly matrixf32_zeros: (a: number, b: number) => number;
  readonly matrixf32_identity: (a: number) => number;
  readonly matrixf32_rows: (a: number) => number;
  readonly matrixf32_cols: (a: number) => number;
  readonly matrixf32_is_square: (a: number) => number;
  readonly matrixf32_transpose: (a: number) => number;
  readonly matrixf32_trace: (a: number) => [number, number, number];
  readonly matrixf32_determinant: (a: number) => [number, number, number];
  readonly matrixf32_rank: (a: number) => [number, number, number];
  readonly matrixf32_inverse: (a: number) => number;
  readonly matrixf32_data: (a: number) => [number, number];
  readonly matrixf32_rref: (a: number) => [number, number, number];
  readonly matrixf32_rrefWith: (a: number, b: number) => [number, number, number];
  readonly matrixf32_lu: (a: number) => [number, number, number];
  readonly __wbg_vectorf32_free: (a: number, b: number) => void;
  readonly vectorf32_add: (a: number, b: number) => number;
  readonly vectorf32_sub: (a: number, b: number) => number;
  readonly vectorf32_mul: (a: number, b: number) => number;
  readonly vectorf32_new: (a: number, b: number) => number;
  readonly vectorf32_zeros: (a: number) => number;
  readonly vectorf32_ones: (a: number) => number;
  readonly vectorf32_dim: (a: number) => number;
  readonly vectorf32_is_empty: (a: number) => number;
  readonly vectorf32_dot: (a: number, b: number) => number;
  readonly vectorf32_argmax: (a: number) => number;
  readonly vectorf32_argmin: (a: number) => number;
  readonly vectorf32_max: (a: number) => number;
  readonly vectorf32_min: (a: number) => number;
  readonly vectorf32_data: (a: number) => [number, number];
  readonly __wbg_matrixi32_free: (a: number, b: number) => void;
  readonly matrixi32_add: (a: number, b: number) => number;
  readonly matrixi32_sub: (a: number, b: number) => number;
  readonly matrixi32_mul: (a: number, b: number) => number;
  readonly matrixi32_new: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly matrixi32_identity: (a: number) => number;
  readonly matrixi32_transpose: (a: number) => number;
  readonly matrixi32_trace: (a: number) => [number, number, number];
  readonly matrixi32_data: (a: number) => [number, number];
  readonly __wbg_vectori32_free: (a: number, b: number) => void;
  readonly vectori32_add: (a: number, b: number) => number;
  readonly vectori32_sub: (a: number, b: number) => number;
  readonly vectori32_mul: (a: number, b: number) => number;
  readonly vectori32_ones: (a: number) => number;
  readonly vectori32_dot: (a: number, b: number) => number;
  readonly vectori32_argmax: (a: number) => number;
  readonly vectori32_argmin: (a: number) => number;
  readonly vectori32_max: (a: number) => number;
  readonly vectori32_min: (a: number) => number;
  readonly vectori32_data: (a: number) => [number, number];
  readonly matrixf64_is_square: (a: number) => number;
  readonly matrixi32_is_square: (a: number) => number;
  readonly vectori32_is_empty: (a: number) => number;
  readonly vectori32_new: (a: number, b: number) => number;
  readonly matrixf64_columns: (a: number) => number;
  readonly matrixf64_rows: (a: number) => number;
  readonly matrixf64_cols: (a: number) => number;
  readonly matrixf32_columns: (a: number) => number;
  readonly matrixi32_rows: (a: number) => number;
  readonly matrixi32_cols: (a: number) => number;
  readonly matrixi32_columns: (a: number) => number;
  readonly matrixi32_zeros: (a: number, b: number) => number;
  readonly matrixi32_with_default: (a: number, b: number) => number;
  readonly vectorf64_len: (a: number) => number;
  readonly vectorf32_len: (a: number) => number;
  readonly vectori32_dim: (a: number) => number;
  readonly vectori32_len: (a: number) => number;
  readonly vectori32_zeros: (a: number) => number;
  readonly __wbg_wasmsignal_free: (a: number, b: number) => void;
  readonly wasmsignal_new: (a: number, b: number, c: number) => number;
  readonly wasmsignal_data: (a: number) => [number, number];
  readonly wasmsignal_sample_rate: (a: number) => number;
  readonly wasmsignal_len: (a: number) => number;
  readonly wasmsignal_dft: (a: number) => number;
  readonly wasmsignal_convolve: (a: number, b: number) => number;
  readonly wasmsignal_apply_fir: (a: number, b: number, c: number) => number;
  readonly wasmsignal_downsample: (a: number, b: number, c: number) => number;
  readonly wasmsignal_upsample: (a: number, b: number, c: number) => number;
  readonly wasmsignal_resample: (a: number, b: number, c: number, d: number) => number;
  readonly wasmsignal_decimate: (a: number, b: number) => number;
  readonly wasmsignal_expand: (a: number, b: number) => number;
  readonly wasmsignal_save_svg_simple: (a: number, b: number, c: number) => [number, number];
  readonly wasmsignal_save_svg_with_axes: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly __wbg_wasmspectrum_free: (a: number, b: number) => void;
  readonly wasmspectrum_data_interleaved: (a: number) => [number, number];
  readonly wasmspectrum_len: (a: number) => number;
  readonly wasmspectrum_ift: (a: number) => number;
  readonly wasmspectrum_magnitude_db_svg: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly window_hann: (a: number) => [number, number];
  readonly window_hamming: (a: number) => [number, number];
  readonly window_blackman: (a: number) => [number, number];
  readonly window_rectangular: (a: number) => [number, number];
  readonly window_kaiser: (a: number, b: number) => [number, number];
  readonly sp_design_fir_lowpass: (a: number, b: number) => [number, number];
  readonly sp_design_fir_highpass: (a: number, b: number) => [number, number];
  readonly sp_design_fir_bandpass: (a: number, b: number, c: number) => [number, number];
  readonly sp_design_fir_bandstop: (a: number, b: number, c: number) => [number, number];
  readonly sp_design_iir_butter_lowpass: (a: number, b: number, c: number) => number;
  readonly sp_design_iir_butter_highpass: (a: number, b: number, c: number) => number;
  readonly sp_design_iir_butter_bandpass: (a: number, b: number, c: number, d: number) => number;
  readonly sp_design_iir_butter_bandstop: (a: number, b: number, c: number, d: number) => number;
  readonly sp_design_iir_cheby1_lowpass: (a: number, b: number, c: number, d: number) => number;
  readonly sp_design_iir_cheby2_lowpass: (a: number, b: number, c: number, d: number) => number;
  readonly __wbg_wasmlms_free: (a: number, b: number) => void;
  readonly wasmlms_new: (a: number, b: number) => number;
  readonly wasmlms_process_sample: (a: number, b: number, c: number) => [number, number];
  readonly wasmlms_process_series: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly __wbg_wasmnlms_free: (a: number, b: number) => void;
  readonly wasmnlms_new: (a: number, b: number, c: number) => number;
  readonly wasmnlms_process_sample: (a: number, b: number, c: number) => [number, number];
  readonly wasmnlms_process_series: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly img_convolve2d_f32_simple: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => [number, number];
  readonly img_convolve2d_f32: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => [number, number];
  readonly img_convolve2d_u8: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => [number, number];
  readonly img_dft2d: (a: number, b: number, c: number, d: number) => [number, number];
  readonly img_idft2d: (a: number, b: number, c: number, d: number) => [number, number];
  readonly img_fftshift: (a: number, b: number, c: number, d: number) => [number, number];
  readonly img_magnitude: (a: number, b: number, c: number, d: number) => [number, number];
  readonly img_gaussian_blur_f32: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
  readonly img_gaussian_blur_u8: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
  readonly img_box_filter_f32: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly img_box_filter_u8: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly img_unsharp_mask_f32: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
  readonly img_unsharp_mask_u8: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
  readonly img_sobel_magnitude_f32: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly img_sobel_magnitude_u8: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly img_laplacian_f32: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly img_laplacian_u8: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly img_median_filter_f32: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly img_median_filter_u8: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly img_bilateral_filter_f32: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
  readonly img_bilateral_filter_u8: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
  readonly rgb_u8_to_gray_f64: (a: number, b: number, c: number, d: number) => [number, number];
  readonly rgba_u8_to_gray_f64: (a: number, b: number, c: number, d: number) => [number, number];
  readonly gray_f64_to_rgba_u8: (a: number, b: number, c: number, d: number) => [number, number];
  readonly u8_to_gray_f64: (a: number, b: number) => [number, number];
  readonly gray_f64_to_u8_clamped: (a: number, b: number) => [number, number];
  readonly img_convolve2d_f32_io: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => [number, number];
  readonly img_gaussian_blur_f32_io: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
  readonly wasmspectrum_new: (a: number, b: number, c: number) => number;
  readonly wasmspectrum_sample_rate: (a: number) => number;
  readonly dftComplexF64: (a: number, b: number) => [number, number];
  readonly iftComplexF64: (a: number, b: number) => [number, number];
  readonly erf: (a: number) => number;
  readonly erfc: (a: number) => number;
  readonly erfInv: (a: number) => number;
  readonly gamma: (a: number) => number;
  readonly logGamma: (a: number) => number;
  readonly regularizedGamma: (a: number, b: number) => number;
  readonly beta: (a: number, b: number) => number;
  readonly logBeta: (a: number, b: number) => number;
  readonly regularizedBeta: (a: number, b: number, c: number) => number;
  readonly __wbg_discretetf_free: (a: number, b: number) => void;
  readonly discretetf_new: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly discretetf_sample_rate: (a: number) => number;
  readonly discretetf_set_sample_rate: (a: number, b: number) => void;
  readonly discretetf_b_coeffs: (a: number) => [number, number];
  readonly discretetf_a_coeffs: (a: number) => [number, number];
  readonly discretetf_is_stable: (a: number) => number;
  readonly discretetf_impulse_response: (a: number, b: number) => [number, number];
  readonly discretetf_step_response: (a: number, b: number) => [number, number];
  readonly discretetf_frequency_response_mag_phase: (a: number, b: number) => [number, number];
  readonly discretetf_bode_svg: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly discretetf_nyquist_svg: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly discretetf_series: (a: number, b: number) => number;
  readonly discretetf_parallel: (a: number, b: number) => number;
  readonly discretetf_feedback_unity: (a: number) => number;
  readonly discretetf_block_feedback_svg: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly __wbg_continuoustf_free: (a: number, b: number) => void;
  readonly continuoustf_new: (a: number, b: number, c: number, d: number) => number;
  readonly continuoustf_b_coeffs: (a: number) => [number, number];
  readonly continuoustf_a_coeffs: (a: number) => [number, number];
  readonly continuoustf_is_stable: (a: number) => number;
  readonly continuoustf_impulse_response: (a: number, b: number, c: number) => [number, number];
  readonly continuoustf_step_response: (a: number, b: number, c: number) => [number, number];
  readonly continuoustf_frequency_response_mag_phase: (a: number, b: number, c: number) => [number, number];
  readonly continuoustf_to_discrete_bilinear: (a: number, b: number) => number;
  readonly continuoustf_to_discrete_bilinear_prewarp: (a: number, b: number, c: number) => number;
  readonly continuoustf_bode_svg: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
  readonly continuoustf_nyquist_svg: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
  readonly continuoustf_block_feedback_svg: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly __wbg_continuouszpk_free: (a: number, b: number) => void;
  readonly continuouszpk_new: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly continuouszpk_from_tf: (a: number) => number;
  readonly continuouszpk_to_tf: (a: number) => number;
  readonly continuouszpk_zeros_interleaved: (a: number) => [number, number];
  readonly continuouszpk_poles_interleaved: (a: number) => [number, number];
  readonly continuouszpk_gain: (a: number) => number;
  readonly __wbg_discretezpk_free: (a: number, b: number) => void;
  readonly discretezpk_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly discretezpk_from_tf: (a: number) => number;
  readonly discretezpk_to_tf: (a: number) => number;
  readonly discretezpk_zeros_interleaved: (a: number) => [number, number];
  readonly discretezpk_poles_interleaved: (a: number) => [number, number];
  readonly discretezpk_sample_rate: (a: number) => number;
  readonly __wbg_continuousss_free: (a: number, b: number) => void;
  readonly continuousss_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number) => number;
  readonly continuousss_from_tf_siso: (a: number, b: number, c: number, d: number) => number;
  readonly continuousss_to_tf_siso: (a: number) => [number, number];
  readonly continuousss_c2d_zoh: (a: number, b: number) => number;
  readonly continuousss_a_flat: (a: number) => [number, number];
  readonly continuousss_b_flat: (a: number) => [number, number];
  readonly continuousss_c_flat: (a: number) => [number, number];
  readonly continuousss_d_flat: (a: number) => [number, number];
  readonly continuousss_a_shape: (a: number) => [number, number];
  readonly continuousss_b_shape: (a: number) => [number, number];
  readonly continuousss_c_shape: (a: number) => [number, number];
  readonly continuousss_d_shape: (a: number) => [number, number];
  readonly __wbg_discretess_free: (a: number, b: number) => void;
  readonly discretess_to_tf_siso: (a: number) => [number, number];
  readonly discretess_a_flat: (a: number) => [number, number];
  readonly discretess_b_flat: (a: number) => [number, number];
  readonly discretess_c_flat: (a: number) => [number, number];
  readonly discretess_d_flat: (a: number) => [number, number];
  readonly discretess_a_shape: (a: number) => [number, number];
  readonly discretess_b_shape: (a: number) => [number, number];
  readonly discretess_c_shape: (a: number) => [number, number];
  readonly discretess_d_shape: (a: number) => [number, number];
  readonly __wbg_wasmlinearmodel_free: (a: number, b: number) => void;
  readonly wasmlinearmodel_solveLinearSystem: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number, number];
  readonly wasmlinearmodel_ridgeRegression: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number, number];
  readonly wasmlinearmodel_lassoRegression: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => [number, number, number, number];
  readonly discretezpk_gain: (a: number) => number;
  readonly discretess_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
