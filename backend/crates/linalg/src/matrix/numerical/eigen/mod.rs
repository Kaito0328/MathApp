use num_complex::Complex;

use crate::{
    matrix::{numerical::QrDecomposition, Matrix},
    Vector,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Eigen {
    pub eigen_values: Vec<f64>,
    pub eigen_vectors: Matrix<f64>,
}

// Complex<f64>はserdeサポートが必要なため、一時的にserde対応を省略
pub struct EigenComplex {
    pub eigen_values: Vec<Complex<f64>>,
    pub eigen_vectors: Matrix<Complex<f64>>,
}

pub trait EigenDecomposition {
    /// LU分解を行う。成功した場合はLU構造体を返す。
    /// 行列が正方行列でない場合はNoneを返す。
    fn eigen_decomposition(&self) -> crate::Result<Eigen>;
    fn eigen_decomposition_complex(&self) -> crate::Result<EigenComplex>;
}

impl EigenDecomposition for Matrix<f64> {
    fn eigen_decomposition(&self) -> crate::Result<Eigen> {
        // --- エッジケースの事前処理 (変更なし) ---
        if self.rows == 0 {
            return Ok(Eigen {
                eigen_values: vec![],
                eigen_vectors: Matrix { rows: 0, cols: 0, data: vec![] },
            });
        }
        if !self.is_square() {
            return Err(crate::LinalgError::NotSquareMatrix);
        }
        if self.rows == 1 {
            return Ok(Eigen {
                eigen_values: vec![self[(0, 0)]],
                eigen_vectors: Matrix::identity(1),
            });
        }

        let n = self.rows;
        // Hessenberg 化して h を得る（固有値収束のために QR 反復はそのまま使う）
                let (mut h, _v_dummy) = self
                    .to_hessenberg()
                    .ok_or(crate::LinalgError::InvalidArgument {
                        text: "Hessenberg reduction failed".into(),
                    })?; // v はここでは使わない（零空間で再構築する）
        let mut end = n;

        let max_total_iterations = 30 * n;
        let mut total_iterations = 0;

        while end > 1 {
            let m = end - 1;
            let mut iterations_since_deflation = 0;

            loop {
                if total_iterations >= max_total_iterations {
                    println!("Maximum total iterations reached, returning None.");
                    return Err(crate::LinalgError::InvalidArgument { text: "Maximum total iterations reached".into() });
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

                if let Ok(qr) = shifted_h
                    .submatrix(0, end, 0, end)
                    .qr_decomposition()
                {
                    let q = qr.q;
                    let mut q_full = Matrix::identity(n);
                            q_full.set_submatrix(0, 0, &q)?;

                    h = &(&q_full.transpose() * &h) * &q_full;
                    // v は累積しない（零空間で固有ベクトルを再計算するため）
                } else {
                    return Err(crate::LinalgError::InvalidArgument { text: "QR decomposition failed".into() });
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
                return Err(crate::LinalgError::InvalidArgument { text: "Failed to compute nullspace vector".into() });
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
            final_eigenvectors.set_col(col_idx, &eigenvectors_cols[i])?;
        }

        Ok(Eigen {
            eigen_values: final_eigenvalues,
            eigen_vectors: final_eigenvectors,
        })
    }

    fn eigen_decomposition_complex(&self) -> crate::Result<EigenComplex> {
        // --- エッジケースの事前処理 (変更なし) ---
        if self.rows == 0 {
            return Ok(EigenComplex { eigen_values: vec![], eigen_vectors: Matrix { rows: 0, cols: 0, data: vec![] } });
        }
        if !self.is_square() {
            return Err(crate::LinalgError::NotSquareMatrix);
        }
        // 小規模(<=4)では数値安定性を優先して多項式ルート法を採用
        if self.rows <= 4 {
            return Self::eigen_decomposition_complex_via_roots(self);
        }
        if self.rows == 1 {
            return Ok(EigenComplex { eigen_values: vec![Complex::new(self[(0, 0)], 0.0)], eigen_vectors: Matrix { rows: 1, cols: 1, data: vec![Complex::new(1.0, 0.0)] } });
        }
        // 小規模行列に限定せず、まずはHessenberg+QRで試み、必要なら後段でフォールバック
        let n = self.rows;
        let (mut h, mut q) = self
            .to_hessenberg()
            .ok_or(crate::LinalgError::InvalidArgument { text: "Hessenberg reduction failed".into() })?;
        if !Self::qr_iteration_to_schur(&mut h, &mut q, 1e-12) {
            // 最後の手段としてフォールバック
            return Self::eigen_decomposition_complex_via_roots(self);
        }
        let t = h; // 実シュア形式

        let eigenvalues_unsorted = Self::extract_eigenvalues_from_schur(&t, 1e-12);
        let schur_eigenvectors = match Self::compute_schur_eigenvectors(&t, &eigenvalues_unsorted) {
            Some(y) => y,
            None => return Self::eigen_decomposition_complex_via_roots(self),
        };
        let eigenvectors_unsorted = &q.to_complex() * &schur_eigenvectors;

        // 固有値と固有ベクトルを (実部→虚部) で安定ソート
        let mut pairs: Vec<(Complex<f64>, crate::Vector<Complex<f64>>)> = Vec::with_capacity(n);
        for (val, i) in eigenvalues_unsorted.into_iter().zip(0..n) {
            let col = match eigenvectors_unsorted.col(i) {
                Ok(c) => c,
                Err(_) => {
                    return Err(crate::LinalgError::InvalidArgument {
                        text: "Failed to extract eigenvector column".into(),
                    })
                }
            };
            pairs.push((val, col));
        }
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

        let final_eigenvalues: Vec<Complex<f64>> = pairs.iter().map(|(val, _)| *val).collect();
        let mut final_eigenvectors = Matrix::<Complex<f64>>::zeros(n, n);
        for (i, (_, vec)) in pairs.iter().enumerate() {
            final_eigenvectors.set_col(i, vec)?;
        }

        // 追加の健全性チェック(1): 特性多項式 p(λ) の評価で固有値の妥当性を確認
        // 数値的に信頼できない場合はフォールバックへ切り替える
        let coeffs = Self::characteristic_polynomial_coeffs(self);
        let max_poly_resid = final_eigenvalues
            .iter()
            .map(|&lam| Self::poly_eval_complex(&coeffs, lam).norm())
            .fold(0.0_f64, f64::max);

        // 残差が悪ければフォールバック
        let a_c = self.to_complex();
        let mut d = Matrix::zeros(n, n);
        for i in 0..n {
            d[(i, i)] = final_eigenvalues[i];
        }
        let resid = (&a_c * &final_eigenvectors - &final_eigenvectors * &d)
            .data
            .iter()
            .map(|z| z.norm_sqr())
            .sum::<f64>()
            .sqrt();
        if resid.is_finite() && resid < 1e-6 && max_poly_resid < 1e-6 {
            Ok(EigenComplex {
                eigen_values: final_eigenvalues,
                eigen_vectors: final_eigenvectors,
            })
        } else {
            Self::eigen_decomposition_complex_via_roots(self)
        }
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

    /// 明示的なQR反復: H_k = R_k Q_k（トップレフト end×end）を繰り返し、デフレーションで end を縮める。
    fn qr_iteration_to_schur(h: &mut Matrix<f64>, q: &mut Matrix<f64>, tol: f64) -> bool {
        let n = h.rows;
        let mut end = n;
        let max_iter = 5000usize;

        while end > 0 {
            let mut progressed = false;
            for _ in 0..max_iter {
                let Some(qr) = h
                    .submatrix(0, end, 0, end)
                    .qr_decomposition()
                    .ok()
                else {
                    return false;
                };
                let q_step = qr.q;
                let r_step = qr.r;

                // H <- R Q をトップレフトに反映
                let rq = &r_step * &q_step;
                if h.set_submatrix(0, 0, &rq).is_err() {
                    return false;
                }

                // Q 累積
                let mut q_full = Matrix::identity(n);
                if q_full.set_submatrix(0, 0, &q_step).is_err() {
                    return false;
                }
                *q = &(*q) * &q_full;

                // 小さいサブ対角を0に
                for i in 1..end {
                    if h[(i, i - 1)].abs() <= tol * (h[(i, i)].abs() + h[(i - 1, i - 1)].abs()) {
                        h[(i, i - 1)] = 0.0;
                    }
                }

                // 末尾デフレーションのチェック（1x1 または 2x2）
                let m = end - 1;
                if m == 0 || h[(m, m - 1)] == 0.0 {
                    end -= 1;
                    progressed = true;
                    break;
                }
                if m > 0 && (m == 1 || h[(m - 1, m - 2)] == 0.0) {
                    end -= 2;
                    progressed = true;
                    break;
                }
            }
            if !progressed {
                return false;
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

    // ---- フォールバック: 係数法 + 多項式根（Durand–Kerner） + 複素ガウス消去 ----
    fn characteristic_polynomial_coeffs(a: &Matrix<f64>) -> Vec<f64> {
        // Faddeev–LeVerrier: det(λI - A) = λ^n + c1 λ^{n-1} + ... + c_n
        // B0 = 0, c0 = 1;  Bk = A (B_{k-1} + c_{k-1} I),  ck = -(1/k) tr(Bk)
        let n = a.rows;
        let mut coeffs = vec![0.0; n + 1];
        coeffs[0] = 1.0;
        if n == 0 {
            return coeffs;
        }
        let mut b = Matrix::<f64>::zeros(n, n); // B0 = 0
        let mut c_prev = 1.0; // c0
        for (k, coeff) in coeffs.iter_mut().enumerate().take(n + 1).skip(1) {
            // Bk = A (B_{k-1} + c_{k-1} I)
            let mut inner = b.clone();
            for i in 0..n {
                inner[(i, i)] += c_prev;
            }
            b = a * &inner;
            let trace = (0..n).map(|i| b[(i, i)]).sum::<f64>();
            let ck = -trace / (k as f64);
            *coeff = ck;
            c_prev = ck;
        }
        coeffs
    }

    fn poly_eval_complex(coeffs: &[f64], z: Complex<f64>) -> Complex<f64> {
        // Horner 法: c0 z^n + c1 z^{n-1} + ... + c_n
        let mut acc = Complex::new(0.0, 0.0);
        for &c in coeffs.iter() {
            acc = acc * z + Complex::new(c, 0.0);
        }
        acc
    }

    fn find_roots_durand_kerner(coeffs: &[f64]) -> Vec<Complex<f64>> {
        let n = coeffs.len() - 1;
        let r = 1.0 + coeffs.iter().skip(1).map(|c| c.abs()).fold(0.0, f64::max);
        let two_pi = std::f64::consts::PI * 2.0;
        let mut roots: Vec<Complex<f64>> = (0..n)
            .map(|k| Complex::from_polar(r, two_pi * (k as f64) / (n as f64)))
            .collect();

        let max_iter = 4000usize;
        let tol = 1e-12;
        for _ in 0..max_iter {
            let mut max_delta = 0.0;
            for i in 0..n {
                let zi = roots[i];
                let fzi = Self::poly_eval_complex(coeffs, zi);
                // 既存の他根との差の積
                let mut denom = Complex::new(1.0, 0.0);
                for (j, root) in roots.iter().enumerate() {
                    if i != j {
                        denom *= zi - root;
                    }
                }
                if denom.norm() == 0.0 {
                    continue;
                }
                let delta = fzi / denom;
                roots[i] -= delta;
                let dn = delta.norm();
                if dn > max_delta {
                    max_delta = dn;
                }
            }
            if max_delta < tol {
                break;
            }
        }
        roots
    }

    fn nullspace_vector_complex(
        m: &Matrix<Complex<f64>>,
        tol: f64,
    ) -> Option<crate::Vector<Complex<f64>>> {
        let n = m.cols; // 方形を想定
        let mut a = m.clone();
        let mut pivots: Vec<Option<usize>> = vec![None; n];
        let mut row = 0;
        for col in 0..n {
            // ピボット探索
            let mut pivot_row = None;
            let mut max_norm = 0.0;
            for r in row..n {
                let val = a[(r, col)];
                let nm = val.norm();
                if nm > max_norm {
                    max_norm = nm;
                    pivot_row = Some(r);
                }
            }
            if let Some(pr) = pivot_row {
                if max_norm <= tol {
                    continue;
                }
                // 行交換
                if pr != row {
                    for j in 0..n {
                        a.data.swap(pr * n + j, row * n + j);
                    }
                }
                // 正規化
                let piv = a[(row, col)];
                for j in col..n {
                    a[(row, j)] /= piv;
                }
                // 前進消去
                for r in 0..n {
                    if r == row {
                        continue;
                    }
                    let factor = a[(r, col)];
                    if factor.norm() > 0.0 {
                        // 行 row の使用部分を一旦コピーして借用衝突を避ける
                        let mut row_slice: Vec<Complex<f64>> = Vec::with_capacity(n - col);
                        for j in col..n {
                            row_slice.push(a[(row, j)]);
                        }
                        for (offset, j) in (col..n).enumerate() {
                            a[(r, j)] -= factor * row_slice[offset];
                        }
                    }
                }
                pivots[col] = Some(row);
                row += 1;
                if row == n {
                    break;
                }
            }
        }

        // 自由変数を一つ選んで1にし、従属変数を解く
        // すべての列がピボット扱い（数値丸めで満ランクに見える）場合は、
        // 最後の列を自由変数としてフォールバック採用する。
        let free_col = (0..n)
            .find(|&c| pivots[c].is_none())
            .unwrap_or(n.saturating_sub(1));
        let mut v = crate::Vector::<Complex<f64>>::zeros(n);
        v[free_col] = Complex::new(1.0, 0.0);
        for col in (0..n).rev() {
            if let Some(r) = pivots[col] {
                let mut sum = Complex::new(0.0, 0.0);
                for j in (col + 1)..n {
                    sum += a[(r, j)] * v[j];
                }
                v[col] = -sum; // 係数は既に単位ピボット
            }
        }
        // 正規化
        let norm = v.data.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
        if norm > tol {
            for z in v.data.iter_mut() {
                *z /= norm;
            }
        }
        Some(v)
    }

    fn eigen_decomposition_complex_via_roots(a: &Matrix<f64>) -> crate::Result<EigenComplex> {
        if !a.is_square() {
            return Err(crate::LinalgError::NotSquareMatrix);
        }
        let n = a.rows;
        if n == 0 {
            return Ok(EigenComplex {
                eigen_values: vec![],
                eigen_vectors: Matrix { rows: 0, cols: 0, data: vec![] },
            });
        }
        let coeffs = Self::characteristic_polynomial_coeffs(a);
        // 小規模では Durand–Kerner を優先して安定に根を求める
        let mut eigs = Self::find_roots_durand_kerner(&coeffs);
        // ソート（実部→虚部）
        eigs.sort_by(|x, y| {
            x.re.partial_cmp(&y.re)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(x.im.partial_cmp(&y.im).unwrap_or(std::cmp::Ordering::Equal))
        });

        let a_c = a.to_complex();
        let mut vecs = Matrix::<Complex<f64>>::zeros(n, n);
        for (i, &lam) in eigs.iter().enumerate() {
            let mut m = a_c.clone();
            for d in 0..n {
                m[(d, d)] -= lam;
            }
            // 零空間ベクトルを段階的に緩い tol で試行
            let mut v_opt = None;
            for &tol in &[1e-10, 1e-8, 1e-6, 1e-4] {
                if let Some(v) = Self::nullspace_vector_complex(&m, tol) {
                    v_opt = Some(v);
                    break;
                }
            }
            let v = match v_opt {
                Some(v) => v,
                None => {
                    return Err(crate::LinalgError::InvalidArgument {
                        text: "Failed to compute complex nullspace vector".into(),
                    })
                }
            };
            vecs.set_col(i, &v)?;
        }

        Ok(EigenComplex {
            eigen_values: eigs,
            eigen_vectors: vecs,
        })
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_complex;
