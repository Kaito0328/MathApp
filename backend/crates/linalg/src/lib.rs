// backend/src/lib.rs

// --- モジュール宣言 ---
// これらの宣言は、各ファイル/ディレクトリをモジュールとして認識させるために必要です。
#[macro_use]
mod macros;

pub mod error;
pub mod matrix;
pub mod traits;
pub mod vector;

// --- 公開APIの再エクスポート ---
// `pub use` を使うことで、ライブラリの利用者が短いパスで型やトレイトにアクセスできるようになります。

// エラーハンドリング関連
pub use error::{LinalgError, Result};

// 主要なデータ構造
pub use matrix::{Direction, Matrix};
pub use vector::Vector;

// 基本的な振る舞いを定義するトレイト
pub use traits::{Field, Ring, Scalar};
