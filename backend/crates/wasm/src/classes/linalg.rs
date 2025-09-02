#![allow(unused_macros)]  // マクロ内マクロのため警告を抑制

#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use linalg::{Matrix as LMatrix, Ring, Field, LinalgError, Result, Vector as LVector, Direction};
#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use linalg::matrix::numerical::qr::{QR, QrDecomposition};
#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use linalg::matrix::numerical::svd::Svd;
#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use linalg::matrix::numerical::eigen::{Eigen, EigenComplex};
#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use wasm_bindgen::JsValue;
#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use linalg::matrix::numerical::{CholeskyDecomposition, EigenDecomposition, MatrixExponential, Pseudoinverse, SvdDeComposition};
#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use linalg::traits::LinalgField;

use wasm_bindgen::prelude::*;

// 内部型のエイリアス（属性マクロのパラメータで <T> を避けるため）
type InternalMatrixF64 = linalg::Matrix<f64>;
type InternalVectorF64 = linalg::Vector<f64>;
type InternalMatrixF32 = linalg::Matrix<f32>;
type InternalVectorF32 = linalg::Vector<f32>;
type InternalMatrixI32 = linalg::Matrix<i32>;
type InternalVectorI32 = linalg::Vector<i32>;

// ==============================
// 階層的メソッド宣言マクロ（空ボディ→wasm_classが自動委譲）
// ==============================

// Matrix: Ringレベル（全型共通）
macro_rules! matrix_ring_methods {
    ($t:ty) => {
        #[constructor]
        pub fn new(rows: usize, cols: usize, data: Vec<$t>) -> std::result::Result<Self, wasm_bindgen::JsValue> {}
        pub fn with_default(rows: usize, cols: usize) -> Self {}
        pub fn zeros(rows: usize, cols: usize) -> Self {}
        pub fn identity(size: usize) -> Self {}
        pub fn rows(&self) -> usize {}
        pub fn cols(&self) -> usize {}
        pub fn is_square(&self) -> bool {}
        pub fn transpose(&self) -> Self {}
        pub fn trace(&self) -> std::result::Result<$t, wasm_bindgen::JsValue> {}
    };
}

// Matrix: Fieldレベル（Ringと重複しない追加）
macro_rules! matrix_field_methods {
    ($t:ty) => {
        pub fn determinant(&self) -> std::result::Result<$t, wasm_bindgen::JsValue> {}
        pub fn rank(&self) -> std::result::Result<usize, wasm_bindgen::JsValue> {}
        pub fn inverse(&self) -> Option<Self> {}
    };
}

// Matrix: f64専用の数値メソッド
macro_rules! matrix_f64_methods {
    () => {
        pub fn frobenius_norm(&self) -> f64 {}
        pub fn expm(&self) -> Self {}
        pub fn qr_decomposition(&self) -> std::result::Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {}
        pub fn svd(&self) -> std::result::Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {}
        pub fn eigen_decomposition(&self) -> std::result::Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {}
        pub fn cholesky(&self) -> std::result::Result<Self, wasm_bindgen::JsValue> {}
        pub fn pinv(&self) -> std::result::Result<Self, wasm_bindgen::JsValue> {}
    };
}

// Vector: Ringレベル
macro_rules! vector_ring_methods {
    ($t:ty) => {
        #[constructor]
        pub fn new(data: Vec<$t>) -> Self {}
        pub fn zeros(dim: usize) -> Self {}
        pub fn ones(dim: usize) -> Self {}
        pub fn dim(&self) -> usize {}
        pub fn len(&self) -> usize {}
        pub fn is_empty(&self) -> bool {}
        pub fn dot(&self, other: &Self) -> $t {}
        pub fn argmax(&self) -> Option<usize> {}
        pub fn argmin(&self) -> Option<usize> {}
        pub fn max(&self) -> Option<$t> {}
        pub fn min(&self) -> Option<$t> {}
    };
}

// Vector: f64専用
macro_rules! vector_f64_methods {
    () => {
        pub fn norm(&self) -> f64 {}
        pub fn normalize(&self) -> Self {}
        pub fn cosine_similarity(&self, other: &Self) -> f64 {}
        pub fn mean(&self) -> Option<f64> {}
        pub fn std(&self) -> f64 {}
        pub fn linspace(start: f64, end: f64, num: usize) -> std::result::Result<Self, wasm_bindgen::JsValue> {}
    };
}

// ==============================
// 改善されたジェネリック実装アプローチ
// ==============================
// 
// 現在のwasm_macrosシステムでは以下の制約があります:
// 1. 空のブロック{}内でマクロが正常に展開されない
// 2. trait_method属性が正しく機能しない場合がある
// 3. ジェネリック型の制約が複雑
//
// 解決策: 個別実装 + DRY原則
// - 各型ごとに明示的に実装
// - linalgクレートへの直接委譲でコード重複を最小化
// - 型安全性と機能の完全性を保証

// ==============================
// MatrixF64: f64専用数値計算行列
// ==============================

// ラッパーマクロ: 属性付きimplを外側のmacro_rulesで生成し、内部メソッド宣言マクロを事前展開させる
#[wasm_macros::wasm_class(
    internal = "InternalMatrixF64",
    js_name = "MatrixF64",
    ops(Add, Sub, Mul),
    indexer = false,
    iterator = false
)]
impl MatrixF64 {
    // Ring level
    #[constructor]
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> std::result::Result<Self, wasm_bindgen::JsValue> {}
    pub fn with_default(rows: usize, cols: usize) -> Self {}
    pub fn zeros(rows: usize, cols: usize) -> Self {}
    pub fn identity(size: usize) -> Self {}
    pub fn rows(&self) -> usize {}
    pub fn cols(&self) -> usize {}
    pub fn is_square(&self) -> bool {}
    pub fn transpose(&self) -> Self {}
    pub fn trace(&self) -> std::result::Result<f64, wasm_bindgen::JsValue> {}

    // Field level
    pub fn determinant(&self) -> std::result::Result<f64, wasm_bindgen::JsValue> {}
    pub fn rank(&self) -> std::result::Result<usize, wasm_bindgen::JsValue> {}
    pub fn inverse(&self) -> Option<Self> {}

    // f64 numerical
    pub fn frobenius_norm(&self) -> f64 {}
    pub fn expm(&self) -> Self {}
    pub fn qr_decomposition(&self) -> std::result::Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {}
    pub fn svd(&self) -> std::result::Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {}
    pub fn eigen_decomposition(&self) -> std::result::Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {}
    pub fn cholesky(&self) -> std::result::Result<Self, wasm_bindgen::JsValue> {}
    pub fn pinv(&self) -> std::result::Result<Self, wasm_bindgen::JsValue> {}
}

// wasm_classでは扱いにくいクロスリファレンスや型包み替えだけ個別に残す

// ==============================
// VectorF64: f64専用数値計算ベクトル
// ==============================

#[wasm_macros::wasm_class(
    internal = "InternalVectorF64",
    js_name = "VectorF64", 
    ops(Add, Sub, Mul),
    indexer = false,
    iterator = false
)]
impl VectorF64 {
    // Ring level
    #[constructor]
    pub fn new(data: Vec<f64>) -> Self {}
    pub fn zeros(dim: usize) -> Self {}
    pub fn ones(dim: usize) -> Self {}
    pub fn dim(&self) -> usize {}
    pub fn len(&self) -> usize {}
    pub fn is_empty(&self) -> bool {}
    pub fn dot(&self, other: &Self) -> f64 {}
    pub fn argmax(&self) -> Option<usize> {}
    pub fn argmin(&self) -> Option<usize> {}
    pub fn max(&self) -> Option<f64> {}
    pub fn min(&self) -> Option<f64> {}

    // f64 numerical
    pub fn norm(&self) -> f64 {}
    pub fn normalize(&self) -> Self {}
    pub fn cosine_similarity(&self, other: &Self) -> f64 {}
    pub fn mean(&self) -> Option<f64> {}
    pub fn std(&self) -> f64 {}
    pub fn linspace(start: f64, end: f64, num: usize) -> std::result::Result<Self, wasm_bindgen::JsValue> {}
}
// wasm_classでは扱いにくいもの（Resultや独自ロジック）は個別に実装
#[wasm_bindgen]
impl VectorF64 {
    #[wasm_bindgen]
    pub fn sum(&self) -> f64 { self.0.sum() }

    #[wasm_bindgen]
    pub fn multiply_matrix(&self, matrix: &MatrixF64) -> std::result::Result<MatrixF64, JsValue> {
        self.0
            .checked_mul_matrix(&matrix.0)
            .map(MatrixF64)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }
}
// ==============================
// Matrix ↔ Vector クロスリファレンス実装（DRY原則遵守）
// ==============================

#[wasm_bindgen]
impl MatrixF64 {
    // 要素取得
    #[wasm_bindgen]
    pub fn get(&self, row: usize, col: usize) -> f64 { self.0[(row, col)] }
    // === Row/Column extraction（linalgクレートの実装を使用） ===
    #[wasm_bindgen]
    pub fn row(&self, index: usize) -> Option<VectorF64> {
        self.0.row(index).ok().map(VectorF64)
    }
    
    #[wasm_bindgen]
    pub fn col(&self, index: usize) -> Option<VectorF64> {
        self.0.col(index).ok().map(VectorF64)
    }
    
    // === Matrix-Vector操作（linalgクレートの実装を使用） ===
    #[wasm_bindgen]
    pub fn multiply_vector(&self, vector: &VectorF64) -> Option<VectorF64> {
        if self.0.cols != vector.0.len() {
            return None;
        }
        // linalgクレートのMatrix * Vector演算を使用
        Some(VectorF64(&self.0 * &vector.0))
    }
    
    #[wasm_bindgen]
    pub fn diagonal(&self) -> VectorF64 { VectorF64(self.0.diagonal()) }

    // === solve系メソッド（linalgクレートに実装済みの可能性を活用） ===
    #[wasm_bindgen]
    pub fn solve(&self, b: &VectorF64) -> Option<VectorF64> {
        // Matrix::solve メソッドを使用
        self.0.solve(&b.0).ok().map(VectorF64)
    }
}

#[wasm_bindgen]
impl VectorF64 {
    // ベクトルの転置（行/列ベクトル→行列）
    #[wasm_bindgen]
    pub fn transpose(&self) -> MatrixF64 { MatrixF64(self.0.transpose()) }
    // === Matrix conversion（linalgクレートの実装を使用） ===
    #[wasm_bindgen]
    pub fn to_column_matrix(&self) -> MatrixF64 {
        let data = self.0.as_slice().to_vec();
        MatrixF64(linalg::Matrix::new(self.0.len(), 1, data).unwrap())
    }
    
    #[wasm_bindgen]
    pub fn to_row_matrix(&self) -> MatrixF64 {
        let data = self.0.as_slice().to_vec();
        let matrix = linalg::Matrix::new(1, data.len(), data).unwrap();
        MatrixF64(matrix)
    }
    // outer_product は Vector * Matrix で代替可能なため公開しない
}

// ==============================
// MatrixF32: Field制約レベル
// ==============================

macro_rules! declare_matrix_f32_impl { () => {
    #[wasm_macros::wasm_class(
        internal = "InternalMatrixF32",
        js_name = "MatrixF32",
        ops(Add, Sub, Mul),
        indexer = false,
        iterator = false
    )]
    impl MatrixF32 { matrix_ring_methods!(f32); matrix_field_methods!(f32); }
} }
declare_matrix_f32_impl!();
// Provide explicit ring-level methods for MatrixF32
#[wasm_bindgen]
impl MatrixF32 {
    #[wasm_bindgen(constructor)]
    pub fn new(rows: usize, cols: usize, data: Vec<f32>) -> std::result::Result<Self, wasm_bindgen::JsValue> {
        linalg::Matrix::new(rows, cols, data).map(MatrixF32).map_err(|e| JsValue::from_str(&format!("{e}")))
    }
    pub fn with_default(rows: usize, cols: usize) -> Self { MatrixF32(linalg::Matrix::with_default(rows, cols)) }
    pub fn zeros(rows: usize, cols: usize) -> Self { MatrixF32(linalg::Matrix::zeros(rows, cols)) }
    pub fn identity(size: usize) -> Self { MatrixF32(linalg::Matrix::identity(size)) }
    pub fn rows(&self) -> usize { self.0.rows }
    pub fn cols(&self) -> usize { self.0.cols }
    pub fn is_square(&self) -> bool { self.0.is_square() }
    pub fn transpose(&self) -> Self { MatrixF32(self.0.transpose()) }
    pub fn trace(&self) -> std::result::Result<f32, wasm_bindgen::JsValue> { self.0.trace().map_err(|e| JsValue::from_str(&format!("{e}"))) }
    pub fn determinant(&self) -> std::result::Result<f32, wasm_bindgen::JsValue> { self.0.determinant().map_err(|e| JsValue::from_str(&format!("{e}"))) }
    pub fn rank(&self) -> std::result::Result<usize, wasm_bindgen::JsValue> { self.0.rank().map_err(|e| JsValue::from_str(&format!("{e}"))) }
    pub fn inverse(&self) -> Option<Self> { self.0.inverse().map(MatrixF32) }
}

// ==============================
// VectorF32: Field制約レベル
// ==============================

macro_rules! declare_vector_f32_impl { () => {
    #[wasm_macros::wasm_class(
        internal = "InternalVectorF32",
        js_name = "VectorF32", 
        ops(Add, Sub, Mul),
        indexer = false,
        iterator = false
    )]
    impl VectorF32 { vector_ring_methods!(f32); }
} }
declare_vector_f32_impl!();
#[wasm_bindgen]
impl VectorF32 {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<f32>) -> Self { VectorF32(linalg::Vector::new(data)) }
    pub fn zeros(dim: usize) -> Self { VectorF32(linalg::Vector::zeros(dim)) }
    pub fn ones(dim: usize) -> Self { VectorF32(linalg::Vector::ones(dim)) }
    pub fn dim(&self) -> usize { self.0.dim() }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn dot(&self, other: &Self) -> f32 { self.0.dot(&other.0) }
    pub fn argmax(&self) -> Option<usize> { self.0.argmax() }
    pub fn argmin(&self) -> Option<usize> { self.0.argmin() }
    pub fn max(&self) -> Option<f32> { self.0.max() }
    pub fn min(&self) -> Option<f32> { self.0.min() }
}

// ==============================
// MatrixI32: Ring制約レベル（整数型）
// ==============================

macro_rules! declare_matrix_i32_impl { () => {
    #[wasm_macros::wasm_class(
        internal = "InternalMatrixI32",
        js_name = "MatrixI32",
        ops(Add, Sub, Mul),
        indexer = false,
        iterator = false
    )]
    impl MatrixI32 { matrix_ring_methods!(i32); }
} }
declare_matrix_i32_impl!();
#[wasm_bindgen]
impl MatrixI32 {
    #[wasm_bindgen(constructor)]
    pub fn new(rows: usize, cols: usize, data: Vec<i32>) -> std::result::Result<Self, wasm_bindgen::JsValue> {
        linalg::Matrix::new(rows, cols, data).map(MatrixI32).map_err(|e| JsValue::from_str(&format!("{e}")))
    }
    pub fn with_default(rows: usize, cols: usize) -> Self { MatrixI32(linalg::Matrix::with_default(rows, cols)) }
    pub fn zeros(rows: usize, cols: usize) -> Self { MatrixI32(linalg::Matrix::zeros(rows, cols)) }
    pub fn identity(size: usize) -> Self { MatrixI32(linalg::Matrix::identity(size)) }
    pub fn rows(&self) -> usize { self.0.rows }
    pub fn cols(&self) -> usize { self.0.cols }
    pub fn is_square(&self) -> bool { self.0.is_square() }
    pub fn transpose(&self) -> Self { MatrixI32(self.0.transpose()) }
    pub fn trace(&self) -> std::result::Result<i32, wasm_bindgen::JsValue> { self.0.trace().map_err(|e| JsValue::from_str(&format!("{e}"))) }
}

// ==============================
// VectorI32: Ring制約レベル（整数型）
// ==============================

macro_rules! declare_vector_i32_impl { () => {
    #[wasm_macros::wasm_class(
        internal = "InternalVectorI32",
        js_name = "VectorI32", 
        ops(Add, Sub, Mul),
        indexer = false,
        iterator = false
    )]
    impl VectorI32 { vector_ring_methods!(i32); }
} }
declare_vector_i32_impl!();
#[wasm_bindgen]
impl VectorI32 {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<i32>) -> Self { VectorI32(linalg::Vector::new(data)) }
    pub fn zeros(dim: usize) -> Self { VectorI32(linalg::Vector::zeros(dim)) }
    pub fn ones(dim: usize) -> Self { VectorI32(linalg::Vector::ones(dim)) }
    pub fn dim(&self) -> usize { self.0.dim() }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn dot(&self, other: &Self) -> i32 { self.0.dot(&other.0) }
    pub fn argmax(&self) -> Option<usize> { self.0.argmax() }
    pub fn argmin(&self) -> Option<usize> { self.0.argmin() }
    pub fn max(&self) -> Option<i32> { self.0.max() }
    pub fn min(&self) -> Option<i32> { self.0.min() }
}

// ==============================
// 型エイリアスをJavaScript向けに公開
// ==============================

pub use linalg::matrix::numerical::qr::QR as QRResult;
pub use linalg::matrix::numerical::svd::Svd as SvdResult;
pub use linalg::matrix::numerical::eigen::Eigen as EigenResult;
pub use linalg::{LinalgError as LinalgErrorJs, Direction as DirectionJs};