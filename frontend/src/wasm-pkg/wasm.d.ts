/* tslint:disable */
/* eslint-disable */
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
export function convolveNaiveF64(x: Float64Array, h: Float64Array): Float64Array;
export function convolveFftF64(x: Float64Array, h: Float64Array): Float64Array;
export function convolveAutoF64(x: Float64Array, h: Float64Array, threshold: number): Float64Array;
export function defaultConvolutionThreshold(): number;
export function dftComplexF64(x_flat: Float64Array): Float64Array;
export function iftComplexF64(x_flat: Float64Array): Float64Array;
export function binom(n: number, k: number): number;
export function stirling2(n: number, k: number): number;
export function fallingFactorialPoly(m: number): Float64Array;
export function risingFactorialPoly(m: number): Float64Array;
export function shiftPolyXPlusH(coeffs_flat: Float64Array, h: number): Float64Array;
export function discreteDiff(coeffs_flat: Float64Array): Float64Array;
export function discreteSum(coeffs_flat: Float64Array): Float64Array;
export function solveRecurrence(coeffs: Float64Array, nh_polys_flat: Float64Array, nh_offsets: Uint32Array, nh_bases: Float64Array, initial_values: Float64Array): ClosedForm;
export function hammingDistanceGF2(a: Uint8Array, b: Uint8Array): number;
export function weightDistributionGF2(codebook_flat: Uint8Array, n: number): Uint32Array;
export function nt_factor_u64(n: bigint): BigUint64Array;
export function nt_factor_bigint_str(n_str: string): string[];
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
  encode(u: Uint8Array): Uint8Array;
  k(): number;
  n(): number;
  t(): number;
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
}
export class Categorical {
  free(): void;
  constructor(probs: Float64Array);
  pmf(k: number): number;
  log_pmf(k: number): number;
  cdf(k: number): number;
  quantile(p: number): number;
  pmf_svg(width: number, height: number): string;
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
}
export class ClosedForm {
  private constructor();
  free(): void;
  termsCount(): number;
  termPoly(i: number): Float64Array;
  termBase(i: number): Float64Array;
  term(n: number): Float64Array;
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
export class CyclicCode {
  free(): void;
  constructor(n: number, g: Uint8Array);
  encode(u: Uint8Array): Uint8Array;
  k(): number;
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
}
export class Hamming74 {
  private constructor();
  free(): void;
}
export class LinearCode {
  free(): void;
  constructor(k: number, n: number, g_data: Uint8Array);
  encode(u: Uint8Array): Uint8Array;
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
}
export class ReedSolomon {
  free(): void;
  constructor(k: number, alphas: Uint8Array);
  encode(f: Uint8Array): Uint8Array;
  decode(r: Uint8Array): Uint8Array;
  n(): number;
  t(): number;
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
