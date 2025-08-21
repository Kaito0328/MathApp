// ローカルマクロ定義（utils クレートのマクロと同等）。
// &Lhs op &Rhs 実装を前提に、所有/参照の派生を自動生成。

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

#[macro_export]
macro_rules! impl_mixed_ops_by_ref_variants {
    ($Lhs:ident<$T:ident>, $Rhs:ident<$T2:ident>, $Out:ident<$T3:ident>, $Trait:ident, $method:ident, $Bound:path) => {
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
