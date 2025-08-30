// Minimal TS mirrors for linalg crate where helpful on the frontend side

export type Scalar = number;

export interface Vector {
  data: Scalar[];
}

export interface Matrix {
  rows: number;
  cols: number;
  // row-major data length = rows*cols
  data: Scalar[];
}
