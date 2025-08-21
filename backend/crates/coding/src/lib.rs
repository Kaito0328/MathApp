pub mod bch;
pub mod code_utils;
pub mod cyclic;
pub mod gf256;
pub mod gfext;
pub mod hamming;
pub mod linear;
pub mod prime;
pub mod rs;

pub use bch::BCHCode;
pub use code_utils::*;
pub use cyclic::CyclicCode;
pub use gf256::{PolyGF256, GF256};
pub use gfext::GFExt;
pub use hamming::Hamming74;
pub use linear::LinearCode;
pub use prime::GFp;
pub use rs::{RSDecodeResult, ReedSolomon};

// polynomial クレートの多項式型を一般に使うための型エイリアス
pub type Poly<F> = polynomial::polynomial::Polynomial<F>;
