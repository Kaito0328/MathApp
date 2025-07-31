// backend/src/lib.rs

// 各ファイルをモジュールとして宣言する
pub mod error;
pub mod matrix;
mod ops;
pub mod vector; // `pub`を付けなければ、ライブラリ内部でのみ使用するプライベートモジュールになる

// 外部に公開したい型や関数を `pub use` で再エクスポートする
pub use matrix::Matrix;
pub use vector::Vector;
