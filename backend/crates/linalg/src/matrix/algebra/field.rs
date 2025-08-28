use crate::{Field, LinalgError, Matrix, Result, Vector};

// 簡易LU分解（ピボットなし、Doolittle法）: 任意の Field で動作
pub struct BasicLU<T: Field> {
    pub l: Matrix<T>,
    pub u: Matrix<T>,
}
// --- Level 3: Fieldに対する実装 (除算を必要とする操作) ---
impl<T: Field> Matrix<T> {
    /// ピボットなしの簡易LU分解（Doolittle）。
    /// 失敗時（ゼロピボット）の場合は SingularMatrix を返す。
    pub fn lu_decompose_basic(&self) -> Result<BasicLU<T>> {
        if !self.is_square() {
            return Err(LinalgError::NotSquareMatrix);
        }
        let n = self.rows;
        let mut l: Matrix<T> = Matrix::identity(n);
        let mut u: Matrix<T> = Matrix::zeros(n, n);

        for i in 0..n {
            // U の i 行を計算（列 j は i..n-1）
            for j in i..n {
                let mut sum = T::zero();
                for k in 0..i {
                    sum = sum + l[(i, k)].clone() * u[(k, j)].clone();
                }
                u[(i, j)] = self[(i, j)].clone() - sum;
            }

            // ピボット（u[i,i]）がゼロなら失敗
            if u[(i, i)].clone().is_zero() {
                return Err(LinalgError::SingularMatrix);
            }

            // L の i 列を計算（行 k は i+1..n-1）
            for k in (i + 1)..n {
                let mut sum = T::zero();
                for s in 0..i {
                    sum = sum + l[(k, s)].clone() * u[(s, i)].clone();
                }
                l[(k, i)] = (self[(k, i)].clone() - sum) / u[(i, i)].clone();
            }
        }

        Ok(BasicLU { l, u })
    }

    fn _gauss_elimination(&self) -> Result<(Self, T)> {
        let mut pivot_row = 0;
        let mut det_factor = T::one();
        let mut rref_matrix = self.clone();

        for c in 0..self.cols {
            if pivot_row >= self.rows {
                break;
            }

            // ピボット要素を見つける
            let mut not_zero_index = None;
            for r in pivot_row..self.rows {
                if !rref_matrix[(r, c)].clone().is_zero() {
                    not_zero_index = Some(r);
                    break;
                }
            }

            if not_zero_index.is_none() {
                det_factor = T::zero(); // ピボットが見つからない場合、行列式はゼロ
                continue; // この列にはピボットがない
            }

            if let Some(nzi) = not_zero_index {
                if pivot_row != nzi {
                    rref_matrix.swap_rows(pivot_row, nzi)?;
                    det_factor = -det_factor;
                }
            }

            let pivot_value = rref_matrix[(pivot_row, c)].clone();
            rref_matrix.scale_row(pivot_row, T::one() / pivot_value.clone())?;
            det_factor = det_factor * pivot_value.clone();

            for r in 0..self.rows {
                if r == pivot_row {
                    continue; // ピボット行はスキップ
                }

                let scale = rref_matrix[(r, c)].clone();
                rref_matrix.add_scaled_row_to_row(pivot_row, r, -scale)?;
            }

            pivot_row += 1; // 次のピボット行へ
        }

        Ok((rref_matrix, det_factor))
    }

    pub fn rref(&self) -> Result<Matrix<T>> {
        let (mat, _) = self._gauss_elimination()?;
        Ok(mat)
    }

    pub fn rank(&self) -> Result<usize> {
        let rref = self.rref()?;
        let rank = Self::rank_from_rref(&rref);
        Ok(rank)
    }

    pub fn rank_from_rref(rref_matrix: &Matrix<T>) -> usize {
        let mut rank = 0;
        for r in 0..rref_matrix.rows {
            // is_zero()を実装している前提
            // rowは範囲内で必ず成功するはずだが、安全のため失敗時はスキップ
            if let Ok(row_vec) = rref_matrix.row(r) {
                if row_vec.iter().any(|x| !x.is_zero()) {
                    rank += 1;
                }
            }
        }
        rank
    }

    pub fn determinant(&self) -> Result<T> {
        if !self.is_square() {
            return Err(LinalgError::DimensionMismatch {
                expected: "square matrix".to_string(),
                found: format!("{}x{}", self.rows, self.cols),
            });
        }
        let (_, det) = self._gauss_elimination()?;
        Ok(det)
    }

    pub fn inverse(&self) -> Option<Matrix<T>> {
        if !self.is_square() {
            return None; // 正方行列でない場合は逆行列は存在しない
        }

        let i_mat = Self::identity(self.rows);
        let augmented = match self.hstack(&i_mat) {
            Ok(m) => m,
            Err(_) => return None,
        };

        let (rref, _) = augmented._gauss_elimination().ok()?;
        let left_half = rref.submatrix(0, self.rows, 0, self.cols);

        if Self::rank_from_rref(&left_half) != self.rows {
            return None; // 逆行列が存在しない
        }

        Some(rref.submatrix(0, self.rows, self.cols, self.cols * 2))
    }

    pub fn forward_substitution(&self, b: &Vector<T>) -> Result<Vector<T>> {
        if self.rows != self.cols {
            return Err(LinalgError::DimensionMismatch {
                expected: "square matrix".to_string(),
                found: format!("{}x{}", self.rows, self.cols),
            });
        }
        if self.rows != b.len() {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{}-dimensional vector", self.rows),
                found: format!("{}-dimensional vector", b.len()),
            });
        }

        let mut x: Vector<T> = Vector::zeros(self.rows);
        for i in 0..self.rows {
            let mut sum = b[i].clone();
            for j in 0..i {
                sum = sum - self[(i, j)].clone() * x[j].clone();
            }
            let diag = self[(i, i)].clone();
            if diag.is_zero() {
                // num_traits::Zero を利用
                return Err(LinalgError::SingularMatrix); // 専用のエラーを返す
            }
            x[i] = sum / diag;
        }
        Ok(x)
    }

    pub fn backward_substitution(&self, b: &Vector<T>) -> Result<Vector<T>> {
        if self.rows != self.cols {
            return Err(LinalgError::DimensionMismatch {
                expected: "square matrix".to_string(),
                found: format!("{}x{}", self.rows, self.cols),
            });
        }
        if self.rows != b.len() {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{}-dimensional vector", self.rows),
                found: format!("{}-dimensional vector", b.len()),
            });
        }

        let mut x: Vector<T> = Vector::zeros(self.rows);
        for i in (0..self.rows).rev() {
            let mut sum = b[i].clone();
            for j in (i + 1)..self.rows {
                sum = sum - self[(i, j)].clone() * x[j].clone();
            }
            let diag = self[(i, i)].clone();
            if diag.is_zero() {
                // num_traits::Zero を利用
                return Err(LinalgError::SingularMatrix); // 専用のエラーを返す
            }
            x[i] = sum / diag;
        }
        Ok(x)
    }

    /// BasicLU（無ピボットLU）を使って A x = b を解く
    pub fn solve_with_basic_lu(&self, lu: &BasicLU<T>, b: &Vector<T>) -> Result<Vector<T>> {
        if !self.is_square() {
            return Err(LinalgError::NotSquareMatrix);
        }
        if self.rows != b.len() {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{}-dimensional vector", self.rows),
                found: format!("{}-dimensional vector", b.len()),
            });
        }
        // ピボットが無いので P*b は不要
        let y = lu.l.forward_substitution(b)?;
        let x = lu.u.backward_substitution(&y)?;
        Ok(x)
    }

    /// BasicLU（無ピボットLU）を使って A X = B を解く
    pub fn solve_matrix_with_basic_lu(&self, lu: &BasicLU<T>, b: &Matrix<T>) -> Result<Matrix<T>> {
        if !self.is_square() {
            return Err(LinalgError::NotSquareMatrix);
        }
        if self.rows != b.rows {
            return Err(LinalgError::DimensionMismatch {
                expected: format!("{}x? (rows match)", self.rows),
                found: format!("{}x{}", b.rows, b.cols),
            });
        }
        let n = self.rows;
        let m = b.cols;
        let mut x = Matrix::zeros(n, m);
        for j in 0..m {
            let bj = b.col(j)?;
            let y = lu.l.forward_substitution(&bj)?;
            let xj = lu.u.backward_substitution(&y)?;
            x.set_col(j, &xj)?;
        }
        Ok(x)
    }

    /// A x = b を Field 上で解く（ピボットなしLU使用）
    pub fn solve_generic(&self, b: &Vector<T>) -> Result<Vector<T>> {
        let lu = self.lu_decompose_basic()?;
        self.solve_with_basic_lu(&lu, b)
    }

    /// A X = B を Field 上で解く（Bは複数右辺）。X を返す。
    pub fn solve_matrix_generic(&self, b: &Matrix<T>) -> Result<Matrix<T>> {
        let lu = self.lu_decompose_basic()?;
        self.solve_matrix_with_basic_lu(&lu, b)
    }
}
