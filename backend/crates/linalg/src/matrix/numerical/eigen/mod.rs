use num_complex::Complex;

use crate::{
    matrix::{numerical::QrDecomposition, Matrix},
    Vector,
};

pub struct Eigen {
    pub eigen_values: Vec<f64>,
    pub eigen_vectors: Matrix<f64>,
}

pub struct EigenComplex {
    pub eigen_values: Vec<Complex<f64>>,
    pub eigen_vectors: Matrix<Complex<f64>>,
}

pub trait EigenDecomposition {
    /// LU分解を行う。成功した場合はLU構造体を返す。
    /// 行列が正方行列でない場合はNoneを返す。
    fn eigen_decomposition(&self) -> Option<Eigen>;
    fn eigen_decomposition_complex(&self) -> Option<EigenComplex>;
}

impl EigenDecomposition for Matrix<f64> {
    fn eigen_decomposition(&self) -> Option<Eigen> {
        // --- エッジケースの事前処理 (変更なし) ---
        if self.rows == 0 {
            return Some(Eigen {
                eigen_values: vec![],
                eigen_vectors: Matrix::new(0, 0, vec![]).unwrap(),
            });
        }
        if !self.is_square() {
            return None;
        }
        if self.rows == 1 {
            return Some(Eigen {
                eigen_values: vec![self[(0, 0)]],
                eigen_vectors: Matrix::identity(1),
            });
        }

        let n = self.rows;
        // Hessenberg 化して h を得る（固有値収束のために QR 反復はそのまま使う）
        let (mut h, _v_dummy) = self.to_hessenberg()?; // v はここでは使わない（零空間で再構築する）
        let mut end = n;

        let max_total_iterations = 30 * n;
        let mut total_iterations = 0;

        while end > 1 {
            let m = end - 1;
            let mut iterations_since_deflation = 0;

            loop {
                if total_iterations >= max_total_iterations {
                    println!("Maximum total iterations reached, returning None.");
                    return None;
                }
                total_iterations += 1;
                iterations_since_deflation += 1;

                let tol = f64::EPSILON * (h[(m, m)].abs() + h[(m - 1, m - 1)].abs());
                if h[(m, m - 1)].abs() <= tol {
                    h[(m, m - 1)] = 0.0;
                    end -= 1;
                    break;
                }

                let shift: f64;
                if iterations_since_deflation > 10 {
                    shift = h[(m, m)] + 0.75 * h[(m, m - 1)].abs();
                    iterations_since_deflation = 0;
                } else {
                    let s = h[(m, m)];
                    let t = h[(m - 1, m - 1)];
                    let u = h[(m - 1, m)];
                    let p = h[(m, m - 1)];
                    let trace = t + s;
                    let det = t * s - u * p;
                    let discriminant = (trace * trace / 4.0) - det;

                    let mu1_denom = trace / 2.0 + discriminant.abs().sqrt().copysign(trace / 2.0);
                    let mu1 = if mu1_denom.abs() > 1e-14 {
                        mu1_denom
                    } else {
                        0.0
                    };
                    let mu2 = if mu1.abs() > 1e-14 { det / mu1 } else { 0.0 };
                    shift = if (mu1 - s).abs() < (mu2 - s).abs() {
                        mu1
                    } else {
                        mu2
                    };
                }

                let mut shifted_h = h.clone();
                for j in 0..end {
                    shifted_h[(j, j)] -= shift;
                }

                if let Some(qr) = shifted_h.submatrix(0, end, 0, end).qr_decomposition() {
                    let q = qr.q;
                    let mut q_full = Matrix::identity(n);
                    q_full.set_submatrix(0, 0, &q).ok()?;

                    h = &(&q_full.transpose() * &h) * &q_full;
                    // v は累積しない（零空間で固有ベクトルを再計算するため）
                } else {
                    return None;
                }
            }
        }

        // 固有値を対角から取得
        let eigenvalues_unsorted: Vec<f64> = (0..n).map(|i| h[(i, i)]).collect();

        // 各固有値について零空間を解いて固有ベクトルを得る
        let tol_ns = 1e-12;
        let mut eigenvectors_cols: Vec<crate::Vector<f64>> = Vec::with_capacity(n);
        for &lambda in &eigenvalues_unsorted {
            // B = A - lambda * I
            let mut b = self.clone();
            for i in 0..n {
                b[(i, i)] -= lambda;
            }
            // nullspace_vector を呼ぶ
            if let Some(vec) = b.nullspace_vector(tol_ns) {
                eigenvectors_cols.push(vec);
            } else {
                // 零空間が見つからなければ（数値的問題）、
                // 代替として単位ベクトルを返すか None にする。ここでは None を返す。
                return None;
            }
        }

        // インデックスソート（固有値順）
        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by(|&i, &j| {
            eigenvalues_unsorted[i]
                .partial_cmp(&eigenvalues_unsorted[j])
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let final_eigenvalues: Vec<f64> =
            indices.iter().map(|&i| eigenvalues_unsorted[i]).collect();
        let mut final_eigenvectors = Matrix::<f64>::zeros(n, n);
        for (col_idx, &i) in indices.iter().enumerate() {
            final_eigenvectors
                .set_col(col_idx, &eigenvectors_cols[i])
                .ok()?;
        }

        Some(Eigen {
            eigen_values: final_eigenvalues,
            eigen_vectors: final_eigenvectors,
        })
    }

    fn eigen_decomposition_complex(&self) -> Option<EigenComplex> {
        // --- エッジケースの事前処理 (変更なし) ---
        if self.rows == 0 {
            return Some(EigenComplex {
                eigen_values: vec![],
                eigen_vectors: Matrix::new(0, 0, vec![]).unwrap(),
            });
        }
        if !self.is_square() {
            return None;
        }
        if self.rows == 1 {
            return Some(EigenComplex {
                eigen_values: vec![Complex::new(self[(0, 0)], 0.0)],
                eigen_vectors: Matrix::new(1, 1, vec![Complex::new(1.0, 0.0)]).unwrap(),
            });
        }

        let n = self.rows;

        // ステップ1: ヘッセンベルグ形式への変換
        let (mut h, mut q) = self.to_hessenberg()?;

        // ステップ2: QR反復によるシュア形式への変換
        if !Self::qr_iteration_to_schur(&mut h, &mut q, 1e-12) {
            println!("QR iteration did not converge.");
            return None;
        }
        let t = h; // hは今やシュア形式 T

        // ステップ3: シュア形式 T から固有値を抽出
        let eigenvalues_unsorted = Self::extract_eigenvalues_from_schur(&t, 1e-12);

        // ステップ4: シュア形式 T の固有ベクトルを計算
        let schur_eigenvectors = Self::compute_schur_eigenvectors(&t, &eigenvalues_unsorted)?;

        // ステップ5: 固有ベクトルを元の基底に逆変換
        let eigenvectors_unsorted = &q.to_complex() * &schur_eigenvectors;

        // ★★★ API改善: 固有値と固有ベクトルをソートして対応付ける ★★★
        // 1. (固有値, 固有ベクトル) のペアを作成
        let mut pairs: Vec<_> = eigenvalues_unsorted
            .into_iter()
            .zip(0..n)
            .map(|(val, i)| (val, eigenvectors_unsorted.col(i).unwrap()))
            .collect();

        // 2. 固有値に基づいてペアをソート (実部 -> 虚部の昇順)
        pairs.sort_by(|(val_a, _), (val_b, _)| {
            val_a
                .re
                .partial_cmp(&val_b.re)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(
                    val_a
                        .im
                        .partial_cmp(&val_b.im)
                        .unwrap_or(std::cmp::Ordering::Equal),
                )
        });

        // 3. ソートされた結果から最終的なリストと行列を再構築
        let final_eigenvalues: Vec<Complex<f64>> = pairs.iter().map(|(val, _)| *val).collect();
        let mut final_eigenvectors = Matrix::<Complex<f64>>::zeros(n, n);
        for (i, (_, vec)) in pairs.iter().enumerate() {
            final_eigenvectors.set_col(i, vec).ok()?;
        }

        Some(EigenComplex {
            eigen_values: final_eigenvalues,
            eigen_vectors: final_eigenvectors,
        })
    }
}
impl Matrix<f64> {
    /// 2x2行列の固有値を解析的に解く。
    fn solve_2x2_eigenvalues(a: f64, b: f64, c: f64, d: f64) -> (Complex<f64>, Complex<f64>) {
        let trace = a + d;
        let det = a * d - b * c;
        let discriminant = trace * trace - 4.0 * det;

        if discriminant >= 0.0 {
            let sqrt_disc = discriminant.sqrt();
            (
                Complex::new((trace + sqrt_disc) / 2.0, 0.0),
                Complex::new((trace - sqrt_disc) / 2.0, 0.0),
            )
        } else {
            let real_part = trace / 2.0;
            let imag_part = (-discriminant).sqrt() / 2.0;
            (
                Complex::new(real_part, imag_part),
                Complex::new(real_part, -imag_part),
            )
        }
    }

    /// フランシスのダブルシフトQR法を用いて、ヘッセンベルグ行列 `h` を実シュア形式に変換する。
    fn qr_iteration_to_schur(h: &mut Matrix<f64>, q: &mut Matrix<f64>, tol: f64) -> bool {
        let n = h.rows;
        let mut end = n;

        while end > 0 {
            let mut iter = 0;
            loop {
                // ★★★ 修正点 1: LAPACKに倣った、より適応的な反復回数の上限 ★★★
                // サブ問題のサイズ 'end' に応じた上限を設定し、無限ループを防ぐ
                if iter >= 30 * end {
                    return false; // このサブ問題は収束しなかった
                }
                iter += 1;

                let m = end - 1;

                // デフレーションのチェック (変更なし)
                if m == 0 || h[(m, m - 1)].abs() < tol * (h[(m, m)].abs() + h[(m - 1, m - 1)].abs())
                {
                    end -= 1;
                    break;
                }

                if m > 0
                    && (m == 1
                        || h[(m - 1, m - 2)].abs()
                            < tol * (h[(m - 1, m - 1)].abs() + h[(m - 2, m - 2)].abs()))
                {
                    end -= 2;
                    break;
                }

                let trace: f64;
                let det: f64;

                // ★★★ 修正点 2: より頻繁で堅牢な例外シフト戦略 ★★★
                // 10回反復しても収束しない場合、停滞しているとみなし、サイクルを破壊するための
                // 例外的なシフトを適用する。
                if iter % 10 == 0 {
                    // このシフトは、意図的に通常のフランシスシフトとは異なる値を生成する
                    let exceptional_shift = h[(m, m)].abs() + h[(m - 1, m - 1)].abs();
                    trace = exceptional_shift * 1.5;
                    det = exceptional_shift.powi(2);
                } else {
                    // 通常のフランシス・ダブルシフト
                    trace = h[(m - 1, m - 1)] + h[(m, m)];
                    det = h[(m - 1, m - 1)] * h[(m, m)] - h[(m - 1, m)] * h[(m, m - 1)];
                }

                // 最初の列に暗黙的なシフトを適用してギブンス回転を開始する (変更なし)
                let x = h[(0, 0)] * h[(0, 0)] + h[(0, 1)] * h[(1, 0)] - trace * h[(0, 0)] + det;
                let y = h[(1, 0)] * (h[(0, 0)] + h[(1, 1)] - trace);

                // ギブンス回転の適用 (変更なし)
                for k in 0..end - 1 {
                    let (c, s) = Matrix::<f64>::givens_rotation(
                        if k == 0 { x } else { h[(k, k - 1)] },
                        if k == 0 { y } else { h[(k + 1, k - 1)] },
                    );

                    for j in k..end {
                        let h_kj = h[(k, j)];
                        let h_k1j = h[(k + 1, j)];
                        h[(k, j)] = c * h_kj + s * h_k1j;
                        h[(k + 1, j)] = -s * h_kj + c * h_k1j;
                    }

                    for i in 0..end {
                        let h_ik = h[(i, k)];
                        let h_ik1 = h[(i, k + 1)];
                        h[(i, k)] = c * h_ik + s * h_ik1;
                        h[(i, k + 1)] = -s * h_ik + c * h_ik1;
                    }

                    for i in 0..n {
                        let q_ik = q[(i, k)];
                        let q_ik1 = q[(i, k + 1)];
                        q[(i, k)] = c * q_ik + s * q_ik1;
                        q[(i, k + 1)] = -s * q_ik + c * q_ik1;
                    }
                }
            }
        }
        true
    }
    /// 実シュア形式 `t` の対角を走査し、1x1および2x2ブロックから固有値を抽出する。
    fn extract_eigenvalues_from_schur(t: &Matrix<f64>, tol: f64) -> Vec<Complex<f64>> {
        let n = t.rows;
        let mut eigenvalues = Vec::with_capacity(n);
        let mut i = 0;
        while i < n {
            if i == n - 1 || t[(i + 1, i)].abs() < tol * (t[(i, i)].abs() + t[(i + 1, i + 1)].abs())
            {
                eigenvalues.push(Complex::new(t[(i, i)], 0.0));
                i += 1;
            } else {
                let a = t[(i, i)];
                let b = t[(i, i + 1)];
                let c = t[(i + 1, i)];
                let d = t[(i + 1, i + 1)];
                let (lambda1, lambda2) = Self::solve_2x2_eigenvalues(a, b, c, d);
                eigenvalues.push(lambda1);
                eigenvalues.push(lambda2);
                i += 2;
            }
        }
        eigenvalues
    }

    fn compute_schur_eigenvectors(
        t: &Matrix<f64>,
        eigenvalues: &[Complex<f64>],
    ) -> Option<Matrix<Complex<f64>>> {
        let n = t.rows;
        let mut eigenvectors = Matrix::<Complex<f64>>::zeros(n, n);

        let mut i = n;
        while i > 0 {
            let lambda: Complex<f64>;
            let block_size: usize;

            if i == 1 || t[(i - 1, i - 2)].abs() < 1e-12 {
                block_size = 1;
                lambda = eigenvalues[i - 1];
            } else {
                block_size = 2;
                lambda = if eigenvalues[i - 1].im >= 0.0 {
                    eigenvalues[i - 1]
                } else {
                    eigenvalues[i - 2]
                };
            }

            let k = i - block_size;
            let mut y = Vector::<Complex<f64>>::zeros(n);

            if block_size == 1 {
                y[k] = Complex::new(1.0, 0.0);
                if k > 0 {
                    for j in (0..k).rev() {
                        let mut s = Complex::new(0.0, 0.0);
                        for l in (j + 1)..i {
                            s += t[(j, l)] * y[l];
                        }
                        let diag = Complex::new(t[(j, j)], 0.0) - lambda;
                        if diag.norm() < 1e-12 {
                            y[j] = Complex::new(1.0, 0.0);
                        } else {
                            y[j] = -s / diag;
                        }
                    }
                }
            } else {
                // block_size == 2
                let m = i - 1;
                let b11 = Complex::new(t[(k, k)], 0.0) - lambda;
                let b12 = Complex::new(t[(k, m)], 0.0);
                let b21 = Complex::new(t[(m, k)], 0.0);
                let b22 = Complex::new(t[(m, m)], 0.0) - lambda;

                if b11.norm() > b21.norm() {
                    y[k] = -b12 / b11;
                    y[m] = Complex::new(1.0, 0.0);
                } else {
                    y[k] = Complex::new(1.0, 0.0);
                    if b22.norm() < 1e-12 {
                        // b22がゼロに近い場合、b11*y[k] + b12*y[m] = 0 を使う
                        y[m] = if b12.norm() < 1e-12 {
                            Complex::new(0.0, 0.0)
                        } else {
                            -b11 / b12
                        };
                    } else {
                        y[m] = -b21 / b22;
                    }
                }

                // 後退代入
                if k > 0 {
                    for j in (0..k).rev() {
                        let mut s = Complex::new(0.0, 0.0);
                        // ★★★ バグ修正(1): ループ範囲を k..i に修正 ★★★
                        for l in k..i {
                            s += t[(j, l)] * y[l];
                        }
                        // ★★★ バグ修正(2): 対角項の計算を複素数で行う ★★★
                        let diag = Complex::new(t[(j, j)], 0.0) - lambda;
                        if diag.norm() < 1e-12 {
                            y[j] = Complex::new(1.0, 0.0);
                        } else {
                            y[j] = -s / diag;
                        }
                    }
                }
            }

            // ベクトルを正規化
            let norm = y.norm();
            if norm > 1e-12 {
                for val in y.data.iter_mut() {
                    *val /= norm;
                }
            }

            // 結果を格納
            eigenvectors.set_col(k, &y).ok()?;

            if block_size == 2 {
                let y_conj = y.map(|c| c.conj());
                eigenvectors.set_col(k + 1, &y_conj).ok()?;
            }

            i -= block_size;
        }

        Some(eigenvectors)
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_complex;
