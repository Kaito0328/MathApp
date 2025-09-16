use crate::traits::LinalgField;
use crate::{LinalgError, Matrix, Result, Vector};

pub struct LU<T: LinalgField> {
    pub p: Matrix<T>,
    pub l: Matrix<T>,
    pub u: Matrix<T>,
}

impl<T: LinalgField> Matrix<T> {
    pub fn lu_decompose(&self) -> Result<LU<T>> {
        if !self.is_square() {
            return Err(LinalgError::NotSquareMatrix);
        }
        let n = self.rows;
        let mut l: Matrix<T> = Matrix::zeros(n, n);
        let mut u: Matrix<T> = self.clone();
        let mut p: Matrix<T> = Matrix::identity(n);

        for k in 0..n {
            // pivot
            let mut max_val = T::zero();
            let mut pivot_row = k;
            for i in k..n {
                let v = u[(i, k)].clone().abs();
                if v > max_val {
                    max_val = v;
                    pivot_row = i;
                }
            }
            if max_val <= T::epsilon() {
                return Err(LinalgError::SingularMatrix);
            }

            if pivot_row != k {
                p.swap_rows(k, pivot_row)?;
                u.swap_rows(k, pivot_row)?;
                // これまでに確定している列 (0..k) の L の要素は行入れ替えに追随してスワップする
                for j in 0..k {
                    let tmp = l[(k, j)].clone();
                    l[(k, j)] = l[(pivot_row, j)].clone();
                    l[(pivot_row, j)] = tmp;
                }
            }

            l[(k, k)] = T::one();
            for i in k + 1..n {
                l[(i, k)] = u[(i, k)].clone() / u[(k, k)].clone();
            }
            for i in k + 1..n {
                for j in k..n {
                    let val = u[(i, j)].clone() - l[(i, k)].clone() * u[(k, j)].clone();
                    u[(i, j)] = val;
                }
            }
        }
        Ok(LU { p, l, u })
    }

    /// 事前計算した LU を使って A x = b を解く
    pub fn solve_with_lu(lu: &LU<T>, b: &Vector<T>) -> Result<Vector<T>> {
        let pb = &lu.p * b;
        let y = lu.l.forward_substitution(&pb)?;
        let x = lu.u.backward_substitution(&y)?;
        Ok(x)
    }

    /// 自身を LU 分解してから A x = b を解く
    pub fn solve(&self, b: &Vector<T>) -> Result<Vector<T>> {
        if !self.is_square() {
            return Err(LinalgError::NotSquareMatrix);
        }
        if self.rows != b.len() {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{}-dimensional vector", self.rows),
                found: format!("{}-dimensional vector", b.len()),
            });
        }
        let lu = self.lu_decompose()?;
        Self::solve_with_lu(&lu, b)
    }

    /// 事前計算した LU を使って A X = B を解く
    pub fn solve_matrix_with_lu(lu: &LU<T>, b: &Matrix<T>) -> Result<Matrix<T>> {
        let n = lu.l.rows;
        let m = b.cols;
        let pb = &lu.p * b;
        let mut x = Matrix::zeros(n, m);
        for j in 0..m {
            let pbj = pb.col(j)?;
            let y = lu.l.forward_substitution(&pbj)?;
            let xj = lu.u.backward_substitution(&y)?;
            x.set_col(j, &xj)?;
        }
        Ok(x)
    }

    /// 自身を LU 分解してから A X = B を解く
    pub fn solve_matrix(&self, b: &Matrix<T>) -> Result<Matrix<T>> {
        if !self.is_square() {
            return Err(LinalgError::NotSquareMatrix);
        }
        if self.rows != b.rows {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{}x? (rows match)", self.rows),
                found: format!("{}x{}", b.rows, b.cols),
            });
        }
        let lu = self.lu_decompose()?;
        Self::solve_matrix_with_lu(&lu, b)
    }
}
