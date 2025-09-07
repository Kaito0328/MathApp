import { Complex } from "./complex";

export interface Polynomial {
    coeffs: number[]; // coeffs[0] + coeffs[1] * x + coeffs[2] * x^2 + ...
}

export interface RationalFunction {
    numerator: Polynomial;
    denominator: Polynomial;
}

export interface Root {
    value: Complex;
    multiplicity: number;
}

export type Pole = Root;

export interface PoleTerm {
    pole: Complex;
    coefficients: Complex[]; // coefficients for (s - pole)^-1, (s - pole)^-2, ...
}

export interface PartialFractionExpansion {
    polynomial_part: Polynomial; // polynomial part (if improper)
    pole_terms: PoleTerm[]; // terms for each pole
}