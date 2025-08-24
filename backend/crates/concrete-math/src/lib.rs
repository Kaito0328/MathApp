pub mod combinatorics;
pub mod sequence;
pub mod sum;
pub mod error;
pub mod prelude { pub use crate::error::{ConcreteMathError, Result as ConcreteMathResult}; }
