#[macro_use]
extern crate utils;
pub mod combinatorics;
pub mod format;
pub mod polynomial;
pub mod rational_function;
pub mod sequence;
pub mod sum;

// 使い勝手のため、代表的な型をルート再エクスポート
pub use polynomial::Polynomial;
pub use rational_function::RationalFunction;
