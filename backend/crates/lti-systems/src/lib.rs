pub mod continuous;
pub mod conversions;
pub mod discrete;
pub mod plot;
mod responses;
pub mod statespace;
pub mod transfer; // deprecated placeholder
pub mod zpk;
pub mod error;
pub mod prelude { pub use crate::error::{LtiError, Result as LtiResult}; }

// 再エクスポートは poly から
pub use poly::polynomial::Polynomial;
pub use poly::rational_function::RationalFunction;
// 新API: モジュール分割により短い型名に
pub use continuous::TransferFunction as ContinuousTransferFunction;
pub use discrete::TransferFunction as DiscreteTransferFunction;

// 省略名での利用も可能にする場合は以下を公開:
pub use continuous as continuous_tf;
pub use discrete as discrete_tf;
pub use statespace::{ContinuousStateSpace, DiscreteStateSpace};
