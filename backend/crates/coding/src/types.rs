use finite_field::gfp::GFp;
use linalg::{Field, Matrix, Scalar, Vector};
use std::ops::Deref;

// メッセージ（情報語）
#[derive(Clone, Debug)]
pub struct Message<F: Scalar>(pub Vector<F>);

// 符号語
#[derive(Clone, Debug)]
pub struct Codeword<F: Scalar>(pub Vector<F>);

// シンドローム（GF(2)）
#[derive(Clone, Debug)]
pub struct Syndrome(pub Vector<GFp<2>>);

// 生成行列 / 検査行列
#[derive(Clone, Debug)]
pub struct GeneratorMatrix<F: Field>(pub Matrix<F>);
#[derive(Clone, Debug)]
pub struct ParityCheckMatrix<F: Field>(pub Matrix<F>);

// 使いやすさのための From/AsRef/Deref 実装
impl<F: Scalar> From<Vector<F>> for Message<F> {
    fn from(v: Vector<F>) -> Self { Self(v) }
}
impl<F: Scalar> From<Vector<F>> for Codeword<F> {
    fn from(v: Vector<F>) -> Self { Self(v) }
}
impl From<Vector<GFp<2>>> for Syndrome {
    fn from(v: Vector<GFp<2>>) -> Self { Self(v) }
}
impl<F: Field> From<Matrix<F>> for GeneratorMatrix<F> {
    fn from(m: Matrix<F>) -> Self { Self(m) }
}
impl<F: Field> From<Matrix<F>> for ParityCheckMatrix<F> {
    fn from(m: Matrix<F>) -> Self { Self(m) }
}

impl<F: Scalar> AsRef<Vector<F>> for Message<F> { fn as_ref(&self) -> &Vector<F> { &self.0 } }
impl<F: Scalar> AsRef<Vector<F>> for Codeword<F> { fn as_ref(&self) -> &Vector<F> { &self.0 } }
impl AsRef<Vector<GFp<2>>> for Syndrome { fn as_ref(&self) -> &Vector<GFp<2>> { &self.0 } }
impl<F: Field> AsRef<Matrix<F>> for GeneratorMatrix<F> { fn as_ref(&self) -> &Matrix<F> { &self.0 } }
impl<F: Field> AsRef<Matrix<F>> for ParityCheckMatrix<F> { fn as_ref(&self) -> &Matrix<F> { &self.0 } }

impl<F: Scalar> Deref for Message<F> { type Target = Vector<F>; fn deref(&self) -> &Self::Target { &self.0 } }
impl<F: Scalar> Deref for Codeword<F> { type Target = Vector<F>; fn deref(&self) -> &Self::Target { &self.0 } }
impl Deref for Syndrome { type Target = Vector<GFp<2>>; fn deref(&self) -> &Self::Target { &self.0 } }
impl<F: Field> Deref for GeneratorMatrix<F> { type Target = Matrix<F>; fn deref(&self) -> &Self::Target { &self.0 } }
impl<F: Field> Deref for ParityCheckMatrix<F> { type Target = Matrix<F>; fn deref(&self) -> &Self::Target { &self.0 } }
