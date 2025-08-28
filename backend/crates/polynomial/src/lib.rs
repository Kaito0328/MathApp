pub mod format;
pub mod polynomial;
pub mod rational_function;
#[macro_use]
mod macros;
pub mod error;
pub mod prelude {
    pub use crate::error::{PolynomialError, Result as PolyResult};
}

// 使い勝手のため、代表的な型をルート再エクスポート
pub use format::{PolyDisplay, PolyStyle, RfDisplay};
pub use polynomial::Polynomial;
pub use rational_function::RationalFunction;
