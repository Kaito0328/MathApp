import { Complex } from "./complex";

export interface Signal {
    data: number[];
    sample_rate: number;
};

export interface Spectrum {
    data: Complex[]; // length N
    sample_rate: number;
}