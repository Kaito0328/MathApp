// WASM-side convenience DTO for matrices created/consumed via wrappers
export interface MatrixDTO {
  rows: number;
  cols: number;
  data: Float64Array;
}
