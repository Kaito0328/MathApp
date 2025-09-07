// Mirrors for lti-systems crate key structures (simplified DTO view)

export interface TransferFunction {
  // numerator and denominator coefficients (ascending powers)
  num: number[];
  den: number[];
  sample_time?: number | null; // null for continuous
}

export interface Zpk {
  zeros: number[]; // represent complex as interleaved [re, im, ...] in DTOs when needed
  poles: number[];
  gain: number;
  sample_time?: number | null;
}
