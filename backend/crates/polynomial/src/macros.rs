// 共通マクロ: 「&T op &T」の実装を前提に、
// 所有/参照の3バリエーション (T op &T, &T op T, T op T) を自動実装する
// 使い方例: impl_ops_by_ref_variants!(Polynomial<F>, Add, add);
//           impl_ops_by_ref_variants!(RationalFunction<F>, Sub, sub);
// 注意: T は同種の二項演算を想定。出力は常に T とする。
macro_rules! impl_ops_by_ref_variants {
    ($Type:ident<$F:ident>, $Trait:ident, $method:ident) => {
        impl<$F: linalg::Field> ::std::ops::$Trait<&$Type<$F>> for $Type<$F> {
            type Output = $Type<$F>;
            #[inline]
            fn $method(self, rhs: &$Type<$F>) -> Self::Output {
                (&self).$method(rhs)
            }
        }

        impl<$F: linalg::Field> ::std::ops::$Trait<$Type<$F>> for &$Type<$F> {
            type Output = $Type<$F>;
            #[inline]
            fn $method(self, rhs: $Type<$F>) -> Self::Output {
                self.$method(&rhs)
            }
        }

        impl<$F: linalg::Field> ::std::ops::$Trait<$Type<$F>> for $Type<$F> {
            type Output = $Type<$F>;
            #[inline]
            fn $method(self, rhs: $Type<$F>) -> Self::Output {
                (&self).$method(&rhs)
            }
        }
    };
}
