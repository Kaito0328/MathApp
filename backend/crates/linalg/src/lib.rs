// backend/src/lib.rs

// 各ファイルをモジュールとして宣言する
pub mod error;
pub mod matrix;
pub mod ops;
pub mod vector; // `pub`を付けなければ、ライブラリ内部でのみ使用するプライベートモジュールになる

// 外部に公開したい型や関数を `pub use` で再エクスポートする
pub use error::LinalgError;
pub use matrix::{EigenDecomposition, Matrix};
pub use vector::Vector;
