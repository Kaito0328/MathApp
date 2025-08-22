use crate::continuous::TransferFunction as CTF;
use crate::discrete::TransferFunction as DTF;
use num_complex::Complex;
use poly::polynomial::Polynomial;
use poly::rational_function::RationalFunction as CRational;

fn poly_pow(mut base: Polynomial<f64>, mut exp: usize) -> Polynomial<f64> {
    let mut result = Polynomial::one();
    while exp > 0 {
        if exp & 1 == 1 {
            result = &result * &base;
        }
        if exp > 1 {
            base = &base * &base;
        }
        exp >>= 1;
    }
    result
}

// 双一次変換（一般化）: s = K * (1 - z^{-1})/(1 + z^{-1})（内部用）
fn bilinear_transform_with_k_polys(
    analog_b: &Polynomial<f64>,
    analog_a: &Polynomial<f64>,
    k: f64,
) -> (Polynomial<f64>, Polynomial<f64>) {
    use std::cmp::max;
    assert!(k.is_finite() && k > 0.0, "k must be positive");
    let n = max((analog_b.deg() + 1) as usize, (analog_a.deg() + 1) as usize);

    let x = Polynomial::new(vec![0.0, 1.0]); // x = z^{-1}
    let one = Polynomial::one();
    let base_plus = &one + &x; // (1 + x)
    let base_minus = &one - &x; // (1 - x)

    let mut bz = vec![Polynomial::zero(); n];
    let mut az = vec![Polynomial::zero(); n];

    let mut power_k = 1.0; // K^i
    for i in 0..n {
        let p = poly_pow(base_plus.clone(), n - 1 - i);
        let m = poly_pow(base_minus.clone(), i);
        let coeffs = &p * &m; // (1+x)^{n-1-i} (1-x)^i

        let b_i = analog_b.get(i);
        if b_i != 0.0 {
            bz[i] = &coeffs * (b_i * power_k);
        }
        let a_i = analog_a.get(i);
        if a_i != 0.0 {
            az[i] = &coeffs * (a_i * power_k);
        }
        power_k *= k;
    }

    let num = bz.iter().fold(Polynomial::zero(), |acc, p| &acc + p);
    let den = az.iter().fold(Polynomial::zero(), |acc, p| &acc + p);
    (num, den)
}

// 逆Tustin（離散→連続）。z^{-1} = (1 - s/K)/(1 + s/K)（内部用）
fn inverse_tustin_polys(
    discrete_b: &Polynomial<f64>,
    discrete_a: &Polynomial<f64>,
    k: f64,
) -> (Polynomial<f64>, Polynomial<f64>) {
    use std::cmp::max;
    assert!(k.is_finite() && k > 0.0);
    let n = max(
        (discrete_b.deg() + 1) as usize,
        (discrete_a.deg() + 1) as usize,
    );

    let s = Polynomial::new(vec![0.0, 1.0]); // s
    let one = Polynomial::one();
    let t = &s / k; // s/K
    let plus = &one + &t; // (1 + s/K)
    let minus = &one - &t; // (1 - s/K)

    let mut bn = vec![Polynomial::zero(); n];
    let mut an = vec![Polynomial::zero(); n];
    for i in 0..n {
        let p = poly_pow(plus.clone(), n - 1 - i);
        let m = poly_pow(minus.clone(), i);
        let basis = &p * &m; // (1 + s/K)^{n-1-i} (1 - s/K)^i
        let b_i = discrete_b.get(i);
        if b_i != 0.0 {
            bn[i] = &basis * b_i;
        }
        let a_i = discrete_a.get(i);
        if a_i != 0.0 {
            an[i] = &basis * a_i;
        }
    }
    let num = bn.iter().fold(Polynomial::zero(), |acc, p| &acc + p);
    let den = an.iter().fold(Polynomial::zero(), |acc, p| &acc + p);
    (num, den)
}

// === Public TF-based API ===

/// 双一次（一般化・K指定）: s = K * (1 - z^{-1})/(1 + z^{-1})
/// 返す離散TFには fs を保持させます（fs はメタ情報としてセット）。
pub fn bilinear_transform_with_k(ctf: &CTF, k: f64, fs: f64) -> DTF {
    assert!(k.is_finite() && k > 0.0);
    assert!(fs > 0.0);
    let (b, a) = bilinear_transform_with_k_polys(&ctf.ratio.numerator, &ctf.ratio.denominator, k);
    DTF::new_with_fs(b, a, fs)
}

/// 双一次（Tustin）: 連続TF → 離散TF（fsを保持）
pub fn bilinear_transform(ctf: &CTF, fs: f64) -> DTF {
    // ユーザー提案に合わせ、K=2fs の一般化Tustinを用いる
    bilinear_transform_with_k(ctf, 2.0 * fs, fs)
}

/// プリワープ付き双一次: 連続TF → 離散TF
pub fn bilinear_transform_prewarp(ctf: &CTF, fs: f64, f_warp_hz: f64) -> DTF {
    use std::f64::consts::PI;
    assert!(fs > 0.0 && f_warp_hz >= 0.0);
    let k = if f_warp_hz == 0.0 {
        2.0 * fs
    } else {
        let omega_a = 2.0 * PI * f_warp_hz; // rad/s
        let omega_d = 2.0 * PI * f_warp_hz / fs; // rad/sample
        omega_a / (omega_d / 2.0).tan()
    };
    bilinear_transform_with_k(ctf, k, fs)
}

/// 逆Tustin: 離散TF → 連続TF（k を与える）
pub fn inverse_tustin(dtf: &DTF, k: f64) -> CTF {
    let (b, a) = inverse_tustin_polys(&dtf.ratio.numerator, &dtf.ratio.denominator, k);
    CTF::new(b, a)
}

/// 逆Tustin（Hz 指定プリワープ定数）
pub fn inverse_tustin_prewarp(dtf: &DTF, fs: f64, f_warp_hz: f64) -> CTF {
    use std::f64::consts::PI;
    assert!(fs > 0.0 && f_warp_hz >= 0.0);
    let k = if f_warp_hz == 0.0 {
        2.0 * fs
    } else {
        let omega_a = 2.0 * PI * f_warp_hz; // rad/s
        let omega_d = 2.0 * PI * f_warp_hz / fs; // rad/sample
        omega_a / (omega_d / 2.0).tan()
    };
    inverse_tustin(dtf, k)
}

/// インパルス不変法（Impulse Invariance / Matched-Z）: 連続TF → 離散TF（fsを保持）
/// 前提: 真にプロパー（deg(num) <= deg(den)）。定数項の多項式部分は直接項として扱います。
pub fn impulse_invariant(ctf: &CTF, fs: f64) -> DTF {
    assert!(fs > 0.0);
    let ts = 1.0 / fs;

    // 部分分数分解（複素極・重極に対応）
    let pfe = ctf.ratio.partial_fraction_expansion();

    // 多項式部分（定数項のみ許容）: c0 は直接項として B に加算するため後で使用
    let poly_part = pfe.polynomial_part;
    assert!(
        poly_part.deg() <= 0,
        "impulse_invariant requires strictly proper or at most constant polynomial part"
    );
    let c0 = poly_part.get(0);

    // 変数は r = z^{-1}。複素多項式で合成し、最後に実数化する。
    type PolyC = Polynomial<Complex<f64>>;

    // 極ごとの (1 - alpha_i r)^{m_i} を構築し、共通分母 A(r) を作る
    let mut factors: Vec<PolyC> = Vec::new();
    let mut alphas: Vec<Complex<f64>> = Vec::new();
    let mut multiplicities: Vec<usize> = Vec::new();
    for term in &pfe.pole_terms {
        let p = term.pole; // Complex<f64>
        let m = term.coefficients.len();
        let alpha = (p * Complex::new(ts, 0.0)).exp();
        // (1 - alpha r)^m = sum_{k=0..m} C(m,k) (-alpha)^k r^k
        let one = Complex::new(1.0, 0.0);
        let mut factor = PolyC::new(vec![one]);
        let lin = PolyC::new(vec![one, -alpha]); // 1 - alpha r
        for _ in 0..m {
            factor = &factor * &lin;
        }
        factors.push(factor);
        alphas.push(alpha);
        multiplicities.push(m);
    }
    // A(r) = Π_i (1 - alpha_i r)^{m_i}
    let mut a_c = PolyC::one();
    for f in &factors {
        a_c = &a_c * f;
    }

    // B(r) 複素を構築
    let mut b_c = PolyC::zero();
    for (i, term) in pfe.pole_terms.iter().enumerate() {
        let m_i = multiplicities[i];
        let alpha_i = alphas[i];

        // other = A / factor_i （積で再計算）
        let mut other = PolyC::one();
        for (j, f) in factors.iter().enumerate() {
            if j != i {
                other = &other * f;
            }
        }

        // S_k(q) を q で構築するための基底
        // S_0(q) = 1 / (1 - q), S_{k+1}(q) = q * d/dq S_k(q)
        // ここでは個々の j に対して都度生成する
        for j in 1..=m_i {
            let c_j = term.coefficients[j - 1]; // Complex residue for (s - p)^{-j}

            // S_{j-1}(q) を構築（有理関数）
            let s_num = Polynomial::one();
            let s_den = Polynomial::one()
                - Polynomial::new(vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]); // 1 - q
            let mut s_rf = CRational::new(s_num.clone(), s_den.clone());
            for _k in 0..(j - 1) {
                // s_rf = q * d/dq s_rf
                let deriv = s_rf.differentiate();
                let q_poly = Polynomial::new(vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]);
                s_rf = CRational::new(deriv.numerator * &q_poly, deriv.denominator);
            }

            // 置換 q -> alpha_i * r （r = z^{-1}）
            let sub_num = poly_substitute_scale(&s_rf.numerator, alpha_i);
            let _sub_den = poly_substitute_scale(&s_rf.denominator, alpha_i);

            // スケール: T^{j-1} / (j-1)! * c_j
            let scale = c_j
                * Complex::new(
                    ts.powi((j - 1) as i32) / factorial((j - 1) as u32) as f64,
                    0.0,
                );
            let mut term_num = &sub_num * scale;

            // 共通分母 A に合わせる: × (1 - alpha_i r)^{m_i - j} × Π_{k≠i} (1 - alpha_k r)^{m_k}
            // sub_den = (1 - alpha_i r)^j になっているはず
            if m_i > j {
                let mut extra = PolyC::one();
                let lin = PolyC::new(vec![Complex::new(1.0, 0.0), -alpha_i]);
                for _ in 0..(m_i - j) {
                    extra = &extra * &lin;
                }
                term_num = &term_num * &extra;
            }
            term_num = &term_num * &other;

            // 加算
            b_c = &b_c + &term_num;
        }
    }

    // 定数多項式部分 c0 を反映（H(z) = c0 + ...）→ B += c0 * A
    if c0 != 0.0 {
        let c0c = Complex::new(c0, 0.0);
        b_c = &b_c + &(&a_c * c0c);
    }

    // 実数化（共役対のはずなので虚部は消える想定）。数値誤差を丸める。
    let b_real = poly_complex_to_real(&b_c);
    let a_real = poly_complex_to_real(&a_c);

    DTF::new_with_fs(b_real, a_real, fs)
}

// --- helpers (complex polynomial/rational utilities) ---

fn factorial(n: u32) -> u64 {
    (1..=n as u64).product::<u64>().max(1)
}

fn poly_substitute_scale(
    poly: &Polynomial<Complex<f64>>,
    alpha: Complex<f64>,
) -> Polynomial<Complex<f64>> {
    // 各係数 c_k に alpha^k を掛ける（q^k -> (alpha r)^k）
    let mut out: Vec<Complex<f64>> = Vec::with_capacity(poly.coeffs.len());
    let mut alpha_k = Complex::new(1.0, 0.0);
    for &ck in &poly.coeffs {
        out.push(ck * alpha_k);
        alpha_k *= alpha;
    }
    Polynomial::new(out)
}

fn poly_complex_to_real(pc: &Polynomial<Complex<f64>>) -> Polynomial<f64> {
    let tol = 1e-9;
    let mut out: Vec<f64> = Vec::with_capacity(pc.coeffs.len());
    for c in &pc.coeffs {
        assert!(
            c.im.abs() < tol,
            "expected real coefficients after pairing, got imag {}",
            c.im
        );
        out.push(c.re);
    }
    Polynomial::new(out)
}
