pub mod eigen;
pub mod exp;
pub mod lu;
pub mod qr;
pub mod svd;

mod helpers;

// --- トレイトを短いパスで使えるように再エクスポートする ---
// これを書いておくと `use crate::matrix::numerical::Svd;` のように書ける
pub use eigen::EigenDecomposition;
pub use lu::LuDecomposition;
pub use qr::QrDecomposition;
pub use svd::SvdDeComposition;
