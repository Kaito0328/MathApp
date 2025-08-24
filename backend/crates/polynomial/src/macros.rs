// 簡易マクロ: 所有/参照の派生をまとめて実装
// utils::impl_ops_by_ref_variants と同等の機能をローカルに持たせる

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
