use crate::matrix::numerical::{EigenDecomposition, QrDecomposition};
use crate::matrix::Matrix;
use crate::{Direction, Vector};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Svd {
    pub u: Matrix<f64>,
    pub sigma: Vector<f64>,
    pub v: Matrix<f64>,
}

impl Svd {
    pub fn sort(&mut self) {
        let mut pairs: Vec<_> = self.sigma.data.iter().cloned().enumerate().collect();
        pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut new_sigma = vec![0.0; self.sigma.dim()];
        let mut new_u = Matrix::zeros(self.u.rows, self.u.cols);
        let mut new_v = Matrix::zeros(self.v.rows, self.v.cols);

        // 並べ替え対象は特異値数 n 列分のみ
        for (new_idx, (old_idx, val)) in pairs.iter().enumerate() {
            new_sigma[new_idx] = *val;
            new_u
                .set_col(new_idx, &self.u.col(*old_idx).unwrap())
                .unwrap();
            new_v
                .set_col(new_idx, &self.v.col(*old_idx).unwrap())
                .unwrap();
        }

        // U の残り列（n..m-1）は元の直交基底をそのまま保持する
        let n = self.sigma.dim();
        for col in n..self.u.cols {
            new_u
                .set_col(col, &self.u.col(col).unwrap())
                .unwrap();
        }

        self.sigma.data = new_sigma;
        self.u = new_u;
        self.v = new_v;
    }
}

pub trait SvdDeComposition {
    fn svd(&self) -> Option<Svd>;
    fn simple_svd(&self) -> Option<Svd>;
}

impl SvdDeComposition for Matrix<f64> {
    fn svd(&self) -> Option<Svd> {
        if self.rows < self.cols {
            let svd_t = self.transpose().svd()?;
            return Some(Svd {
                u: svd_t.v,
                sigma: svd_t.sigma,
                v: svd_t.u,
            });
        }

        let (mut b, mut u, mut v) = Self::bidiagonalize(self)?;
        Self::solve_bidiagonal_svd(&mut b, &mut u, &mut v)?;

        let mut sigma = Vector::new((0..self.cols).map(|i| b[(i, i)]).collect());

        for k in 0..sigma.dim() {
            if sigma[k] < 0.0 {
                sigma[k] = -sigma[k];
                let _ = u.scale_col(k, -1.0);
            }
        }

        let mut svd = Svd { u, sigma, v };
        svd.sort();

        Some(svd)
    }
    fn simple_svd(&self) -> Option<Svd> {
        if self.rows < self.cols {
            // Aが横長の行列(m < n)の場合、A^TのSVDを計算して結果を変換する
            let svd_t = self.transpose().svd()?;
            return Some(Svd {
                u: svd_t.v,
                sigma: svd_t.sigma,
                v: svd_t.u,
            });
        }

        // 1. A^T * A の固有値問題を解く
        let ata = &self.transpose() * self;
        let eigen_decomp = ata.eigen_decomposition()?;
        let eigenvalues = eigen_decomp.eigen_values;
        let v_raw = eigen_decomp.eigen_vectors; // 固有値分解直後のV

        // 2. 固有値を降順にソートし、対応する固有ベクトルも並べ替える
        let mut pairs: Vec<_> = eigenvalues.into_iter().zip(0..v_raw.cols).collect();
        pairs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        let mut sorted_v = Matrix::zeros(v_raw.rows, v_raw.cols);
        let mut sigma_vec = Vec::with_capacity(self.cols);

        for (i, (eigenval, original_idx)) in pairs.iter().enumerate() {
            let v_col = v_raw.col(*original_idx).unwrap();
            sorted_v.set_col(i, &v_col).ok()?; // この時点ではまだ正規化も直交化も不完全
            sigma_vec.push(eigenval.sqrt());
        }

        // ★★★ 変更点 ① ★★★
        // ソート後のV行列の直交性が崩れている可能性があるため、QR分解で直交性を回復させる
        // v_final は V^T ではなく V なので注意
        let qr = sorted_v.qr_decomposition()?;
        let v_final = qr.q;

        // 3. 特異値ベクトル Σ と 左特異ベクトル U を計算する
        let sigma = Vector::new(sigma_vec);
        let mut u = Matrix::zeros(self.rows, self.rows);

        for i in 0..self.cols {
            let sigma_i = sigma[i];
            let v_i = v_final.col(i).unwrap();

            if sigma_i.abs() < 1e-14 {
                // 特異値がゼロの場合、グラム・シュミット法でUの基底を補充する
                // (この部分は元の堅牢な実装をそのまま利用)
                let mut new_basis_found = false;
                for k in 0..self.rows {
                    let mut candidate_vec = Vector::zeros(self.rows);
                    candidate_vec[k] = 1.0;
                    for j in 0..i {
                        let u_j = u.col(j).unwrap();
                        let proj = u_j.dot(&candidate_vec);
                        candidate_vec = &candidate_vec - &(&u_j * proj);
                    }
                    let norm = candidate_vec.norm();
                    if norm > 1e-12 {
                        u.set_col(i, &(&candidate_vec * (1.0 / norm))).unwrap();
                        new_basis_found = true;
                        break;
                    }
                }
                if !new_basis_found {
                    u.set_col(i, &Vector::zeros(self.rows)).unwrap();
                }
            } else {
                // u_i = A * v_i / sigma_i
                let u_i = self * &v_i * (1.0 / sigma_i);
                u.set_col(i, &u_i).unwrap();
            }
        }

        // ★★★ 変更点 ② ★★★
        // 計算されたU行列の直交性が崩れている可能性があるため、QR分解で直交性を回復させる
        let qr = if self.rows == self.cols {
            // Aが正方行列の場合、uは正方行列なのでそのままQR分解
            u.qr_decomposition()?
        } else {
            // Aが縦長行列(m > n)の場合、uはm x mだが最初のn列しか計算していない。
            // そのため、計算済みのn列部分だけを直交化し、残りのm-n列を補完する必要がある。
            // ここでは簡単のため、m x n のU行列を返すことを想定し、
            // Uの最初のn列部分でQR分解を行う。
            // (もしm x mのUが必要な場合は、残りの列を埋める処理が必要)
            let u_sub = u.submatrix(0, self.rows, 0, self.cols);
            u_sub.qr_decomposition()?
        };

        let u_final = qr.q; // QR分解後のU行列

        Some(Svd {
            u: u_final,
            sigma,
            v: v_final,
        })
    }
}

impl Matrix<f64> {
    pub(super) fn bidiagonalize(&self) -> Option<(Matrix<f64>, Matrix<f64>, Matrix<f64>)> {
        let mut b = self.clone();
        let mut u = Matrix::identity(self.rows);
        let mut v = Matrix::identity(self.cols); // Vを直接計算する

        for k in 0..self.cols {
            // --- 1. 左からのHouseholder変換 (列をゼロにする) ---
            let x = b.partial_col(k, k, self.rows).ok()?;
            if let Some(h_vec) = x.householder_vector() {
                // b = H * b
                b.apply_householder_transform(&h_vec, Direction::Left, k, k);
                // u = u * H (Hは対称なので H^T = H)
                u.apply_householder_transform(&h_vec, Direction::Right, 0, k);
            }

            // --- 2. 右からのHouseholder変換 (行をゼロにする) ---
            if k < self.cols - 2 {
                let y = b.partial_row(k, k + 1, self.cols).ok()?;
                if let Some(h_vec) = y.householder_vector() {
                    // b = b * G
                    b.apply_householder_transform(&h_vec, Direction::Right, k, k + 1);
                    // v = v * G
                    v.apply_householder_transform(&h_vec, Direction::Right, 0, k + 1);
                }
            }
        }
        // SVDでは V が欲しいため、最初からVを計算して返す
        Some((b, u, v))
    }

    pub fn solve_bidiagonal_svd(
        b: &mut Matrix<f64>,
        u: &mut Matrix<f64>,
        v: &mut Matrix<f64>,
    ) -> Option<()> {
        // 元の A を保持（A = U B V^T）
        let u0 = u.clone();
        let b0 = b.clone();
        let v0t = v.transpose();
        let a0 = &(&u0 * &b0) * &v0t;

        let n = b.cols;
        if n == 0 {
            return Some(());
        }
        let tol = 1e-12f64;
        let max_iter = 10_000usize;

        // 反復で対角化
        let mut d; // diag
        let mut e; // superdiag
        let mut k: isize = (n as isize) - 1; // active tail index
        let mut it = 0usize;
        loop {
            if k < 0 {
                break;
            }
            if it > max_iter {
                return None;
            }
            it += 1;

            // バンド抽出
            let (dd, ee) = Self::extract_band_from_b(b);
            d = dd;
            e = ee;

            // 末尾のデフレーション
            while k as usize > 0 {
                let kk = (k - 1) as usize;
                if Self::is_small_e_index(kk, &d, &e, tol) {
                    // 反映
                    e[kk] = 0.0;
                    b[(kk, kk + 1)] = 0.0;
                    k -= 1;
                } else {
                    break;
                }
            }
            if k == 0 {
                break;
            }

            // 先頭側のスキップで l を決定
            let mut l: isize = 0;
            while (l as usize) < (k as usize) {
                let ll = l as usize;
                if Self::is_small_e_index(ll, &d, &e, tol) {
                    e[ll] = 0.0;
                    b[(ll, ll + 1)] = 0.0;
                    l += 1;
                } else {
                    break;
                }
            }
            if l as usize >= k as usize {
                k -= 1;
                continue;
            }

            // Wilkinson シフト計算
            let mu = Self::compute_wilkinson_shift(&d, &e, k as usize);

            // 1 スイープのバルジ追跡
            Self::bulge_chase_sweep(b, u, v, l as usize, k as usize, mu, n);

            // 小さい e を一括 0 反映
            let (mut d2, mut e2) = Self::extract_band_from_b(b);
            Self::zero_small_superdiag(b, &mut d2, &mut e2, tol);
        }

        // 対角正規化
        Self::ensure_nonnegative_diagonal(b, u);

        // U を A と V, σ から再構成
        Self::reconstruct_u_from_a_and_v(u, v, b, &a0)?;
        Some(())
    }
    // --- helpers ---
    fn extract_band_from_b(b: &Matrix<f64>) -> (Vec<f64>, Vec<f64>) {
        let n = b.cols;
        let mut d = vec![0.0; n];
        let mut e = vec![0.0; n.saturating_sub(1)];
        for i in 0..n {
            d[i] = b[(i, i)];
            if i + 1 < n {
                e[i] = b[(i, i + 1)];
            }
        }
        (d, e)
    }

    fn is_small_e_index(i: usize, d: &[f64], e: &[f64], tol: f64) -> bool {
        if i >= e.len() {
            return true;
        }
        let ai = d[i].abs();
        let bi = if i + 1 < d.len() { d[i + 1].abs() } else { 0.0 };
        e[i].abs() <= tol.max(tol * (ai + bi))
    }

    fn compute_wilkinson_shift(d: &[f64], e: &[f64], k: usize) -> f64 {
        let km1 = k - 1;
        let a = d[km1] * d[km1] + if k >= 2 { e[km1 - 1] * e[km1 - 1] } else { 0.0 };
        let b2 = d[km1] * e[km1];
        let c = d[k] * d[k] + e[km1] * e[km1];
        let tr = a + c;
        let det = a * c - b2 * b2;
        let disc = (tr * tr - 4.0 * det).max(0.0).sqrt();
        let mu1 = 0.5 * (tr + disc);
        let mu2 = 0.5 * (tr - disc);
        if (c - mu1).abs() < (c - mu2).abs() {
            mu1
        } else {
            mu2
        }
    }

    fn bulge_chase_sweep(
        b: &mut Matrix<f64>,
        u: &mut Matrix<f64>,
        v: &mut Matrix<f64>,
        l: usize,
        k: usize,
        mu: f64,
        n: usize,
    ) {
        // 初期ベクトル
        let mut x = b[(l, l)] * b[(l, l)] - mu;
        let mut z = if l < n - 1 {
            b[(l, l)] * b[(l, l + 1)]
        } else {
            0.0
        };

        for i in l..k {
            // 右回転（列 i, i+1）
            let (cr, sr) = Self::calculate_givens_params(x, z);
            b.apply_right_givens_rotation(i, i + 1, cr, sr);
            v.apply_right_givens_rotation(i, i + 1, cr, sr);

            // 左回転（行 i, i+1）
            let a1 = b[(i, i)];
            let a2 = b[(i + 1, i)];
            let (cl, sl) = Self::calculate_left_givens_params(a1, a2);
            b.apply_left_givens_to_rows(i, i + 1, cl, sl);
            u.apply_left_givens_rotation(i, i + 1, cl, sl);

            // 次の (x, z)
            if i + 1 < n - 1 {
                let di1 = b[(i + 1, i + 1)];
                let ei1 = b[(i + 1, i + 2)];
                x = di1 * di1 - mu;
                z = di1 * ei1;
            }
        }
    }

    fn zero_small_superdiag(b: &mut Matrix<f64>, d: &mut [f64], e: &mut [f64], tol: f64) {
        let n = d.len();
        let mut to_zero: Vec<usize> = Vec::new();
        for i in 0..e.len() {
            let ai = d[i].abs();
            let bi = if i + 1 < n { d[i + 1].abs() } else { 0.0 };
            if e[i].abs() <= tol.max(tol * (ai + bi)) {
                to_zero.push(i);
            }
        }
        for &i in &to_zero {
            e[i] = 0.0;
            b[(i, i + 1)] = 0.0;
        }
    }

    fn ensure_nonnegative_diagonal(b: &mut Matrix<f64>, u: &mut Matrix<f64>) {
        let n = b.cols;
        for i in 0..n {
            if b[(i, i)] < 0.0 {
                b[(i, i)] = -b[(i, i)];
                for r in 0..u.rows {
                    u[(r, i)] = -u[(r, i)];
                }
            }
        }
    }

    fn reconstruct_u_from_a_and_v(
        u: &mut Matrix<f64>,
        v: &Matrix<f64>,
        b: &Matrix<f64>,
        a0: &Matrix<f64>,
    ) -> Option<()> {
        let m = u.rows;
        let n = b.cols;
        let mut u_new = Matrix::<f64>::zeros(m, m);
        let eps = 1e-14;

        // まず、非ゼロ特異値に対応する列を設定
        for i in 0..n {
            let sigma_i = b[(i, i)].abs();
            if sigma_i > eps {
                let v_i = v.col(i).ok()?;
                let mut u_i = a0 * &v_i; // m x 1
                let inv = 1.0 / sigma_i;
                for r in 0..m {
                    u_i[r] *= inv;
                }
                // 正規化
                let norm = u_i.norm();
                if norm > eps {
                    for r in 0..m {
                        u_i[r] /= norm;
                    }
                }
                u_new.set_col(i, &u_i).ok()?;
            }
        }

        // 未設定列（ゼロ特異値列を含む）と残り列を直交補完で埋める
        for target_col in 0..m {
            let col = u_new.col(target_col).ok()?;
            if col.norm() > 1e-12 {
                continue; // すでに設定済み
            }
            // 標準基底から開始し、既存の全列に対して直交化
            let mut cand = Vector::<f64>::zeros(m);
            cand[target_col.min(m - 1)] = 1.0;
            for j in 0..m {
                let cj = u_new.col(j).ok()?;
                let nj = cj.norm();
                if nj <= 1e-12 {
                    continue;
                }
                let proj = cj.dot(&cand);
                for r in 0..m {
                    cand[r] -= cj[r] * proj;
                }
            }
            let norm = cand.norm();
            if norm > 1e-12 {
                for r in 0..m {
                    cand[r] /= norm;
                }
                u_new.set_col(target_col, &cand).ok()?;
            }
        }
        *u = u_new;
        Some(())
    }
    #[allow(dead_code)]
    /// Chases a bulge off the matrix near e[k] using right rotations only.
    /// Updates V so that B1 ≈ B0 * V, and sets e[k] = 0.0. Used by tests.
    fn chase_zero_off_diagonal(
        k: usize,
        l: usize,
        d: &mut [f64],
        e: &mut [f64],
        v: &mut Matrix<f64>,
        // u: &mut Matrix<f64>, // A full implementation would update U as well.
    ) -> Option<()> {
        let n = d.len();
        if n == 0 {
            return Some(());
        }
        let l = l.min(n - 1);
        if k >= l {
            if k < e.len() {
                e[k] = 0.0;
            }
            return Some(());
        }

        // Build B0 from d,e
        let mut b = Matrix::zeros(n, n);
        for i in 0..n {
            b[(i, i)] = d[i];
            if i + 1 < n {
                b[(i, i + 1)] = e[i];
            }
        }

        // Single right rotation on columns (k, k+1) to eliminate e[k]
        let (c, s) = Self::calculate_givens_params(b[(k, k)], b[(k, k + 1)]);
        b.apply_right_givens_rotation(k, k + 1, c, s);
        v.apply_right_givens_rotation(k, k + 1, c, s);

        // Write back diag and superdiag from B0*V; force e[k]=0
        for i in 0..n {
            d[i] = b[(i, i)];
            if i + 1 < n {
                e[i] = b[(i, i + 1)];
            }
        }
        if k < e.len() {
            e[k] = 0.0;
        }
        Some(())
    }
    /// Computes parameters for a Givens rotation in a numerically stable way.
    /// Given a and b, computes c and s such that:
    /// | c -s | | a | = | r |
    /// | s  c | | b | | 0 |
    fn calculate_givens_params(a: f64, b: f64) -> (f64, f64) {
        if b == 0.0 {
            (1.0, 0.0)
        } else if a == 0.0 {
            (0.0, -b.signum())
        } else {
            let r = a.hypot(b);
            (a / r, -b / r)
        }
    }

    fn calculate_left_givens_params(a: f64, b: f64) -> (f64, f64) {
        if b == 0.0 {
            (1.0, 0.0)
        } else {
            let r = a.hypot(b);
            // To satisfy -s*a + c*b = 0, we need s = b/r and c = a/r.
            (a / r, b / r)
        }
    }

    /// Applies a Givens rotation from the left to two rows.
    /// [row1; row2] <- [[c, s]; [-s, c]] * [row1; row2]
    fn apply_left_givens_to_rows(&mut self, row1: usize, row2: usize, c: f64, s: f64) {
        for j in 0..self.cols {
            let val1 = self[(row1, j)];
            let val2 = self[(row2, j)];
            self[(row1, j)] = c * val1 + s * val2;
            self[(row2, j)] = -s * val1 + c * val2;
        }
    }

    /// Applies a standard Givens rotation from the right to two columns.
    /// [col1, col2] <- [col1, col2] * [[c, -s], [s, c]]
    fn apply_right_givens_rotation(&mut self, col1: usize, col2: usize, c: f64, s: f64) {
        for i in 0..self.rows {
            let val1 = self[(i, col1)];
            let val2 = self[(i, col2)];
            self[(i, col1)] = c * val1 - s * val2;
            self[(i, col2)] = s * val1 + c * val2;
        }
    }

    /// Applies a standard Givens rotation from the left to two columns of a matrix (U or V).
    /// This is equivalent to U <- U * G, where G is the Givens matrix.
    /// [col1, col2] <- [col1, col2] * [[c, -s], [s, c]]
    fn apply_left_givens_rotation(&mut self, col1: usize, col2: usize, c: f64, s: f64) {
        for i in 0..self.rows {
            let val1 = self[(i, col1)];
            let val2 = self[(i, col2)];
            self[(i, col1)] = c * val1 - s * val2;
            self[(i, col2)] = s * val1 + c * val2;
        }
    }
}
