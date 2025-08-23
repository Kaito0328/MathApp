pub mod convolution;
// 公開インターフェースの明確化: simple と auto
pub use convolution::{
    convolve2d_f32 as convolve2d_auto_f32, convolve2d_simple_f32, convolve2d_simple_u8,
    convolve2d_u8 as convolve2d_auto_u8,
};
pub mod core;
pub use self::convolution as conv;
pub mod dft;
pub mod filter;
