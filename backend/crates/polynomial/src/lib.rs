pub mod format;
pub mod polynomial;
pub mod rational_function;
#[macro_use]
mod macros;

// 使い勝手のため、代表的な型をルート再エクスポート
pub use polynomial::Polynomial;
pub use rational_function::RationalFunction;
