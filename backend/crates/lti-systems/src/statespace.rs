use linalg::matrix::numerical::exp::MatrixExponential;
use linalg::matrix::Matrix;
use poly::polynomial::Polynomial;
use poly::rational_function::RationalFunction;
use std::fmt;

/// 連続時間の状態空間表現
#[derive(Clone, Debug, PartialEq)]
pub struct ContinuousStateSpace {
    pub a: Matrix<f64>,
    pub b: Matrix<f64>,
    pub c: Matrix<f64>,
    pub d: Matrix<f64>,
}

/// 離散時間の状態空間表現
#[derive(Clone, Debug, PartialEq)]
pub struct DiscreteStateSpace {
    pub a: Matrix<f64>,
    pub b: Matrix<f64>,
    pub c: Matrix<f64>,
    pub d: Matrix<f64>,
}

impl ContinuousStateSpace {
    /// ZOH 厳密離散化（サンプリング周期 Ts = 1/fs）
    pub fn c2d_zoh(&self, fs: f64) -> DiscreteStateSpace {
        assert!(fs > 0.0);
        let ts = 1.0 / fs;
        let n = self.a.rows;
        let m = self.b.cols; // 入力数
                             // ブロック行列 M = [[A, B], [0, 0]] の指数関数で Bd を得る方法を使う
                             // ただし A は n×n, B は n×m。M は (n+m)×(n+m)
        let mut mtx = Matrix::zeros(n + m, n + m);
        // 左上 A*Ts, 右上 B*Ts
        for i in 0..n {
            for j in 0..n {
                mtx[(i, j)] = self.a[(i, j)] * ts;
            }
        }
        for i in 0..n {
            for j in 0..m {
                mtx[(i, n + j)] = self.b[(i, j)] * ts;
            }
        }
        // expm(M)
        let em = mtx.expm();
        // Ad = expm(A*Ts) は左上 n×n
        let mut ad = Matrix::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                ad[(i, j)] = em[(i, j)];
            }
        }
        // Bd は左上の右隣 n×m ブロック
        let mut bd = Matrix::zeros(n, m);
        for i in 0..n {
            for j in 0..m {
                bd[(i, j)] = em[(i, n + j)];
            }
        }
        // ZOH では C, D は同一
        DiscreteStateSpace {
            a: ad,
            b: bd,
            c: self.c.clone(),
            d: self.d.clone(),
        }
    }

    /// 伝達関数（単入力単出力）から可制御正準形を構成
    pub fn from_tf_siso(num: &Polynomial<f64>, den: &Polynomial<f64>) -> Self {
        // 正規化: 最高次の係数で割る
        let a_den = den.monic();
        let a_coeffs = &a_den.coeffs; // a0 + a1 s + ... + s^n (monic)
        let n = a_den.deg().max(0) as usize;
        assert!(n >= 1, "denominator degree must be >= 1");

        // 分子次数を調整（真にプロパー前提: deg(num) <= deg(den)）
        let mut b_num = num.clone();
        if b_num.deg() > a_den.deg() {
            // 余剰は無視（厳密には除算してDへ回すべきだが、SISOの典型では真にプロパー）
            let (_q, r) = b_num.div_rem(&a_den);
            b_num = r;
        }
        // 同じ次数に合わせる
        let mut b_coeffs = b_num.coeffs.clone();
        b_coeffs.resize(n + 1, 0.0);
        // D は直接項 = b_n（s^nの係数）
        let d = b_coeffs[n];
        // 残りの係数を調整: b_tilde(s) = num(s) - d * den(s)
        let b_tilde = b_num - &(&a_den * d);
        // 次数は最大 n-1 に揃える
        let mut bt = b_tilde.coeffs.clone();
        bt.resize(n, 0.0);

        // A (n×n): コンパニオン行列（可制御正準形）
        let mut a = Matrix::zeros(n, n);
        for i in 0..n - 1 {
            a[(i, i + 1)] = 1.0;
        }
        for j in 0..n {
            // 最下行に -a_j （ただし monic なので最後の係数は 1）
            a[(n - 1, j)] = -a_coeffs[j];
        }

        // B (n×1): e_n
        let mut b = Matrix::zeros(n, 1);
        b[(n - 1, 0)] = 1.0;

        // C (1×n): b_tilde の係数
        let mut c = Matrix::zeros(1, n);
        for j in 0..n {
            c[(0, j)] = bt[j];
        }

        // D (1×1)
        let dmat = match Matrix::new(1, 1, vec![d]) {
            Ok(m) => m,
            Err(e) => panic!("1x1 D must construct: {e}"),
        };

        ContinuousStateSpace { a, b, c, d: dmat }
    }

    /// 簡易: 可制御正準形から TF を構成（SISO 前提）
    pub fn to_tf_siso(&self) -> RationalFunction<f64> {
        // 期待形（from_tf_sisoが作った形）を前提として逆変換
        // ここでは簡易に C(zI - A)^{-1}B + D の厳密導出は行わず、
        // controllable canonical の既知構造から多項式係数を復元する。
        // 最下行に -a_j が入っている前提
        let n = self.a.rows;
        assert!(
            self.a.rows == self.a.cols
                && self.b.cols == 1
                && self.c.rows == 1
                && self.d.rows == 1
                && self.d.cols == 1
        );
        let mut den = vec![0.0; n + 1];
        den[n] = 1.0; // monic
        for (j, v) in den.iter_mut().enumerate().take(n) {
            *v = -self.a[(n - 1, j)];
        }
        // C は b_tilde の係数が入っている前提（次数 n-1 まで）
        let mut num = vec![0.0; n + 1];
        for (j, v) in num.iter_mut().enumerate().take(n) {
            *v = self.c[(0, j)];
        }
        num[n] = self.d[(0, 0)];
        RationalFunction::new(Polynomial::new(num), Polynomial::new(den))
    }
}

// ---- Display ----
impl fmt::Display for ContinuousStateSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ContinuousStateSpace A:{}x{}, B:{}x{}, C:{}x{}, D:{}x{}",
            self.a.rows,
            self.a.cols,
            self.b.rows,
            self.b.cols,
            self.c.rows,
            self.c.cols,
            self.d.rows,
            self.d.cols
        )
    }
}

impl fmt::Display for DiscreteStateSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DiscreteStateSpace A:{}x{}, B:{}x{}, C:{}x{}, D:{}x{}",
            self.a.rows,
            self.a.cols,
            self.b.rows,
            self.b.cols,
            self.c.rows,
            self.c.cols,
            self.d.rows,
            self.d.cols
        )
    }
}

impl DiscreteStateSpace {
    /// 離散の状態空間から SISO 伝達関数へ（多項式比）
    pub fn to_tf_siso(&mut self) -> RationalFunction<f64> {
        // 厳密: Faddeev–LeVerrier で p(z)=det(zI-A)=z^n + c1 z^{n-1}+...+c_n を求め、
        // adj(zI-A)B の係数列 w_k を w_0=B, w_k = A w_{k-1} + c_k B で作り、
        // u(z) = Σ (C w_k) z^{n-1-k} として、最終 num(z) = u(z) + D*p(z) を構成する。
        use poly::rational_function::RationalFunction;
        use poly::Polynomial as Poly;

        let n = self.a.rows;
        assert!(
            self.a.rows == self.a.cols && self.b.cols == 1 && self.c.rows == 1 && self.d.rows == 1 && self.d.cols == 1,
            "SISO expected: A square, B(n×1), C(1×n), D(1×1)"
        );
        if n == 0 {
            let num = Poly::new(vec![self.d[(0, 0)]]);
            let den = Poly::new(vec![1.0]);
            return RationalFunction::new(num, den);
        }

        // 1) Faddeev–LeVerrier: 係数 c1..c_n を得る
        let a = &self.a;
        let mut coeffs: Vec<f64> = vec![0.0; n + 1];
        coeffs[0] = 1.0; // z^n の係数
        // B0 = 0, c0 = 1
        let mut b_k = Matrix::<f64>::zeros(n, n);
        let mut c_prev = 1.0;
        for k in 1..=n {
            // Bk = A (B_{k-1} + c_{k-1} I)
            let mut inner = b_k.clone();
            for i in 0..n {
                inner[(i, i)] += c_prev;
            }
            b_k = a * &inner;
            let trace = (0..n).map(|i| b_k[(i, i)]).sum::<f64>();
            let ck = -trace / (k as f64);
            coeffs[k] = ck;
            c_prev = ck;
        }

        // 2) w_k の生成（w_0..w_{n-1}）
        let mut w: Vec<Matrix<f64>> = Vec::with_capacity(n);
        w.push(self.b.clone());
        for k in 1..n {
            let mut next = &self.a * w.last().unwrap();
            // next += c_k * B
            let ck = coeffs[k];
            if ck != 0.0 {
                for i in 0..self.b.rows {
                    next[(i, 0)] += ck * self.b[(i, 0)];
                }
            }
            w.push(next);
        }

        // 3) u(z) 係数（長さ n）: u_k = C w_k, k=0..n-1 で z^{n-1-k}
        let mut num_coeffs: Vec<f64> = vec![0.0; n + 1]; // +1 は後で D*p(z) を加えるため
        for k in 0..n {
            let mut s = 0.0;
            for j in 0..self.c.cols {
                s += self.c[(0, j)] * w[k][(j, 0)];
            }
            // 位置: z^{n-1-k} なので index = k にせず、先頭側に割り当て
            // poly表現は coeffs[0] が定数項なので、後でひっくり返す必要あり。
            // ここでは最終的に Polynomial::new([a0,a1,...]) へ合わせるため、
            // 一旦逆順に格納して最後に回転する。
            num_coeffs[k] = s; // 仮置き
        }
        // 4) p(z) を構成（linalgの係数は [1, c1, ..., c_n] が z^n..）→ ポリ表現用に反転
        let mut den_coeffs: Vec<f64> = coeffs.clone();
        den_coeffs.reverse(); // 低次→高次（a0..a_n）

        // 5) u(z) の並びを z^{n-1}..z^0 → a0..a_{n-1} に反転
        num_coeffs.truncate(n); // n 要素
        num_coeffs.reverse(); // 低次→高次

        // 6) D * p(z) を足す
        let d00 = self.d[(0, 0)];
        if d00 != 0.0 {
            // p(z) は次数 n、u(z) は最大 n-1。num_total は次数 n まで。
            if num_coeffs.len() < den_coeffs.len() {
                num_coeffs.resize(den_coeffs.len(), 0.0);
            }
            for i in 0..den_coeffs.len() {
                num_coeffs[i] += d00 * den_coeffs[i];
            }
        } else {
            // 次数を合わせておく（n 次まで）
            if num_coeffs.len() < den_coeffs.len() {
                num_coeffs.resize(den_coeffs.len(), 0.0);
            }
        }

        let num = Poly::new(num_coeffs);
        let den = Poly::new(den_coeffs);
        RationalFunction::new(num, den)
    }
}

/// 連続伝達関数（SISO）から ZOH で厳密離散化し、離散の状態空間を返すショートカット
pub fn tf_c2d_zoh_siso(
    num: &Polynomial<f64>,
    den: &Polynomial<f64>,
    fs: f64,
) -> DiscreteStateSpace {
    let css = ContinuousStateSpace::from_tf_siso(num, den);
    css.c2d_zoh(fs)
}
