#![allow(unused_macros)]  // マクロ内マクロのため警告を抑制

#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use linalg::{Matrix, Ring, Field, LinalgError, Result, Vector, Direction};
#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use linalg::matrix::numerical::qr::{QR, QrDecomposition};
#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use linalg::matrix::numerical::svd::Svd;
#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use linalg::matrix::numerical::eigen::{Eigen, EigenComplex};
#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use linalg::matrix::numerical::{CholeskyDecomposition, EigenDecomposition, MatrixExponential, Pseudoinverse, SvdDeComposition};
#[allow(unused_imports)]  // マクロ展開後に使用されるため警告を抑制
use linalg::traits::LinalgField;

/// 基本的なMatrix操作（Scalar trait + inherent methods）
macro_rules! matrix_basic_methods {
    ($T:ty) => {
        // === 基本コンストラクタ（Scalar trait, inherent methods） ===
        #[constructor]
        pub fn new(rows: usize, cols: usize, data: Vec<$T>) -> Result<Self>;

        #[constructor]
        pub fn with_default(rows: usize, cols: usize) -> Self;

        // === 基本メソッド（inherent methods） ===
        pub fn transpose(&self) -> Self;
        pub fn rows(&self) -> usize;
        pub fn cols(&self) -> usize;
        pub fn is_square(&self) -> bool;
        
        // 変更メソッド
        pub fn swap_rows(&mut self, r1: usize, r2: usize) -> Result<()>;
        
        // サブマトリクス操作
        pub fn submatrix(&self, start_row: usize, end_row: usize, start_col: usize, end_col: usize) -> Result<Self>;
        pub fn set_submatrix(&mut self, start_row: usize, start_col: usize, submat: &Self) -> Result<()>;
        
        // スタック操作
        pub fn hstack(&self, other: &Self) -> Result<Self>;
        pub fn vstack(&self, other: &Self) -> Result<Self>;
    };
}

/// Ring trait のメソッド群
macro_rules! matrix_ring_methods {
    ($T:ty) => {
        // === Ring trait の静的メソッド ===
        #[constructor]
        #[trait_method(Ring)]
        pub fn zeros(rows: usize, cols: usize) -> Self;

        #[constructor] 
        #[trait_method(Ring)]
        pub fn identity(size: usize) -> Self;

        // === Ring trait の計算メソッド ===
        // 注意: add, sub, mul は ops=[Add, Sub, Mul] で自動生成されるため省略

        #[trait_method(Ring)]
        pub fn checked_mul_scalar(&self, scalar: $T) -> Self;

        #[trait_method(Ring)]
        pub fn checked_add_scalar(&self, scalar: $T) -> Self;

        #[trait_method(Ring)]
        pub fn checked_sub_scalar(&self, scalar: $T) -> Self;

        #[trait_method(Ring)]
        pub fn checked_neg(&self) -> Self;

        #[trait_method(Ring)]
        pub fn scale_row(&mut self, r: usize, scalar: $T) -> Result<()>;

        #[trait_method(Ring)]
        pub fn scale_col(&mut self, c: usize, scalar: $T) -> Result<()>;

        #[trait_method(Ring)]
        pub fn add_scaled_row_to_row(&mut self, from_row: usize, to_row: usize, scalar: $T) -> Result<()>;

        #[trait_method(Ring)]
        pub fn trace(&self) -> Result<$T>;
    };
}

/// Field trait のメソッド群
macro_rules! matrix_field_methods {
    ($T:ty) => {
        // === Field trait のメソッド ===
        #[trait_method(Field)]
        pub fn rref(&self) -> Result<Self>;

        #[trait_method(Field)]
        pub fn rank(&self) -> Result<usize>;

        #[trait_method(Field)]
        pub fn determinant(&self) -> Result<$T>;

        #[trait_method(Field)]
        pub fn inverse(&self) -> Option<Self>;

        // === LinalgField対応LU分解（f32, f64で利用可能） ===
        // Note: LU<T>構造体はジェネリックのため、WASMでは戻り値をSerdeラッパーで処理する必要がある
        // LU分解の結果は、f32とf64のそれぞれに対してラッパーを用意する
        // pub fn lu_decompose(&self) -> Result<LU<$T>>;  // これは後でラッパー経由で実装
        
        // === Vector関連メソッド（Vector WASMクラス実装後に利用可能） ===
        pub fn col(&self, c: usize) -> Result<Vector<$T>>;
        pub fn row(&self, r: usize) -> Result<Vector<$T>>;
        pub fn partial_col(&self, col_idx: usize, start_row: usize, end_row: usize) -> Result<Vector<$T>>;
        pub fn partial_row(&self, row_idx: usize, start_col: usize, end_col: usize) -> Result<Vector<$T>>;
        pub fn set_col(&mut self, c: usize, col_vec: &Vector<$T>) -> Result<()>;
        pub fn set_row(&mut self, r: usize, row_vec: &Vector<$T>) -> Result<()>;

        // 注意: solve系メソッドもVector<T>が利用可能になったため追加可能
        // forward_substitution, backward_substitution, solve_generic, solve_matrix_generic
    };
}

/// f64専用の数値計算メソッド群
macro_rules! matrix_numerical_methods {
    () => {
        // === QrDecomposition trait のメソッド（f64専用） ===
        #[trait_method(QrDecomposition)]
        pub fn qr_decomposition(&self) -> Result<QR>;

        // === SvdDeComposition trait のメソッド（f64専用） ===
        #[trait_method(SvdDeComposition)]
        pub fn svd(&self) -> Result<Svd>;

        // === EigenDecomposition trait のメソッド（f64専用） ===
        #[trait_method(EigenDecomposition)]
        pub fn eigen(&self) -> Result<Eigen>;

        #[trait_method(EigenDecomposition)]
        pub fn eigenvalues(&self) -> Result<Vector<f64>>;

        // === CholeskyDecomposition trait のメソッド（f64専用） ===
        #[trait_method(CholeskyDecomposition)]
        pub fn cholesky(&self) -> Result<Matrix<f64>>;

        // === MatrixExponential trait のメソッド（f64専用） ===
        #[trait_method(MatrixExponential)]
        pub fn matrix_exp(&self) -> Result<Matrix<f64>>;

        // === Pseudoinverse trait のメソッド（f64専用） ===
        #[trait_method(Pseudoinverse)]
        pub fn pinv(&self) -> Result<Matrix<f64>>;

        // === 数値計算メソッド（f64専用） ===
        pub fn frobenius_norm(&self) -> f64;
    };
}

// ===============================
// Vector用の階層的マクロ群
// ===============================

/// 基本的なVector操作（Scalar trait + inherent methods）
macro_rules! vector_basic_methods {
    ($T:ty) => {
        // === 基本コンストラクタ ===
        #[constructor]
        pub fn new(data: Vec<$T>) -> Self;

        // === 基本メソッド ===
        pub fn dim(&self) -> usize;
        pub fn len(&self) -> usize;
        pub fn is_empty(&self) -> bool;
        
        // === 変換メソッド ===
        pub fn transpose(&self) -> Matrix<$T>;
        
        // === アクセスメソッド ===
        pub fn as_slice(&self) -> &[$T];
        pub fn into_inner(self) -> Vec<$T>;
        
        // === 統計メソッド (PartialOrd制約必要) ===
        // argmax, argmin, max, min は型によって利用可能性が異なるため省略
    };
}

/// Ring trait のVector向けメソッド群
macro_rules! vector_ring_methods {
    ($T:ty) => {
        // === Ring trait の静的メソッド ===
        #[constructor]
        #[trait_method(Ring)]
        pub fn zeros(dim: usize) -> Self;

        #[constructor]
        #[trait_method(Ring)]
        pub fn ones(dim: usize) -> Self;

        // === Ring trait の計算メソッド ===
        #[trait_method(Ring)]
        pub fn checked_neg(&self) -> Self;

        #[trait_method(Ring)]
        pub fn hadamard_product(&self, rhs: &Self) -> Result<Self>;

        // === スカラー演算 ===
        #[trait_method(Ring)]
        pub fn checked_add_scalar(&self, scalar: $T) -> Self;

        #[trait_method(Ring)]
        pub fn checked_sub_scalar(&self, scalar: $T) -> Self;

        #[trait_method(Ring)]
        pub fn checked_mul_scalar(&self, scalar: $T) -> Self;

        // === ベクトル積 ===
        #[trait_method(Ring)]
        pub fn dot(&self, other: &Self) -> $T;

        #[trait_method(Ring)]
        pub fn conv(&self, other: &Self) -> Self;

        // Note: checked_mul_matrix は Matrix<T> を返すため、Matrixが実装された後に対応検討
        // Note: cross は 3次元ベクトル専用のため省略
    };
}

/// Field trait のVector向けメソッド群
macro_rules! vector_field_methods {
    ($T:ty) => {
        // Field制約があるVectorでは特別な追加メソッドは少ない
        // 必要に応じて将来的に追加
    };
}

/// f64専用のVector数値計算メソッド群
macro_rules! vector_numerical_methods {
    () => {
        // === 数値計算メソッド（f64専用） ===
        #[constructor]
        pub fn linspace(start: f64, end: f64, num: usize) -> Result<Self>;

        pub fn norm(&self) -> f64;
        pub fn normalize(&self) -> Self;
        pub fn cosine_similarity(&self, other: &Self) -> f64;
        pub fn mean(&self) -> Option<f64>;

        // === FFT系畳み込み（f64専用） ===
        pub fn conv_simple(&self, other: &Self) -> Self;
        pub fn conv_fft(&self, other: &Self) -> Result<Self>;
        pub fn conv_auto(&self, other: &Self) -> Result<Self>;

        // === 統計メソッド（f64では必ず利用可能） ===
        pub fn argmax(&self) -> Option<usize>;
        pub fn argmin(&self) -> Option<usize>;
        pub fn max(&self) -> Option<f64>;
        pub fn min(&self) -> Option<f64>;
    };
}

/// Vector型用のWASMクラスを生成する統合マクロ
macro_rules! define_vector_wasm {
    // 基本版: Ring制約まで（整数型など）
    ($T:ty, $js_name:ident) => {
        #[wasm_macros::wasm_class(
            internal = stringify!(linalg::Vector<$T>),
            js_name = stringify!($js_name),
            ops = [Add, Sub, Mul],
            indexer = false,
            iterator = false
        )]
        impl $js_name {
            vector_basic_methods!($T);
            vector_ring_methods!($T);
        }
    };

    // Field制約がある型向けの拡張版（f32など）
    ($T:ty, $js_name:ident, field) => {
        #[wasm_macros::wasm_class(
            internal = stringify!(linalg::Vector<$T>),
            js_name = stringify!($js_name),
            ops = [Add, Sub, Mul],
            indexer = false,
            iterator = false
        )]
        impl $js_name {
            vector_basic_methods!($T);
            vector_ring_methods!($T);
            vector_field_methods!($T);
        }
    };

    // f64専用の数値計算メソッド付き
    (f64, $js_name:ident, numerical) => {
        #[wasm_macros::wasm_class(
            internal = "linalg::Vector<f64>",
            js_name = stringify!($js_name),
            ops = [Add, Sub, Mul],
            indexer = false,
            iterator = false
        )]
        impl $js_name {
            vector_basic_methods!(f64);
            vector_ring_methods!(f64);
            vector_field_methods!(f64);
            vector_numerical_methods!();
        }
    };
}

/// Matrix型用のWASMクラスを生成する統合マクロ
macro_rules! define_matrix_wasm {
    // 基本版: Ring制約まで（整数型など）
    ($T:ty, $js_name:ident) => {
        #[wasm_macros::wasm_class(
            internal = stringify!(linalg::Matrix<$T>),
            js_name = stringify!($js_name),
            ops = [Add, Sub, Mul],
            indexer = false,
            iterator = false
        )]
        impl $js_name {
            matrix_basic_methods!($T);
            matrix_ring_methods!($T);
        }
    };

    // Field制約がある型向けの拡張版（f32など）
    ($T:ty, $js_name:ident, field) => {
        #[wasm_macros::wasm_class(
            internal = stringify!(linalg::Matrix<$T>),
            js_name = stringify!($js_name),
            ops = [Add, Sub, Mul],
            indexer = false,
            iterator = false
        )]
        impl $js_name {
            matrix_basic_methods!($T);
            matrix_ring_methods!($T);
            matrix_field_methods!($T);
        }
    };

    // f64専用の数値計算メソッド付き
    (f64, $js_name:ident, numerical) => {
        #[wasm_macros::wasm_class(
            internal = "linalg::Matrix<f64>",
            js_name = stringify!($js_name),
            ops = [Add, Sub, Mul],
            indexer = false,
            iterator = false
        )]
        impl $js_name {
            matrix_basic_methods!(f64);
            matrix_ring_methods!(f64);
            matrix_field_methods!(f64);
            matrix_numerical_methods!();
            
            // === f64専用 Vector関連の線形代数解法 ===
            pub fn solve(&self, b: &Vector<f64>) -> Result<Vector<f64>>;
            // Note: solve_with_luは静的関数のため、WASM bindingでは実装が複雑
        }
    };
}

// 各型に対してMatrix・Vectorマクロを適用
define_matrix_wasm!(f32, MatrixF32, field);
define_matrix_wasm!(f64, MatrixF64, numerical);  
define_matrix_wasm!(i32, MatrixI32);
define_matrix_wasm!(i64, MatrixI64);

define_vector_wasm!(f32, VectorF32, field);
define_vector_wasm!(f64, VectorF64, numerical);
define_vector_wasm!(i32, VectorI32);
define_vector_wasm!(i64, VectorI64);

// ==============================
// データ構造体の直接使用（Serde有効）
// ==============================
// 既存のlinalg構造体が直接Serde対応済みなので、ラッパー不要

// 利用可能な構造体（serdeフィーチャー有効時）:
// - QR: QR分解結果
// - Svd: SVD分解結果  
// - Eigen: 固有値分解結果
// - LinalgError: エラー情報
// - Direction: 方向enum

// 型エイリアスをJavaScript向けに用意（オプション）
pub use linalg::matrix::numerical::qr::QR as QRResult;
pub use linalg::matrix::numerical::svd::Svd as SvdResult;
pub use linalg::matrix::numerical::eigen::Eigen as EigenResult;
pub use linalg::{LinalgError as LinalgErrorJs, Direction as DirectionJs};