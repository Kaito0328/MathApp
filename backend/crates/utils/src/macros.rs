// 汎用二項演算マクロ群（所有/参照の派生をまとめて実装）
// 前提: &Lhs op &Rhs 実装が存在し、出力が Out の場合。

// 1) 同型二項演算（Lhs == Rhs == Out 型）
#[macro_export]
macro_rules! impl_ops_by_ref_variants {
    ($Type:ident<$T:ident>, $Trait:ident, $method:ident, $Bound:path) => {
        impl<$T: $Bound> ::std::ops::$Trait<&$Type<$T>> for $Type<$T> {
            type Output = $Type<$T>;
            #[inline]
            fn $method(self, rhs: &$Type<$T>) -> Self::Output {
                (&self).$method(rhs)
            }
        }
        impl<$T: $Bound> ::std::ops::$Trait<$Type<$T>> for &$Type<$T> {
            type Output = $Type<$T>;
            #[inline]
            fn $method(self, rhs: $Type<$T>) -> Self::Output {
                self.$method(&rhs)
            }
        }
        impl<$T: $Bound> ::std::ops::$Trait<$Type<$T>> for $Type<$T> {
            type Output = $Type<$T>;
            #[inline]
            fn $method(self, rhs: $Type<$T>) -> Self::Output {
                (&self).$method(&rhs)
            }
        }
    };
}

// 2) 異型二項演算（行列×ベクトルのように Lhs != Rhs で Out が別型）
// 例: impl_mixed_ops_by_ref_variants!(Matrix<T>, Vector<T>, Vector<T>, Mul, mul, crate::Ring);
#[macro_export]
macro_rules! impl_mixed_ops_by_ref_variants {
    ($Lhs:ident<$T:ident>, $Rhs:ident<$T2:ident>, $Out:ident<$T3:ident>, $Trait:ident, $method:ident, $Bound:path) => {
        // &Lhs op &Rhs は既存実装を前提にし、ここでは生成しない
        impl<$T: $Bound> ::std::ops::$Trait<$Rhs<$T>> for &$Lhs<$T> {
            type Output = $Out<$T>;
            #[inline]
            fn $method(self, rhs: $Rhs<$T>) -> Self::Output {
                self.$method(&rhs)
            }
        }
        impl<$T: $Bound> ::std::ops::$Trait<&$Rhs<$T>> for $Lhs<$T> {
            type Output = $Out<$T>;
            #[inline]
            fn $method(self, rhs: &$Rhs<$T>) -> Self::Output {
                (&self).$method(rhs)
            }
        }
        impl<$T: $Bound> ::std::ops::$Trait<$Rhs<$T>> for $Lhs<$T> {
            type Output = $Out<$T>;
            #[inline]
            fn $method(self, rhs: $Rhs<$T>) -> Self::Output {
                (&self).$method(&rhs)
            }
        }
    };
}

// 3) スカラー右辺（&Type op &Scalar がある前提で派生を生成）
// 例: impl_scalar_rhs_by_ref_variants!(Vector<T>, Add, add, crate::Ring);
#[macro_export]
macro_rules! impl_scalar_rhs_by_ref_variants {
    ($Type:ident<$T:ident>, $Trait:ident, $method:ident, $Bound:path) => {
        impl<'b, $T: $Bound> ::std::ops::$Trait<&'b $T> for $Type<$T> {
            type Output = $Type<$T>;
            #[inline]
            fn $method(self, rhs: &'b $T) -> Self::Output {
                (&self).$method(rhs)
            }
        }
        impl<$T: $Bound> ::std::ops::$Trait<$T> for &$Type<$T> {
            type Output = $Type<$T>;
            #[inline]
            fn $method(self, rhs: $T) -> Self::Output {
                self.$method(&rhs)
            }
        }
        impl<$T: $Bound> ::std::ops::$Trait<$T> for $Type<$T> {
            type Output = $Type<$T>;
            #[inline]
            fn $method(self, rhs: $T) -> Self::Output {
                (&self).$method(&rhs)
            }
        }
    };
}
