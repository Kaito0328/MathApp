// DTOs for distribution parameter inputs

export interface NormalParams { mu: number; sigma: number }
export interface UniformParams { a: number; b: number }
export interface ExponentialParams { lambda: number }
export interface GammaParams { shape: number; rate: number }
export interface ChiSquareParams { k: number }
export interface StudentTParams { df: number }
export interface FParams { d1: number; d2: number }

export interface BernoulliParams { p: number }
export interface BinomialParams { n: number; p: number }
export interface PoissonParams { lambda: number }
export interface CategoricalParams { probs: number[] }
