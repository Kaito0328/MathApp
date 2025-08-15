use crate::gfext::GFExt;
use crate::prime::GFp;
use std::sync::Arc;

// GF256 を GFExt<GFp<2>> の具象として提供
pub type GF256 = GFExt<GFp<2>>;

// 既約多項式: x^8 + x^4 + x^3 + x + 1 （AES）
pub fn gf256_modulus() -> Arc<Vec<GFp<2>>> {
	Arc::new(vec![GFp::<2>(1),GFp::<2>(1),GFp::<2>(0),GFp::<2>(1),GFp::<2>(1),GFp::<2>(0),GFp::<2>(0),GFp::<2>(0),GFp::<2>(1)])
}

pub fn gf256_from_u8(x: u8) -> GF256 { GFExt::<GFp<2>>::from_u8(gf256_modulus(), x) }

// 互換のため PolyGF256 は generic poly を使う型エイリアスに変更
pub type PolyGF256 = crate::poly::Poly<GF256>;

