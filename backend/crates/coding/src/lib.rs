pub mod bch;
pub mod code_utils;
pub mod cyclic;
// finite-field クレートへ移動した型の互換モジュール
pub mod hamming;
pub mod linear;
pub mod rs;
pub mod types;
pub mod error;
pub mod lfsr;
pub mod codec_common;
pub mod prelude { pub use crate::error::{CodingError, Result as CodingResult}; }

pub use bch::BCHCode;
pub use code_utils::*;
pub use cyclic::CyclicCode;
pub use finite_field::gf256::{PolyGF256, GF256};
pub use finite_field::gfext::GFExt;
pub use finite_field::gfp::GFp;
pub use hamming::Hamming74;
pub use linear::LinearCode;
pub use rs::{RSDecodeResult, ReedSolomon};
pub use types::{Codeword, GeneratorMatrix, Message, ParityCheckMatrix, Syndrome};

// polynomial クレートの多項式型を一般に使うための型エイリアス
pub type Poly<F> = polynomial::polynomial::Polynomial<F>;
