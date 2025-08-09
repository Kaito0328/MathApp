use crate::matrix::numerical::{EigenDecomposition, QrDecomposition};
use crate::matrix::Matrix;
use crate::{Direction, Vector};

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

        for (new_idx, (old_idx, val)) in pairs.iter().enumerate() {
            new_sigma[new_idx] = *val;
            new_u
                .set_col(new_idx, &self.u.col(*old_idx).unwrap())
                .unwrap();
            new_v
                .set_col(new_idx, &self.v.col(*old_idx).unwrap())
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

    fn solve_bidiagonal_svd(
        b: &mut Matrix<f64>,
        u: &mut Matrix<f64>,
        v: &mut Matrix<f64>,
    ) -> Option<()> {
        let n = b.cols;
        if n == 0 {
            return Some(());
        }

        let mut d: Vec<f64> = (0..n).map(|i| b[(i, i)]).collect();
        let mut e: Vec<f64> = if n > 1 {
            (0..n - 1).map(|i| b[(i, i + 1)]).collect()
        } else {
            vec![]
        };

        const MAX_ITERATIONS: usize = 50; // 1つの特異値あたりの最大反復回数

        // 下のブロックから処理していく
        for l in (0..n).rev() {
            'iteration: for iter in 0..MAX_ITERATIONS {
                println!("Processing block ending at index {l}, iteration {iter}");
                println!("Current d: {d:?}");
                println!("Current e: {e:?}");
                println!("Current u: {u}");
                println!("Current v: {v}");
                // --- 1. 分割判定 ---
                // e の要素がゼロに近い箇所を探し、行列を分割できるかチェック
                let mut k = l;
                while k > 0 {
                    // |e[k-1]| が d[k-1] と d[k] に比べて十分に小さいか
                    if e[k - 1].abs() <= f64::EPSILON * (d[k].abs() + d[k - 1].abs()) {
                        e[k - 1] = 0.0;
                        break;
                    }
                    k -= 1;
                }

                // k == l の場合、d[l]が収束した（ブロックが分離した）とみなす
                if k == l {
                    break 'iteration; // この l の処理は完了
                }

                // --- 2. d[k]がゼロに近い場合、ゼロを追い出す ---
                if d[k].abs() <= f64::EPSILON {
                    d[k] = 0.0;
                    // e[k]をゼロにするための左からのギブンス回転
                    for i in k..l {
                        let (c, s, _) = Self::calculate_givens_rotation(d[i + 1], e[i]);
                        d[i + 1] = c * d[i + 1] - s * e[i];
                        e[i] = 0.0;
                        // V に適用
                        v.apply_column_rotation(k, i + 1, c, s)?;
                    }
                    continue 'iteration;
                }

                // --- 3. QR反復 (バルジチェイシング) ---
                // Wilkinson Shiftを計算
                let t11 = d[l - 1].powi(2) + if l > 1 { e[l - 2].powi(2) } else { 0.0 };
                let t22 = d[l].powi(2) + e[l - 1].powi(2);
                let t12 = d[l - 1] * e[l - 1];
                let delta = (t11 - t22) / 2.0;
                let sign_delta = if delta == 0.0 { 1.0 } else { delta.signum() };
                let shift =
                    t22 - t12.powi(2) / (delta + sign_delta * (delta.powi(2) + t12.powi(2)).sqrt());

                // 最初の回転
                let mut f = d[k].powi(2) - shift;
                let mut h = d[k] * e[k];

                for i in k..l {
                    // 右回転 (Vを更新)
                    let (c, s, _) = Self::calculate_givens_rotation(f, h);
                    if i != k {
                        e[i - 1] = c * e[i - 1] - s * d[i];
                    }
                    d[i] = c * d[i] - s * h;
                    h = s * d[i + 1];
                    d[i + 1] *= c;
                    v.apply_column_rotation(i, i + 1, c, s)?;

                    // 左回転 (Uを更新)
                    f = d[i];
                    let (c2, s2, _) = Self::calculate_givens_rotation(f, h);
                    d[i] = c2 * f - s2 * h;
                    if i != l - 1 {
                        e[i] = c2 * e[i] - s2 * d[i + 1];
                        d[i + 1] = s2 * e[i] + c2 * d[i + 1];
                    }
                    u.apply_column_rotation(i, i + 1, c2, s2)?;

                    if i != l - 1 {
                        f = e[i];
                        h = d[i + 1];
                    }
                }

                if iter == MAX_ITERATIONS - 1 {
                    println!(
                        "Maximum iterations reached for block ending at index {l}, returning None."
                    );
                    return None; // 収束しなかった
                }
            }
        }

        // 更新された d, e を b に書き戻す
        for i in 0..n {
            b[(i, i)] = d[i];
            if i < n - 1 {
                b[(i, i + 1)] = e[i];
            }
        }

        Some(())
    }

    fn calculate_givens_rotation(a: f64, b: f64) -> (f64, f64, f64) {
        if b == 0.0 {
            return (1.0, 0.0, a);
        }
        if a == 0.0 {
            return (0.0, 1.0, b);
        }

        let abs_a = a.abs();
        let abs_b = b.abs();
        let r;
        let c;
        let s;

        if abs_a > abs_b {
            let t = b / a;
            let u = (1.0 + t * t).sqrt();
            r = abs_a * u;
            c = 1.0 / u;
            s = t * c;
        } else {
            let t = a / b;
            let u = (1.0 + t * t).sqrt();
            r = abs_b * u;
            s = 1.0 / u;
            c = t * s;
        }

        // 元の符号を維持する
        let signed_r = a.signum() * r;
        let signed_c = a.signum() * c;
        let signed_s = b.signum() * s;

        (signed_c, signed_s, signed_r)
    }

    /// 列にギブンス回転を適用します。
    /// [col1, col2] <- [col1, col2] * [[c, -s], [s, c]]
    fn apply_column_rotation(&mut self, col1: usize, col2: usize, c: f64, s: f64) -> Option<()> {
        if col1 >= self.cols || col2 >= self.cols {
            return None;
        }
        for i in 0..self.rows {
            let val1 = self[(i, col1)];
            let val2 = self[(i, col2)];
            self[(i, col1)] = c * val1 - s * val2;
            self[(i, col2)] = s * val1 + c * val2;
        }
        Some(())
    }
}
