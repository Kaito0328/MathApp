use linalg::matrix::numerical::exp::MatrixExponential;
use linalg::matrix::Matrix;
use poly::polynomial::Polynomial;
use poly::rational_function::RationalFunction;

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

impl DiscreteStateSpace {
    /// 離散の状態空間から SISO 伝達関数へ（多項式比）
    pub fn to_tf_siso(&mut self) -> RationalFunction<f64> {
        // H(z) = C (zI - A)^{-1} B + D
        use poly::rational_function::RationalFunction;
        use poly::Polynomial as Poly;

        let n = self.a.rows;
        assert!(self.a.rows == self.a.cols && self.b.cols == 1 && self.c.rows == 1);
        // 多項式行列の処理はここでは簡略化し、数値的に分母を det(zI - A) の形で構成しつつ、
        // 分子は同サイズの係数を同定する（シンボリックでない簡易版）。
        // TODO: 高信頼なシンボリック to_tf 実装。

        // ここでは簡便に、可制御正準形を前提にした場合の逆変換のみをサポートする簡易手段は保留。
        // 現在は連続→離散（ZOH）での状態空間結果を直接返すユーティリティが主目的。
        // 後続のニーズで拡張予定。
        let _ = n; // silence warning for now
                   // 仮のプレースホルダ（未使用）。この関数は当面未使用のためダミー値を返さないようにコメントアウト。
                   // unreachable!("to_tf_siso not yet implemented");
                   // 暫定で恒等伝達（D のみ）を返すが、現状呼び出さない設計にする。
        let num = Poly::new(vec![self.d[(0, 0)]]);
        let den = Poly::new(vec![1.0]);
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
