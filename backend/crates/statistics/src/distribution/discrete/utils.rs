use crate::distribution::discrete::core::Distribution;
use num_traits::PrimInt; // 1. PrimIntトレイトをインポート
use std::fmt::Debug; // デバッグ用にDebugトレイトを追加すると便利

// 2. トレイト境界をPrimIntに変更
pub fn find_quantile_bs<F, D>(p: f64, distribution: &D, lower: F, higher: F) -> F
where
    F: PrimInt + Debug, // Fは基本的な整数型
    D: Distribution<Item = F>,
{
    let mut result = F::zero(); // PrimIntは.zero()を提供
    let mut low = lower; // FはCopyなので.clone()は不要
    let mut high = higher;

    // 3. F::from(2)の代わりに定数`two`を使う
    let two = F::from(2u8).unwrap(); // 2をF型に変換

    while low <= high {
        // 4. midの計算がジェネリックな型で安全に行える
        let mid = low + (high - low) / two;
        if distribution.cdf(mid) >= p {
            result = mid;
            if mid == F::zero() {
                break;
            }
            high = mid - F::one(); // PrimIntは.one()も提供
        } else {
            low = mid + F::one();
        }
    }
    result
}
